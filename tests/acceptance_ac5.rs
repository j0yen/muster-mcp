//! AC5: A tools/call naming an unregistered tool (e.g. sessions_reap) yields
//! a JSON-RPC error (-32601 METHOD_NOT_FOUND), not a shell call.

use mcp_core::serve::serve;
use mcp_core::{Tool, ToolError};
use serde_json::{json, Value};
use std::io::Cursor;

// ── Minimal stubs ─────────────────────────────────────────────────────────

struct StubCensus;
impl Tool for StubCensus {
    fn name(&self) -> &str {
        "sessions_census"
    }
    fn description(&self) -> &str {
        "stub"
    }
    fn input_schema(&self) -> Value {
        json!({"type":"object","properties":{}})
    }
    fn call(&self, _: &Value) -> Result<Value, ToolError> {
        Ok(json!([]))
    }
}

struct StubVerdict;
impl Tool for StubVerdict {
    fn name(&self) -> &str {
        "sessions_verdict"
    }
    fn description(&self) -> &str {
        "stub"
    }
    fn input_schema(&self) -> Value {
        json!({"type":"object","properties":{}})
    }
    fn call(&self, _: &Value) -> Result<Value, ToolError> {
        Ok(json!([]))
    }
}

// ── Test ─────────────────────────────────────────────────────────────────────

/// Calling a non-existent tool (sessions_reap) returns a JSON-RPC error.
#[test]
fn unknown_tool_call_returns_rpc_error_not_shell() {
    let input = concat!(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        "\n",
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"sessions_reap","arguments":{}}}"#,
        "\n",
    );

    let tools: Vec<Box<dyn Tool>> = vec![Box::new(StubCensus), Box::new(StubVerdict)];
    let mut output = Vec::new();
    serve(Cursor::new(input), &mut output, tools, "muster-mcp", "0.1.0").unwrap();

    let text = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    // First line is initialize response, second is the tools/call response
    assert_eq!(lines.len(), 2);

    let err_resp: Value = serde_json::from_str(lines[1]).unwrap();
    assert!(
        err_resp["error"].is_object(),
        "calling unknown tool must return an error object, got: {err_resp}"
    );
    let code = err_resp["error"]["code"].as_i64().unwrap_or(0);
    // -32601 = METHOD_NOT_FOUND — unknown tool name maps to this per mcp-core dispatch
    assert_eq!(
        code, -32601,
        "error code must be -32601 (METHOD_NOT_FOUND) for unknown tool"
    );
}

/// Calling sessions_census (a registered tool) does NOT return a method-not-found error.
#[test]
fn known_tool_call_does_not_return_method_not_found() {
    let input = concat!(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        "\n",
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"sessions_census","arguments":{}}}"#,
        "\n",
    );

    let tools: Vec<Box<dyn Tool>> = vec![Box::new(StubCensus), Box::new(StubVerdict)];
    let mut output = Vec::new();
    serve(Cursor::new(input), &mut output, tools, "muster-mcp", "0.1.0").unwrap();

    let text = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2);

    let resp: Value = serde_json::from_str(lines[1]).unwrap();
    // sessions_census stub returns Ok([]) so we should get a result, not an error
    assert!(
        resp["result"].is_object(),
        "known tool call must return a result, got: {resp}"
    );
}
