use anyhow::Result;
use std::{error::Error, str::FromStr};

pub enum Command {
    Exit,
    Help,
    Profile { name: String },
    Prompt,
    Set { option: String, value: String },
}

impl Command {
    pub fn help() -> Vec<String> {
        vec![
            "exit/quit".to_string(),
            "help".to_string(),
            "profile [name]".to_string(),
            "set <option> <value>".to_string(),
        ]
    }
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let command = tokens.next().ok_or(ParseError::CommandNotFound)?;

        match command {
            "exit" | "quit" => {
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
            "set" => {
                let option = tokens.next().ok_or(ParseError::MissingArgument)?;
                let value = tokens.next().ok_or(ParseError::MissingArgument)?;

                Ok(Command::Set {
                    option: option.to_lowercase(),
                    value: value.to_lowercase(),
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
    MissingArgument,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Self::CommandNotFound => "command not found",
            Self::BadArgument => "bad argument",
            Self::MissingArgument => "missing argument",
        };
        write!(f, "Parse error: {}", kind)
    }
}

impl Error for ParseError {}
