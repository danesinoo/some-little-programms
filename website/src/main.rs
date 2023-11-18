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
use notify::{RecursiveMode, Result};
mod md_to_html;
use chrono::Local;
use md_to_html::md_to_html;
use notify_debouncer_mini::new_debouncer;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Replace "path/to/your/source/directory" with the actual path to your source directory
    let mut dest = "./site".to_string();
    let mut target = "./".to_string();

    let args = env::args().collect::<Vec<_>>();
    let mut args_iter = args[1..].iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-d" => dest = args_iter.next().unwrap().to_string(),
            "-t" => target = args_iter.next().unwrap().to_string(),
            _ => {
                // help
                println!("Usage: {} [-d destination] [-t target]", args[0]);
                return Ok(());
            }
        }
    }

    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(std::time::Duration::from_secs(1), tx).unwrap();

    debouncer
        .watcher()
        .watch(&PathBuf::from(target.clone()), RecursiveMode::Recursive)
        .unwrap();

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
    /*
    while let Ok(event) = rx.recv() {
        match event.kind {
            EventKind::Create(CreateKind::Any) | EventKind::Modify(ModifyKind::Any) => {
                match md_to_html(&target, &dest) {
                    Ok(_) => println!("success"),
                    Err(e) => println!("error: {:?}", e),
                }
            }
            _ => {}
        };
    }
    */

    Ok(())
}
