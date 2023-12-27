use crate::show_data;
use chrono::Local;
use std::fs;

pub fn pausa(session: &str) {
    let content = fs::read_to_string(show_data::path()).unwrap();
    content.lines().rev().for_each(|line| {
        if line.contains(session) {
            let time_as_string = line.split_whitespace().collect::<Vec<&str>>()[2..].join(" ");
            let start_time = show_data::parse_time(&time_as_string);

            let now = Local::now();

            let diff = now.signed_duration_since(start_time);

            let hours = diff.num_hours();
            let minutes = diff.num_minutes() - hours * 60;
            println!("{}h {}min", hours, minutes);
            std::process::exit(0);
        }
    });
}
