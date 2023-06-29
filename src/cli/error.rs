use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CliError {
    message: String,
}

impl CliError {
    pub fn new<T: Into<String>>(
        message: T,
    ) -> Self {
        CliError {
            message: message.into(),
        }
    }
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CLI Error:\n  {}",
            self.message,
        )
    }
}
