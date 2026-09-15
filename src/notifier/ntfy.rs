use super::payload::NtfyPayload;
use crate::ai::AiAnalysis;
use crate::diff::DiffResult;
use crate::parser::ParsedPage;
use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct NtfyNotifier {
    client: Client,
    server_url: String,
    topic: String,
    token: Option<String>,
}

impl NtfyNotifier {
    pub fn new(server_url: String, topic: String, token: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .context("Failed to build ntfy HTTP client")?;

        Ok(Self {
            client,
            server_url,
            topic,
            token,
        })
    }

    pub fn build_payload(
        &self,
        page: &ParsedPage,
        diff: &DiffResult,
        analysis: &AiAnalysis,
    ) -> NtfyPayload {
        let mut sections = Vec::new();

        if let Some(fait) = &analysis.fait_en_classe {
            if !fait.trim().is_empty() {
                sections.push(format!("FAIT EN CLASSE :\n{}", fait.trim()));
            }
        }

        if let Some(a_faire) = &analysis.a_faire {
            if !a_faire.trim().is_empty() {
                let echeance = match &analysis.date_echeance {
                    Some(d) if !d.trim().is_empty() => format!(" (pour le {})", d.trim()),
                    _ => String::new(),
                };
                sections.push(format!("A FAIRE{} :\n{}", echeance, a_faire.trim()));
            }
        }

        if let Some(eval) = &analysis.evaluation {
            if !eval.trim().is_empty() {
                sections.push(format!("EVALUATION :\n{}", eval.trim()));
            }
        }

        if !diff.new_attachments.is_empty() {
            let mut docs_lines = Vec::new();
            for att in &diff.new_attachments {
                docs_lines.push(format!("- {} : {}", att.text, att.url));
            }
            sections.push(format!("NOUVEAUX DOCUMENTS :\n{}", docs_lines.join("\n")));
        }

        let msg = if sections.is_empty() {
            format!("Mise a jour detectee sur {}", page.page_title)
        } else {
            sections.join("\n\n")
        };

        let title = if analysis.title.is_empty() {
            page.page_title.clone()
        } else {
            analysis.title.clone()
        };

        let mut payload = NtfyPayload::new(&self.topic, &title, &msg)
            .with_priority(analysis.priority)
            .with_click(&page.source_url)
            .add_action("Ouvrir site", &page.source_url);

        if let Some(first_att) = diff.new_attachments.first() {
            payload = payload
                .with_attachment(&first_att.url, Some(&first_att.text))
                .add_action(&format!("PDF: {}", first_att.text), &first_att.url);
        }

        if let Some(second_att) = diff.new_attachments.get(1) {
            payload = payload.add_action(&format!("PDF 2: {}", second_att.text), &second_att.url);
        }

        payload
    }

    pub async fn send(&self, payload: &NtfyPayload) -> Result<()> {
        let endpoint = format!("{}/", self.server_url.trim_end_matches('/'));

        let mut req = self
            .client
            .post(&endpoint)
            .header("Markdown", "yes")
            .json(payload);

        if let Some(tok) = &self.token {
            let mut headers = HeaderMap::new();
            if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", tok)) {
                headers.insert(AUTHORIZATION, val);
                req = req.headers(headers);
            }
        }

        let response = req
            .send()
            .await
            .context("Failed to send notification request to ntfy server")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("ntfy server returned error ({}): {}", status, body);
        }

        Ok(())
    }
}
