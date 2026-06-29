use dialoguer::Input;

pub fn search() {
    let input: String = Input::new()
        .with_prompt("What do you want to search for? ")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");

    println!("Searching for: {input}");
}