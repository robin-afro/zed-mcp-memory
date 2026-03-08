# Memory MCP Server for Zed

A [Zed](https://zed.dev) extension that provides a persistent knowledge graph through the [Model Context Protocol](https://modelcontextprotocol.io). It wraps the official [`@modelcontextprotocol/server-memory`](https://www.npmjs.com/package/@modelcontextprotocol/server-memory) package, giving your AI assistant long-term memory across sessions.

## Features

The extension exposes the following tools to the assistant:

| Tool | Description |
| --- | --- |
| `create_entities` | Create new entities in the knowledge graph |
| `create_relations` | Create relations between entities |
| `add_observations` | Add observations to existing entities |
| `delete_entities` | Delete entities and their relations |
| `delete_observations` | Delete specific observations from entities |
| `delete_relations` | Delete relations between entities |
| `read_graph` | Read the entire knowledge graph |
| `search_nodes` | Search entities by name, type, or observation content |
| `open_nodes` | Retrieve specific entities by name |

## Installation

1. Open Zed
2. Open the extensions panel (`zed: extensions` in the command palette)
3. Search for **Memory MCP Server** and install it

## Configuration

Add the following to the MCP's settings to configure the memory file location:

```json
{
  "settings": {
    "memory_file_path": "/absolute/path/to/memory.jsonl"
  }
}
```

> **Note:** The path must be **absolute**. Tilde expansion (`~`) is not supported.

If `memory_file_path` is left empty (the default), the memory file is stored next to the server's `index.js` inside Zed's extensions directory.

## How It Works

Memories are stored as a knowledge graph in a JSONL file. Each line is a JSON object representing either an entity or a relation:

- **Entities** have a name, a type, and a list of observations (free-text strings).
- **Relations** connect two entities with a named relationship (in active voice).

The assistant can read and write to this graph, allowing it to remember facts about you, your projects, preferences, and anything else across conversations.

## Development

### Prerequisites

- [Rust](https://rustup.rs) with the `wasm32-wasip1` target:
  ```sh
  rustup target add wasm32-wasip1
  ```

### Dev Extension

1. Clone this repository
2. In Zed, go to `zed: install dev extension` in the command palette
3. Select the cloned directory

Zed will compile the extension and load it automatically. Changes to the source require reinstalling the dev extension.

## License

MIT
