use hmac::{Hmac, Mac, NewMac};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha2::Sha256;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Key {
    pub public: String,
    pub private: String,
}

#[derive(Debug)]
pub struct AuthError(pub &'static str);

type HmacSha256 = Hmac<Sha256>;

pub struct Credentials {
    signature: String,
    id: String,
    channel: String,
    channel_data: Option<String>,
}

impl Credentials {
    pub fn new(signature: Option<String>, id: String, channel: String, channel_data: Option<String>) -> Result<Self, AuthError> {
        Ok(Self {
            signature: signature.ok_or(AuthError("missing signature"))?,
            id,
            channel,
            channel_data,
        })
    }

    pub fn parts(&self) -> Vec<String> {
        if let Some(channel_data) = self.channel_data.clone() {
            vec![self.id.clone(), self.channel.clone(), channel_data.clone()]
        } else {
            vec![self.id.clone(), self.channel.clone()]
        }
    }
}

impl Key {
    pub fn generate() -> Key {
        Key {
            public: generate_public_key(),
            private: generate_secret_key(),
        }
    }

    pub fn validate(&self, credentials: &Credentials) -> Result<(), AuthError> {
        let sig_components = credentials.signature.split(':').collect::<Vec<&str>>();

        if sig_components.len() != 2 || sig_components[0] != self.public {
            return Err(AuthError("invalid signature"));
        };

        let decoded_signature = hex::decode(sig_components[1].as_bytes()).unwrap_or_default();

        let message = credentials.parts()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(":");

        match HmacSha256::new_from_slice(self.private.as_bytes()) {
            Ok(mut mac) => {
                mac.update(message.as_bytes());

                if mac.verify(decoded_signature.as_slice()).is_ok() {
                    Ok(())
                } else {
                    Err(AuthError("failed to verify mac"))
                }
            }
            Err(_e) => Err(AuthError("failed to create hash")),
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

#[cfg(test)]
mod tests {
    use crate::{Credentials, Key};

    #[test]
    fn create_new_key() {
        let key = Key::generate();

        assert_eq!(24, key.public.len());
        assert!(!key.private.is_empty());
    }

    #[test]
    fn validates_signatures() {
        let key = Key {
            public: "278d425bdf160c739803".to_string(),
            private: "7ad3773142a6692b25b8".to_string(),
        };

        let signature =
            "278d425bdf160c739803:58df8b0c36d6982b82c3ecf6b4662e34fe8c25bba48f5369f135bf843651c3a4"
                .to_string();

        let credentials = Credentials::new(
            Some(signature),
            "1234.1234".to_string(),
            "private-foobar".to_string(),
                None,
        ).unwrap();

        let res = key.validate(&credentials);

        assert!(res.is_ok());
    }
}
