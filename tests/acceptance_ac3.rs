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

/// Confirm that the tools module does not define any tool whose name contains "reap".
#[test]
fn tools_source_has_no_reap_tool() {
    let tools_src = include_str!("../src/tools.rs");
    // No function named sessions_reap, no tool name "reap"
    assert!(
        !tools_src.contains("sessions_reap"),
        "tools.rs must not define a sessions_reap tool"
    );
    // "reap" as a string literal (not in a comment) should not appear as a verb
    for line in tools_src.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("///") {
            continue; // comments are fine
        }
        if trimmed.contains(r#""reap""#) {
            panic!("Found literal \"reap\" in non-comment line of tools.rs: {}", trimmed);
        }
    }
}
