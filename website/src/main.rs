use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let mut arg = match env::args().nth(1) {
        Some(arg) => arg.to_string(),
        None => String::from(""),
    };

    let content = file_to_str(&arg)?;

    let md_content = content.split("---").collect::<Vec<&str>>();
    if md_content.len() < 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Layout not found",
        ));
    }

    /* we can assume md_content.len() > 3
     * where: md_content[0] = ""
     * md_content[1]: settings
     * md_content[2]: content
     */

    let mut vars: HashMap<&str, String> = get_vars(&md_content[1]);

    vars.insert("content", markdown::to_html(&md_content[2..].join("---")));

    let base = file_to_str(&"layout/{}.html".replace("{}", &vars["layout"]))?;

    let mut html = replace_use(&base);
    html = replace_vars(&html, &vars);

    arg = arg.replace(".md", ".html");

    // Write the new content to the file.
    if let Err(err) = fs::write(&arg, html) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("File saved successfully.");
    }
    Ok(())
}

fn file_to_str(path: &str) -> Result<String, std::io::Error> {
    let _path = std::path::Path::new(path);
    match std::fs::read_to_string(_path) {
        Ok(content) => Ok(content),
        Err(e) => {
            println!("Path: {}\n", path);
            Err(e)
        }
    }
}

fn get_vars(content: &str) -> HashMap<&str, String> {
    let mut vars: HashMap<&str, String> = HashMap::new();
    content
        .split("\n")
        .filter(|s| s.contains(": "))
        .map(|s| {
            let mut res = s.splitn(2, ": ");
            (res.next().unwrap(), res.next().unwrap().trim())
        })
        .for_each(|(key, value)| {
            vars.insert(key, value.to_string());
        });
    vars
}

fn replace_use(content: &str) -> String {
    let mut html = String::new();
    for s in content.lines() {
        if s.starts_with("use") {
            html += &file_to_str(&("layout/".to_string() + s[4..].trim() + ".html")).unwrap();
        } else {
            html = html + "\n" + s;
        }
    }
    html
}

// takes in a string and a hashmap of variables
// replaces all instances of {{ var }} with the value of var
fn replace_vars(content: &str, vars: &HashMap<&str, String>) -> String {
    let mut html = String::new();

    let mut iter = content.split("{{");
    html += iter.next().unwrap();
    for s in iter {
        let mut iter = s.splitn(2, "}}");
        let var = iter.next().unwrap().trim();
        html += &vars.get(var).unwrap_or(&String::from("")).to_string();
        html += iter.next().unwrap();
    }
    html
}
