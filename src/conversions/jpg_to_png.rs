use dialoguer::Input;
use image::{codecs::png, open, ImageFormat};
use native_dialog::FileDialog;
use std::env::current_dir;
use std::path::Path;
use std::path::PathBuf;


pub fn convert_jpeg_to_png() {
    let path_result = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("JPEG Image", &["jpeg"])
        .show_open_single_file();

    match path_result {
        Ok(Some(path)) => {
            println!("Selected file: {:?}", path);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_name2 = file_name.trim_end_matches(".jpeg");
            let output_path = PathBuf::from(format!("src/results/pngs/{}.png", file_name2));
            let png_path = Path::new(&output_path);
            match open(&path) {
                Ok(jpg_image) => {
                    if let Err(err) = jpg_image.save_with_format(&png_path, ImageFormat::Png) {
                        eprintln!("Failed to save image as PNG: {}", err);
                    } else {
                        println!("Successfully saved {}.png to {:?}", file_name2, output_path)
                    }
                }
                Err(err) => {
                    eprintln!("Failed to open JPEG image: {}", err);
                }
            }
        }
        Ok(None) => {
            println!("No file selected.");
        }
        Err(err) => {
            eprintln!("Failed to show file dialog: {}", err);
        }
    }
}


