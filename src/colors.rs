use crate::types::Pixcel;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write;

pub fn green(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", 0, pixel.g, 0);
    }

    let mut file: File = File::create("green.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn red(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", pixel.r, 0, 0);
    }

    let mut file: File = File::create("red.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn blue(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", 0, 0, pixel.b);
    }

    let mut file: File = File::create("blue.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn magenta(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", pixel.b, 0, pixel.b);
    }

    let mut file: File = File::create("magenta.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn yellow(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", pixel.r, pixel.r, 0);
    }

    let mut file: File = File::create("yellow.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn cyan(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", 0, pixel.g, pixel.g);
    }

    let mut file: File = File::create("cyan.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn blue_green(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", 0, 128, pixel.b);
    }

    let mut file: File = File::create("bg.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn green_blue(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", 0, pixel.g, 128);
    }

    let mut file: File = File::create("gb.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn red_blue(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", pixel.r, 0, 128);
    }

    let mut file: File = File::create("rb.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn magenta_red(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", 128, 0, pixel.b);
    }

    let mut file: File = File::create("mr.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}

pub fn magenta_blue(pixels: &[Pixcel], mut data: String) {
    for pixel in pixels {
        let _ = writeln!(data, "{} {} {}", pixel.r, 0, 188);
    }

    let mut file: File = File::create("rg.ppm").expect("Failed to create file");
    file.write_all(data.as_bytes())
        .expect("Failed to write image!");
}
