use anyhow::Result;
use didrit_watcher::config::AppConfig;
use didrit_watcher::service::WatcherService;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;
    let mut service = WatcherService::new(config)?;
    service.start().await?;
    Ok(())
}
