mod chat;
mod config;

use anyhow::{Context, Result};
use chat::ChatSession;
use config::CONF_FILE;
use config::Config;
use gemini_client_rs::GeminiClient;
use std::mem;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::from_file(
        dirs::home_dir()
            .context("Could not find home dir.")?
            .join(CONF_FILE),
    )
    .context("Could not parse config file.")?;

    let client = GeminiClient::new(mem::take(&mut config.api_key));
    let mut session = ChatSession::new(&mut config, client);

    session.run().await?;

    Ok(())
}
