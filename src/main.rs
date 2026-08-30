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

use std::{fs, process};

use cargo_util_schemas::manifest::TomlManifest;
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

    let (rustc_lints_grouped, clippy_lints_grouped) = Initialized::new(&stdout)
        .parse_rustc_lints()
        .parse_rustc_groups()
        .parse_clippy_lints()
        .parse_clippy_groups()
        .collect();

    for rustc_lint in rustc_lints_grouped {
        println!(
            "{}\n\t{}\n\t{}\n\t{}\n",
            rustc_lint.name(),
            rustc_lint.group().map_or_else(
                || "NO GROUP".to_owned(),
                |group| group.to_string()
            ),
            rustc_lint.default_level(),
            rustc_lint.meaning()
        );
    }

    for clippy_lint in clippy_lints_grouped {
        println!(
            "{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
            clippy_lint.name(),
            clippy_lint.primary_group(),
            if clippy_lint.is_in_clippy_all() {
                "in clippy::all"
            } else {
                "not in clippy::all"
            },
            clippy_lint.default_level(),
            clippy_lint.meaning()
        );
    }

    let manifest_contents = fs::read_to_string("./Cargo.toml")?;
    let manifest = toml::from_str::<TomlManifest>(&manifest_contents)?;

    let manifest_lints = manifest.normalized_lints()?.unwrap();

    let rust_lint_config = manifest_lints.get("rust").unwrap();

    for (lint_name, config) in rust_lint_config {
        println!("{lint_name}\n\t{config:?}\n");
    }

    let clippy_lint_config = manifest_lints.get("clippy").unwrap();

    for (lint_name, config) in clippy_lint_config {
        println!("{lint_name}\n\t{config:?}\n");
    }

    println!("Found config for {} rustc lints!\n", rust_lint_config.len());
    println!(
        "Found config for {} clippy lints!\n",
        clippy_lint_config.len()
    );

    Ok(())
}
