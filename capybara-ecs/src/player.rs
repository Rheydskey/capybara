use std::sync::Arc;

use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use bevy::prelude::{Bundle, Component};
use capybara_packet::helper::PacketState;
use parking_lot::RwLock;

use crate::parsing::ParseTask;

#[derive(Debug, Component)]
pub struct Name(pub String);

#[derive(Debug, Component)]
pub struct Uuid(pub uuid::Uuid);

#[derive(Bundle)]
pub struct Player {
    pub event: ParseTask,
    pub player_status: PlayerStatus,
    pub encryption_state: EncryptionState,
    pub compression_state: CompressionState,
}

#[derive(Debug, Component)]
pub struct PlayerStatus(pub PacketState);

impl PlayerStatus {
    pub fn get_status(&self) -> PacketState {
        self.0.clone()
    }

    pub fn set_status(&mut self, state: PacketState) {
        self.0 = state;
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionLayer {
    pub encrypt: cfb8::Encryptor<aes::Aes128>,
    pub decrypt: cfb8::Decryptor<aes::Aes128>,
}

impl EncryptionLayer {
    pub fn new(shared_key: &[u8]) -> Self {
        Self {
            encrypt: cfb8::Encryptor::new(shared_key.into(), shared_key.into()),
            decrypt: cfb8::Decryptor::new(shared_key.into(), shared_key.into()),
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct EncryptionState(Arc<RwLock<Option<EncryptionLayer>>>);

impl EncryptionState {
    pub fn encrypt(&self, bytes: &mut [u8]) {
        let encryption = self.0.read().clone();
        if let Some(encryption) = encryption {
            info!("Encrypting....");
            encryption.encrypt.encrypt(bytes);
        }
    }

    pub fn decrypt(&self, bytes: &mut [u8]) {
        let encryption = self.0.read().clone();
        if let Some(encryption) = encryption {
            info!("Decrypting...");
            encryption.decrypt.decrypt(bytes);
        }
    }

    pub fn set_encryption(&self, encryption_layer: EncryptionLayer) {
        *self.0.write() = Some(encryption_layer)
    }
}

impl Default for EncryptionState {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(None)))
    }
}

#[derive(Debug, Component, Clone)]
pub struct CompressionState;

#[derive(Debug, Component, Clone)]
pub struct VerifyToken(pub Vec<u8>);

pub mod PlayerStatusMarker {
    use bevy::prelude::Component;

    #[derive(Debug, Component)]
    pub struct Handshaking;

    #[derive(Debug, Component)]
    pub struct Status;

    #[derive(Debug, Component)]
    pub struct Login;

    #[derive(Debug, Component)]
    pub struct Configuration;

    #[derive(Debug, Component)]
    pub struct Play;
}
