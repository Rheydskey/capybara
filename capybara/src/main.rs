mod network;
mod player;
mod state;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    capybara_ecs::init();
}
