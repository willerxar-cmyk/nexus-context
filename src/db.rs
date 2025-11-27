use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub vector: Vec<f32>,
    pub metadata: String,
}

pub struct VectorDB {
    path: PathBuf,
    documents: RwLock<Vec<Document>>,
}

impl VectorDB {
    pub async fn new(storage_path: &str) -> Result<Self> {
        let path = PathBuf::from(storage_path);
        
        // Create dir if not exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let documents = if path.exists() {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_default()
        } else {
            Vec::new()
        };

        Ok(Self {
            path,
            documents: RwLock::new(documents),
        })
    }

    pub async fn add(&self, id: &str, text: &str, vector: Vec<f32>, metadata: &str) -> Result<()> {
        let doc = Document {
            id: id.to_string(),
            text: text.to_string(),
            vector,
            metadata: metadata.to_string(),
        };

        let mut docs = self.documents.write().await;
        docs.push(doc);
        
        // Persist immediately (simplistic approach)
        // In prod, use WAL or async save
        let file = File::create(&self.path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &*docs)?;
        
        Ok(())
    }

    pub async fn search(&self, query_vector: Vec<f32>, limit: usize) -> Result<Vec<(String, String, f32)>> {
        let docs = self.documents.read().await;
        
        let mut scores: Vec<(usize, f32)> = docs.iter().enumerate().map(|(i, doc)| {
            let score = cosine_similarity(&query_vector, &doc.vector);
            (i, score)
        }).collect();

        // Sort by score descending
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let top_k = scores.into_iter().take(limit).map(|(i, score)| {
            let doc = &docs[i];
            (doc.text.clone(), doc.metadata.clone(), score)
        }).collect();

        Ok(top_k)
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    // Assuming vectors are already normalized by the embedder
    dot_product
}
