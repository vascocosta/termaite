mod config;

use anyhow::{Context, Result};
use config::Config;
use gemini_client_rs::{
    GeminiClient,
    types::{Content, ContentPart, GenerateContentRequest, PartResponse, Role},
};
use serde_json::json;
use std::{
    error::Error,
    io::{self, Write},
    mem,
};
use termimad::*;

const CONF_FILE: &str = ".termaite.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::from_file(
        dirs::home_dir()
            .ok_or("Could not find home dir.")?
            .join(CONF_FILE),
    )
    .context("Could not parse config file.")?;
    let client = GeminiClient::new(mem::take(&mut config.api_key));

    main_loop(&mut config, &client).await?;

    Ok(())
}

async fn main_loop(config: &mut Config, client: &GeminiClient) -> Result<(), Box<dyn Error>> {
    let profile = config
        .profiles
        .get_mut(&config.active_profile)
        .ok_or("Could not get profile.")?;

    let mut history: Vec<Content> = Vec::new();
    let mut input = String::new();

    let system_instruction = Content {
        parts: mem::take(&mut profile.system_prompt)
            .into_iter()
            .map(ContentPart::Text)
            .collect(),
        role: Role::System,
        // parts: profile
        //     .system_prompt
        //     .iter_mut()
        //     .map(|p| ContentPart::Text(mem::take(p)))
        //     .collect(),
        // role: Role::System,
    };

    let mut skin = MadSkin::default();
    skin.set_fg(termimad::crossterm::style::Color::Blue);

    while prompt(&mut input).unwrap_or_default() {
        history.push(Content {
            parts: vec![ContentPart::Text(mem::take(&mut input))],
            role: Role::User,
        });

        let req_json = json!(
            {
                "system_instruction": system_instruction,
                "contents": history,
            }
        );

        let request: GenerateContentRequest =
            serde_json::from_value(req_json).expect("Invalid JSON");

        let response = client
            .generate_content(&profile.model_name, &request)
            .await?;

        if let Some(candidates) = response.candidates {
            for candidate in candidates {
                for part in candidate.content.parts {
                    if let PartResponse::Text(text) = part {
                        println!("{}", skin.term_text(&text));
                        history.push(Content {
                            parts: vec![ContentPart::Text(text)],
                            role: Role::Model,
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

fn prompt(input: &mut String) -> Result<bool, Box<dyn Error>> {
    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(input)?;
    println!();

    if input.trim().eq_ignore_ascii_case("exit") {
        return Ok(false);
    }

    Ok(true)
}
