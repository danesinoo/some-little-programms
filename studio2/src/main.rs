use generic;
mod log;
mod session;
mod pause;
mod report;

use log::{Log, FileLog};
use pause::{Pause, PauseImpl};

fn main() {
    let args = generic::get_args();
    let help = build_help();

    for (key, value) in args.iter() {
        match key.as_str() {
            "begin" => begin(value),
            "end" => end(value),
            "info" => info(value),
            "pause" => pause(value),
            "help" => {
                help.print();
                break;
            }
            _ => {
                println!("Unknown argument: {}", key);
                break;
            }
        }
    }
}

fn build_help() -> generic::Help {
    let mut help = generic::Help::new();
    help.add("help", "Print help message");
    help.add("version", "Print version");
    help
}

const LOG_FILE: &str = "/Users/carlorosso/.config/programmini/buff/log.csv";

pub fn begin(value: &str) {
    let log = FileLog::new(LOG_FILE);
    log.begin(value);
}

pub fn end(value: &str) {
    let log = FileLog::new(LOG_FILE);
    log.end(value);
}

pub fn info(value: &str) {
    println!("Session info: {}", value);
}

pub fn pause(value: &str) {
    let log = FileLog::new(LOG_FILE);
    let pause = PauseImpl::new(log);
    pause.pause(value);
}
