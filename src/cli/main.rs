mod external_command;
mod facades;
mod grass_command;
mod more_itertools;
mod output;

use clap::Parser;
use grass::dev::use_local_strategy_with_default_config;
use grass_command::GrassCommand;

fn main() {
    let grass_command = GrassCommand::parse();

    match use_local_strategy_with_default_config(|api| {
        Ok(grass_command.handle(&api, &external_command::get_external_commands()))
    }) {
        Err(error) => println!("Something went wrong!\n{}", error),
        Ok(Err(error)) => println!("Something went wrong!\n{}", error),
        _ => (),
    };
}
