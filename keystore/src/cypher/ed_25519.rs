//! generate seed,secret_key,public_key

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::option::Option::Some;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
// use generic_array::GenericArray;
use sha2::{Digest, Sha512};

#[derive(Debug, Clone)]
pub struct Secret {
    pub seed: Option<Vec<u8>>,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
}

impl Secret {
    pub async fn gen() -> Self {
        let mut secret = Secret {
            seed: None,
            secret_key: None,
            public_key: None,
        };

        let rand32 = mw_std::rand::gen_rand32().await;

        let mut hasher = Sha512::new();
        hasher.update(rand32.as_slice());
        let rand64 = hasher.finalize();

        let result_secret_key = SecretKey::from_bytes(&rand64.as_slice());

        if result_secret_key.as_ref().is_err() {
            return secret;
        }

        let secret_key: SecretKey = result_secret_key.unwrap();

        let public_key: PublicKey = (&secret_key).into();

        secret.seed = Some(rand32);
        secret.public_key = Some(public_key.to_bytes().to_vec());
        secret.secret_key = Some(secret_key.to_bytes().to_vec());

        secret
    }
}

pub fn sign(public_key: Vec<u8>, secret_key: Vec<u8>, data: &[u8]) -> Option<Vec<u8>> {
    let publickey_result = PublicKey::from_bytes(public_key.as_slice());
    let secretkey_result = SecretKey::from_bytes(secret_key.as_slice());

    if publickey_result.as_ref().is_err() || secretkey_result.as_ref().is_err() {
        return None;
    }

    let keypair = ed25519_dalek::Keypair {
        public: PublicKey::from_bytes(public_key.as_slice()).unwrap(),
        secret: SecretKey::from_bytes(secret_key.as_slice()).unwrap(),
    };

    let mut prehashed = Sha512::new();
    prehashed.update(data);

    let sig = keypair.sign_prehashed(prehashed, None).unwrap();

    Some(sig.to_bytes().to_vec())
}

pub fn verify_sign(public_key: &[u8], sign: &[u8], data: &[u8]) -> Option<bool> {
    let publickey_result = PublicKey::from_bytes(public_key);

    if publickey_result.as_ref().is_err() {
        return None;
    }

    let publickey = publickey_result.unwrap();

    let mut prehashed_again: Sha512 = Sha512::default();
    prehashed_again.update(data);

    let sig_result = Signature::try_from(sign);

    if sig_result.as_ref().is_err() {
        return None;
    }

    let sig = sig_result.unwrap();

    let verified = publickey.verify_prehashed(prehashed_again, None, &sig);

    if verified.is_err() {
        return Some(false);
    }

    Some(true)
}
