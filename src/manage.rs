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

    if selection == 0 {
        history.push("Add User".to_string());

    } else if selection == 1 {
        history.push("Delete User".to_string());
    } else if selection == 2 {
        history.push("List Users".to_string());

    } else if selection == 3 {
        history.push("Read db".to_string());
        let mut urls = conn.prepare("SELECT * FROM urls").expect("Failed to prepare statement");
        let url_iter = urls.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let url: String = row.get(1)?;
            Ok((id, url))
        }).expect("Failed to query urls");

        for url in url_iter {
            match url {
                Ok((id, url)) => {
                    println!("ID: {}, URL: {}", id, url);
                }
                Err(err) => {
                    // println!("Error reading URL: {}", err);
                }
            }
        }

    } else if selection == 4 {
        history.push("Exit".to_string());

    } else {
        history.push("Invalid selection".to_string());
        panic!("Invalid selection");
    }
}