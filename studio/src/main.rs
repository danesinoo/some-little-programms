use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
mod pausa;
mod show_data;

fn main() {
    if let Err(e) = crate::setup() {
        eprintln!("Couldn't create file: {}", e);
    }

    let mut args = env::args();
    args.next(); // skip the first argument

    if let Some(arg1) = args.next() {
        if start_end_time(&arg1, "-") || start_end_time(&arg1, "_") || start_end_time(&arg1, ",") {
            return;
        } else {
            match arg1.as_ref() {
                "info" => {
                    show_data::display_info(&args.next().unwrap_or("all".to_string()));
                }
                "help" | "-h" | "--help" => {
                    print_help();
                }
                "pausa" | "break" => {
                    pausa::pausa(&args.next().unwrap());
                }
                _ => {
                    println!("Didn't get it");
                }
            }
        }
    }
}

fn start_end_time(arg: &str, delimiter: &str) -> bool {
    if let Some((arg1, arg2)) = arg.split_once(delimiter) {
        match arg2 {
            "begin" | "b" | "start" => {
                write_log(&format!("{arg1},begin"));
                true
            }
            "end" | "e" | "stop" => {
                write_log(&format!("{arg1},end"));
                true
            }
            _ => {
                eprintln!("Didn't get it");
                true
            }
        }
    } else {
        false
    }
}

fn write_log(s: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(crate::show_data::path())
        .unwrap();

    if let Err(e) = writeln!(file, "{},{}", s, Local::now()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

// this function create the folder and the file if they don't exist
fn setup() -> std::io::Result<()> {
    let path = crate::show_data::path();
    if path.exists() {
        return Ok(());
    } else if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::File::create(path)?;
    Ok(())
}

fn print_help() {
    println!("Usage:");
    println!("<task_name>,begin -> record the start time of the task");
    println!("<task_name>,end   -> record the end time of the task");
    println!("info              -> show the info of the tasks");
}
