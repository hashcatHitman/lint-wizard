// SPDX-FileCopyrightText: © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This isn't a real project.

#![allow(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::expect_used,
    clippy::type_complexity,
    clippy::use_debug,
    clippy::unwrap_used,
    reason = "todo later"
)]

use std::process;

use lint_wizard::{self as _, Initialized};

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

    let (rustc_lints, rustc_groups, clippy_lints, clippy_groups) =
        Initialized::new(&stdout)
            .parse_rustc_lints()
            .parse_rustc_groups()
            .parse_clippy_lints()
            .parse_clippy_groups()
            .collect();

    for rustc_lint in rustc_lints {
        println!(
            "{}\n\t{}\n\t{}\n",
            rustc_lint.name(),
            rustc_lint.default_level(),
            rustc_lint.meaning()
        );
    }

    for (group, members) in rustc_groups {
        println!("{group}\n\t{members:?}\n");
    }

    for clippy_lint in clippy_lints {
        println!(
            "{}\n\t{}\n\t{}\n",
            clippy_lint.name(),
            clippy_lint.default_level(),
            clippy_lint.meaning()
        );
    }

    for (group, members) in clippy_groups {
        println!("{group}\n\t{members:?}\n");
    }

    Ok(())
}
