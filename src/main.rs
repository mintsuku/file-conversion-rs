use std::{fs::File};
use std::path::Path;
use image::{open, save_buffer_with_format, ImageFormat};
use std::env::{current_dir};
use std::io;


mod conversions {
    pub mod gif;
    pub mod mp4;
}


fn main() {
    loop {
    println!("Select a module");
    let mut module = String::new();
    io::stdin().read_line(&mut module).expect("Failed to read input");

    let pick = module.trim();
    match pick {
        "1" => {
            conversions::gif::convert_gif_file();
            break;
        }
        "2" => {
            conversions::mp4::convert_mp4_file()
        }

        "3" => {
            break;
        }

        "4" => {
            
        }

        "exit" => {
            break;
        }
        _ => {
            println!("Invalid module selected");
        }
        
    }
}
   
}

