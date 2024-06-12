use std::sync::Arc;

use aes::cipher::{
    generic_array::GenericArray, BlockDecryptMut, BlockEncryptMut, BlockSizeUser, KeyIvInit,
};
use anyhow::anyhow;
use bevy_ecs::prelude::Component;
use parking_lot::RwLock;

pub mod parsing;

type Encryptor = cfb8::Encryptor<aes::Aes128>;
type Decryptor = cfb8::Decryptor<aes::Aes128>;

#[derive(Debug, Clone)]
pub struct EncryptionLayer {
    pub encrypt: Encryptor,
    pub decrypt: Decryptor,
}

impl EncryptionLayer {
    pub fn new(shared_key: &[u8]) -> Self {
        Self {
            encrypt: Encryptor::new_from_slices(shared_key, shared_key).unwrap(),
            decrypt: Decryptor::new_from_slices(shared_key, shared_key).unwrap(),
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct EncryptionState(Arc<RwLock<Option<EncryptionLayer>>>);

impl EncryptionState {
    pub fn encrypt(&self, bytes: &mut [u8]) {
        if let Some(encryption) = &mut *self.0.write() {
            let cipher = &mut encryption.encrypt;
            for chunk in bytes.chunks_mut(Encryptor::block_size()) {
                let gen_arr = GenericArray::from_mut_slice(chunk);
                cipher.encrypt_block_mut(gen_arr);
            }
        }
    }

    pub fn decrypt(&self, bytes: &mut [u8]) -> anyhow::Result<()> {
        if let Some(decryption) = &mut *self.0.write() {
            let cipher = &mut decryption.decrypt;
            for chunk in bytes.chunks_mut(Decryptor::block_size()) {
                let gen_arr = GenericArray::from_mut_slice(chunk);
                cipher.decrypt_block_mut(gen_arr);
            }

            return Ok(());
        }

        Err(anyhow!("No encryption"))
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
