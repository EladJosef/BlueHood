mod convert;
mod encryption;
mod file;

fn main() {
    let text_dir = ".\\files\\text.txt";
    let img_dir = ".\\files\\image.png";
    let music_dir = ".\\files\\music.mp3";

    let text_file_value = file::get_file_data(text_dir);
    let img_file_value = file::get_file_data(img_dir);
    let music_file_value = file::get_file_data(music_dir);

    let text_file_encryption_value = encryption::encrypt_file_data(text_file_value, "test key");
    let img_file_encryption_value = encryption::encrypt_file_data(img_file_value, "test key");
    let music_file_encryption_value = encryption::encrypt_file_data(music_file_value, "test key");

    convert::data_to_image(text_file_encryption_value, ".\\convert_files\\text.png");
    convert::data_to_image(img_file_encryption_value, ".\\convert_files\\img.png");
    convert::data_to_image(music_file_encryption_value, ".\\convert_files\\music.png");
}
