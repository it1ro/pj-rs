use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
fn test_help_shows() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pj")?;
    cmd.arg("--help");
    cmd.assert().success().stdout(predicates::str::contains("Dump project context"));
    Ok(())
}

#[test]
fn test_tree_shows() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pj")?;
    cmd.arg("--tree");
    cmd.assert().success();
    Ok(())
}
