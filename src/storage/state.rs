use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageState {
    pub url: String,
    pub last_hash: String,
    pub last_checked_at: String,
    pub known_attachments: Vec<String>,
    pub last_cahier_text: String,
    pub last_devoir_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub pages: HashMap<String, PageState>,
}

impl AppState {
    pub fn load_from_file(path_str: &str) -> Result<Self> {
        let path = Path::new(path_str);
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read state file at {}", path_str))?;

        let state: AppState = serde_json::from_str(&content)
            .with_context(|| format!("Failed to deserialize state from {}", path_str))?;

        Ok(state)
    }

    pub fn save_to_file(&self, path_str: &str) -> Result<()> {
        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize application state")?;

        if let Some(parent) = Path::new(path_str).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create state directory {:?}", parent))?;
            }
        }

        fs::write(path_str, content)
            .with_context(|| format!("Failed to write state file to {}", path_str))?;

        Ok(())
    }

    pub fn get_page(&self, url: &str) -> Option<&PageState> {
        self.pages.get(url)
    }

    pub fn update_page(
        &mut self,
        url: &str,
        hash: String,
        attachments: Vec<String>,
        cahier: String,
        devoir: String,
    ) {
        let page_state = PageState {
            url: url.to_string(),
            last_hash: hash,
            last_checked_at: Utc::now().to_rfc3339(),
            known_attachments: attachments,
            last_cahier_text: cahier,
            last_devoir_text: devoir,
        };
        self.pages.insert(url.to_string(), page_state);
    }
}
