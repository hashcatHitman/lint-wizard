// SPDX-FileCopyrightText: © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This isn't a real project.

use std::process;

use lint_wizard as _;

/// This isn't a real project.
///
/// ```rust
/// println!("hello")
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = process::Command::new("clippy-driver")
        .arg("-Whelp")
        .output()?;

    let stdout = String::from_utf8(out.stdout)?;
    let stderr = String::from_utf8(out.stderr)?;

    println!("STDOUT:\n");
    println!("{stdout}\n");

    println!("STDERR:\n");
    println!("{stderr}\n");

    Ok(())
}
