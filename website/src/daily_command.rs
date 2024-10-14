const COMMANDS: &[&str] = &["brew update", "brew upgrade", "brew cleanup"];

pub async fn execute_daily() {
    // set a timer for the next day
    let secs_per_day = 24 * 60 * 60;

    loop {
        COMMANDS.iter().for_each(|command| {
            println!("Executing: {}", command);
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process");
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        });

        std::thread::sleep(std::time::Duration::from_secs(secs_per_day));
    }
}
