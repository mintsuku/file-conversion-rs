use dialoguer::Input;
use image::{codecs::png, open, ImageFormat};
use native_dialog::FileDialog;
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn convert_png_to_jpeg() {
    let path_result = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["png"])
        .show_open_single_file();

    match path_result {
        Ok(Some(path)) => {
            println!("Selected file: {:?}", path);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_name2 = file_name.trim_end_matches(".png");
            let output_path = PathBuf::from(format!("src/results/jpegs/{}.jpeg", file_name2));
            let jpeg_path = Path::new(&output_path);
            match open(&path) {
                Ok(png_image) => {
                    if let Err(err) = png_image.save_with_format(&jpeg_path, ImageFormat::Jpeg) {
                        eprintln!("Failed to save image as JPEG: {}", err);
                    } else {
                        println!("Successfully saved {} to {:?}", file_name2, output_path)
                    }
                }
                Err(err) => {
                    eprintln!("Failed to open PNG image: {}", err);
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


