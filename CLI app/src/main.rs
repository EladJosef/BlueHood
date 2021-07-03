use std::env;

/// main function (Run CLI)
fn main() {
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
            app::encrypt_file(&argument_vector[2], &argument_vector[3]);
        } else if argument_vector[1] == "de" {
            // decrypt file
            app::decrypt_file(&argument_vector[2], &argument_vector[3]);
        } else {
            // print help
            help();
        }
    }
}

/// print help message into terminal
fn help() {
    println!("Input need to be provided like:\n- encryption : bluehood.exe en [path] [key]\n- decryption : bluehood.exe de [path] [key]");
}
