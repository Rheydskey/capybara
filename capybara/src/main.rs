mod network;
mod state;

#[cfg(test)]
mod tests;

use capybara_packet::types::State;
use log::{Level, LevelFilter};
use pretty_env_logger::env_logger::fmt::{Color, Style, StyledValue};
use std::{io::Write, sync::Arc};
use tokio::net::{TcpListener, TcpStream};

use crate::network::ClientConnection;

#[macro_use]
extern crate log;

async fn handle_stream(stream: TcpStream, state: Arc<State>) {
    let mut client = ClientConnection::new(stream, state);

    client.handle().await;
}

fn init_log() {
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
        .init();
}

fn colored_level(style: &mut Style, level: Level) -> StyledValue<'_, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO"),
        Level::Warn => style.set_color(Color::Yellow).value("WARN"),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

#[tokio::main]
async fn main() {
    init_log();
    let state = Arc::new(State::new());
    info!("{state:?}");
    let socket = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    loop {
        match socket.accept().await {
            Ok(stream) => handle_stream(stream.0, state.clone()).await,
            Err(err) => error!("{err:?}"),
        }
    }
}
