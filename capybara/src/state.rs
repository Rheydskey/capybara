use rsa::RsaPrivateKey;

use crate::{network::ClientConnection, player::Player};

#[derive(Debug)]
pub struct State {
    connection: Vec<Player>,
    pub rsa: rsa::RsaPrivateKey,
}

impl State {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            connection: Vec::new(),
            rsa: RsaPrivateKey::new(&mut rng, 1024).unwrap(),
        }
    }
}
