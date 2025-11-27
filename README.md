# 🧠 mcp-nexus-context MCP
### *Local Vector Database & Infinite Context Memory*

![Rust](https://img.shields.io/badge/Built_with-Rust-d33803?style=for-the-badge&logo=rust)
![MCP](https://img.shields.io/badge/Protocol-MCP-blue?style=for-the-badge)
![AI](https://img.shields.io/badge/AI-Local_Inference-green?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)

> **"A Second Brain for your AI Agents, running locally on your GPU."**

---

## 👨‍💻 Author & Credits
**Created & Architected by:** [Willer Xavier Reis]  
**Date:** November 2025  
**Concept:** High-performance local context expansion using Rust & Vector Embeddings.

---

## 🚀 Overview

**mcp-nexus-context** is a specialized **Model Context Protocol (MCP)** server designed to give your AI Assistant (Windsurf, Cursor, Claude) **infinite long-term memory**. 

Unlike cloud-based solutions, Nexus runs **100% locally** on your machine. It indexes your conversations, architectural decisions, and code snippets into a vector database, allowing the AI to retrieve relevant context from weeks or months ago instantly.

### ✨ Key Features
- **⚡ Blazing Fast:** Written in **Rust** for near-zero latency.
- **🔒 Privacy First:** No data leaves your machine. Embeddings are generated locally.
- **🧠 SOTA Embeddings:** Uses `BAAI/bge-base-en-v1.5` (BERT architecture) for high-precision semantic search.
- **🔋 Hybrid Hardware:** Auto-detects CPU or GPU (CUDA) availability (configured for generic compatibility).
- **💾 Persistence:** Simple, robust JSON-based vector storage (no complex setups required).

---

## 🛠️ Installation

### Prerequisites
- **Rust Toolchain:** [Install Rust](https://www.rust-lang.org/tools/install)
- **Python 3.10+:** (Only for initial model download)

### Build Steps
1. **Clone the repository:**
   ```bash
   git clone https://github.com/willerxar-cmyk/mcp-nexus-context.git
   cd mcp-nexus-context
   ```

2. **Download the AI Model:**
   This script fetches the optimized BGE model to your local `data/` folder.
   ```bash
   python download_model.py
   ```

3. **Compile in Release Mode:**
   ```bash
   cargo build --release
   ```
   *The executable will be at `target/release/mcp-nexus-context.exe`*

---

## ⚙️ Configuration (MCP)

To use mcp-nexus-context with your AI Agent, add this to your **MCP Settings** file (e.g., `mcp_config.json` in Windsurf/Cursor):

```json
{
  "mcpServers": {
    "mcp-nexus-context": {
      "command": "C:/ABSOLUTE/PATH/TO/mcp-nexus-context/target/release/mcp-nexus-context.exe",
      "args": [],
      "env": {
        "RUST_LOG": "info",
        "HF_ENDPOINT": "https://huggingface.co"
      },
      "disabled": false,
      "autoApprove": ["search_context", "add_memory"]
    }
  }
}
```
> **⚠️ Important:** Replace `C:/ABSOLUTE/PATH/TO/...` with the actual path on your system.

---

## 🤖 AI Instructions (System Prompt)

Teach your Agent to use mcp-nexus-context by adding this to your **Custom Instructions**:

```markdown
# mcp-nexus-context PROTOCOL
You have access to 'mcp-nexus-context', a local vector memory tool.

1. **Retrieval:** ALWAYS call `search_context(query)` before answering questions about history, architecture, or past decisions.
2. **Memory:** When the user shares important info, call `add_memory(text, metadata)` to save it forever.
```

---

## 🧪 Testing

You can verify the installation using the included Python test script:
```bash
python test_mcp_client.py
```
*Expected Output: Successful initialization, memory addition, and semantic search results.*

---

## 🏗️ Architecture

```mermaid
graph TD
    A[AI Agent / IDE] <-->|JSON-RPC (MCP)| B(mcp-nexus-context Server);
    B <-->|Inference| C{Embedder Engine};
    C -->|BGE Model| D[Local GPU/CPU];
    B <-->|Read/Write| E[(Vector Store JSON)];
    E -->|Persist| F[Disk Storage];
```

---

*Developed with passion for the future of Local AI.*
