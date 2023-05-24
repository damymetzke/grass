mod grass_command;
mod more_itertools;
mod output;

use clap::Parser;
use grass_command::GrassCommand;

fn main() {
    let grass_command = GrassCommand::parse();

    grass_command.handle();
}
