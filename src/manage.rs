use dialoguer::Input;
use dialoguer::Password;


pub fn manage(history: &mut Vec<String>) {
    let user: String = Input::new()
        .with_prompt("Enter your username")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");

    history.push(format!("Manage: {}", user.clone()));
    
    let password: String = Password::new()
        .with_prompt("Enter your password")
        .interact()
        .expect("Failed to read line");
    
    if password == "admin" {
        println!("Welcome, {user}!");
        history.push("Manage".to_string());
    } else {
        println!("Incorrect password.");
        history.push("Failed Manage".to_string());
    }
}