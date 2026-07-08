use std::{env, fs};

pub struct Pixcel {
    r: u8,
    g: u8,
    b: u8,
}

fn parser(args: Vec<String>){
    fs::exists(&args[1]).expect("File path is wrong or file doesn't exist!");
    fs::exists(&args[2]).expect("Filter wasn't provided!");

    let img: String = fs::read_to_string(&args[1]).expect("Failed to read file!");
    let sensitized_data: String  = img.replace("\n", " ");


    let mut metadata: Vec<u16> = vec![];
    let mut pixels: Vec<Pixcel> = vec![];
    
    let mut one_pixel: Vec<u8> = vec![];
    for word in sensitized_data.split_whitespace() {
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
    
    let num_pixels: u64 = pixels.len() as u64;
    println!("Number of pixels: {:?}", num_pixels);
    println!("Meta Data: {:?}", metadata);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parser(args);
}
