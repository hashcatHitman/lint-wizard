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
use lint_wizard::config::ConfigManager;
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

    let manifest_contents = fs::read_to_string("./Cargo.toml")?;
    let manifest = toml::from_str::<TomlManifest>(&manifest_contents)?;
    let manager = ConfigManager::new(manifest);

    let (configured_rustc_lints, configured_rustc_groups) =
        manager.configure_rustc_lints(rustc_lints_grouped)?;

    let mut configured_rustc_groups: Vec<_> =
        configured_rustc_groups.iter().collect();
    configured_rustc_groups.sort_by_key(|&(group, _)| group);

    println!("configured rustc groups:");
    for (group, config) in configured_rustc_groups {
        println!("{group}\n\t{config:?}\n");
    }

    println!("configured rustc lints:");
    for rustc_lint in configured_rustc_lints {
        println!(
            "{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
            rustc_lint.name(),
            rustc_lint.group().map_or_else(
                || "NO GROUP".to_owned(),
                |group| group.to_string()
            ),
            rustc_lint.default_level(),
            rustc_lint
                .configured_level()
                .map_or_else(|| "unset".to_owned(), |level| level.to_string()),
            rustc_lint.meaning()
        );
    }

    println!("grouped clippy lints:");
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

    Ok(())
}
