# muster-mcp

Live-session census as a read-only MCP server.

`muster-mcp` exposes two MCP tools that let any MCP-capable agent query the
current wintermute session population — without exposing the kill path.

## Tools

### `sessions_census`

Enumerates live Claude sessions with origin attribution and bus reconciliation.
Equivalent to running `muster census --format json`.

- **Input**: no arguments (empty object `{}`)
- **Output**: JSON array of session objects with PID, origin, and bus state

### `sessions_verdict`

Annotates the live-session census with `live`, `duplicate`, `orphan`, or `stale`
verdicts plus supporting evidence. Equivalent to running `muster verdict --format json`.

- **Input**: no arguments (empty object `{}`)
- **Output**: JSON array of session objects with verdict and evidence fields

### Why `reap` is excluded

`muster reap` kills sessions. This server is an **inspection surface only**.
Exposing reap over MCP would allow any connected agent to terminate sessions
autonomously — which violates the principle that kill paths must remain
human-gated. An agent using `muster-mcp` can see the roster but never act on it.

## Usage

```sh
# Start the MCP server on stdio
muster-mcp serve

# Use a custom muster binary (also: MUSTER_BIN=/path/to/muster)
muster-mcp serve --muster-bin /usr/local/bin/muster
```

## MCP client configuration

Add to your MCP client config (e.g. Claude Code `.mcp.json`):

```json
{
  "mcpServers": {
    "muster": {
      "command": "muster-mcp",
      "args": ["serve"]
    }
  }
}
```

Or with a custom binary path:

```json
{
  "mcpServers": {
    "muster": {
      "command": "muster-mcp",
      "args": ["serve", "--muster-bin", "/home/jsy/.local/bin/muster"]
    }
  }
}
```

## Dependencies

- `muster` binary must be on `$PATH` (or specified via `--muster-bin` / `MUSTER_BIN`)
- `mcp-core` (path dependency, part of the wintermute workspace)

## Building

```sh
cargo build --release
# Binary at: target/release/muster-mcp
```

## License

MIT OR Apache-2.0 — Joe Yen
