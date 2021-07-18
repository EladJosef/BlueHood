/// use for vector encryption
mod encryption;
/// use for file management
mod file;
/// used for hendle rust panics
use console_error_panic_hook;
use std::panic;
/// used for WebAssembly compilation
use wasm_bindgen::prelude::*;

/// used for encrypt file with key and file path
#[wasm_bindgen]
pub fn encrypt_file(path: &str, encryption_key: &str) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    file::data_to_image(encryption::encrypt_data(
        file::get_file_data(path),
        encryption_key,
    ));
}

/// used for decrypt file with key and file path
#[wasm_bindgen]
pub fn decrypt_file(path: &str, encryption_key: &str) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    file::data_to_file(&mut encryption::decrypt_data(
        file::image_to_data(path),
        encryption_key,
    ));
}

/// used for encrypt data
#[wasm_bindgen]
pub fn encrypt_data(file_data: Vec<u8>, encryption_key: &str, file_name: &str) -> Vec<u8> {
    encryption::encrypt_data(
        file::add_file_name_to_buffer(file_data, file_name),
        encryption_key,
    )
}

/// used for decrypt data
#[wasm_bindgen]
pub fn decrypt_data(file_data: Vec<u8>, encryption_key: &str) -> Vec<u8> {
    encryption::decrypt_data(file_data, encryption_key)
}
