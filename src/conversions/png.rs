use std::process::Command;
use native_dialog::FileDialog;
use dialoguer::Input;
use image::*;
use webp::*;
use crate::logo;
use std::thread::sleep;
use std::time::Duration;
use dialoguer::Select;
use std::path::Path;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
extern crate dirs;


pub fn convert_png_to_jpeg(input_path: &Path) {
    let mut output_path = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            let mut stdout = std::io::stdout();
            stdout.execute(Clear(ClearType::All)).unwrap();
            return;
        }
    };


    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let file_name2 = file_name.trim_end_matches(".png");
    output_path.push(format!("{}.jpeg", file_name2));
    let jpeg_path = Path::new(&output_path);

    match open(&input_path) {
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


pub fn convert_png_to_ico(input_path: &Path) {
    let mut output_dir = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            return;
        }
    };

    let img = match open(input_path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Failed to open PNG image: {}", err);
            return;
        }
    };

    let (w, h) = img.dimensions();
    if w < 1 || w > 256 || h < 1 || h > 256 {
        println!("PNG width and height must be 1 - 256");
        sleep(Duration::from_secs(2));
        let mut stdout = std::io::stdout();
        stdout.execute(Clear(ClearType::All)).unwrap();
        logo();
        convert_png_file();
    } else {
        let file_name = input_path.file_name().unwrap().to_str().unwrap();
        let file_name2 = file_name.trim_end_matches(".png");
        output_dir.push(format!("{}.ico", file_name2));
        let ico_path = Path::new(&output_dir);

        match img.save_with_format(ico_path, ImageFormat::Ico) {
            Ok(_) => println!("Successfully saved {} to {:?}", file_name2, output_dir),
            Err(err) => eprintln!("Failed to save image as ICO: {}", err),
        };
    }
}


pub fn convert_png_to_webp(input_path: &Path) {
    let mut output_dir = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            return;
        }
    };

    let img = match open(input_path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Failed to open PNG image: {}", err);
            return;
        }
    };

    let (w, h) = img.dimensions();
    let size_factor = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(imageops::resize(
        &img,
        (w as f64 * size_factor) as u32,
        (h as f64 * size_factor) as u32,
        imageops::FilterType::Triangle,
    ));

    let encoder = Encoder::from_image(&img).unwrap();
    let webp_data = encoder.encode(90f32);

    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let file_name2 = file_name.trim_end_matches(".png");
    output_dir.push(format!("{}.webp", file_name2));
    let webp_path = Path::new(&output_dir);

    if let Err(err) = std::fs::write(webp_path, webp_data.as_ref()) {
        eprintln!("Failed to save image as WebP: {}", err);
    } else {
        println!("Successfully saved {} to {:?}", file_name2, output_dir);
    }
}


pub fn convert_png_to_avif(input_path: &Path) {
    let mut output_path = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            return;
        }
    };

    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let file_name2 = file_name.trim_end_matches(".png");
    output_path.push(format!("{}.avif", file_name2));
    let avif_path = Path::new(&output_path);

    let status = Command::new("cavif")
        .args(&[
            "-o",
            &avif_path.to_string_lossy(),
            &input_path.to_string_lossy(),
        ])
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Successfully converted PNG to AVIF: {:?}", avif_path);
            } else {
                eprintln!("Failed to convert PNG to AVIF.");
            }
        }
        Err(err) => {
            eprintln!("Failed to execute cavif command: {}", err);
        }
    }
}



pub fn convert_png_file() {
    println!("Please select the desired conversion:");
    let choices = vec!["Single Conversion", "Bulk Conversion"];
    let index = Select::new()
        .items(&choices)
        .default(0) // Set the default choice (optional)
        .interact()
        .unwrap();

    match index {
        0 => {
            let path_result = FileDialog::new()
                .set_location("~/Desktop")
                .add_filter("PNG's", &["png"])
                .show_open_single_file();
            
            match path_result {
                Ok(Some(path)) => {
                    let single_choices = vec!["PNG to JPEG", "PNG to WEBP", "PNG to AVIF", "PNG to ICO"];
                    let single_index = Select::new()
                        .items(&single_choices)
                        .default(0)
                        .interact()
                        .unwrap();

                    match single_index {
                        0 => convert_png_to_jpeg(&path),
                        1 => convert_png_to_webp(&path),
                        2 => convert_png_to_avif(&path),
                        3 => convert_png_to_ico(&path),
                        _ => println!("Invalid choice."),
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
        1 => {
            let bulk_choices = vec!["PNG to JPEG", "PNG to WEBP", "PNG to AVIF", "PNG to ICO"];
            let bulk_index = Select::new()
                .items(&bulk_choices)
                .default(0)
                .interact()
                .unwrap();

            match bulk_index {
                0 => {
                    let folder_path = Input::<String>::new()
                        .with_prompt("Enter folder path:")
                        .interact()
                        .unwrap();

                    let dir_entries = match std::fs::read_dir(&folder_path) {
                        Ok(entries) => entries,
                        Err(err) => {
                            eprintln!("Failed to read directory entries: {}", err);
                            return;
                        }
                    };

                    for entry in dir_entries {
                        if let Ok(entry) = entry {
                            if let Some(extension) = entry.path().extension() {
                                if extension == "png" {
                                    convert_png_to_jpeg(&entry.path());
                                }
                            }
                        }
                    }
                }

                1 => {
                    let folder_path = Input::<String>::new()
                        .with_prompt("Enter folder path:")
                        .interact()
                        .unwrap();

                    let dir_entries = match std::fs::read_dir(&folder_path) {
                        Ok(entries) => entries,
                        Err(err) => {
                            eprintln!("Failed to read directory entries: {}", err);
                            return;
                        }
                    };

                    for entry in dir_entries {
                        if let Ok(entry) = entry {
                            if let Some(extension) = entry.path().extension() {
                                if extension == "png" {
                                    convert_png_to_webp(&entry.path());
                                }
                            }
                        }
                    }
                }

                2 => {
                    let folder_path = Input::<String>::new()
                        .with_prompt("Enter folder path:")
                        .interact()
                        .unwrap();

                    let dir_entries = match std::fs::read_dir(&folder_path) {
                        Ok(entries) => entries,
                        Err(err) => {
                            eprintln!("Failed to read directory entries: {}", err);
                            return;
                        }
                    };

                    for entry in dir_entries {
                        if let Ok(entry) = entry {
                            if let Some(extension) = entry.path().extension() {
                                if extension == "png" {
                                    convert_png_to_avif(&entry.path());
                                }
                            }
                        }
                    }
                }

                3 =>  {
                    let folder_path = Input::<String>::new()
                        .with_prompt("Enter folder path:")
                        .interact()
                        .unwrap();

                    let dir_entries = match std::fs::read_dir(&folder_path) {
                        Ok(entries) => entries,
                        Err(err) => {
                            eprintln!("Failed to read directory entries: {}", err);
                            return;
                        }
                    };

                    for entry in dir_entries {
                        if let Ok(entry) = entry {
                            if let Some(extension) = entry.path().extension() {
                                if extension == "png" {
                                    convert_png_to_ico(&entry.path());
                                }
                            }
                        }
                    }
                }
                _ => println!("Invalid choice."),
            }
        }
        _ => println!("Invalid choice."),
    }
}
