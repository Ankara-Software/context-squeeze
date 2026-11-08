Maintained by [Ankara Yazılım](https://github.com/Ankara-Software) | [Project page](https://ankarayazilim.org/acik-kaynak/context-squeeze)

**Status: beta.** It builds, the 5 parser unit tests pass and the three MCP tools were called over stdio against sample Python, TypeScript and Go files. Symbol extraction uses regular expressions, not a real parser, so it misses some symbols (see [Limitations](#limitations)). No prebuilt binaries or npm package have been published yet; install from source.

# Context-Squeeze

<p align="center">
  <img src="assets/banner.png" alt="Context-Squeeze Banner" width="100%">
</p>

**Context-Squeeze** is a small Model Context Protocol (MCP) server written in Rust that helps AI agents (such as Claude Desktop, Cursor, Cline or local LLM clients) navigate a codebase. Instead of reading whole files, the agent first asks for a compact **symbol map** (signatures of classes, structs, interfaces and functions) and then fetches the body of only the symbol it needs. This keeps less source text in the model's context window.

## Türkçe

Context-Squeeze, yapay zeka ajanlarının (Claude Desktop, Cursor, Cline vb.) büyük kod tabanlarında gezinmesine yardım eden, Rust ile yazılmış küçük bir Model Context Protocol (MCP) sunucusudur. Ajan, dosyaların tamamını okumak yerine önce sınıf, struct, arayüz ve fonksiyon imzalarından oluşan kısa bir sembol haritası ister, sonra yalnızca ihtiyaç duyduğu sembolün gövdesini getirir. Böylece modelin bağlam penceresinde daha az kaynak kodu tutulur. Python, TypeScript/JavaScript, Rust ve Go dosyalarını destekler; sembolleri düzenli ifadelerle çıkarır, bu yüzden bazı semboller (örneğin Python ve TypeScript sınıf metotları) haritada görünmez. Proje beta aşamasındadır; hazır ikili dosya ve npm paketi henüz yayımlanmamıştır, kaynaktan kurulur.

---

## Installation

Build from source (Rust toolchain required):

```bash
cargo install --git https://github.com/Ankara-Software/context-squeeze
```

This installs a `context-squeeze` binary that speaks MCP over stdin and stdout. Or build it manually:

```bash
git clone https://github.com/Ankara-Software/context-squeeze.git
cd context-squeeze
cargo build --release

# The compiled binary will be in target/release/context-squeeze
```

The `package.json` and `bin/cli.js` files describe an npm wrapper (`context-squeeze-mcp`) that downloads a release binary. That package is not on the npm registry and the `v0.1.0` release has no binary assets, so `npx -y context-squeeze-mcp` does not work yet.

## Client Setup

### Claude Desktop
Add the following to your `claude_desktop_config.json` (`%APPDATA%\Claude\claude_desktop_config.json` on Windows, `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS). Use the full path of the binary if it is not on your `PATH`:

```json
{
  "mcpServers": {
    "context-squeeze": {
      "command": "context-squeeze"
    }
  }
}
```

### Cursor
1. Open Cursor Settings -> **Features** -> **MCP**.
2. Click **+ Add New MCP Server**.
3. Set:
   - **Name**: `context-squeeze`
   - **Type**: `stdio`
   - **Command**: `context-squeeze`
4. Save.

`get_code_item` and `search_code` read files relative to the working directory of the server process, so start the server from the project you want to explore.

---

## Exposed MCP Tools

### 1. `get_repo_map`
Walks the workspace and builds a compact outline. It respects `.gitignore` rules and skips hidden files.
* **Returns**: for each file with symbols, the file path and the signature line of each class, struct, interface, trait, enum and function found. Bodies are omitted.
* **Arguments**:
  - `path`: (Optional) Directory to scan. Defaults to the current directory.

### 2. `get_code_item`
Returns the full body of one symbol in one file.
* **Arguments**:
  - `file_path`: (Required) Path to the source file, relative to the working directory.
  - `symbol_name`: (Required) Name of the class, struct, function or interface.

### 3. `search_code`
Case-insensitive substring search over symbol names and signatures across the workspace.
* **Arguments**:
  - `query`: (Required) Text to look for.
* **Returns**: file, symbol, kind, signature and line range for each match.

### Supported languages
Files with these extensions are indexed: `py`, `ts`, `tsx`, `js`, `jsx`, `rs`, `go`.

---

## Limitations

- Symbols are found with regular expressions. Methods inside Python and TypeScript classes are not listed (the class line is); Rust `impl` functions and Go methods are.
- Only the extensions listed above are indexed.
- The server does not restrict paths: `get_code_item` joins the given path to the working directory. Run it only for clients and projects you trust.

## Measured Example

One run of `get_repo_map` on this repository's own source plus three small sample files (28,853 bytes of source in total) returned a map of 1,968 bytes. This is a byte count from a single run, not a token count, and the ratio depends heavily on the codebase.

---

## Development

```bash
cargo build
cargo test
```

See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## License

MIT. Copyright (c) 2026 Anılcan Kara and Ankara Yazılım contributors.
