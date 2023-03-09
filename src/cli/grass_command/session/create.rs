use clap::Parser;

#[derive(Parser, Debug)]
pub struct CreateCommand {
    category: String,
    repository: String,
}

impl CreateCommand {
    pub fn handle(&self) {
    }
}
