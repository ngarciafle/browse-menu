use dialoguer::Input;
use dialoguer::Password;
use dialoguer::Select;
use bcrypt::{hash, DEFAULT_COST};
use rusqlite::Connection;
// Maybe import log in ??

pub fn remove_user(history: &mut Vec<String>, conn: &Connection) {
    // First check if admin is logged in & log in case

    // Then show usernames

    // Delete the user from the database
    match conn.execute(
        "DELETE FROM credentials WHERE username = ?1",
        &[&username],
    ) {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                println!("User '{}' removed successfully.", username);
                history.push(format!("Removed user: {}", username));
            } else {
                println!("User '{}' not found.", username);
                history.push(format!("Failed to remove user: {} - not found", username));
            }
        }
        Err(e) => {
            println!("Failed to remove user '{}': {}", username, e);
            history.push(format!("Failed to remove user: {} - {}", username, e));
        }
    }
}