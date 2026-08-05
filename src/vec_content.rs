use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

pub fn generate_vector(model: &mut TextEmbedding, text: &str) -> Vec<f32> {
    let doc = vec![text];
    let embedding = model.embed(doc, None).expect("Error generando el vector");
    let vector: Vec<f32> = embedding[0].clone();

    vector
}