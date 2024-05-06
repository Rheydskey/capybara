use base64::Engine;
use bevy_ecs::prelude::Resource;

use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Deserialize, Serialize, Debug)]
pub struct NetworkConfig {
    pub rsa_public: String,
    pub rsa_private: String,
    pub port: u16,
}

impl NetworkConfig {
    pub fn get_privkey(&self) -> RsaPrivateKey {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&self.rsa_private)
            .unwrap();

        RsaPrivateKey::from_pkcs8_der(&bytes).unwrap()
    }

    pub fn generate() -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();

        let priv_key = RsaPrivateKey::new(&mut rng, 1024)?;

        let pub_key = RsaPublicKey::from(&priv_key);

        Ok(Self {
            rsa_public: base64::engine::general_purpose::STANDARD
                .encode(pub_key.to_public_key_der()?.as_bytes()),
            rsa_private: base64::engine::general_purpose::STANDARD
                .encode(priv_key.to_pkcs8_der()?.as_bytes()),
            port: 25565,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MotdConfig {
    motd: String,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct GlobalServerConfig {
    pub network_config: NetworkConfig,
    pub motd_config: MotdConfig,
}

impl GlobalServerConfig {
    pub fn from_file(filepath: &str) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(filepath)?;
        let mut string = String::new();

        file.read_to_string(&mut string)?;

        Ok(toml::from_str(&string)?)
    }

    pub fn from_file_or_create(filepath: &str) -> anyhow::Result<Self> {
        if let Ok(config) = Self::from_file(filepath) {
            return Ok(config);
        }

        let mut file = std::fs::File::create(filepath)?;

        let config = Self {
            network_config: NetworkConfig::generate()?,
            motd_config: MotdConfig {
                motd: "A beautiful rust minecraft server".to_string(),
            },
        };

        _ = file.write(toml::to_string(&config)?.as_bytes())?;

        Ok(config)
    }
}
