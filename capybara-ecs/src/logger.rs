use std::time::Instant;

use bevy_app::{Plugin, PostUpdate, PreUpdate};
use bevy_ecs::system::{Res, ResMut, Resource};
use tracing::Level;

pub struct Log;
impl Plugin for Log {
    fn build(&self, _: &mut bevy_app::App) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        // use that subscriber to process traces emitted after this point
        tracing::subscriber::set_global_default(subscriber).unwrap();

        tracing_log::log_tracer::Builder::new()
            .with_max_level(log::LevelFilter::Info)
            .init()
            .unwrap();
    }
}

#[derive(Resource)]
pub struct LastTick(pub Instant);

pub struct TickTime;

impl Plugin for TickTime {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(PreUpdate, preupdate)
            .add_systems(PostUpdate, postupdate)
            .insert_resource(LastTick(Instant::now()));
    }
}

pub fn preupdate(mut last_tick: ResMut<LastTick>) {
    last_tick.0 = Instant::now();
}

pub fn postupdate(last_tick: Res<LastTick>) {
    info!("Take {}ms", last_tick.0.elapsed().as_millis());
}
