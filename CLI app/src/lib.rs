use std::env;
/// use for vector encryption
mod encryption;
/// use for file management
mod file;
/// used for WebAssembly compilation
use wasm_bindgen::prelude::*;

/// used for encrypt file with key and file path
#[wasm_bindgen]
pub fn encrypt_file(path: &str, encryption_key: &str) {
    file::data_to_image(encryption::encrypt_file_data(
        file::get_file_data(path),
        encryption_key,
    ));
}

/// used for decrypt file with key and file path
#[wasm_bindgen]
pub fn decrypt_file(path: &str, encryption_key: &str) {
    file::data_to_file(&mut encryption::decrypt_data(
        file::image_to_data(path),
        encryption_key,
    ));
}
