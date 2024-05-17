mod config;
mod connection;
mod events;
mod logger;
mod player;
mod server;

#[cfg(test)]
mod tests;

use crate::logger::Log;

use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::TaskPoolPlugin;

use config::GlobalServerConfig;
// use logger::TickTime;
use server::ServerPlugin;
use std::time::Duration;

#[macro_use]
extern crate log;

pub fn init() -> anyhow::Result<()> {
    let config = GlobalServerConfig::from_file_or_create("./config.toml")?;
    App::new()
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(Log)
        // .add_plugins(TickTime)
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1. / 200.,
        )))
        .add_plugins(ServerPlugin)
        .insert_resource(config)
        .run();

    Ok(())
}
