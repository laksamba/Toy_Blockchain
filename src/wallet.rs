use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand_core::OsRng;
use hex;

pub struct Wallet{
    pub secret:SigningKey,
    pub public:VerifyingKey,
}

impl Wallet{
    pub fn new() -> Self{
        let mut rng = OsRng;
        let secret = SigningKey::generate(&mut rng);
        let public = VerifyingKey::from(&secret);
        Self{secret,public}
    }

    pub fn get_address(&self) -> String{
        hex::encode(self.public.as_bytes())
    }

    pub fn sign(&self, data:&[u8])->String{
        let signature = self.secret.sign(data);
        hex::encode(signature.to_bytes())
    }
}