mod event;
mod logger;
mod parsing;
mod player;
mod server;

use crate::logger::Log;

use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::{Component, TaskPoolPlugin},
};

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
        .run();
}

#[derive(Debug, Component)]
struct PacketName(String);
