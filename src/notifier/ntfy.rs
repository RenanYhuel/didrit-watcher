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
        let mut msg = String::new();
        msg.push_str(&format!("**{}**\n\n", analysis.summary));

        if !analysis.homework_items.is_empty() {
            msg.push_str("### Travail a faire\n");
            for item in &analysis.homework_items {
                msg.push_str(&format!("* {}\n", item));
            }
            msg.push('\n');
        }

        if !analysis.new_documents.is_empty() {
            msg.push_str("### Documents / Chapitres\n");
            for doc in &analysis.new_documents {
                msg.push_str(&format!("* {}\n", doc));
            }
            msg.push('\n');
        }

        if !diff.new_attachments.is_empty() {
            msg.push_str("### Liens directs\n");
            for att in &diff.new_attachments {
                msg.push_str(&format!("* [{}]({})\n", att.text, att.url));
            }
        }

        let mut tags = vec!["school".to_string(), "books".to_string()];
        if analysis.priority >= 4 {
            tags.push("warning".to_string());
        }

        let title = if analysis.title.is_empty() {
            format!("Nouveau contenu : {}", page.page_title)
        } else {
            analysis.title.clone()
        };

        let mut payload = NtfyPayload::new(&self.topic, &title, &msg)
            .with_priority(analysis.priority)
            .with_tags(tags)
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

        let mut req = self.client.post(&endpoint).json(payload);

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
