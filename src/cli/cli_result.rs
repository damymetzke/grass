use anyhow::Error;

/// Output for the CLI program
///
/// The primary output of the application is the terminal.
/// It is important that the application avoids side-effects as much as possible.
/// Because of that, this output is used to move those effects to the root of the crate.
///
/// This is meant only for structured output.
/// That is, output meant for display but not for warnings.
/// Warning and errors are done throug the logging framework.
/// This usage is acceptable as the logging framework handles a lot more context.
///
/// In the future this will likely be expanded to include asynchronous output.
/// The exact form has not yet been decided.
#[non_exhaustive]
pub enum CliOutput {
    /// Don't output anything
    None,
    /// Output to stdout, usually meant for output that should be read by a script
    Stdout(Box<str>),
    /// Output to stderr, usually meant for output to be read by a human
    Stderr(Box<str>),
}

/// Alias for a CliResult
pub type CliResult = Result<CliOutput, Error>;

impl CliOutput {
    pub fn append_newline(self) -> Self {
        match self {
            CliOutput::None => CliOutput::None,
            CliOutput::Stdout(content) => CliOutput::Stdout(format!("{}\n", content).into()),
            CliOutput::Stderr(content) => CliOutput::Stderr(format!("{}\n", content).into()),
        }
    }
}
