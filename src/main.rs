use crossterm::terminal::{ClearType, Clear};
use console::style;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use crossterm::ExecutableCommand;


mod conversions {
    pub mod gif;
    pub mod mp4;
    pub mod png;
    pub mod jpeg;
}










pub fn logo() {
    let text = r"
   
______              _____                           _   
| ___ \            /  __ \                         | |  
| |_/ /   _ _   _  | /  \/ ___  _ ____   _____ _ __| |_ 
|    / | | | | | | | |    / _ \| '_ \ \ / / _ \ '__| __|
| |\ \ |_| | |_| | | \__/\ (_) | | | \ V /  __/ |  | |_ 
\_| \_\__, |\__,_|  \____/\___/|_| |_|\_/ \___|_|   \__|
       __/ |                                            
      |___/                                             

                                                                                                               ";
    println!("{}", style(text).red());
}

fn main() {
    logo();
    let mut stdout = std::io::stdout();
    loop {
    println!("{}", style("Module Selection").bold());
    println!("----------------");
    let modules = [
        ("1", "GIF Conversions"),
        ("2", "MP4 Conversions"),
        ("3", "PNG Conversions"),
        ("4", "JPEG Conversions\n\n"),
        ("Exit", "Type Exit to halt the program.")
    ];

    

    println!("Select a module:\n");
        for (_index, module) in modules.iter().enumerate() {
            println!("{}{}{} - {}", style("[").bold(),style(module.0).red(), style("]").bold(), module.1);
            
        }
    let mut module = String::new();
    io::stdin().read_line(&mut module).expect("Failed to read input");

    



    let pick = module.trim();
    match pick {
        "1" => {
            stdout.execute(Clear(ClearType::All)).unwrap();
            logo();
            conversions::gif::convert_gif_file();
            sleep(Duration::from_secs(2));
            stdout.execute(Clear(ClearType::All)).unwrap();
            main();
        }
        "2" => {
            stdout.execute(Clear(ClearType::All)).unwrap();
            logo();
            conversions::mp4::convert_mp4_file();
            sleep(Duration::from_secs(2));
            stdout.execute(Clear(ClearType::All)).unwrap();
            main();

}

        "3" => {
            stdout.execute(Clear(ClearType::All)).unwrap();
            logo();
            conversions::png::convert_png_file();
            sleep(Duration::from_secs(2));
            stdout.execute(Clear(ClearType::All)).unwrap();
            main();
        }

        "4" => {
            stdout.execute(Clear(ClearType::All)).unwrap();
            logo();
            conversions::jpeg::convert_jpeg_file();
            sleep(Duration::from_secs(2));
            stdout.execute(Clear(ClearType::All)).unwrap();
            main();
        }

        "exit" => {
            break;
        }
        _ => {
            println!("Invalid module selected");
            stdout.execute(Clear(ClearType::All)).unwrap();
            logo();
        }
        
    }
}
   
}

