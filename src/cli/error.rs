use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CliError {
    Error { message: String },
    NotSupported { reason: Box<str> },
}

impl CliError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        CliError::Error {
            message: message.into(),
        }
    }

    pub fn not_supported(reason: &str) -> Self {
        CliError::NotSupported {
            reason: reason.into(),
        }
    }
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Error { message } => write!(f, "CLI Error:\n  {}", message,),
            CliError::NotSupported { reason } => {
                write!(f, "Operation not supported because:\n  {}", reason,)
            }
        }
    }
}
