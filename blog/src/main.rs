use std::env;
use std::io::Write;

const PATH: &str = "/Users/carlorosso/.config/programmini/note/src/";

const STARTER: &str = "--- \nlayout: {{ layout }}
title: {{ date }}
category: {{ topic }}
---\n\n";

struct Note {
    topic: String,
}

const PAGES: [&str; 3] = ["page", "pensiero", "poesia"];
const PRESENTATIONS: [&str; 1] = ["preview"];

impl Note {
    fn new(s: &str) -> Note {
        let topic = match s {
            "page" | "diario" | "d" | "diary" => "pagina",
            "p" | "pensiero" | "pensieri" | "public" => "pensiero",
            "poesia" | "poesie" | "poem" | "poems" => "poesia",
            "r" | "riflessione" | "riflessioni" | "private" => "riflessione",
            _ => s,
        };

        Note {
            topic: topic.to_string(),
        }
    }

    fn layout(&self) -> String {
        if PAGES.contains(&self.topic.as_str()) {
            "page".to_string()
        } else if PRESENTATIONS.contains(&self.topic.as_str()) {
            "preview".to_string()
        } else {
            "page-private".to_string()
        }
    }

    fn header(&self) -> Vec<u8> {
        let mut header = STARTER.to_string();
        header = header.replace("{{ layout }}", &self.layout());
        header = header.replace(
            "{{ date }}",
            &chrono::Local::now().format("%d/%m").to_string(),
        );
        header
            .replace("{{ topic }}", &self.topic)
            .as_bytes()
            .to_vec()
    }

    fn name(&self) -> String {
        PATH.to_owned()
            + &self.topic
            + "/"
            + &chrono::Local::now().format("%Y-%m-%d").to_string()
            + ".md"
    }

    fn path(&self) -> std::path::PathBuf {
        std::path::Path::new(&self.name()).to_path_buf()
    }

    fn store(&self) -> Result<(), std::io::Error> {
        if !self.path().exists() {
            match std::fs::File::create(self.path()) {
                Ok(file) => {
                    let mut writer = std::io::BufWriter::new(file);
                    writer.write_all(&self.header())?;
                }
                Err(_) => {
                    eprintln!("You do not have notes with such name :)");
                    std::process::exit(1);
                }
            }
        }
        Ok(())
    }

    fn open_editor(&self) -> Result<(), std::io::Error> {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
        let status = std::process::Command::new(editor)
            .arg(self.path())
            .status()?;

        if !status.success() {
            println!("Error: {}", status);
        }

        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    let note = match env::args().nth(1) {
        Some(arg) => Note::new(&arg),
        None => Note {
            topic: "pagina".to_string(),
        },
    };

    note.store()?;
    note.open_editor()
}
