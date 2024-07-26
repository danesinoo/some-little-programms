use std::io::Write;
use std::path::Path;
use std::io::Error;

const PATH: &str = "./diario/";
const HEADER: &str = "---
title: \"Diario\"
author: \"Carlo Rosso\"
date: {{ date }}
---\n";

fn main() -> Result<(), Error> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    let file_name = PATH.to_string() + &date + ".md";
    let path = Path::new(&file_name);

    let mut content = HEADER.to_string();
    content = content.replace("{{ date }}", &date);

    if !path.exists() {
        let mut file = std::fs::File::create(&path)?;
        file.write_all(content.as_bytes())?;
    }

    // open the file with the default editor
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
    let _ = std::process::Command::new(editor).arg(path).status()?;

    Ok(())
}
