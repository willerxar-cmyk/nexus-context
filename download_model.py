from huggingface_hub import snapshot_download
import os

model_id = "BAAI/bge-base-en-v1.5"
local_dir = "data/models/bge-base-en-v1.5"

print(f"Downloading {model_id} to {local_dir}...")
try:
    path = snapshot_download(repo_id=model_id, local_dir=local_dir, allow_patterns=["*.json", "*.safetensors", "*.txt"])
    print(f"Successfully downloaded to: {path}")
except Exception as e:
    print(f"Error downloading: {e}")
