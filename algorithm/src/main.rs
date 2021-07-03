use std::env;
mod encryption;
mod file;

fn main() {
    let mut argument_vector: Vec<String> = vec![];

    for argument in env::args() {
        argument_vector.push(argument);
    }

    if argument_vector.len() != 4 {
        help();
    } else {
        if argument_vector[1] == "en" {
            encrypt_file(&argument_vector[2], &argument_vector[3]);
        } else if argument_vector[1] == "de" {
            decrypt_file(&argument_vector[2], &argument_vector[3]);
        } else {
            help();
        }
    }
}

fn decrypt_file(path: &str, encryption_key: &str) {
    file::data_to_file(&mut encryption::decrypt_data(
        file::image_to_data(path),
        encryption_key,
    ));
}

fn encrypt_file(path: &str, encryption_key: &str) {
    file::data_to_image(encryption::encrypt_file_data(
        file::get_file_data(path),
        encryption_key,
    ));
}

fn help() {
    println!("Input need to be provided like:\n- encryption : bluehood.exe -en [path] [key]\n- decryption : bluehood.exe -de [path] [key]");
}
