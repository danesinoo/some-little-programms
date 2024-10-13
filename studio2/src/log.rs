use chrono;
use std::path;
use std::io::{Write, Read};

pub trait Log {
    fn write(&self, value: &str);

    fn read(&self) -> String;

    fn begin(&self, value: &str) {
        self.write(format!("{},{},{}", value, "begin", chrono::Local::now().format("%Y-%m-%d %H:%M")).as_str());
    }

    fn end(&self, value: &str) {
        self.write(format!("{},{},{}", value, "end", chrono::Local::now().format("%Y-%m-%d %H:%M")).as_str());
    }
}

pub struct FileLog {
    file: path::PathBuf,
}

impl FileLog {
    pub fn new(file: &str) -> FileLog {
        let path = path::PathBuf::from(file);

        if !path.exists() {
            std::fs::File::create(&path).unwrap();
        }

        FileLog {
            file: path::PathBuf::from(file),
        }
    }
}

impl Log for FileLog {
    fn write(&self, value: &str) {
        let mut file = std::fs::OpenOptions::new().append(true).open(&self.file).unwrap();
        writeln!(file, "{}", value).unwrap();
    }

    fn read(&self) -> String {
        let mut file = std::fs::File::open(&self.file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}
