# Guia Avançado: mcp-nexus-context

## 📍 Configuração por IDE

### Windsurf
Edite `~/.windsurf/mcp_config.json`:
```json
{
  "mcpServers": {
    "mcp-nexus-context": {
      "command": "cargo",
      "args": ["run", "--release"],
      "cwd": "/absolute/path/to/mcp-nexus-context"
    }
  }
}
```

### Cursor
Edite `~/.cursor/mcp_config.json` (mesmo formato acima).

### Claude Desktop
Edite `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) ou `%APPDATA%\Claude\config.json` (Windows).

---

## 🤖 System Prompt Completo

Adicione isso às **Custom Instructions** do seu agente:

```markdown
# MEMORY PROTOCOL
You have access to 'mcp-nexus-context', a local vector memory tool with infinite context.

## USAGE RULES
1. **Search First:** Before answering questions about past work, architecture, or decisions, ALWAYS call `search_context(query)`.
2. **Save Important Info:** When the user shares crucial information, architectural decisions, or asks you to "remember this", YOU MUST call `add_memory(text, metadata)`.

## EXAMPLES
- Query: "What was our decision on authentication?"
  Action: `search_context("authentication decision")`

- User: "Remember that we use PostgreSQL for production."
  Action: `add_memory("Production database: PostgreSQL", "{\"type\": \"infrastructure\"}")`

## BEHAVIOR
- Be proactive: Search without being asked if context might help.
- Be honest: If you find conflicting info, present both and ask for clarification.
- Ground answers: Always cite retrieved context when available.
```

---

## 🧪 Teste de Validação

```bash
# 1. Inicie o servidor manualmente
cargo run --release

# 2. Em outra janela, rode o teste
python test_mcp_client.py
```

**Saída esperada:**
```
✓ Initialize: OK
✓ Add Memory: "O projeto usa Rust"
✓ Search: Found 1 result (score: 0.80+)
```

---

## 🔧 Troubleshooting

### Erro: "Model files not found"
```bash
python download_model.py
```

### Servidor não responde
- Verifique se o caminho no `mcp_config.json` está correto
- Teste manualmente: `cargo run --release`
- Veja logs: `RUST_LOG=debug cargo run --release`

### Busca não retorna resultados
- O banco está vazio? Use `add_memory` primeiro
- Verifique `data/vectors.json` (deve existir e ter conteúdo)

### Performance lenta
- Use o binário compilado (`cargo build --release`) em vez de `cargo run`
- CPU lenta? Considere habilitar GPU (requer CUDA Toolkit)

---

## 🎯 Boas Práticas

1. **Metadados Estruturados:** Use JSON válido em `metadata`:
   ```json
   {"type": "architecture", "module": "auth", "date": "2025-11-27"}
   ```

2. **Queries Descritivas:** Seja específico:
   - ✅ "decisão sobre usar Postgres em vez de MySQL"
   - ❌ "banco de dados"

3. **Backup Regular:** `data/vectors.json` contém toda a memória. Faça backup!

---

## 📊 Estrutura de Dados

**Vector Store (`data/vectors.json`):**
```json
[
  {
    "id": "manual_id",
    "text": "O projeto usa arquitetura Hexagonal",
    "vector": [0.123, -0.456, ...],
    "metadata": "{\"type\": \"architecture\"}"
  }
]
```

**Model Cache (`data/models/bge-base-en-v1.5/`):**
- `config.json` - Configuração do modelo BERT
- `model.safetensors` - Pesos do modelo (~438MB)
- `tokenizer.json` - Tokenizador

---

*Para configuração básica, veja o [README.md](README.md)*
