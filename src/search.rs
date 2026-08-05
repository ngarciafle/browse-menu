use dialoguer::Input;
use fastembed::TextEmbedding;
use rusqlite::{Connection, params};
use crate::vec_content::generate_vector;

pub fn search(history: &mut Vec<String>, model: &mut TextEmbedding, conn: &Connection) {
    let input: String = Input::new()
        .with_prompt("What do you want to search for? ")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");

    history.push(format!("Searched for: {input}"));
    println!("Searching for: {input}");

    let vector = generate_vector(model, &input);

    let mut stmt = conn.prepare(
        "SELECT id_web, vector FROM browser_vec WHERE vct MATCH ?1 ORDER BY vct_distance(vector, ?) ASC LIMIT 10"
    ).expect("Failed to prepare statement");

    let treated_vector: Vec<u8> = vector.iter().flat_map(|f| f.to_le_bytes().to_vec()).collect();

    let results = stmt.query_map(params![treated_vector], |row| {
        let id_web: i32 = row.get(0)?;
        Ok((id_web))
    }).expect("Failed to query results");

    println!("Search results:");
    for result in results {
        match result {
            Ok((id_web)) => {
                println!("ID: {}", id_web);
            },
            Err(e) => eprintln!("Error retrieving result: {}", e),
        }
    }
}