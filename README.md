# Context-Squeeze 🎯

<p align="center">
  <img src="assets/banner.png" alt="Context-Squeeze Banner" width="100%">
</p>

**Context-Squeeze** is a blazingly fast, lightweight Model Context Protocol (MCP) server written in Rust that helps AI agents (like Claude Desktop, Cursor, Cline, or local LLMs) navigate large codebases efficiently. 

By parsing your project directory and building a compressed **Symbol & API Map** (signatures, structs, types, interfaces) instead of raw files, it **reduces LLM context window bloat by up to 80%**, saving massive token budgets and preventing LLM hallucinations.

---

## 🚀 Quick Setup (Zero-Install via npx)

The easiest way to run Context-Squeeze is using `npx`, which automatically resolves the correct precompiled native binary (Linux, macOS, Windows) for your machine.

### 1. Claude Desktop Integration
Add the following to your `claude_desktop_config.json` (located at `%APPDATA%\Claude\claude_desktop_config.json` on Windows, or `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "context-squeeze": {
      "command": "npx",
      "args": ["-y", "context-squeeze-mcp"]
    }
  }
}
```

### 2. Cursor Integration
1. Open Cursor Settings -> **Features** -> **MCP**.
2. Click **+ Add New MCP Server**.
3. Set:
   - **Name**: `context-squeeze`
   - **Type**: `stdio`
   - **Command**: `npx -y context-squeeze-mcp`
4. Save and let it connect!

---

## 🛠️ Installation Alternatives

### 1. Install globally via NPM
```bash
npm install -g context-squeeze-mcp

# Now configure it using the direct command:
# Command: context-squeeze-mcp
```

### 2. Build from Source (Cargo required)
```bash
git clone https://github.com/anilcan-kara/context-squeeze.git
cd context-squeeze
cargo build --release

# The compiled binary will be in target/release/context-squeeze
```

---

## 🧰 Exposed MCP Tools

Once connected, the AI agent gets access to the following high-speed structural tools:

### 1. `get_repo_map`
Analyzes the workspace and builds a compressed structural outline. It automatically respects `.gitignore` rules.
* **Returns**: File names and their class, interface, and function signatures. Method bodies are omitted to save token budget.
* **Arguments**:
  - `path`: (Optional) Target directory path to scan.

### 2. `get_code_item`
Retrieves the full implementation body of a specific symbol inside a file. This allows the LLM to fetch implementation code only when it's actively modifying or reading it.
* **Arguments**:
  - `file_path`: (Required) Relative path to the source file.
  - `symbol_name`: (Required) Name of the class, struct, function, or interface.

### 3. `search_code`
Performs a fast, structural-aware search for symbol names or patterns across the workspace.
* **Arguments**:
  - `query`: (Required) Symbol name or pattern to find.

---

## 📊 Performance & Token Savings Example

| File | Raw Size (Tokens) | Context-Squeeze Map Size (Tokens) | Savings |
| --- | --- | --- | --- |
| `src/main.rs` | ~2,500 | ~150 (Signatures only) | **94%** |
| `src/indexer.rs` | ~1,200 | ~80 (Signatures only) | **93%** |
| **Entire Workspace** | **~25,000** | **~1,800** | **92.8%** |

---

## License

MIT — Anilcan Kara
