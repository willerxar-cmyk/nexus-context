use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{Config as BertConfig, BertModel};
use tokenizers::{PaddingParams, Tokenizer};

pub struct Embedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl Embedder {
    pub fn new() -> Result<Self> {
        let device = Device::Cpu;
        println!("Using device: {:?}", device);

        // Load from local path downloaded by python script
        let local_path = "data/models/bge-base-en-v1.5";
        
        // Verify files exist
        let config_path = std::path::Path::new(local_path).join("config.json");
        let tokenizer_path = std::path::Path::new(local_path).join("tokenizer.json");
        let weights_path = std::path::Path::new(local_path).join("model.safetensors");

        if !config_path.exists() || !tokenizer_path.exists() || !weights_path.exists() {
            return Err(E::msg(format!(
                "Model files not found at {}. Please run 'python download_model.py' first.", 
                local_path
            )));
        }

        let config: BertConfig = serde_json::from_slice(&std::fs::read(config_path)?)?;
        let mut tokenizer = Tokenizer::from_file(tokenizer_path).map_err(E::msg)?;
        
        // Configure tokenizer for padding/truncation if needed
        if let Some(pp) = tokenizer.get_padding_mut() {
            pp.strategy = tokenizers::PaddingStrategy::BatchLongest
        } else {
            let pp = PaddingParams {
                strategy: tokenizers::PaddingStrategy::BatchLongest,
                ..Default::default()
            };
            tokenizer.with_padding(Some(pp));
        }

        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)? };
        let model = BertModel::load(vb, &config)?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
        let tokens = self.tokenizer.encode(text, true).map_err(E::msg)?;
        let token_ids = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;
        
        // Run inference
        let outputs = self.model.forward(&token_ids, &token_type_ids, None)?;
        
        // Use CLS token embedding (first token) or Mean Pooling?
        // Nomic v1.5 uses mean pooling + normalization.
        // Output shape: (batch, seq, hidden)
        let (_b, seq_len, _h) = outputs.dims3()?;
        
        // Mean Pooling
        // Note: Nomic uses a specific instruction prefix usually "search_query: ", but let's keep it raw for now.
        let sum = outputs.sum(1)?;
        let embeddings = (sum / (seq_len as f64))?;
        
        let embeddings_vec = embeddings.squeeze(0)?.to_vec1::<f32>()?;
        Ok(normalize(&embeddings_vec))
    }
}

fn normalize(v: &[f32]) -> Vec<f32> {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm == 0.0 { return v.to_vec(); }
    v.iter().map(|x| x / norm).collect()
}
