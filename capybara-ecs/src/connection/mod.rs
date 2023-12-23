use std::sync::Arc;

use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use bevy_ecs::prelude::Component;
use parking_lot::RwLock;

pub mod parsing;

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
        let decryption = self.0.read().clone();
        if let Some(decryption) = decryption {
            info!("Decrypting...");
            decryption.decrypt.decrypt(bytes);
        }
    }

    pub fn set_encryption(&self, encryption_layer: EncryptionLayer) {
        *self.0.write() = Some(encryption_layer);
    }
}

impl Default for EncryptionState {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(None)))
    }
}

#[derive(Debug, Component, Clone)]
pub struct CompressionState;
