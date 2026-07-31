use dialoguer::Input;
use dialoguer::Password;
use dialoguer::Select;
use crate::log_in::log_in;
use crate::add_user::add_user;
use crate::remove_user::remove_user;
use crate::list_users::list_users;
use rusqlite::Connection;


pub fn manage(history: &mut Vec<String>, conn: &rusqlite::Connection, logged: &mut bool) {
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
    if !*logged {
        *logged = log_in(history, &conn);
    }
    

    if !*logged {
        println!("Incorrect password.");
        history.push("Failed Manage".to_string());
        return;
    }

    println!("Welcome!");
    history.push("Manage".to_string());
    
    let choices = vec!["Add User", "Delete User", "List Users", "Read db", "Close Session", "Exit"];

    loop {
        let selection = Select::new()
            .with_prompt("How are you feeling?")
            .items(&choices)
            .default(0)
            .interact()
            .expect("Failed to read selection");
    
        if selection == 0 {
            history.push("Add User".to_string());
            add_user(history, &conn);
    
        } else if selection == 1 {
            history.push("Delete User".to_string());
            remove_user(history, &conn);
        } else if selection == 2 {
            history.push("List Users".to_string());
            list_users(history, &conn);
    
        } else if selection == 3 {
            history.push("Read db".to_string());
            let mut urls = conn.prepare("SELECT id, url, counter, title FROM crawl").expect("Failed to prepare statement");
            // **
            let url_iter = urls.query_map([], |row| {
                let id: i32 = row.get(0)?;
                let url: String = row.get(1)?;
                let counter: i32 = row.get(2)?;
                let title: String = row.get(3)?;
                Ok((id, url, counter, title))
            }).expect("Failed to query urls");
    
            for url in url_iter {
                match url {
                    Ok((id, url, counter, title)) => {
                        println!("ID: {}, URL: {}, Counter: {}, Title: {}", id, url, counter, title);
                    }
                    Err(e) => {
                        println!("Error reading URL from database: {}", e);
                    }
                }
            }
    
        } else if selection == 4 {
            history.push("Close Session".to_string());
            *logged = false;
            return;
        } else if selection == 5 {
            history.push("Exit".to_string());
            return;
        } else {
            history.push("Invalid selection".to_string());
            panic!("Invalid selection");
        }
    }
}