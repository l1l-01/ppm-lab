use std::{env, fs};
mod colors;
mod filters;
mod parser;
mod types;
use colors::{
    blue, blue_green, cyan, green, green_blue, magenta, magenta_blue, magenta_red, red, red_blue,
    yellow,
};
use filters::{box_blur, contrast, grayscale, invert};
use parser::parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    fs::exists(&args[1]).expect("File path is wrong or file doesn't exist!");
    fs::exists(&args[2]).expect("Filter is missing!");

    let filter = args[2].clone();
    let (pixels, metadata) = parser(args);

    let data: String = format!("P3\n{} {} {}\n", metadata[0], metadata[1], metadata[2]);

    match filter.as_str() {
        "grayscale" => grayscale(&pixels, data),
        "invert" => invert(&pixels, data),
        "boxblur" => box_blur(&pixels, &metadata, data),
        "contrast" => contrast(&pixels, data),
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
}
