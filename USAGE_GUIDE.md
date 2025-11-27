# Guia de Uso e Configuração: MCP-Nexus-Context

Este documento explica como instalar, configurar e instruir Agentes de IA para utilizar o **MCP-Nexus-Context**, seu banco de dados vetorial local acelerado por GPU/CPU.

## 1. Instalação e Execução

Certifique-se de ter compilado o projeto em modo release:
```powershell
cargo build --release
```
O executável será gerado em: `target/release/mcp-nexus-context.exe` (Windows) ou `mcp-nexus-context` (Linux/Mac).

## 2. Configuração no Cliente MCP (IDE)

Adicione uma das seguintes configurações ao seu arquivo de settings do MCP (geralmente `mcp_config.json`):

### Opção A: Binário Compilado (Recomendado)
Substitua `c:/Caminho/Para/...` pelo caminho completo onde o projeto foi clonado.

```json
{
  "mcpServers": {
    "mcp-nexus-context": {
      "command": "c:/Users/admin/Documents/MPC-ContextExpander/target/release/mcp-nexus-context.exe",
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

### Opção B: Rodar via Cargo (Cross-Platform)
Ideal para desenvolvimento ou se você usa Linux/Mac e não quer gerenciar o binário manualmente.
```json
{
  "mcpServers": {
    "mcp-nexus-context": {
      "command": "cargo",
      "args": ["run", "--release", "--"],
      "cwd": "/caminho/absoluto/para/mcp-nexus-context",
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
*Nota: Ajuste o caminho absoluto (`c:/Users/...`) se necessário.*

## 3. Instruções para o Agente (System Prompt)

Para garantir que o Agente (Cascade, Claude, GPT-4) utilize o Nexus-Context de forma proativa, adicione o seguinte bloco ao **Custom Instructions** ou **System Prompt** do seu editor:

```markdown
# NEXUS-CONTEXT PROTOCOL
You have access to a powerful local vector database tool called 'nexus-context'.
This tool is your long-term memory and knowledge base.

## WHEN TO USE
1. **Retrieval:** Before answering complex questions about the codebase history, architecture decisions, or previous discussions, ALWAYS call `search_context(query)`.
   - Query Examples: "architecture patterns used in module X", "why did we use Rust instead of Python?", "previous bugs in authentication".
   
2. **Memory Storage:** When the user provides crucial information, architectural decisions, or specifically asks you to "remember this", YOU MUST call `add_memory(text, metadata)`.
   - Example: `add_memory("User prefers functional programming style in Rust", "{\"type\": \"preference\"}")`
   - Example: `add_memory("The API endpoint /v1/auth was deprecated in v2.0", "{\"tag\": \"api_change\"}")`

## BEHAVIOR RULES
- **Proactive Search:** Do not wait for the user to tell you to search. If a query is ambiguous or refers to past context, search first.
- **Self-Correction:** If you search and find conflicting info, present the findings to the user to resolve the conflict.
- **Context Awareness:** Use the retrieved context to ground your code generation and explanations.
```

## 4. Testando a Integração

1. **Reinicie seu IDE/Cliente MCP** para carregar a nova configuração.
2. **Abra o Chat** e digite:
   > "Salve na memória que este projeto usa arquitetura Hexagonal."
3. O agente deve responder confirmando o uso da ferramenta `add_memory`.
4. Em seguida, pergunte:
   > "Qual arquitetura estamos usando?"
5. O agente deve chamar `search_context`, encontrar a informação ("Hexagonal") e responder corretamente.

## 5. Solução de Problemas

*   **Erro "Model files not found":**
    Execute o script de download novamente na pasta do projeto:
    `python download_model.py`
*   **Erro de DLL/GPU:**
    O projeto está configurado para usar CPU por padrão para compatibilidade máxima. Se quiser usar GPU (Nvidia), edite o `Cargo.toml` para habilitar a feature `cuda` e recompile, garantindo que o CUDA Toolkit v12+ esteja instalado.
