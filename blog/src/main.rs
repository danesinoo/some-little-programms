use std::env;
use std::io::Write;

const PATH: &str = "/Users/carlorosso/.config/programmini/site/_";

fn main() -> Result<(), std::io::Error> {
    // get the current date: yyyy-mm-dd
    // create a file with the name of the date
    // if the file already exists, does nothing

    let (arg, private) = match env::args().nth(1) {
        Some(arg) => convert(&arg),
        None => ("pagina".to_string(), false),
    };

    let mut starter = "--- \nlayout: page".to_owned();

    if private {
        starter += "-private";
    }

    starter += "\ntitle: ";
    starter += &chrono::Local::now().format("%d/%m").to_string();
    starter += "\ncategory: ";
    let mut date = chrono::Local::now().format("%Y-%m-%d").to_string();

    date = date.clone();
    starter = starter + &arg + "\n---\n\n";
    let file_name = PATH.to_owned() + &arg + "/" + &date + ".md";
    let path = std::path::Path::new(&file_name);

    if !path.exists() {
        let file = std::fs::File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(starter.as_bytes())?;
    }

    // open the file with the default editor
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
    let status = std::process::Command::new(editor).arg(path).status()?;

    if !status.success() {
        println!("Error: {}", status);
    }

    Ok(())
}

fn convert(arg: &str) -> (String, bool) {
    match arg {
        "page" | "diario" | "d" | "diary" => ("pagina".to_string(), false),
        "p" | "pensiero" | "pensieri" | "public" => ("pensiero".to_string(), false),
        "poesia" | "poesie" | "poem" | "poems" => ("poesia".to_string(), false),
        "r" | "riflessione" | "riflessioni" | "private" => ("riflessione".to_string(), true),
        _ => (arg.to_string(), true),
    }
}
