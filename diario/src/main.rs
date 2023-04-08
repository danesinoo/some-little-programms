const PATH: &str = "/Users/carlorosso/.config/programmini/buff/diario/";

fn main() -> Result<(), std::io::Error> {
    // get the current date: yyyy-mm-dd
    // create a file with the name of the date
    // if the file already exists, does nothing

    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let fname = format!("{}{}.md", PATH, date);
    let path = std::path::Path::new(&fname);

    if !path.exists() {
        std::fs::File::create(path)?;
    }

    // open the file with the default editor
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
    let status = std::process::Command::new(editor).arg(path).status()?;

    if !status.success() {
        println!("Error: {}", status);
    }

    Ok(())
}
