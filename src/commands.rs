use anyhow::Result;
use std::{error::Error, str::FromStr};

pub enum Command {
    Exit,
    Help,
    Profile { name: String },
    Prompt,
}

impl Command {
    pub fn help() -> Vec<String> {
        vec![
            "exit".to_string(),
            "help".to_string(),
            "profile [name]".to_string(),
        ]
    }
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let command = tokens.next().ok_or(ParseError::CommandNotFound)?;

        match command {
            "exit" => {
                if tokens.next().is_some() {
                    return Err(ParseError::BadArgument);
                }

                Ok(Command::Exit)
            }
            "help" => Ok(Command::Help),
            "profile" => {
                let arg = tokens.next().unwrap_or("list");

                if tokens.next().is_some() {
                    return Err(ParseError::BadArgument);
                }

                Ok(Command::Profile {
                    name: arg.to_string(),
                })
            }
            _ => Ok(Command::Prompt),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    CommandNotFound,
    BadArgument,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error.")
    }
}

impl Error for ParseError {}
