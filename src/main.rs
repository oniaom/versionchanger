use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};


fn handle_args(args: Vec<String>) -> Option<String> {
    // Program should only accept 2 args.
    if args.len() < 2 || args.len() > 2 {
        return None;
    }
    return Some(args[1].to_string());
}

fn replace_version(file: &str) -> Result<(), std::io::Error> {
    // Use regex to change "?version=x" inside file

    let file1: File = File::open(file)?;
    let reader = io::BufReader::new(file1);

    // Define a regular expression to match "?version=" and a number
    let re: Regex = Regex::new(r"\?version=(\d+)").unwrap();
    let mut new_file_content: String = String::new();

    for line in reader.lines() {
        let line = line?;
        let new_line = re.replace_all(&line, |caps: &regex::Captures| {
            let num = caps.get(1).unwrap().as_str().parse::<u32>().unwrap() + 1;
            format!("?version={}", num)
        });

        new_file_content.push_str(&new_line);
        new_file_content.push('\n');
    }

    fs::write(file, new_file_content)?;

    Ok(())
}

fn get_folder_path(file: String) -> Option<String> {
    // Return the path of the file, without the file name
    let path = Path::new(&file);
    let parent = path.parent();

    // Check if there is a parent directory
    match parent {
        Some(parent_path) => Some(parent_path.to_string_lossy().to_string()),
        None => None,
    }
}

fn get_file_name(file: String) -> Option<String> {
    // Returns the file name from the path given

    let path = Path::new(&file);
    let file_name: Option<&std::ffi::OsStr> = path.file_name();

    match file_name {
        Some(name) => Some(String::from(name.to_string_lossy())),
        None => None,
    }
}

fn backup_old_file(file: String) -> Option<bool> {
    // Copies file to file.old for backup
    let folder_path_option = get_folder_path(file.clone());
    if folder_path_option.is_none() {
        println!("Error parsing folder path.");
        return None;
    }

    let folder_path: String = folder_path_option.unwrap();
    let file_name_option = get_file_name(file.clone());

    let mut _file_name: String = String::new();
    match file_name_option {
        Some(name) => _file_name = name,
        None => return None,
    }

    let mut final_path = PathBuf::from(folder_path.clone());
    let backup_file_name: String = _file_name + ".old";
    final_path.push(backup_file_name);
    let x = String::from(final_path.to_str().unwrap());

    let copy_res = fs::copy(file, x.clone());
    match copy_res {
        Ok(_) => {}
        Err(error_message) => {
            println!(
                "Something went wrong creating the backup file: {} {}",
                error_message, x
            );
            return None;
        }
    }
    println!("folder: {folder_path}");
    println!("created: {x}");
    return Some(true);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: Option<String> = handle_args(args);
    if target.is_none() {
        println!("No file selected");
        return;
    }
    let file: String = target.unwrap();

    // Backup old file first!
    let backup_ok = backup_old_file(file.clone());
    if backup_ok.is_none() {
        println!("Something bad in backup_old_file");
        return;
    }

    let res: Result<(), io::Error> = replace_version(&file);
    match res {
        Ok(_) => {}
        Err(err) => println!("{err}"),
    }
}
