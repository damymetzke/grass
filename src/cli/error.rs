use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CliError {
    message: String,
    command: Vec<String>,
}

impl CliError {
    pub fn new<T: Into<String>, U: IntoIterator<Item = V>, V: Into<String>>(
        message: T,
        command: U,
    ) -> Self {
        CliError {
            message: message.into(),
            command: command.into_iter().map(|value| value.into()).collect(),
        }
    }
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CLI Error:\n{}\n\nWhen running command:\n  {}",
            self.message,
            self.command.join(" ")
        )
    }
}
