use super::prompt::PromptBuilder;
use super::schema::{
    AiAnalysis, GeminiContent, GeminiGenerationConfig, GeminiPart, GeminiRequestBody,
    GeminiResponseBody,
};
use crate::diff::DiffResult;
use crate::parser::ParsedPage;
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiClient {
    pub fn new(api_key: String, model: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to build Gemini HTTP client")?;

        Ok(Self {
            client,
            api_key,
            model,
        })
    }

    pub async fn analyze_diff(&self, page: &ParsedPage, diff: &DiffResult) -> Result<AiAnalysis> {
        let prompt = PromptBuilder::build(page, diff);

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let body = GeminiRequestBody {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart { text: prompt }],
            }],
            generation_config: Some(GeminiGenerationConfig {
                response_mime_type: "application/json".to_string(),
            }),
        };

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error ({}): {}", status, error_text);
        }

        let resp_body: GeminiResponseBody = response
            .json()
            .await
            .context("Failed to parse Gemini API response body")?;

        let json_text = resp_body
            .candidates
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|c| c.content.as_ref())
            .and_then(|c| c.parts.as_ref())
            .and_then(|p| p.first())
            .and_then(|p| p.text.as_ref())
            .context("No text found in Gemini response candidate")?;

        let analysis: AiAnalysis = serde_json::from_str(json_text)
            .with_context(|| format!("Failed to parse Gemini JSON output: {}", json_text))?;

        Ok(analysis)
    }
}
