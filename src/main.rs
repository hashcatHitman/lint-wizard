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

use core::error::Error;
use std::fs;

use cargo_util_schemas::manifest::TomlManifest;
use lint_wizard::config::ConfigManager;
use lint_wizard::report::LintReport;
use lint_wizard::{self as _, Initialized, get_command_output};

/// This isn't a real project.
///
/// ```rust
/// println!("hello")
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    let stdout = get_command_output("clippy-driver", "-Whelp")?;

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

    let (configured_clippy_lints, configured_clippy_groups) =
        manager.configure_clippy_lints(clippy_lints_grouped)?;

    let lint_report = LintReport::new(
        configured_rustc_groups,
        configured_rustc_lints,
        configured_clippy_groups,
        configured_clippy_lints,
    );

    fs::write("./lint_wizard_report.toml", lint_report.to_string())?;

    Ok(())
}
