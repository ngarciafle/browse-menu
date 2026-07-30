use dialoguer::Input;
use dialoguer::Password;
use dialoguer::Select;
use bcrypt::{hash, DEFAULT_COST};
use rusqlite::Connection;
// Maybe import log in ??

pub fn remove_user(history: &mut Vec<String>, conn: &Connection) {
    // Must change expect to match to handle errors


    // First check if admin is logged in & log in case
    // Implementing showing users & then asking for the password

    // Then show usernames
    let mut stmt = conn.prepare("SELECT username FROM credentials").expect("Failed to prepare statement");
    let usernames_iter = stmt.query_map([], |row| row.get(0)).expect("Failed to query usernames");
    let usernames: Vec<String> = usernames_iter.map(|res| res.expect("Failed to get username")).collect();

    let username = Select::new()
        .with_prompt("Select a user to remove")
        .items(&usernames)
        .default(0)
        .interact()
        .expect("Failed to read selection");
    let selected_username: String = usernames[username].clone();

    // Check the password
    let password: String = Password::new()
        .with_prompt("Enter your password")
        .interact()
        .expect("Failed to read line");

    let hashed_password: String = conn.query_row(
        "SELECT password FROM credentials WHERE username = ?1",
        [selected_username.clone()],
        |row| row.get(0),
    ).expect("Failed to get hashed password");

    if !bcrypt::verify(password, &hashed_password).unwrap_or(false) {
        println!("Incorrect password.");
        history.push(format!("Failed to remove user: {} - incorrect password", selected_username));
        return;
    }

    // Delete the user from the database
    match conn.execute(
        "DELETE FROM credentials WHERE username = ?1",
        &[&selected_username],
    ) {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                println!("User '{}' removed successfully.", selected_username);
                history.push(format!("Removed user: {}", selected_username));
            } else {
                println!("User '{}' not found.", selected_username);
                history.push(format!("Failed to remove user: {} - not found", selected_username));
            }
        }
        Err(e) => {
            println!("Failed to remove user '{}': {}", selected_username, e);
            history.push(format!("Failed to remove user: {} - {}", selected_username, e));
        }
    }
    //Check if there are no users left in the database, if so, create a new admin user
}