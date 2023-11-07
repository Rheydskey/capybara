use std::time::SystemTime;

use bevy_app::Plugin;

pub struct Log;
impl Plugin for Log {
    fn build(&self, _: &mut bevy_app::App) {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}/{}:{}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                    record.file().unwrap_or("??"),
                    record.line().unwrap_or(0),
                    message
                ));
            })
            .level(log::LevelFilter::Trace)
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    }
}
