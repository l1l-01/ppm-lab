use std::{env, fs};

mod blur;
mod color;
mod other;
mod parser;
mod types;

use blur::box_blur;
use color::{
    blue, blue_green, cyan, green, green_blue, magenta, magenta_blue, magenta_red, red, red_blue,
    yellow,
};
use other::{contrast, grayscale, invert, saturation};
use parser::parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &str = match args.get(1) {
        Some(_) => &args[1],
        None => "def",
    };

    // Process image or create it from an algorithm
    if path != "def" {
        fs::exists(&args[2]).expect("Filter is missing!");

        let filter = args[2].clone();
        let (pixels, metadata) = parser(args);

        let data: String = format!("P3\n{} {} {}\n", metadata[0], metadata[1], metadata[2]);

        match filter.as_str() {
            "grayscale" => grayscale(&pixels, data),
            "invert" => invert(&pixels, data),
            "boxblur" => box_blur(&pixels, &metadata, data),
            "contrast" => contrast(&pixels, data),
            "saturation" => saturation(&pixels, data),
            "g" => green(&pixels, data),
            "r" => red(&pixels, data),
            "b" => blue(&pixels, data),
            "m" => magenta(&pixels, data),
            "y" => yellow(&pixels, data),
            "c" => cyan(&pixels, data),
            "bg" => blue_green(&pixels, data),
            "gb" => green_blue(&pixels, data),
            "rb" => red_blue(&pixels, data),
            "mr" => magenta_red(&pixels, data),
            "rg" => magenta_blue(&pixels, data),
            _ => panic!("Please use a supported filter!"),
        }
    } else {
        // hmmmmmmmmmmmmm
    }
}
