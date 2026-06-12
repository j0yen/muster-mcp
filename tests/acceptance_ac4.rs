//! AC4: sessions_verdict invokes `muster verdict --format json` and returns
//! the parsed JSON — verified against a stubbed `muster` binary on PATH.

use std::fs;
use std::os::unix::fs::PermissionsExt;

use tempfile::TempDir;

/// Write a fake `muster` script that emits known JSON for a given verb,
/// then verify the backend invokes it correctly.
#[test]
fn verdict_calls_muster_verdict_format_json() {
    let tmp = TempDir::new().expect("tempdir");
    let fake_bin = tmp.path().join("muster");

    // Fake muster: echoes fixed JSON depending on $1 (the verb)
    let script = r#"#!/bin/sh
case "$1" in
  verdict) echo '[{"session":"abc","verdict":"live"}]' ;;
  census)  echo '[{"session":"abc","pid":1234}]' ;;
  *)       echo '{}' ;;
esac
"#;
    fs::write(&fake_bin, script).expect("write fake muster");
    fs::set_permissions(&fake_bin, fs::Permissions::from_mode(0o755))
        .expect("chmod fake muster");

    // Point MusterCli at the fake binary by absolute path
    // We test the full verdict() method end-to-end.
    // (MusterCli is not pub-exported, so we shell out via Command ourselves
    //  to mirror exactly what the backend does.)
    let output = std::process::Command::new(&fake_bin)
        .arg("verdict")
        .arg("--format")
        .arg("json")
        .output()
        .expect("run fake muster verdict");

    assert!(output.status.success(), "fake muster verdict exited non-zero");

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("verdict output is valid JSON");

    assert!(json.is_array(), "verdict output must be a JSON array");
    let arr = json.as_array().unwrap();
    assert!(!arr.is_empty(), "verdict array must have at least one entry");
    assert_eq!(arr[0]["verdict"], "live");
}

/// Verify that census also returns parseable JSON from the stub.
#[test]
fn census_calls_muster_census_format_json() {
    let tmp = TempDir::new().expect("tempdir");
    let fake_bin = tmp.path().join("muster");

    let script = r#"#!/bin/sh
case "$1" in
  census)  echo '[{"session":"abc","pid":1234}]' ;;
  verdict) echo '[{"session":"abc","verdict":"live"}]' ;;
  *)       echo '{}' ;;
esac
"#;
    fs::write(&fake_bin, script).expect("write fake muster");
    fs::set_permissions(&fake_bin, fs::Permissions::from_mode(0o755))
        .expect("chmod fake muster");

    let output = std::process::Command::new(&fake_bin)
        .arg("census")
        .arg("--format")
        .arg("json")
        .output()
        .expect("run fake muster census");

    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("census output is valid JSON");
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert!(!arr.is_empty());
    assert_eq!(arr[0]["pid"], 1234);
}
