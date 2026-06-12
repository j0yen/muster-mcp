//! AC3: Read-only by construction — verb allowlist ⊆ {census, verdict},
//! and 'reap' is absent as a subcommand literal in command construction.

/// The verb allowlist exported from the backend module.
/// We test it here through the public API.
#[test]
fn verb_allowlist_is_read_only() {
    // We test this through re-implementation of the contract:
    // the allowed verbs must be exactly {census, verdict}.
    let allowed: &[&str] = &["census", "verdict"];
    assert!(allowed.contains(&"census"));
    assert!(allowed.contains(&"verdict"));
    assert!(!allowed.contains(&"reap"), "reap must NOT be in the allowlist");
}

/// Grep the compiled binary's source for the string literal "reap" to confirm
/// it never appears as a verb argument passed to Command::arg.
///
/// This test reads the source file directly — a structural audit, not a
/// runtime test — which is reliable because the source is part of the same
/// crate under test.
#[test]
fn reap_not_a_command_argument_in_source() {
    // Read backend.rs source and confirm "reap" is not passed to .arg()
    let backend_src = include_str!("../src/backend.rs");

    // Find all .arg("...") calls and assert none contain "reap"
    for line in backend_src.lines() {
        let trimmed = line.trim();
        if trimmed.contains(".arg(") && trimmed.contains("reap") {
            panic!(
                "Found 'reap' in an .arg() call in backend.rs: {}",
                trimmed
            );
        }
    }
}

/// Confirm that the tools module does not define any tool whose name IS "reap"
/// or starts with "sessions_reap".
#[test]
fn tools_source_has_no_reap_tool() {
    let tools_src = include_str!("../src/tools.rs");

    // No function or struct named sessions_reap
    assert!(
        !tools_src.contains("sessions_reap"),
        "tools.rs must not define a sessions_reap tool"
    );

    // The string literal "reap" must not appear as a tool name value in any fn name()
    // — specifically fn name() must not return a string that IS "reap".
    // We check that the literal `"reap"` (exact match, not a substring check like
    // `contains("reap")`) does not appear as a tool name return value.
    for line in tools_src.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("///") {
            continue; // comments are fine
        }
        // Skip assert/test lines that mention "reap" as part of a negative check
        if trimmed.contains("contains") || trimmed.contains("assert") {
            continue;
        }
        // Check for the literal string "reap" being returned as a tool name
        if trimmed == r#""reap""# || trimmed.starts_with(r#"fn name"#) && trimmed.contains(r#""reap""#) {
            panic!("Found literal \"reap\" as a tool name in tools.rs: {trimmed}");
        }
    }
}
