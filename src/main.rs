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

    for (lint, tail) in stdout
        .lines()
        .map(str::trim)
        .skip_while(|line| !line.contains("----  -------  -------"))
        .skip(1)
        .map(|line| line.split_once(char::is_whitespace))
        .take_while(Option::is_some)
        .flatten()
    {
        println!("{lint}\n\t{tail}\n");
    }

    Ok(())
}
