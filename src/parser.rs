use crate::types::Pixcel;
use std::fs;

pub fn parser(args: Vec<String>) -> (Vec<Pixcel>, Vec<u16>) {
    let img: String = fs::read_to_string(&args[1]).expect("Failed to read file!");
    let sensitized_data: String = img.replace("\n", " ");

    let mut metadata: Vec<u16> = vec![];
    let mut pixels: Vec<Pixcel> = vec![];

    let mut one_pixel: Vec<u8> = vec![];
    for word in sensitized_data.split_whitespace() {
        // Escape non numerical values
        let val: u16 = match word.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Parse the header
        if metadata.len() < 3 {
            metadata.push(val);
            continue;
        }

        // Collected enough values to construct a pixel
        one_pixel.push(val as u8);

        // Add one pixel at a time to pixels
        if one_pixel.len() == 3 {
            pixels.push(Pixcel {
                r: one_pixel[0],
                g: one_pixel[1],
                b: one_pixel[2],
            });

            one_pixel.clear();
        }
    }

    println!(
        "Image width: {:?}\nHeight {:?}\nMax Value: {:?}\nNumber of pixels: {:?}",
        metadata[0],
        metadata[1],
        metadata[2],
        pixels.len()
    );

    return (pixels, metadata);
}
