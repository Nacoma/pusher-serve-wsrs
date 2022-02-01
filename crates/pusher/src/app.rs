use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub secret: String,
}

impl App {
    pub fn new(name: String) -> App {
        App {
            id: thread_rng().gen::<u32>() as i64,
            name,
            key: generate_public_key(),
            secret: generate_secret_key(),
        }
    }
}

/// Generate a hex encoded secret key
fn generate_secret_key() -> String {
    hex::encode(thread_rng().gen::<[u8; 16]>().to_vec())
}

/// Generate a human readable public key
fn generate_public_key() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}
