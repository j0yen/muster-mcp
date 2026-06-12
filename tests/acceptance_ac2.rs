//! AC2: muster-mcp serve completes initialize and tools/list returns exactly
//! the two read tools with valid JSON-Schema inputSchema.

use mcp_core::serve::serve;
use mcp_core::{Tool, ToolError};
use serde_json::{json, Value};
use std::io::Cursor;

// ── Minimal stub tool (mirrors the real tools without shelling muster) ──────

struct StubCensus;
impl Tool for StubCensus {
    fn name(&self) -> &str {
        "sessions_census"
    }
    fn description(&self) -> &str {
        "stub"
    }
    fn input_schema(&self) -> Value {
        json!({"type":"object","properties":{},"additionalProperties":false})
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
        json!({"type":"object","properties":{},"additionalProperties":false})
    }
    fn call(&self, _: &Value) -> Result<Value, ToolError> {
        Ok(json!([]))
    }
}

// ── Test ─────────────────────────────────────────────────────────────────────

#[test]
fn tools_list_returns_exactly_two_read_tools() {
    let input = concat!(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        "\n",
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":null}"#,
        "\n",
    );

    let tools: Vec<Box<dyn Tool>> = vec![Box::new(StubCensus), Box::new(StubVerdict)];
    let mut output = Vec::new();
    serve(Cursor::new(input), &mut output, tools, "muster-mcp", "0.1.0").unwrap();

    let text = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2, "expected exactly 2 response lines");

    // Line 0: initialize response
    let init: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(init["result"]["protocolVersion"], "2024-11-05");

    // Line 1: tools/list response
    let list_resp: Value = serde_json::from_str(lines[1]).unwrap();
    let tools_arr = &list_resp["result"]["tools"];
    assert!(tools_arr.is_array());
    let tools_arr = tools_arr.as_array().unwrap();
    assert_eq!(tools_arr.len(), 2, "expected exactly 2 tools");

    let names: Vec<&str> = tools_arr
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert!(
        names.contains(&"sessions_census"),
        "sessions_census must be present"
    );
    assert!(
        names.contains(&"sessions_verdict"),
        "sessions_verdict must be present"
    );

    // Each tool must have a valid JSON-Schema inputSchema
    for tool in tools_arr {
        let schema = &tool["inputSchema"];
        assert_eq!(
            schema["type"], "object",
            "inputSchema type must be 'object' for tool {}",
            tool["name"]
        );
        assert!(
            schema["properties"].is_object(),
            "inputSchema must have 'properties' for tool {}",
            tool["name"]
        );
    }
}
