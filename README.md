# muster-mcp

A read-only MCP server that lets an agent see the live wintermute session roster — and only see it. The kill path is deliberately not exposed.

## Why it exists

`muster` is the tool that counts live Claude sessions and judges them: which are real, which are duplicates, which are orphaned or stale, and which should be reaped. An agent that could read that census could coordinate; an agent that could act on it could terminate its siblings. Those are different privileges, and they should not arrive together.

`muster-mcp` separates them. It exposes the inspection half of `muster` over MCP and leaves `reap` behind. Any MCP-capable agent can ask who is running and how each session looks, but the decision to kill stays human-gated. The server is a window, not a lever.

## Tools

### `sessions_census`

Enumerates live Claude sessions with origin attribution and bus reconciliation — the equivalent of `muster census --format json`.

- **Input**: none (empty object `{}`)
- **Output**: a JSON array of session objects with PID, origin, and bus state.

### `sessions_verdict`

Annotates the census with a `live`, `duplicate`, `orphan`, or `stale` verdict plus the evidence behind it — the equivalent of `muster verdict --format json`.

- **Input**: none (empty object `{}`)
- **Output**: a JSON array of session objects with verdict and evidence fields.

`reap` is not here, and that is the point. It kills sessions; exposing it over MCP would let any connected agent end a session on its own. An agent using `muster-mcp` sees the roster but never acts on it.

## Run

```sh
# Start the MCP server on stdio
muster-mcp serve

# Point it at a specific muster binary (or set MUSTER_BIN)
muster-mcp serve --muster-bin /usr/local/bin/muster
```

The server shells out to the `muster` binary, so `muster` must be on `$PATH` — or named via `--muster-bin` / the `MUSTER_BIN` environment variable.

## MCP client configuration

Add to your client config (for Claude Code, `.mcp.json`):

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

With a custom binary path:

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

## Where it fits

`muster-mcp` is the MCP surface over `muster`, part of the wintermute fleet's session-management layer. It depends on `mcp-core` (the fleet's shared MCP stdio scaffolding) as a workspace path dependency, so it builds inside the wintermute workspace rather than standalone from a bare clone:

```sh
cargo build --release   # within the wintermute workspace
# Binary at: target/release/muster-mcp
```

## Status

v0.1. Two tools, both read-only, both thin wrappers over `muster`'s JSON output.

## License

MIT OR Apache-2.0 — Joe Yen.
