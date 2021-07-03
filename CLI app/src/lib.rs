use std::env;
/// use for vector encryption
mod encryption;
/// use for file management
mod file;
/// used for WebAssembly compilation
use wasm_bindgen::prelude::*;

/// main function (Run CLI)
fn run() {
    // init vector for the main arguments
    let mut argument_vector: Vec<String> = vec![];

    // move arguments to the vector
    for argument in env::args() {
        argument_vector.push(argument);
    }

    // manage arguments to request functions
    if argument_vector.len() != 4 {
        help();
    } else {
        if argument_vector[1] == "en" {
            // encrypt file
            encrypt_file(&argument_vector[2], &argument_vector[3]);
        } else if argument_vector[1] == "de" {
            // decrypt file
            decrypt_file(&argument_vector[2], &argument_vector[3]);
        } else {
            // print help
            help();
        }
    }
}

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

/// print help message into terminal
fn help() {
    println!("Input need to be provided like:\n- encryption : bluehood.exe en [path] [key]\n- decryption : bluehood.exe de [path] [key]");
}
