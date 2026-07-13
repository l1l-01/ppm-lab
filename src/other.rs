use crate::types::Pixcel;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write;

pub fn grayscale(pixels: &[Pixcel], mut data: String) {
    // Use BT.2020 method for converting RGB to grayscale
    for pixel in pixels {
        let eff: u8 = ((0.2627 * pixel.r as f32)
            + (0.6780 * pixel.g as f32)
            + (0.0593 * pixel.b as f32)) as u8;
        let _ = writeln!(data, "{} {} {}", &eff, &eff, &eff);
    }

    let mut file: File = File::create("grayscale.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn invert(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let r: u8 = 255 - pixel.r;
        let g: u8 = 255 - pixel.g;
        let b: u8 = 255 - pixel.b;
        let _ = writeln!(data, "{} {} {}", r, g, b);
    }

    let mut file: File = File::create("invert.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn contrast(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let r = ((pixel.r as i32 - 55) as f32 * 1.5 + 55.0) as u8;
        let g = ((pixel.g as i32 - 55) as f32 * 1.5 + 55.0) as u8;
        let b = ((pixel.b as i32 - 55) as f32 * 1.5 + 55.0) as u8;
        let _ = writeln!(data, "{} {} {}", r, g, b);
    }

    let mut file: File = File::create("contrast.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn saturation(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let r = ((pixel.r as f32 * 0.299) as f32
            + (pixel.r as f32 - (pixel.r as f32 * 0.299) as f32) as f32 * 1.3)
            as u8;
        let g = ((pixel.g as f32 * 0.299) as f32
            + (pixel.g as f32 - (pixel.g as f32 * 0.299) as f32) as f32 * 1.3)
            as u8;
        let b = ((pixel.b as f32 * 0.299) as f32
            + (pixel.b as f32 - (pixel.b as f32 * 0.299) as f32) as f32 * 1.3)
            as u8;
        let _ = writeln!(data, "{} {} {}", r, g, b);
    }

    let mut file: File = File::create("saturation.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}
