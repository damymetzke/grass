mod cli_result;
mod error;
mod external_command;
mod facades;
mod grass_command;
mod more_itertools;
mod output;

use std::{
    fs::{self, OpenOptions},
    io,
};

use clap::Parser;
use grass::dev::use_local_strategy_with_default_config;
use grass_command::GrassCommand;
use tracing::{error, info};
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};

use crate::{grass_command::Verbosity, cli_result::CliOutput};

fn setup_logging(output_level: filter::LevelFilter) -> Option<()> {
    let logging_directory = dirs::data_dir().map(|data_dir| data_dir.join("grass").join("logs"))?;

    fs::create_dir_all(&logging_directory).ok()?;

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(logging_directory.join("log"))
        .ok()?;

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file)
        .with_ansi(false)
        .with_filter(filter::LevelFilter::TRACE);

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(output_level);

    // TODO: Handle this more gracefully
    tracing_subscriber::registry()
        .with(file_layer)
        .with(stderr_layer)
        .init();
    Some(())
}

fn main() {
    let grass_command = GrassCommand::parse();
    let level = match grass_command.get_verbosity() {
        Verbosity::Warn => filter::LevelFilter::WARN,
        Verbosity::Info => filter::LevelFilter::INFO,
        Verbosity::Debug => filter::LevelFilter::DEBUG,
        Verbosity::Trace => filter::LevelFilter::TRACE,
    };

    setup_logging(level);

    info!("GRAss CLI started");

    match use_local_strategy_with_default_config(|api| {
        Ok(grass_command.handle(&api, &external_command::get_external_commands()))
    }) {
        Err(error) => error!("Something went wrong!\n{}", error),
        Ok(Err(error)) => error!("Something went wrong!\n{}", error),
        Ok(Ok(output)) => {
                match output {
                    CliOutput::None => (),
                    CliOutput::Stdout(content) => print!("{}", content),
                    CliOutput::Stderr(content) => eprint!("{}", content),
                }
            }
    };
}
