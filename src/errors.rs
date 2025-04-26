use serde::Deserialize;
use std::error::Error;

/// This represents a command parsing error.
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

/// This represents an API error.
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub status: String,
    pub details: Option<Vec<ErrorDetail>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
#[allow(dead_code)]
pub enum ErrorDetail {
    #[serde(rename = "type.googleapis.com/google.rpc.ErrorInfo")]
    ErrorInfo {
        reason: String,
        domain: String,
        metadata: std::collections::HashMap<String, String>,
    },
    #[serde(rename = "type.googleapis.com/google.rpc.LocalizedMessage")]
    LocalizedMessage { locale: String, message: String },
}
