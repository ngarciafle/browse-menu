use rusqlite::{Connection, params};
use scraper::{Html, Selector};
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use crate::vec_content::generate_vector;

// Right now just treating unexisting urls
// ** -> Make it async??
pub fn rank_url(history: &mut Vec<String>, conn: &Connection, url: &str, body: &str, model: &mut TextEmbedding) {
    let body = clean_body(body);

    let vector = generate_vector(model, &body);

    let treated_vector: Vec<u8> = vector.iter().flat_map(|f| f.to_le_bytes().to_vec()).collect();

    let url_id: i64 = conn.query_row(
        "SELECT id FROM crawl WHERE url = ?1",
        params![url],
        |row| row.get(0),
    ).expect("Error querying URL ID");

    conn.execute(
        "INSERT INTO browser_vec (id_web, vector) VALUES ($1, $2)",
        params![&url_id, &treated_vector],
    );
}

fn clean_body(body: &str) -> String {
    let selector = Selector::parse("h1, h2, h3, h4, h5, h6, p, li, blockquote, td, th").unwrap();
    let mut body_cleaned = String::new();

    let document = Html::parse_document(body);
    for element in document.select(&selector) {
        let text_fragment: String = element.text().collect();
        body_cleaned.push_str(&text_fragment);
        body_cleaned.push(' '); // Add a space between text fragments
    }

    // Remove extra whitespace and treat 
    body_cleaned.split_whitespace().collect::<Vec<&str>>().join(" ")
}

// fn init_ia_model() -> TextEmbedding {
//     let options = InitOptions::new(EmbeddingModel::AllMiniLML6V2);
//     let mut model = TextEmbedding::try_new(options).expect("Error cargando el modelo");
//     model
// }

// fn generate_vector(model: &mut TextEmbedding, text: &str) -> Vec<f32> {
//     let doc = vec![text];
//     let embedding = model.embed(doc, None).expect("Error generando el vector");
//     let vector: Vec<f32> = embedding[0].clone();

//     vector
// }