use std::{fs::File};
use crate::conversions::png_to_jpg::convert_png_to_jpeg;
use crate::conversions::{jpg_to_png::convert_jpeg_to_png};
use std::path::Path;
use image::{open, save_buffer_with_format, ImageFormat};
use std::env::{current_dir};
use std::io;


mod conversions {
    pub mod png_to_jpg;
    pub mod jpg_to_png;
}


fn main() {
    loop {
    println!("Select a module");
    let mut module = String::new();
    io::stdin().read_line(&mut module).expect("Failed to read input");

    let pick = module.trim();
    match pick {
        "1" => {
            convert_png_to_jpeg();
            break;
        }
        "2" => {
            convert_jpeg_to_png();
            break;
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
