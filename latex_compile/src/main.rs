use notify::Result;
use notify_debouncer_mini::new_debouncer;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut out = "./site".to_string();
    let mut src = "./".to_string();
    let mut only_file = (false, "".to_string());

    let args = env::args().collect::<Vec<_>>();
    let mut args_iter = args[1..].iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-o" => out = args_iter.next().unwrap().to_string(),
            "-s" => src = args_iter.next().unwrap().to_string(),
            "-f" => only_file = (true, args_iter.next().unwrap().to_string()),
            _ => {
                // help
                println!("Usage: {} [-o output] [-s source]", args[0]);
                return Ok(());
            }
        }
    }

    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(std::time::Duration::from_secs(1), tx)?;
    let src = PathBuf::from(&src);
    let ext = ["aux", "log", "out", "toc"];

    debouncer
        .watcher()
        .watch(&PathBuf::from(&src), notify::RecursiveMode::Recursive)?;

    let cmd = |mut path: PathBuf| {
        let mut command = std::process::Command::new("pdflatex");
        if only_file.0 {
            path = src.join(&only_file.1);
        }
        command
            .arg("-output-directory")
            .arg(&out)
            .arg(path.to_str().unwrap());
        match command.output() {
            Ok(_) => (),
            Err(e) => println!("Build {} failed: {}", path.to_str().unwrap(), e),
        };
        path
    };

    rx.iter()
        .filter_map(|event| event.ok())
        .flatten()
        .filter(|event| event.path.is_file() && event.path.extension().unwrap() == "tex")
        .map(|tex| cmd(tex.path))
        .map(|tex| cmd(tex))
        .map(|tex| println!("Build {} success", tex.to_str().unwrap()))
        .filter_map(|_| std::fs::read_dir(&out).ok())
        .flatten()
        .filter_map(|path| path.ok())
        .map(|path| path.path())
        .filter(|path| path.is_file() && ext.contains(&path.extension().unwrap().to_str().unwrap()))
        .for_each(|path| {
            std::fs::remove_file(path).unwrap();
        });

    Ok(())
}
