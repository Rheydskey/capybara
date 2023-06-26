pub use log::{Level, LevelFilter};
use pretty_env_logger::env_logger::fmt::{Color, Style, StyledValue};
use std::io::Write;

pub fn init_log() {
    pretty_env_logger::formatted_builder()
        .format(|f, record| {
            let mut style = f.style();
            let level = colored_level(&mut style, record.level());
            writeln!(
                f,
                "{}:{} {} [ {} ] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                level,
                record.args()
            )
        })
        .filter(Some("capybara"), LevelFilter::Info)
        .filter(None, LevelFilter::Debug)
        .init();
}

pub fn colored_level(style: &mut Style, level: Level) -> StyledValue<'_, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO"),
        Level::Warn => style.set_color(Color::Yellow).value("WARN"),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}
