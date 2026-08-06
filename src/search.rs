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
        "SELECT c.url, v.distance FROM ( SELECT id_web, distance FROM browser_vec WHERE vector MATCH ?1 ORDER BY distance ASC LIMIT 10) v JOIN crawl c ON v.id_web = c.id"
    ).expect("Failed to prepare statement");

    let treated_vector: Vec<u8> = vector.iter().flat_map(|f| f.to_le_bytes()).collect();

    let results = stmt.query_map(params![treated_vector], |row| {
        let url: String = row.get(0)?;
        Ok((url))
    }).expect("Failed to query results");

    println!("Search results:");
    for result in results {
        match result {
            Ok((url)) => {
                println!("URL: {}", url);
            },
            Err(e) => eprintln!("Error retrieving result: {}", e),
        }
    }
}