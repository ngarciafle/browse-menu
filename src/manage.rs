use dialoguer::Input;
use dialoguer::Password;
use dialoguer::Select;
use crate::log_in::log_in;
use rusqlite::Connection;


pub fn manage(history: &mut Vec<String>, conn: &rusqlite::Connection) {
    // let user: String = Input::new()
    //     .with_prompt("Enter your username")
    //     .show_default(false)
    //     .interact_text()
    //     .expect("Failed to read line");

    // history.push(format!("Manage: {}", user.clone()));
    
    // let password: String = Password::new()
    //     .with_prompt("Enter your password")
    //     .interact()
    //     .expect("Failed to read line");
    
    let logged: bool = log_in(history, &conn);

    if !logged {
        println!("Incorrect password.");
        history.push("Failed Manage".to_string());
        return;
    }

    println!("Welcome!");
    history.push("Manage".to_string());

    let choices = vec!["Add User", "Delete User", "List Users", "Read db", "Exit"];
    let selection = Select::new()
        .with_prompt("How are you feeling?")
        .items(&choices)
        .default(0)
        .interact()
        .expect("Failed to read selection");
}