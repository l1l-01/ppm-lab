use crate::types::Pixcel;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write;

pub fn box_blur(pixels: &[Pixcel], metadata: &[u16], mut data: String) {
    let w: usize = metadata[0] as usize;
    let total_pixcels: u64 = metadata[0] as u64 * metadata[1] as u64 - 1;

    for (i, pixel) in pixels.iter().enumerate() {
        let top: i64 = i as i64 - w as i64;
        let bottom: u64 = w as u64 + i as u64;

        let left: i64 = i as i64 - 1;
        let right: u64 = i as u64 + 1;

        let bottom_left: i32 = (bottom - 1) as i32;
        let top_left: i32 = (top - 1) as i32;
        let bottom_right: u64 = (bottom + 1) as u64;
        let top_right: u64 = (top + 1) as u64;

        let is_border: bool = !(i as f64 / w as f64).to_string().contains(".");

        // Skip at first or last pixel or if the pixcel is in the border
        if left < 0
            || right > total_pixcels
            || top < 0
            || bottom > total_pixcels
            || is_border
            || top_left < 0
            || bottom_right > total_pixcels
        {
            let _ = writeln!(data, "{} {} {}", pixel.r, pixel.g, pixel.b);
            continue;
        }

        let r = ((pixel.r as u64
            + pixels[bottom as usize].r as u64
            + pixels[top as usize].r as u64
            + pixels[left as usize].r as u64
            + pixels[bottom_left as usize].r as u64
            + pixels[top_left as usize].r as u64
            + pixels[right as usize].r as u64
            + pixels[bottom_right as usize].r as u64
            + pixels[top_right as usize].r as u64)
            / 9) as u8;

        let g = ((pixel.g as u64
            + pixels[bottom as usize].g as u64
            + pixels[top as usize].g as u64
            + pixels[left as usize].g as u64
            + pixels[bottom_left as usize].g as u64
            + pixels[top_left as usize].g as u64
            + pixels[right as usize].g as u64
            + pixels[bottom_right as usize].g as u64
            + pixels[top_right as usize].g as u64)
            / 9) as u8;

        let b = ((pixel.b as u64
            + pixels[bottom as usize].b as u64
            + pixels[top as usize].b as u64
            + pixels[left as usize].b as u64
            + pixels[bottom_left as usize].b as u64
            + pixels[top_left as usize].b as u64
            + pixels[right as usize].b as u64
            + pixels[bottom_right as usize].b as u64
            + pixels[top_right as usize].b as u64)
            / 9) as u8;

        let _ = writeln!(data, "{} {} {}", r, g, b);
    }

    let mut file: File = File::create("box_blur.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}
