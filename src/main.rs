use colored::Colorize;
use std::{fs, io, process};
use std::fmt::{Error, format};
use std::io::*;
use std::path::PathBuf;


fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let destination = format!("C:\\Users\\{}\\AppData\\Roaming\\discord\\Cache", whoami::username());

    println!("{} {} {}", "!".yellow(), destination.italic().normal(), "will be overwritten".italic().yellow());
    println!("{} {} {}", "?".yellow(), "All your files located in your discord cache directory will turn into .png do you wish to proceed? ∙".italic().normal(), "yes/no".italic().yellow());

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    match input {
        "y" | "yes" => {
            print!("\x1B[2J\x1B[1;1H");
            println!("{} {} {}", "✓".green(), "All your files located in your discord cache directory will turn into .png Do you wish to proceed? ∙".italic().normal(), "yes".italic().green());
            let paths = fs::read_dir(destination).unwrap();

            for path in paths {
                let mut png = PathBuf::from(path.as_ref().unwrap().path());
                png.set_extension("png");
                println!("{} {} {}", "✓".green(), "File converted successfully".italic().normal(), png.to_str().unwrap().italic().green());
                fs::rename(path.unwrap().path(), png).expect("Failed to rename file");
            }
        }
        "n" | "no" => {
            print!("\x1B[2J\x1B[1;1H");
            process::exit(1);
        }
        _ => {
            print!("\x1B[2J\x1B[1;1H");
            process::exit(1);
        }
    }
}

