mod configuration;
mod encryption;
mod handshake;
mod login;
mod ping;
mod registry;

use bevy_app::{Plugin, Update};
use bevy_ecs::schedule::IntoSystemConfigs;

pub struct PacketEventPlugin;

impl Plugin for PacketEventPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            Update,
            (
                ping::ping_handler,
                handshake::handshake_handler,
                handshake::status_handler,
                login::login_handler,
                encryption::response_encryption,
                login::login_ack,
                configuration::client_information,
                configuration::registry_data,
                configuration::finish_config,
            )
                .chain(),
        );
    }
}
