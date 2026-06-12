//! MCP Tool implementations for `sessions_census` and `sessions_verdict`.

use mcp_core::{Tool, ToolError};
use serde_json::{json, Value};

use crate::backend::MusterCli;

/// Empty-object JSON Schema (both tools take no arguments).
fn empty_schema() -> Value {
    json!({
        "type": "object",
        "properties": {},
        "additionalProperties": false
    })
}

// ---------------------------------------------------------------------------
// sessions_census
// ---------------------------------------------------------------------------

/// MCP tool: enumerate live Claude sessions with origin attribution.
pub struct SessionsCensus {
    cli: MusterCli,
}

impl SessionsCensus {
    /// Create a new `SessionsCensus` tool backed by the given `MusterCli`.
    pub fn new(cli: MusterCli) -> Self {
        SessionsCensus { cli }
    }
}

impl Tool for SessionsCensus {
    fn name(&self) -> &str {
        "sessions_census"
    }

    fn description(&self) -> &str {
        "Enumerate live Claude sessions with origin attribution and bus reconciliation. \
         Read-only. Equivalent to `muster census --format json`."
    }

    fn input_schema(&self) -> Value {
        empty_schema()
    }

    fn call(&self, _args: &Value) -> Result<Value, ToolError> {
        self.cli
            .census()
            .map_err(|e| ToolError::new(e.to_string()))
    }
}

// ---------------------------------------------------------------------------
// sessions_verdict
// ---------------------------------------------------------------------------

/// MCP tool: annotate the census with live/duplicate/orphan/stale verdicts.
pub struct SessionsVerdict {
    cli: MusterCli,
}

impl SessionsVerdict {
    /// Create a new `SessionsVerdict` tool backed by the given `MusterCli`.
    pub fn new(cli: MusterCli) -> Self {
        SessionsVerdict { cli }
    }
}

impl Tool for SessionsVerdict {
    fn name(&self) -> &str {
        "sessions_verdict"
    }

    fn description(&self) -> &str {
        "Annotate the live-session census with live/duplicate/orphan/stale verdicts \
         and supporting evidence. Read-only. Equivalent to `muster verdict --format json`."
    }

    fn input_schema(&self) -> Value {
        empty_schema()
    }

    fn call(&self, _args: &Value) -> Result<Value, ToolError> {
        self.cli
            .verdict()
            .map_err(|e| ToolError::new(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_names_are_correct() {
        let cli = MusterCli::from_path();
        let census = SessionsCensus::new(cli.clone());
        let verdict = SessionsVerdict::new(cli);
        assert_eq!(census.name(), "sessions_census");
        assert_eq!(verdict.name(), "sessions_verdict");
    }

    #[test]
    fn tool_schemas_are_empty_objects() {
        let cli = MusterCli::from_path();
        let census = SessionsCensus::new(cli.clone());
        let verdict = SessionsVerdict::new(cli);
        assert_eq!(census.input_schema()["type"], "object");
        assert!(census.input_schema()["properties"].is_object());
        assert_eq!(verdict.input_schema()["type"], "object");
        assert!(verdict.input_schema()["properties"].is_object());
    }

    #[test]
    fn no_reap_in_tool_names() {
        let cli = MusterCli::from_path();
        let census = SessionsCensus::new(cli.clone());
        let verdict = SessionsVerdict::new(cli);
        assert!(
            !census.name().contains("reap"),
            "sessions_census must not contain 'reap'"
        );
        assert!(
            !verdict.name().contains("reap"),
            "sessions_verdict must not contain 'reap'"
        );
    }
}
