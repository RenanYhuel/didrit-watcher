use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub gemini_api_key: String,
    pub gemini_model: String,
    pub ntfy_url: String,
    pub ntfy_topic: String,
    pub ntfy_token: Option<String>,
    pub target_urls: Vec<String>,
    pub check_interval_secs: u64,
    pub state_file_path: String,
    pub run_once: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let _ = dotenvy::dotenv();

        let gemini_api_key = env::var("GEMINI_API_KEY")
            .context("GEMINI_API_KEY must be set in environment or .env file")?;

        let gemini_model =
            env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-2.5-flash-lite".to_string());

        let ntfy_url = env::var("NTFY_URL").unwrap_or_else(|_| "https://ntfy.sh".to_string());

        let ntfy_topic = env::var("NTFY_TOPIC").unwrap_or_else(|_| "didrit".to_string());

        let ntfy_token = env::var("NTFY_TOKEN").ok().filter(|t| !t.trim().is_empty());

        let target_urls_raw = env::var("TARGET_URLS").unwrap_or_else(|_| {
            "https://www.didrit.fr/Maths_TE.htm,https://www.didrit.fr/Maths_T.htm".to_string()
        });

        let target_urls: Vec<String> = target_urls_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if target_urls.is_empty() {
            anyhow::bail!("TARGET_URLS must contain at least one valid URL");
        }

        let check_interval_secs = env::var("CHECK_INTERVAL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(3600);

        let state_file_path =
            env::var("STATE_FILE_PATH").unwrap_or_else(|_| "state.json".to_string());

        let run_once = env::var("RUN_ONCE")
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);

        Ok(Self {
            gemini_api_key,
            gemini_model,
            ntfy_url,
            ntfy_topic,
            ntfy_token,
            target_urls,
            check_interval_secs,
            state_file_path,
            run_once,
        })
    }
}
