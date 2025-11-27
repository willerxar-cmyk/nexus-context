<<<<<<< HEAD
# 🧠 MCP-Nexus-Context
=======
# 🧠 mcp-nexus-context MCP
>>>>>>> f30aa6ee78c783e17670f9ee342dfd137c0f89c0
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

<<<<<<< HEAD
**MCP-Nexus-Context** is a specialized **Model Context Protocol (MCP)** server designed to give your AI Assistant (Windsurf, Cursor, Claude) **infinite long-term memory**. 
=======
**mcp-nexus-context** is a specialized **Model Context Protocol (MCP)** server designed to give your AI Assistant (Windsurf, Cursor, Claude) **infinite long-term memory**. 
>>>>>>> f30aa6ee78c783e17670f9ee342dfd137c0f89c0

Unlike cloud-based solutions, Nexus runs **100% locally** on your machine. It indexes your conversations, architectural decisions, and code snippets into a vector database, allowing the AI to retrieve relevant context from weeks or months ago instantly.

### ✨ Key Features
- **⚡ Blazing Fast:** Written in **Rust** for near-zero latency.
- **�️ Real-time Watcher:** Monitors your project files and updates context instantly (Infinite Context).
- **�🔒 Privacy First:** No data leaves your machine. Embeddings are generated locally.
- **🧠 SOTA Embeddings:** Uses `BAAI/bge-base-en-v1.5` (BERT architecture) for high-precision semantic search.
- **🔋 Hybrid Hardware:** Auto-detects CPU or GPU (CUDA) availability (configured for generic compatibility).
- **💾 Persistence:** Simple, robust JSON-based vector storage (no complex setups required).
- **🌍 Cross-Platform:** Runs on Windows, Linux, and macOS via Rust Cargo.

---

## 🛠️ Installation

### Prerequisites
- **Rust Toolchain:** [Install Rust](https://www.rust-lang.org/tools/install)
- **Python 3.10+:** (Only for initial model download)

### Setup Steps
1. **Clone the repository:**
   ```bash
<<<<<<< HEAD
   git clone https://github.com/your-repo/mcp-nexus-context.git
=======
   git clone https://github.com/willerxar-cmyk/mcp-nexus-context.git
>>>>>>> f30aa6ee78c783e17670f9ee342dfd137c0f89c0
   cd mcp-nexus-context
   ```

2. **Download the AI Model:**
   This script fetches the optimized BGE model to your local `data/` folder.
   ```bash
   python download_model.py
   ```

3. **Build (Optional):**
   You can compile a binary for maximum performance:
   ```bash
   cargo build --release
   ```
<<<<<<< HEAD
   *Binary location:*
   - **Windows:** `target/release/mcp-nexus-context.exe`
   - **Linux/Mac:** `target/release/mcp-nexus-context`
=======
   *The executable will be at `target/release/mcp-nexus-context.exe`*
>>>>>>> f30aa6ee78c783e17670f9ee342dfd137c0f89c0

---

## ⚙️ Configuration (MCP)

<<<<<<< HEAD
To use Nexus with your AI Agent, add this to your **MCP Settings** file (e.g., `mcp_config.json`).
=======
To use mcp-nexus-context with your AI Agent, add this to your **MCP Settings** file (e.g., `mcp_config.json` in Windsurf/Cursor):
>>>>>>> f30aa6ee78c783e17670f9ee342dfd137c0f89c0

### Option A: Using Pre-compiled Binary (Faster Startup)
```json
{
  "mcpServers": {
    "mcp-nexus-context": {
<<<<<<< HEAD
      "command": "C:/Path/To/mcp-nexus-context/target/release/mcp-nexus-context.exe",
=======
      "command": "C:/ABSOLUTE/PATH/TO/mcp-nexus-context/target/release/mcp-nexus-context.exe",
>>>>>>> f30aa6ee78c783e17670f9ee342dfd137c0f89c0
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

### Option B: Running from Source (Cross-Platform / Dev)
Use this if you want to run directly via Cargo on any OS (Linux, Mac, Windows) without manual compilation steps.

```json
{
  "mcpServers": {
    "mcp-nexus-context": {
      "command": "cargo",
      "args": ["run", "--release", "--"],
      "cwd": "/absolute/path/to/mcp-nexus-context",
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
> **⚠️ Important:** Replace paths with the full path to this project's root folder.

---

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
