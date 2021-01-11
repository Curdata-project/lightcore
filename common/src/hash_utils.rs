use alloc::vec::Vec;
use sha2::{Digest, Sha256, Sha512};

pub fn gen_hash_32_id(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);

    let hash64 = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(hash64);
    let hash32 = hasher.finalize();
    let s = hash32.as_slice();
    s.to_vec()
}
