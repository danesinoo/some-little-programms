//! # Markdown to html
//!
//! This program converts a directory of markdown files to html files.
//! The structure of the directory needs to be the following:
//! ```markdown
//! target
//! ├── folder1
//! │   ├── file1.md
//! │   ├── file2.md
//! │   └── ...
//! ├── layout
//! │   ├── some_layout.md
//! │   └── ...
//! ├── assets
//! │   ├── some_asset.md
//! │   └── ...
//! └── ...
//! ```
//!
//! Once the program is run, it will wait for an event to occur in the target
//! directory.
//!
//! ## Idee
//! - usare axum per creare un server che serve i file html (molto simile a actix-web)

use notify::{RecursiveMode, Result};
mod md_to_html;
use chrono::Local;
use md_to_html::md_to_html;
use notify_debouncer_mini::new_debouncer;
use std::env;
use std::path::PathBuf;
mod daily_command;
mod service;
use actix_web::{web, App, HttpServer};
use daily_command::execute_daily;
use service::{idx, pages};

#[tokio::main]
async fn main() -> Result<()> {
    let (src, out, s) = get_input();

    std::thread::spawn(move || html_generator(&src, &out));
    std::thread::spawn(move || execute_daily());

    if s {
        let addr = ("127.0.0.1", 10000);
        let server = tokio::task::spawn(start_blog(addr));
        println!("Server running at http://{}:{}", addr.0, addr.1);

        if let Err(e) = server.await {
            println!("server error: {}", e);
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn get_input() -> (String, String, bool) {
    let mut out = "./site".to_string();
    let mut src = "./".to_string();
    let mut s = false;

    let args = env::args().collect::<Vec<_>>();
    let mut args_iter = args[1..].iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-s" => src = args_iter.next().unwrap().to_string(),
            "-o" => out = args_iter.next().unwrap().to_string(),
            "--serve" => {
                s = true;
            }
            _ => {
                // help
                println!(
                    "Usage: {} [-s source] [-o output] [--serve]\n entered {}, instead",
                    args[0], arg
                );
                std::process::exit(0);
            }
        }
    }
    (src, out, s)
}

fn html_generator(target: &str, dest: &str) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(std::time::Duration::from_secs(1), tx)?;

    debouncer
        .watcher()
        .watch(&PathBuf::from(target), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(_) => match md_to_html(&target, &dest) {
                // print DD/MM/YYYY-HH:MM:SS
                Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
                Err(e) => println!("error: {:?}", e),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn start_blog(addr: (&str, u16)) {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(idx))
            .route("/{path:.*}", web::get().to(pages))
    })
    .bind(addr)
    .unwrap()
    .run()
    .await
    .unwrap();
}
