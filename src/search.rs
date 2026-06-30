use dialoguer::Input;

pub fn search(history: &mut Vec<String>) {
    let input: String = Input::new()
        .with_prompt("What do you want to search for? ")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");

    history.push(format!("Searched for: {input}"));
    println!("Searching for: {input}");
}