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
use tracing::{info, error};
use tracing_subscriber::{filter, Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

fn setup_logging() -> Option<()> {
    let logging_directory = dirs::data_dir().map(|data_dir| data_dir.join("grass").join("logs"))?;

    fs::create_dir_all(&logging_directory).ok()?;

    let file = OpenOptions::new().create(true).append(true).open(logging_directory.join("log")).ok()?;

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file)
        .with_ansi(false)
        .with_filter(filter::LevelFilter::TRACE);

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(filter::LevelFilter::WARN);

    // TODO: Handle this more gracefully
    tracing_subscriber::registry().with(file_layer).with(stderr_layer).init();
    Some(())
}

fn main() {
    setup_logging();

    info!("GRAss CLI started");

    let grass_command = GrassCommand::parse();

    match use_local_strategy_with_default_config(|api| {
        Ok(grass_command.handle(&api, &external_command::get_external_commands()))
    }) {
        Err(error) => error!("Something went wrong!\n{}", error),
        Ok(Err(error)) => error!("Something went wrong!\n{}", error),
        _ => (),
    };
}
