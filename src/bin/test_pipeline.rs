use anyhow::Result;
use nexus_context::db::VectorDB;
use nexus_context::embeddings::Embedder;

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- Starting Nexus-Context Pipeline Test ---");

    // 1. Test Embedder
    println!("\n[1/3] Initializing Embedder (Qwen)...");
    // Note: This might take time to download the model (~1.2GB)
    let mut embedder = Embedder::new()?;
    println!("Embedder initialized.");

    let text = "The rust compiler is fast and safe.";
    println!("Generating embedding for: '{}'", text);
    let vector = embedder.embed(text)?;
    println!("Vector generated. Dimensions: {}", vector.len());

    // 2. Test Database
    println!("\n[2/3] Initializing Vector Store (JSON)...");
    let db = VectorDB::new("data/test_vectors.json").await?;
    println!("DB initialized.");

    println!("Adding document to DB...");
    db.add("doc_1", text, vector.clone(), "{\"category\": \"test\"}").await?;
    println!("Document added.");

    // 3. Test Search
    println!("\n[3/3] Performing Search...");
    let query = "programming language performance";
    println!("Query: '{}'", query);
    let query_vec = embedder.embed(query)?;
    
    let results = db.search(query_vec, 2).await?;
    println!("Search Results: {:?}", results);

    println!("\n--- Test Completed Successfully ---");
    Ok(())
}
