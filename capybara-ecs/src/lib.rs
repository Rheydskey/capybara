mod config;
mod event;
mod logger;
mod parsing;
mod player;
mod server;

use crate::logger::Log;

use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::TaskPoolPlugin,
};

use config::GlobalServerConfig;
use server::ServerPlugin;
use std::time::Duration;

#[macro_use]
extern crate log;

pub fn init() {
    App::new()
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(Log)
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1. / 200.,
        )))
        .add_plugins(ServerPlugin)
        .insert_resource(GlobalServerConfig::from_file_or_create("./config.toml"))
        .run();
}
