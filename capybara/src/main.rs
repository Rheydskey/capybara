mod network;
mod player;
mod state;

#[cfg(test)]
mod tests;

use capybara_packet::types::State;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

#[macro_use]
extern crate log;

async fn handle_stream(stream: TcpStream, state: Arc<State>) {
    println!("{:?}", player::Player::new(stream).handle(&state.rsa).await);
}

#[tokio::main]
async fn main() {
    capybara_ecs::init();

    /*let state = Arc::new(State::new());
    let socket = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    loop {
        match socket.accept().await {
            Ok(stream) => handle_stream(stream.0, state.clone()).await,
            Err(err) => error!("{err:?}"),
        }
    }*/
}
