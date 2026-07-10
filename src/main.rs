use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::fmt::Write as FmtWrite;
pub struct Pixcel {
    r: u8,
    g: u8,
    b: u8,
}

fn parser(args: Vec<String>) -> (Vec<Pixcel>, String) {
    let img: String = fs::read_to_string(&args[1]).expect("Failed to read file!");
    let sensitized_data: String  = img.replace("\n", " ");


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
    
    let num_pixels: u64 = pixels.len() as u64;
    println!("Image width: {:?}\nHeight {:?}\nMax Value: {:?}\nNumber of pixels: {:?}", metadata[0], metadata[1], metadata[2],num_pixels);

    let data:String = format!("P3\n{} {} {}\n", metadata[0],metadata[1],metadata[2]);

    return (pixels, data);
}

fn grayscale(pixels: &[Pixcel],mut data: String) {
    for pixel in pixels {
        let eff: u8 = ((0.2627 * pixel.r as f32) +  (0.6780 * pixel.g as f32) + (0.0593 * pixel.b as f32)) as u8;
        let _ = writeln!(data, "{} {} {}", eff, eff, eff);
    }

    create_img(data);
}

fn invert(pixels: &[Pixcel], mut data: String){
    for pixel in pixels {
        let r: u8 = 255 - pixel.r;
        let g: u8 = 255 - pixel.g;
        let b: u8 = 255 - pixel.b;
        let _ = writeln!(data, "{} {} {}", r, g, b);
    }

    create_img(data);
}

fn create_img(data: String) {
    let mut file: File = File::create("img.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write image!");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    fs::exists(&args[1]).expect("File path is wrong or file doesn't exist!");
    fs::exists(&args[2]).expect("Filter is missing!");

    
    let filter = args[2].clone();
    let (pixels, metadata) = parser(args);
    match filter.as_str() {
        "grayscale" => grayscale(&pixels,metadata),
        "invert" => invert(&pixels,metadata),
        _ => panic!("Please use a supported filter!"),
    }

}
