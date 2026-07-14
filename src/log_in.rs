use rusqlite::{Connection, Result};
use rusqlite::OptionalExtension;
use dialoguer::{Input, Password};


pub fn log_in(history: &mut Vec<String>, conn: &Connection) -> bool {
    let admin_username: String = Input::new()
            .with_prompt("Enter the admin username")
            .show_default(false)
            .interact_text()
            .expect("Failed to read line");
        let admin_password: String = Password::new()
            .with_prompt("Enter the admin password")
            .interact()
            .expect("Failed to read line");

        let result_cred_search: Result<Option<String>, _> = conn.query_row(
            "SELECT password FROM credentials WHERE username = ?1",
            [&admin_username],
            |row| row.get(0),
        ).optional();

        let is_valid = match result_cred_search {
            Ok(Some(res)) => {
                bcrypt::verify(admin_password, &res).unwrap_or(false)
            }
            Ok(None) => false,
            Err(_) => false,
        };

        return is_valid;
}