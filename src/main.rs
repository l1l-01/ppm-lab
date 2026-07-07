use std::{env, fs};

fn parser(args: Vec<String>){
    fs::exists(&args[1]).expect("File path is wrong or file doesn't exist!");

    let img: String = fs::read_to_string(&args[1]).expect("Failed to read file!");
    let sensitized_data: String  = img.replace("\n", " ");


    let mut i: u64 = 1;
    let mut metadata: Vec<u16> = vec![];
    let mut pixels: Vec<Vec<u8>> = vec![];
    

    let mut pix_increment: u8 = 0;
    let mut one_pixel: Vec<u8> = vec![];
    for word in sensitized_data.split(" ")  {
        if !word.is_empty() {
            let val: u8 = match word.parse() {
                Ok(val) => val,
                Err(_) => continue
            };
            
            if pix_increment == 3 {
                pix_increment = 0;
                pixels.push(one_pixel.clone());
                one_pixel.clear();

                if i == 3 {
                    let data: u16 = val as u16;
                    metadata.push(data);
                }
            }else {
                one_pixel.push(val.clone());
                i += 1;
            }

            pix_increment += 1;
        }
    }
    
    let num_pixels: u64 = pixels.len() as u64;
    println!("Number of pixels: {:?}", num_pixels);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parser(args);
}
