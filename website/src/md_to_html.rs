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
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

extern crate rayon;

const IGNORE_FOLDERS: [&str; 6] = ["layout", "assets", "target", "src", ".git", ".github"];

const IGNORE_FILES: [&str; 1] = ["README.md"]; // only .md files are converted

/// The main function
///
/// Input:
/// - ```target```: the path to the directory which contains the markdown files
/// - ```destination```: the path to the directory in which the html files are
/// going to be saved
pub fn md_to_html(target: &str, destination: &str) -> Result<(), std::io::Error> {
    // vars contiene le variabili che sono sostituite:
    // {{ key }} -> value
    // si può creare un file che contiene dei valori di default
    // e si inizializza vars con tali valori
    let mut vars = HashMap::new();

    vars.insert("DESTINATION", destination.to_string());
    vars.insert("TARGET", target.to_string());

    // create the destination directory if it doesn't already exist
    if !PathBuf::from(&vars.get("DESTINATION").unwrap()).exists() {
        fs::create_dir(&vars.get("DESTINATION").unwrap())?;
    }

    // create the collections
    vars.insert("base-path", "../".to_string());

    let folders = get_folders(&PathBuf::from(vars.get("TARGET").unwrap()));

    folders
        .par_iter()
        .map(|folder| {
            // create the folder in the destination directory
            let mut d = PathBuf::from(&vars.get("DESTINATION").unwrap());
            d.push(folder.file_name().unwrap());

            if !d.exists() {
                fs::create_dir(&d).unwrap();
            }

            // generate the html files for each markdown file in the folder
            make_collection(folder, &vars).unwrap()
        })
        // save the html files
        .for_each(|(html_files, links)| {
            html_files
                .iter()
                .zip(links.iter())
                .for_each(|(html, path)| {
                    fs::write(vars.get("DESTINATION").unwrap().to_string() + path, html).unwrap();
                });
        });

    // arg should now be the -t flag: the path to a directory which contains makdown files
    Ok(())
}

/// Generates the index.html and convert every .md file to .html
/// for each folder in the target directory
#[allow(non_snake_case)]
fn make_collection(
    folder: &PathBuf,
    VARS: &HashMap<&str, String>,
) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    // I wonder, should I return even a vector of PathBuf?,
    // which has the path in which the html files should be saved in?
    let files = fs::read_dir(&folder).unwrap();

    let mut vars = VARS.clone();
    vars.insert(
        "folder",
        folder.file_name().unwrap().to_str().unwrap().to_string(),
    );

    let mut html_filename = files
        .map(|file| {
            file.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .filter(|file| file.ends_with(".md"))
        .filter(|file| !IGNORE_FILES.contains(&file.as_str()))
        .map(|file| file.replace(".md", ".html"))
        .map(|file| vars.get("folder").unwrap().to_string() + "/" + &file)
        .collect::<Vec<_>>();

    // if there are no markdown files in the folder, skip it
    if html_filename.len() < 1 {
        return Ok((Vec::new(), Vec::new()));
    }

    html_filename.sort_by(|a, b| b.to_lowercase().cmp(&a.to_lowercase()));

    // the next-page of the first post is the index.html
    vars.insert(
        "next-page",
        vars.get("folder").unwrap().to_string() + "/index.html",
    );

    // I pass the first post as previous-page, this way previous-page
    // contains the actual-post.
    // The previous-page is passed as input through the iterator
    // check how make_post works
    vars.insert("previous-page", html_filename[0].to_string());

    // generate the list for index.html
    let index = get_links(&html_filename, &mut vars);
    vars.insert("links", index);

    // add the link to the index.html, which is going to be the previos-page
    // of the last post
    html_filename.push(html_filename[0].clone());

    // build the html files for each markdown file
    let html_files = html_filename[1..]
        .iter()
        .map(|md| make_post(&PathBuf::from(md), &mut vars).unwrap())
        .collect::<Vec<_>>();

    Ok((html_files, html_filename))
}

fn get_folders(path: &PathBuf) -> Vec<PathBuf> {
    let mut folders = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        folders = entries
            .map(|dir| dir)
            .filter(|dir| dir.as_ref().unwrap().file_type().unwrap().is_dir())
            .map(|dir| dir.unwrap().path())
            .collect::<Vec<_>>();
    } else {
        eprintln!("Error reading the directory.");
    }

    folders.retain(|x| !IGNORE_FOLDERS.contains(&x.to_str().unwrap()));

    folders
}

/// Generates the list of links of a collection formatted as an html list
///
/// Input:
/// - links: a vector of the path to the posts relative to the target directory (e.g. "folder1/post1.html")
/// - vars: a hashmap of the global variables
/// use the "list" layout to generate the index.html  
/// where ```{{ links }}``` is replaced with the list of links
fn get_links(links: &[String], vars: &HashMap<&str, String>) -> String {
    let mut list = String::new();
    for link in links {
        if link.contains("index") {
            continue;
        }
        list += &format!(
            "<li><a href=\"{}\">{}</a></li>\n",
            link,
            post_title(link, vars.get("TARGET").unwrap())
        );
    }
    list
}

/// Extract the title of a post from its settings
///
/// ```markdown
/// --- <!-- settings section -->
/// title: *Title example*
/// ...
/// ---
/// ...
fn post_title(path: &str, target: &str) -> String {
    let path = target.to_string() + &path.replace(".html", ".md");
    let content = fs::read_to_string(path).unwrap();
    content.lines().find(|s| s.starts_with("title: ")).unwrap()[7..].to_string()
}

/// Generates the html file for a post
///
/// Look for the layout in the settings section of the markdown file
/// and use it to generate the html file, through the ```to_html``` function
///
/// Input:
/// - ```path```: the path to the markdown file which corresponds to ```{{ previous-page }}```
/// - ```vars```: a hashmap of the global variables
///
/// Global variables used:
/// - ```vars["previous-page"]```: it needs to contain the path to the markdown file which is
/// going to be built
/// - ```vars["next-page"]```: it needs to contain the path to the markdown file which corresponds
/// to ```{{ previous-page }}```
///
/// Output:
/// - Ok(html): the html String
/// - Err(error): the error
///
/// The global variables are updated:
/// - ```vars["previous-page"] = path```
/// - ```vars["next-page"] = vars["previous-page"]```
fn make_post(path: &PathBuf, vars: &mut HashMap<&str, String>) -> Result<String, std::io::Error> {
    let current_page = vars
        .insert("previous-page", path.to_str().unwrap().to_string())
        .unwrap()
        .replace(".html", ".md");

    let content =
        match fs::read_to_string(&format!("{}{}", vars.get("TARGET").unwrap(), current_page)) {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };

    let mut var_tmp = match get_vars(&content) {
        Ok(hash) => hash,
        Err(e) => {
            return Err(e);
        }
    };
    vars.iter().for_each(|(key, value)| {
        if !var_tmp.contains_key(key) {
            var_tmp.insert(key, value.to_string());
        }
    });

    let s = match to_html(&var_tmp) {
        Ok(html) => html,
        Err(e) => {
            return Err(e);
        }
    };
    vars.insert("next-page", current_page.replace(".md", ".html"));
    Ok(s)
}

/// Collects the variables from the settings section of the markdown file
///
/// ## Rule:
/// - the keys cannot be in capslock
///
/// Input:
/// - ```content```: the content of the markdown file
///
/// Output:
/// - Ok(vars): a hashmap of the variables
/// - Err(error): the error
///
/// The content of the markdown has the following structure:
///
/// ```markdown
/// ---  <!-- this is the settings section -->
/// key1: value1
/// ...
/// keyN: valueN
/// ---
///
/// <!-- this is the content section -->
///
/// # Title
///
/// content
/// ```
fn get_vars(content: &str) -> Result<HashMap<&str, String>, std::io::Error> {
    let md_content = content.split("---").collect::<Vec<&str>>();
    if md_content.len() < 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Layout not found",
        ));
    }

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

    let content = replace_vars(&md_content[2..].join("---"), &vars);
    vars.insert("content", markdown::to_html(&content));
    Ok(vars)
}

/// Generates the html file
///
/// 1. Looks for the value of the "layout" key in the vars hashmap
/// 1. Replaces all instances of "use file_name" with the contents of "file_name.html"  
/// 1. Replaces all instances of {{ var }} with the value of var
///
/// Input:
/// - ```vars```: a hashmap of the variables
/// - ```vars["layout"]```: the name of the layout to use
/// - ```vars["content"]```: the content of the html file
/// - ```vars["base-path"]```: the path to the root directory
/// - ```vars["title"]```: the title of the html file
///
///
/// Output:
/// - Ok(html): the html String
/// - Err(error): the error
fn to_html(vars: &HashMap<&str, String>) -> Result<String, std::io::Error> {
    // here I should be able to get the target: solution insert it in the HashMap
    let mut layout = vars.get("TARGET").unwrap().to_string() + "layout/";
    layout = layout + vars.get("layout").unwrap() + ".html";
    let mut s = fs::read_to_string(&layout)?;
    s = replace_use(&s, &vars.get("TARGET").unwrap());
    Ok(replace_vars(&s, &vars))
}

/// Replaces all instances of ```use file_name``` with the contents of ```layout/file_name.html```
fn replace_use(content: &str, target: &str) -> String {
    let mut html = String::new();
    for s in content.lines() {
        if s.starts_with("use") {
            let layout = format!("{}layout/", target) + s[4..].trim() + ".html";
            html += &fs::read_to_string(&layout).unwrap();
        } else {
            html = html + "\n" + s;
        }
    }
    html
}

/// Replaces all instances of ```{{ var }}``` with the value of var
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
