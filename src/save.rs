use dialoguer::Input;

pub fn save() {
    let input: String = Input::new()
        .with_prompt("Enter the name of the file to save")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");

    // Save the state with the name

    println!("Saving state...");
}