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

    let mut stdout_lines = stdout.lines().map(str::trim);

    for [lint, default, meaning] in stdout_lines
        .by_ref()
        .skip_while(|line| !line.contains("----  -------  -------"))
        .skip(1)
        .map(|line| line.split_once(char::is_whitespace))
        .take_while(Option::is_some)
        .flatten()
        .filter_map(|(lint, tail)| {
            tail.trim()
                .split_once(char::is_whitespace)
                .map(|(default, meaning)| Some([lint, default, meaning.trim()]))
        })
        .flatten()
    {
        println!("{lint}\n\t{default}\n\t{meaning}\n");
    }

    for (group, members) in stdout_lines
        .by_ref()
        .skip_while(|line| !line.contains("----  ---------"))
        .skip(1)
        .map(|line| line.split_once(char::is_whitespace))
        .take_while(Option::is_some)
        .flatten()
        .map(|(group_name, tail)| {
            (
                group_name,
                tail.trim().split(", ").map(str::trim).collect::<Vec<_>>(),
            )
        })
    {
        println!("{group}\n\t{members:?}\n");
    }

    Ok(())
}
