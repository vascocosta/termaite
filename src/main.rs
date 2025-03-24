mod chat;
mod commands;
mod config;

use crate::commands::Command;
use anyhow::{Context, Result};
use chat::ChatSession;
use config::CONF_FILE;
use config::Config;
use gemini_client_rs::GeminiClient;
use std::mem;

#[tokio::main]
async fn main() -> Result<()> {
    let mut active_profile: Option<String> = None;

    loop {
        let mut config = Config::from_file(
            dirs::home_dir()
                .context("Could not find home dir.")?
                .join(CONF_FILE),
        )
        .context("Could not parse config file.")?;

        if let Some(ref active_profile) = active_profile {
            config.active_profile = active_profile.to_owned();
        }

        let client = GeminiClient::new(mem::take(&mut config.api_key));
        let mut session = ChatSession::new(&mut config, client);

        match session.run().await {
            Ok(Command::Exit) => break,
            Ok(Command::Profile { name }) => {
                active_profile = Some(name);
                continue;
            }
            Err(error) => {
                eprintln!("{}", error);
                break;
            }
            _ => (),
        }
    }

    Ok(())
}
