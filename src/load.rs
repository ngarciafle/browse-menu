use dialoguer::Select;
use std::fs::File;
use std::io::{self, Write, Read};

pub fn load(history: &mut Vec<String>) {
    let entries = std::fs::read_dir("pub").expect("Failed to read directory");
    let mut files: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    files.push(file_name_str.to_string());
                }
            }
        }
    }

    if files.is_empty() {
        println!("No saved states found.");
        history.push("No saved states found".to_string());
        return;
    }

    let selection = Select::new()
        .with_prompt("Select a file to load")
        .items(&files)
        .default(0)
        .interact()
        .expect("Failed to read selection");

    println!("Loading state...");
    let selected_file = &files[selection];
    history.push(format!("Loaded state from: {}", selected_file));

    let mut file = File::open(format!("pub/{}", selected_file)).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read file");
    for line in contents.lines() {
        history.push(line.to_string());
        println!("L -> {}", line);
    }
}