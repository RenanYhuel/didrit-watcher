use crate::ai::GeminiClient;
use crate::config::AppConfig;
use crate::diff::DiffEngine;
use crate::fetcher::HttpFetcher;
use crate::notifier::NtfyNotifier;
use crate::parser::HtmlExtractor;
use crate::storage::AppState;
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::time::sleep;

pub struct WatcherService {
    config: AppConfig,
    fetcher: HttpFetcher,
    gemini: GeminiClient,
    notifier: NtfyNotifier,
    state: AppState,
}

impl WatcherService {
    pub fn new(config: AppConfig) -> Result<Self> {
        let fetcher = HttpFetcher::new()?;
        let gemini = GeminiClient::new(config.gemini_api_key.clone(), config.gemini_model.clone())?;
        let notifier = NtfyNotifier::new(
            config.ntfy_url.clone(),
            config.ntfy_topic.clone(),
            config.ntfy_token.clone(),
        )?;
        let state = AppState::load_from_file(&config.state_file_path)?;

        Ok(Self {
            config,
            fetcher,
            gemini,
            notifier,
            state,
        })
    }

    pub async fn run_cycle(&mut self) -> Result<()> {
        let urls = self.config.target_urls.clone();
        for url in &urls {
            let html = match self.fetcher.fetch_html(url).await {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error fetching {}: {:?}", url, e);
                    continue;
                }
            };

            let parsed = HtmlExtractor::parse(&html, url);
            let previous = self.state.get_page(url);
            let diff = DiffEngine::compute(previous, &parsed);

            if diff.is_first_run {
                println!("Initial state registered for {}", url);
                self.update_page_state(url, &parsed);
                continue;
            }

            if diff.has_changes {
                println!("Modifications detected on {}", url);
                let analysis = match self.gemini.analyze_diff(&parsed, &diff).await {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("Error during Gemini AI analysis: {:?}", e);
                        crate::ai::AiAnalysis {
                            title: format!("Mise a jour : {}", parsed.page_title),
                            fait_en_classe: diff.current_cahier_text.clone(),
                            a_faire: None,
                            date_echeance: None,
                            evaluation: diff.current_devoir_text.clone(),
                            nouveaux_documents: diff
                                .new_attachments
                                .iter()
                                .map(|a| a.text.clone())
                                .collect(),
                            priority: 3,
                        }
                    }
                };

                let payload = self.notifier.build_payload(&parsed, &diff, &analysis);
                if let Err(e) = self.notifier.send(&payload).await {
                    eprintln!("Error sending ntfy notification: {:?}", e);
                } else {
                    println!(
                        "Notification sent successfully to ntfy topic: {}",
                        self.config.ntfy_topic
                    );
                }

                self.update_page_state(url, &parsed);
            }
        }

        self.state
            .save_to_file(&self.config.state_file_path)
            .context("Failed to save application state")?;

        Ok(())
    }

    fn update_page_state(&mut self, url: &str, parsed: &crate::parser::ParsedPage) {
        let attachments = parsed
            .all_attachments
            .iter()
            .map(|a| a.url.clone())
            .collect();
        let cours_titles = parsed
            .cours_entries
            .iter()
            .map(|c| c.title.clone())
            .collect();
        let cahier = parsed
            .cahier_entries
            .iter()
            .map(|e| e.raw_text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let devoir = parsed
            .devoir_entries
            .iter()
            .map(|e| e.raw_text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        self.state.update_page(
            url,
            parsed.full_content_hash.clone(),
            attachments,
            cours_titles,
            cahier,
            devoir,
        );
    }

    pub async fn start(&mut self) -> Result<()> {
        if self.config.run_once {
            println!("Running in single-execution mode");
            return self.run_cycle().await;
        }

        println!(
            "Starting daemon loop with interval: {}s",
            self.config.check_interval_secs
        );

        loop {
            if let Err(e) = self.run_cycle().await {
                eprintln!("Error during check cycle: {:?}", e);
            }
            sleep(Duration::from_secs(self.config.check_interval_secs)).await;
        }
    }
}
