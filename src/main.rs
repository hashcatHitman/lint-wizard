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

    for (lint, default, meaning) in rustc_lints {
        println!("{lint}\n\t{default}\n\t{meaning}\n");
    }

    for (group, members) in rustc_groups {
        println!("{group}\n\t{members:?}\n");
    }

    for (lint, default, meaning) in clippy_lints {
        println!("{lint}\n\t{default}\n\t{meaning}\n");
    }

    for (group, members) in clippy_groups {
        println!("{group}\n\t{members:?}\n");
    }

    Ok(())
}
