mod command;
#[cfg(debug_assertions)]
mod debug;
mod more_itertools;

use clap::Parser;
use command::GrassCommand;

fn main() {
    let grass_command = GrassCommand::parse();

    grass_command.handle();
}
