#![allow(dead_code)]
#![allow(unused)]

use anyhow::{Context, Result};
use std::process::Command;

mod problems;

fn main() -> Result<()> {
    // Run `cargo test` command
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .with_context(|| "Failed to run 'cargo test' command")?;

    // Check if the command execution was successful
    if output.status.success() {
        println!("Tests passed successfully");
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Tests failed: {}", error_message);
    }
}
