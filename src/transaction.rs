use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, signature: String) -> Self {
        Self {
            sender,
            receiver,
            amount,
            signature,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        format!("{}{}{}", self.sender, self.receiver, self.amount).into_bytes()
    }

    pub fn verify(&self) -> bool {
        if self.sender == "Network" {
            return true;
        }

        let public_key_bytes: [u8; 32] = hex::decode(&self.sender)
            .unwrap_or_default()
            .try_into()
            .unwrap_or([0u8; 32]);
        let sig_bytes: [u8; 64] = hex::decode(&self.signature)
            .unwrap()
            .try_into()
            .unwrap_or([0u8; 64]);

        let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
            Ok(key) => key,
            Err(_) => return false,
        };

        let signature = Signature::from_bytes(&sig_bytes);
        public_key.verify(&self.to_bytes(), &signature).is_ok()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let short_sig = if self.signature.len() > 16 {
            &self.signature[..16]
        } else {
            &self.signature
        };
        write!(f,
                "From: {}\n   To:{} \n  Amount:{}\n  signature:{}....",
            self.sender,
            self.receiver,
            self.amount,
            short_sig,
        )
    }
}
