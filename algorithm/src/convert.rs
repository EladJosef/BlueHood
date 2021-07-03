use png;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn data_to_image(mut data: Vec<u8>, img_path: &str) {
    let file_size = ((data.len() as f64 / 3 as f64).sqrt()).ceil() as u32;

    let path = Path::new(img_path);
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
