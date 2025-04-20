use crate::{
    commands::Command,
    config::{Color, Config},
};
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
        let fg_color = config.color.into();

        skin.set_fg(fg_color);

        Self {
            config,
            client,
            history: Vec::new(),
            skin,
        }
    }

    pub async fn run(&mut self) -> Result<Command> {
        let mut input = String::new();

        let profile = match self.config.profiles.get_mut(&self.config.active_profile) {
            Some(profile) => profile,
            None => {
                eprintln!("Profile not found. Loading default profile...");
                self.config
                    .profiles
                    .get_mut("default")
                    .context("Could not load any profile")?
            }
        };

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
                Ok(Command::Prompt) => (),
                Ok(command) => {
                    input.clear();

                    if let Some(command) = self.handle_command(command) {
                        return Ok(command);
                    } else {
                        continue;
                    }
                }
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

            input.clear();

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

    fn handle_command(&mut self, command: Command) -> Option<Command> {
        match command {
            Command::Exit => Some(Command::Exit),
            Command::Help => {
                println!("--- Help:");

                for line in &Command::help() {
                    println!("--- {}", line)
                }

                None
            }
            Command::Profile { name } => {
                if name == "list" {
                    println!("--- List of profiles: ");

                    for profile_name in self.config.profiles.keys() {
                        println!("--- {}", profile_name);
                    }

                    None
                } else {
                    Some(Command::Profile { name })
                }
            }
            Command::Prompt => None,
            Command::Set { option, value } => {
                let color = value.parse().unwrap_or(Color::Blue).into();

                match option.to_lowercase().as_ref() {
                    "fgcolor" => self.skin.set_fg(color),
                    "bgcolor" => self.skin.set_bg(color),
                    _ => eprintln!("--- Unknown option."),
                }

                None
            }
        }
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
