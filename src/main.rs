use std::{env, fs};

fn parser(args: Vec<String>){
    fs::exists(&args[1]).expect("File path is wrong or file doesn't exist!");

    let img: String = fs::read_to_string(&args[1]).expect("Failed to read file!");
    let sensitized_data: String  = img.replace("\n", " ");

    let mut i: u64 = 1;
    let mut metadata: Vec<&str> = vec![];
    let mut pixels: Vec<u8> = vec![];

    for word in sensitized_data.split(" ")  {
        if !word.is_empty() {
            match i {
                1 | 2 | 3 | 4 => metadata.push(word),
                _ => pixels.push(word.parse().unwrap())
            }
            i += 1;
        }
    }
    
    let num_pixels: u64 = pixels.len() as u64 / 3;
    println!("Number of pixels: {:?}", num_pixels);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parser(args);
}
