mod chat;
mod commands;
mod config;
mod errors;

use crate::{commands::Command, errors::ApiErrorResponse};
use anyhow::{Context, Result};
use chat::ChatSession;
use config::{CONF_FILE, Config};
use gemini_client_rs::{GeminiClient, GeminiError};
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
            Err(error) => match error.downcast() {
                Ok(GeminiError::Api(json)) => {
                    match serde_json::from_str::<ApiErrorResponse>(&json) {
                        Ok(api_error_resp) => {
                            eprintln!("--- {}", api_error_resp.error.message)
                        }
                        Err(error) => eprintln!("--- {}", error),
                    }
                }
                _ => eprintln!("--- Unknown error."),
            },
            _ => (),
        }
    }

    Ok(())
}
