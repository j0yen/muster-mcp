//! Backend — shells the `muster` binary with allowlisted verb literals.
//!
//! Only `census` and `verdict` are reachable. `reap` and any other verb
//! are structurally absent from this module.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use serde_json::Value;

/// Allowlisted verbs. `reap` is intentionally absent.
const ALLOWED_VERBS: &[&str] = &["census", "verdict"];

/// Interface to the installed `muster` binary.
///
/// The binary path defaults to `muster` (resolved on `$PATH`).
/// Override via `--muster-bin` CLI flag or the `MUSTER_BIN` environment variable.
#[derive(Debug, Clone)]
pub struct MusterCli {
    bin: PathBuf,
}

impl MusterCli {
    /// Create a `MusterCli` that uses the given binary path.
    pub fn new(bin: impl Into<PathBuf>) -> Self {
        MusterCli { bin: bin.into() }
    }

    /// Create a `MusterCli` resolving `muster` from `$PATH`.
    pub fn from_path() -> Self {
        MusterCli::new("muster")
    }

    /// Return the binary path (read-only access for tests).
    #[cfg(test)]
    pub fn bin(&self) -> &Path {
        &self.bin
    }

    /// Run `muster census --format json` and return the parsed JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if the binary cannot be launched, exits non-zero,
    /// or its output is not valid JSON.
    pub fn census(&self) -> Result<Value> {
        self.run_verb("census")
    }

    /// Run `muster verdict --format json` and return the parsed JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if the binary cannot be launched, exits non-zero,
    /// or its output is not valid JSON.
    pub fn verdict(&self) -> Result<Value> {
        self.run_verb("verdict")
    }

    /// Internal: run an allowlisted verb.
    fn run_verb(&self, verb: &str) -> Result<Value> {
        // Safety: verb is always one of the ALLOWED_VERBS literals called from
        // census() / verdict(). The public API has no way to pass an arbitrary verb.
        assert!(
            ALLOWED_VERBS.contains(&verb),
            "BUG: verb '{verb}' not in allowlist"
        );

        let output = Command::new(&self.bin)
            .arg(verb)
            .arg("--format")
            .arg("json")
            .output()
            .with_context(|| format!("failed to launch {:?}", self.bin))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "muster {verb} exited with {}: {stderr}",
                output.status
            );
        }

        let json: Value = serde_json::from_slice(&output.stdout)
            .with_context(|| format!("muster {verb} output was not valid JSON"))?;

        Ok(json)
    }
}

/// Confirm that the verb set reachable from this module is ⊆ {census, verdict}.
///
/// This is a compile-time-verifiable invariant: the only public methods that
/// invoke the binary are `census()` and `verdict()`, both of which pass a
/// literal string through `ALLOWED_VERBS`. The `reap` subcommand has no
/// representation here.
pub fn allowed_verbs() -> &'static [&'static str] {
    ALLOWED_VERBS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verb_allowlist_is_read_only() {
        let verbs = allowed_verbs();
        assert!(verbs.contains(&"census"), "census must be allowed");
        assert!(verbs.contains(&"verdict"), "verdict must be allowed");
        assert!(!verbs.contains(&"reap"), "reap must NOT be in the allowlist");
    }

    #[test]
    fn default_bin_is_muster() {
        let cli = MusterCli::from_path();
        assert_eq!(cli.bin(), Path::new("muster"));
    }

    #[test]
    fn custom_bin_is_stored() {
        let cli = MusterCli::new("/tmp/fake-muster");
        assert_eq!(cli.bin(), Path::new("/tmp/fake-muster"));
    }
}
