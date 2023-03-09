use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shells {
    Bash,
}

#[derive(Parser, Debug)]
pub struct ShellInsertCommand {
    shell: Shells
}

impl ShellInsertCommand {
    pub fn handle(&self) {
        //TODO: Handle shells
    }
}
