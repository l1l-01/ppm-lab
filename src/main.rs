use std::{env, fs};
mod filters;
mod parser;
mod types;
use filters::{box_blur, grayscale, invert};
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
        _ => panic!("Please use a supported filter!"),
    }
}
