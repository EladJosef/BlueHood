use png;
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Read;
use std::path::Path;

pub fn get_file_data(filename: &str) -> Vec<u8> {
    let mut file = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");

    let mut buffer = vec![0; metadata.len() as usize];

    let mut path: Vec<&str> = filename.split("\\").collect::<Vec<_>>();
    let file_name = path.pop().unwrap().as_bytes();

    file.read(&mut buffer).expect("buffer overflow");
    buffer.push(0);

    for i in file_name.iter() {
        buffer.push(*i);
    }

    buffer
}

pub fn data_to_file(data: &mut Vec<u8>) {
    let mut file_name = String::from("");
    let mut temp = 1;

    while temp != 0 {
        file_name.push(temp as char);
        temp = data.pop().unwrap();
    }

    file_name = file_name.chars().rev().collect::<String>();
    file_name.pop();

    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(&data).expect("Unable to write data");
}

pub fn data_to_image(mut data: Vec<u8>) {
    let file_size = ((data.len() as f64 / 3 as f64).sqrt()).ceil() as u32;

    let path = Path::new("output.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, file_size, file_size);

    for _n in 0..(file_size * file_size * 3) as usize - data.len() {
        data.push(0);
    }

    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap(); // Save
}

pub fn image_to_data(img_path: &str) -> Vec<u8> {
    let decoder = png::Decoder::new(File::open(img_path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).unwrap();

    while buf[buf.len() - 1] == 0 {
        buf.pop();
    }

    buf
}
