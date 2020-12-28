use alloc::vec::Vec;
use c2_chacha::stream_cipher::{NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek};
use c2_chacha::XChaCha20;
use generic_array::GenericArray;

pub fn xchacha20_encryption(key: &[u8], nonce: &[u8], data: &[u8]) -> Vec<u8> {
    let g_key = GenericArray::from_slice(key);
    // nonce是否需要保存
    let g_nonce = GenericArray::from_slice(nonce);
    let mut cipher = XChaCha20::new(g_key, g_nonce);
    //加密
    let mut data = data.to_vec();
    cipher.apply_keystream(&mut data);
    data.to_vec()
}

pub fn xchacha20_decryption(key: &[u8], nonce: &[u8], data: &[u8]) -> Vec<u8> {
    let g_key = GenericArray::from_slice(key);
    // nonce是否需要保存
    let g_nonce = GenericArray::from_slice(nonce);
    let mut cipher = XChaCha20::new(g_key, g_nonce);
    cipher.seek(0);
    let mut data = data.to_vec();
    cipher.apply_keystream(&mut data);
    data.to_vec()
}
