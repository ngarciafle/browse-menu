use dialoguer::{Input, Password};
use bcrypt::{hash, DEFAULT_COST};

pub fn add_user(history: &mut Vec<String>, conn: &rusqlite::Connection) {
    let username: String = Input::new()
        .with_prompt("Enter the new user's username")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");

    let password: String = Password::new()
        .with_prompt("Enter the new user's password")
        .interact()
        .expect("Failed to read line");

    // Hash the password
    let hashed_password = hash(password, DEFAULT_COST).unwrap();

    // Insert the new user into the database
    match conn.execute(
        "INSERT INTO credentials (username, password) VALUES (?1, ?2)",
        &[&username, &hashed_password],
    ) {
        Ok(_) => {
            println!("User '{}' added successfully.", username);
            history.push(format!("Added user: {}", username));
        }
        Err(e) => {
            println!("Failed to add user '{}': {}", username, e);
            history.push(format!("Failed to add user: {} - {}", username, e));
        }
    }

}