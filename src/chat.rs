use crate::{commands::Command, config::Config};
use anyhow::{Context, Result};
use gemini_client_rs::{
    GeminiClient,
    types::{
        Content, ContentPart, GenerateContentRequest, GenerateContentResponse, PartResponse, Role,
    },
};
use serde_json::json;
use std::{
    io::{self, Write},
    mem,
};
use termimad::*;

pub(crate) struct ChatSession<'a> {
    config: &'a mut Config,
    client: GeminiClient,
    history: Vec<Content>,
    skin: MadSkin,
}

impl<'a> ChatSession<'a> {
    pub fn new(config: &'a mut Config, client: GeminiClient) -> Self {
        let mut skin = MadSkin::default();
        skin.set_fg(termimad::crossterm::style::Color::Blue);

        Self {
            config,
            client,
            history: Vec::new(),
            skin,
        }
    }

    pub async fn run(&mut self) -> Result<Command> {
        let mut input = String::new();

        let profile = self
            .config
            .profiles
            .get_mut(&self.config.active_profile)
            .context("Could not get profile.")?;
        let model_name = mem::take(&mut profile.model_name);

        let system_instruction = Content {
            parts: mem::take(&mut profile.system_prompt)
                .into_iter()
                .map(ContentPart::Text)
                .collect(),
            role: Role::System,
        };

        while self.prompt(&mut input).unwrap_or_default() {
            match input.parse() {
                Ok(Command::Exit) => return Ok(Command::Exit),
                Ok(Command::Help) => {
                    input.clear();

                    let usage = Command::help();

                    for line in &usage {
                        println!("--- {}", line)
                    }

                    continue;
                }
                Ok(Command::Profile { name }) => {
                    input.clear();

                    if name == "list" {
                        println!("--- List of profiles: ");

                        for profile_name in self.config.profiles.keys() {
                            println!("--- {}", profile_name);
                        }

                        continue;
                    } else {
                        return Ok(Command::Profile { name });
                    };
                }
                Ok(Command::Prompt) => (),
                Err(error) => {
                    input.clear();
                    println!("--- {}", error);
                    continue;
                }
            };

            self.history.push(Content {
                parts: vec![ContentPart::Text(mem::take(&mut input))],
                role: Role::User,
            });

            let req_json = json!(
                {
                    "system_instruction": system_instruction,
                    "contents": self.history,
                }
            );

            let request: GenerateContentRequest =
                serde_json::from_value(req_json).expect("Invalid JSON");
            let response = self.client.generate_content(&model_name, &request).await?;

            self.print_response(response);
        }

        Ok(Command::Prompt)
    }

    fn prompt(&self, input: &mut String) -> Result<bool> {
        print!(">>> ");
        io::stdout().flush()?;
        io::stdin().read_line(input)?;
        println!();

        Ok(true)
    }

    fn print_response(&mut self, response: GenerateContentResponse) {
        if let Some(candidates) = response.candidates {
            for candidate in candidates {
                for part in candidate.content.parts {
                    if let PartResponse::Text(text) = part {
                        println!("{}", self.skin.term_text(&text));
                        self.history.push(Content {
                            parts: vec![ContentPart::Text(text)],
                            role: Role::Model,
                        });
                    }
                }
            }
        }
    }
}
