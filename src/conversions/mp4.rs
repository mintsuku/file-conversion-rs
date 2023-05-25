use std::process::Command;
use native_dialog::FileDialog;
use dialoguer::Input;
use dialoguer::Select;
use std::path::Path;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
extern crate dirs;


pub fn convert_mp4_to_gif(input_path: &Path) {
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
    let file_name2 = file_name.trim_end_matches(".mp4");
    output_path.push(format!("{}.gif", file_name2));
    let gif_path = Path::new(&output_path);
    let ffmpeg_command = format!(
        "ffmpeg -i {} -vf \"fps=10,scale=320:-1:flags=lanczos\" -c:v gif {}",
        input_path.to_string_lossy(),
        gif_path.to_string_lossy()
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Successfully converted MP4 to GIF, Saved to {:?}", gif_path)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error converting MP4 to GIF: {}", error_message);
        let mut stdout = std::io::stdout();
        stdout.execute(Clear(ClearType::All)).unwrap();
    }
}


pub fn convert_mp4_to_mov(input_path: &Path) {
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
    let file_name2 = file_name.trim_end_matches(".mp4");
    output_path.push(format!("{}.mov", file_name2));
    let mov_path = Path::new(&output_path);
    let ffmpeg_command = format!(
        "ffmpeg -i {} -f mov {}",
        input_path.to_string_lossy(),
        mov_path.to_string_lossy()
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Successfully converted MP4 to MOV, Saved to {:?}", mov_path)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error converting MP4 to MOV: {}", error_message);

    }
}

pub fn convert_mp4_file() {
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
                .add_filter("MP4's", &["mp4"])
                .show_open_single_file();
            
            match path_result {
                Ok(Some(path)) => {
                    let single_choices = vec!["MP4 to GIF", "MP4 to MOV"];
                    let single_index = Select::new()
                        .items(&single_choices)
                        .default(0)
                        .interact()
                        .unwrap();

                    match single_index {
                        0 => convert_mp4_to_gif(&path),
                        1 => convert_mp4_to_mov(&path),
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
            let bulk_choices = vec!["MP4 to GIF", "MP4 to MOV"];
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
                                if extension == "mp4" {
                                    convert_mp4_to_gif(&entry.path());
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
                                if extension == "mp4" {
                                    convert_mp4_to_mov(&entry.path());
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
