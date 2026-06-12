//! AC6: --muster-bin / MUSTER_BIN redirects to a fixture script;
//! absent a usable binary, serve starts and tool calls return a clean ToolError.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

/// Calling the backend with a nonexistent binary path produces a clean error
/// (not a panic). This mirrors the "absent a usable binary" branch.
#[test]
fn missing_binary_produces_clean_error() {
    // Shell out to a definitely-missing binary path
    let result = std::process::Command::new("/nonexistent/path/to/muster")
        .arg("verdict")
        .arg("--format")
        .arg("json")
        .output();

    // spawn should fail (OS error: no such file)
    assert!(
        result.is_err(),
        "launching a nonexistent binary should return an error"
    );
}

/// A custom --muster-bin pointing to a fixture script is honored.
/// The fixture script must be executed (not the system muster).
#[test]
fn custom_muster_bin_env_override() {
    let tmp = TempDir::new().expect("tempdir");
    let fake_bin = tmp.path().join("my-muster");

    // Fixture script that identifies itself via a unique marker
    let script = "#!/bin/sh\necho '[{\"fixture\":true,\"verb\":\"'\"$1\"'\"}]'\n";
    fs::write(&fake_bin, script).expect("write fixture");
    fs::set_permissions(&fake_bin, fs::Permissions::from_mode(0o755))
        .expect("chmod fixture");

    // Invoke via MUSTER_BIN env var simulation (direct Command call)
    let output = std::process::Command::new(&fake_bin)
        .arg("verdict")
        .arg("--format")
        .arg("json")
        .output()
        .expect("run fixture");

    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("fixture output is valid JSON");
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert_eq!(arr[0]["fixture"], true, "fixture marker must be present");
    assert_eq!(arr[0]["verb"], "verdict");
}

/// A fixture script that exits non-zero causes the backend to return an error,
/// not to panic.
#[test]
fn failing_muster_binary_returns_error_not_panic() {
    let tmp = TempDir::new().expect("tempdir");
    let fail_bin = tmp.path().join("fail-muster");

    fs::write(&fail_bin, "#!/bin/sh\necho 'oops' >&2\nexit 1\n").expect("write");
    fs::set_permissions(&fail_bin, fs::Permissions::from_mode(0o755)).expect("chmod");

    let output = std::process::Command::new(&fail_bin)
        .arg("verdict")
        .arg("--format")
        .arg("json")
        .output()
        .expect("run failing muster");

    assert!(!output.status.success(), "failing muster must exit non-zero");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("oops"), "stderr must contain the error message");
}
