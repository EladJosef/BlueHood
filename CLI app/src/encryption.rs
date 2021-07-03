use base64;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::str;

pub fn encrypt_file_data(file_data: Vec<u8>, key: &str) -> Vec<u8> {
    new_magic_crypt!(key, 256)
        .encrypt_str_to_base64(base64::encode(&file_data))
        .into_bytes()
}

pub fn decrypt_data(file_data: Vec<u8>, key: &str) -> Vec<u8> {
    base64::decode(
        new_magic_crypt!(key, 256)
            .decrypt_base64_to_string(str::from_utf8(&file_data).unwrap())
            .unwrap(),
    )
    .unwrap()
}
