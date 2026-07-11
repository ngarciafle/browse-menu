use dialoguer::Input;
use std::fs;
use std::io::{self, Write, Read};

struct State {
    name: String,
    commands: Vec<String>,
}

pub fn save(history: &mut Vec<String>) {
    let input: String = Input::new()
        .with_prompt("Enter the name of the file to save")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");

    // Save the state with the name
    let state = State {
        name: input.clone(),
        commands: history.clone(),
    };

    let mut text = String::new();
    for command in &state.commands {
        text.push_str(command);
        text.push('\n');
    }

    let route: String = format!("pub/{input}.txt");

    if let Err(e) = fs::create_dir_all("pub") {
        panic!("No se pudo crear la carpeta 'pub': {}", e);
    }

    let mut file = fs::File::create(&route).expect("Failed to create file (check names)");
    file.write_all(text.as_bytes()).expect("Failed to write to file");

    println!("Saving state...");
}