use crate::errors::ParseError;
use anyhow::Result;
use std::str::FromStr;

pub enum Command {
    Exit,
    Help(Option<String>),
    Profile { name: String },
    Prompt,
    Set { option: String, value: String },
}

impl Command {
    pub fn help(arg: Option<String>) -> Vec<String> {
        vec![
            "exit/quit - exit this program".to_string(),
            "help - show this help message".to_string(),
            "profile [name] - list or change profiles".to_string(),
            "set <option> <value> - change different settings".to_string(),
        ]
        .into_iter()
        .filter(|e| arg.is_none() || arg.as_ref().is_some_and(|arg| e.contains(arg)))
        .collect()
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
            "help" => Ok(Command::Help(tokens.next().map(|t| t.to_ascii_lowercase()))),
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
