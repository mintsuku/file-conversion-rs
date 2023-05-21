use std::path::PathBuf;
use std::process::Command;
use std::env;
use native_dialog::FileDialog;
use dialoguer::Input;
use image::{open, ImageFormat, image_dimensions};
use std::env::current_dir;
use std::thread::sleep;
use std::time::Duration;
use dialoguer::Select;
use std::path::Path;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
extern crate dirs;

pub fn convert_gif_to_mp4(input_path: &Path) {
    let mut output_path = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            return;
        }
    };
    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let file_name2 = file_name.trim_end_matches(".gif");
    output_path.push(format!("{}.mp4", file_name2));
    let mp4_path = Path::new(&output_path);
    let ffmpeg_command = format!(
        "ffmpeg -i {} -c:v libx264 -strict -2 -preset slow -pix_fmt yuv420p -vf \"scale=trunc(iw/2)*2:trunc(ih/2)*2\" -f mp4 {}",
        input_path.to_string_lossy(),
        mp4_path.to_string_lossy()
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Successfully converted GIF to MP4, Saved to {:?}", mp4_path)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error converting GIF to MP4: {}", error_message);
    }
}

pub fn convert_gif_to_png(input_path: &Path) {
    let mut output_path = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            return;
        }
    };
    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let file_name2 = file_name.trim_end_matches(".gif");
    output_path.push(format!("{}.png", file_name2));
    let png_path = Path::new(&output_path);
    let image = open(input_path).expect("Failed to open GIF");
    image.save_with_format(png_path, ImageFormat::Png)
        .expect("Failed to save PNG");

    println!("Successfully converted GIF to PNG, Saved to {:?}", png_path);
}

pub fn convert_gif_to_jpeg(input_path: &Path) {
    let mut output_path = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            return;
        }
    };
    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let file_name2 = file_name.trim_end_matches(".gif");
    output_path.push(format!("{}.jpeg", file_name2));
    let jpeg_path = Path::new(&output_path);
    let image = open(input_path).expect("Failed to open GIF");
    image.save_with_format(jpeg_path, ImageFormat::Jpeg)
        .expect("Failed to save JPEG");

    println!("Successfully converted GIF to JPEG, Saved to {:?}", jpeg_path);
}

pub fn convert_gif_to_ico(input_path: &Path) {
    let mut output_path = match dirs::download_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to determine the user's download directory.");
            return;
        }
    };
    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let file_name2 = file_name.trim_end_matches(".gif");
    output_path.push(format!("{}.ico", file_name2));
    let ico_path = Path::new(&output_path);
    let image = open(input_path).expect("Failed to open GIF");
    let dimensions = image_dimensions(&input_path).unwrap();
    if dimensions.0 < 1 || dimensions.1 > 256 {
        println!("GIF width and height must be 1 - 256");
        sleep(Duration::from_secs(2));
        let mut stdout = std::io::stdout();
        stdout.execute(Clear(ClearType::All)).unwrap();
        convert_gif_file();
    } else {
        image.save_with_format(ico_path, ImageFormat::Ico)
            .expect("Failed to save ICO");

        println!("Successfully converted GIF to ICO, Saved to {:?}", ico_path);
    }
}

pub fn convert_gif_file() {
    println!("Please select the desired conversion:");
    let choices = vec!["Single Conversion", "Bulk Conversion"];
    let index = Select::new()
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    match index {
        0 => {
            let path_result = FileDialog::new()
                .set_location("~/Desktop")
                .add_filter("GIF's", &["gif"])
                .show_open_single_file();
            
            match path_result {
                Ok(Some(path)) => {
                    let single_choices = vec!["GIF to MP4", "GIF to PNG", "GIF to JPEG", "GIF to ICO"];
                    let single_index = Select::new()
                        .items(&single_choices)
                        .default(0)
                        .interact()
                        .unwrap();

                    match single_index {
                        0 => convert_gif_to_mp4(&path),
                        1 => convert_gif_to_png(&path),
                        2 => convert_gif_to_jpeg(&path),
                        3 => convert_gif_to_ico(&path),
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
            let bulk_choices = vec!["GIF to MP4", "GIF to PNG", "GIF to JPEG", "GIF to ICO"];
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
                                if extension == "gif" {
                                    convert_gif_to_mp4(&entry.path());
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
                                if extension == "gif" {
                                    convert_gif_to_png(&entry.path());
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
                                if extension == "gif" {
                                    convert_gif_to_jpeg(&entry.path());
                                }
                            }
                        }
                    }
                }
                3 => {
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
                                if extension == "gif" {
                                    convert_gif_to_ico(&entry.path());
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
