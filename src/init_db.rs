use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use dialoguer::{Input, Password};
use bcrypt::{hash, DEFAULT_COST};

pub fn init_db(use_public: bool) -> Result<Connection> {
    // Just initialize db
    if let Err(e) = fs::create_dir_all("pub") {
        panic!("No se pudo crear la carpeta 'pub': {}", e);
    }

    let route_db: String = format!("pub/db");

    let was_created = Path::new(&route_db).exists();
    let conn = Connection::open(&route_db)?;

    if !was_created {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS creadentials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS crawl (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                content TEXT,
                depth INTEGER DEFAULT 0, 
                date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .expect("Failed to create table");

        let admin_username: String = Input::new()
            .with_prompt("Enter the admin username")
            .show_default(false)
            .interact_text()
            .expect("Failed to read line");
        let admin_password: String = Password::new()
            .with_prompt("Enter the admin password")
            .interact()
            .expect("Failed to read line");
        
        // Create admin creds -> don't know if create just one admin or multiple with different abilities 
        conn.execute(
            "INSERT INTO credentials (username, password) VALUES (?1, ?2)",
            &[&admin_username, &hash(admin_password, DEFAULT_COST).unwrap()],
        )
        .expect("Failed to insert admin credentials");

    } else if !use_public {
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
    } 
    
    Ok(conn)
}