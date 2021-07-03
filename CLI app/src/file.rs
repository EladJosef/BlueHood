use png;
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Read;
use std::path::Path;

/// Create vector with all file data, and return vector
pub fn get_file_data(filename: &str) -> Vec<u8> {
    // open file
    let mut file = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");

    // init buffer
    let mut buffer = vec![0; metadata.len() as usize];

    // get file name from filename
    let mut path: Vec<&str> = filename.split("\\").collect::<Vec<_>>();
    let file_name = path.pop().unwrap().as_bytes();

    // get data from file
    file.read(&mut buffer).expect("buffer overflow");

    // mark the end of the file title data
    buffer.push(0);

    // push file title to data vector
    for i in file_name.iter() {
        buffer.push(*i);
    }

    buffer
}

/// Create file with data, data written to file with name thet store in vector
pub fn data_to_file(data: &mut Vec<u8>) {
    let mut file_name = String::from("");
    let mut temp = 1;

    // get file name from data
    while temp != 0 {
        file_name.push(temp as char);
        temp = data.pop().unwrap();
    }

    // reveres and remove flag from file name
    file_name = file_name.chars().rev().collect::<String>();
    file_name.pop();

    // create file with data
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(&data).expect("Unable to write data");
}

/// Gets a vector with information and saves it in the image
pub fn data_to_image(mut data: Vec<u8>) {
    // culc the file size by data len
    let file_size = ((data.len() as f64 / 3 as f64).sqrt()).ceil() as u32;

    // create reference to image
    let path = Path::new("output.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, file_size, file_size);

    // add zeros for fill the image size
    for _n in 0..(file_size * file_size * 3) as usize - data.len() {
        data.push(0);
    }

    // set image preferences
    encoder.set_color(png::ColorType::RGB); // can be RGBA (change size calc after set)
    encoder.set_depth(png::BitDepth::Eight);

    // write data to file
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}

/// Extract information from an image
pub fn image_to_data(img_path: &str) -> Vec<u8> {
    // init decoder
    let decoder = png::Decoder::new(File::open(img_path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    // init buffer
    let mut buf = vec![0; info.buffer_size()];

    // get data from image
    reader.next_frame(&mut buf).unwrap();

    // remove the fill zeros
    while buf[buf.len() - 1] == 0 {
        buf.pop();
    }

    buf
}
