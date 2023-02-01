use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
mod show_data;
use show_data::display_info;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        if let Some((arg1, arg2)) = arg1.split_once('_') {
            match arg2 {
                "begin" => write_log(&format!("{arg1}_begin")),
                "end" => {
                    write_log(&format!("{arg1}_end"));
                }
                _ => {
                    eprintln!("Didn't get it");
                }
            }
        } else {
            match arg1.as_ref() {
                "info" => {
                    display_info();
                }
                "help" | "-h" | "--help" => {
                    print_help();
                }
                _ => {
                    println!("Didn't get it");
                }
            }
        }
    }
}

fn write_log(s: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/Users/carlorosso/.config/programmini/studio.log")
        .unwrap();

    if let Err(e) = writeln!(file, "{} {}", s, Local::now()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn print_help() {
    println!("studio_begin: inizio studio");
    println!("studio_end:   fine studio");
    println!("riposo_begin: inizio riposo");
    println!("riposo_end:   fine riposo");
}
