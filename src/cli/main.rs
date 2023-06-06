mod facades;
mod grass_command;
mod more_itertools;
mod output;

use clap::Parser;
use grass::dev::Api;
use grass_command::GrassCommand;

fn main() {
    let grass_command = GrassCommand::parse();

    Api::with_local_strategy(|api| {
        match grass_command.handle(&api) {
            Ok(_) => (),
            Err(error) => println!("Something went wrong!\n{}", error),
        };
    })
}
