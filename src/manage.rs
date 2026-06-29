use dialoguer::Input;
use dialoguer::Password;


pub fn manage() {
    let user: String = Input::new()
        .with_prompt("Enter your username")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");
    
    let password: String = Password::new()
        .with_prompt("Enter your password")
        .interact()
        .expect("Failed to read line");
    
    if password == "admin" {
        println!("Welcome, {user}!");
    } else {
        println!("Incorrect password.");
    }
}