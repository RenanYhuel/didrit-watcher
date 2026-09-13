use anyhow::{Context, Result};
use encoding_rs::{Encoding, UTF_8, WINDOWS_1252};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct HttpFetcher {
    client: Client,
}

impl HttpFetcher {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            ),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(20))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self { client })
    }

    pub async fn fetch_html(&self, url: &str) -> Result<String> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| format!("Failed to send GET request to {}", url))?;

        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("HTTP request to {} returned status {}", url, status);
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        let bytes = response
            .bytes()
            .await
            .with_context(|| format!("Failed to read response bytes from {}", url))?;

        let decoded = Self::decode_bytes(&bytes, &content_type);
        Ok(decoded)
    }

    pub fn decode_bytes(bytes: &[u8], content_type: &str) -> String {
        let encoding = if content_type.contains("iso-8859-1")
            || content_type.contains("windows-1252")
            || content_type.contains("latin1")
        {
            WINDOWS_1252
        } else {
            let (detected_encoding, _, had_errors) = UTF_8.decode(bytes);
            if !had_errors {
                return detected_encoding.into_owned();
            }
            Encoding::for_label(b"windows-1252").unwrap_or(WINDOWS_1252)
        };

        let (cow, _, _) = encoding.decode(bytes);
        cow.into_owned()
    }
}
