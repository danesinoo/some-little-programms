use crate::log::{Log, FileLog};
use chrono::{self, NaiveDateTime};

pub trait Pause {
    fn new(log: FileLog) -> Self;

    fn log(&self) -> &dyn Log;

    fn pause(&self, value: &str) {
        let content = self.log().read();

        match content.lines().rev().find(|line| {
            line.contains("begin") && line.contains(value)
        }) {
            Some(line) => {
                let vals = line.split(",").collect::<Vec<&str>>();
                let time = NaiveDateTime::parse_from_str(vals[2], "%Y-%m-%d %H:%M").unwrap();
                let difference = chrono::Local::now()
                    .naive_local()
                    .signed_duration_since(time);
                let mut hours = difference.num_hours().to_string();
                let mut minutes = (difference.num_minutes() % 60).to_string();
                if hours.len() < 2 {
                    hours = format!("0{}", hours);
                }
                if minutes.len() < 2 {
                    minutes = format!("0{}", minutes);
                }
                println!("{}:{}", hours, minutes);
            },
            None => {
                println!("No session paused");
            }
        }
    }
}

pub struct PauseImpl {
    log: FileLog,
}

impl Pause for PauseImpl {
    fn new(log: FileLog) -> Self {
        PauseImpl { log }
    }

    fn log(&self) -> &dyn Log {
        &self.log
    }
}
