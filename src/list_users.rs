use rusqlite::Connection;

pub fn list_users(history: &mut Vec<String>, conn: &Connection) {
    let mut stmt = conn.prepare("SELECT username FROM credentials").expect("Failed to prepare statement");
    let user_iter = stmt.query_map([], |row| {
        let username: String = row.get(0)?;
        Ok(username)
    }).expect("Failed to query usernames");

    println!("List of users:");
    for user in user_iter {
        match user {
            Ok(username) => println!("{}", username),
            Err(e) => eprintln!("Error retrieving user: {}", e),
        }
    }

    history.push("List Users".to_string());
}