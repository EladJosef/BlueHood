use base64;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn encrypt_file_data(file_data: Vec<u8>, key: &str) -> Vec<u8> {
    new_magic_crypt!(key, 256)
        .encrypt_str_to_base64(base64::encode(&file_data))
        .into_bytes()
}
