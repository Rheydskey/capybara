use bevy_app::Plugin;
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
