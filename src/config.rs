// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::error::Error;
use std::collections::HashMap;

use cargo_util_schemas::manifest::{TomlLint, TomlManifest};

use crate::lints::clippy::{
    ClippyGroup, ConfiguredClippyLint, GroupedClippyLint,
};
use crate::lints::rustc::{ConfiguredRustcLint, GroupedRustcLint, RustcGroup};

#[derive(Debug, Clone)]
pub struct ConfigManager {
    manifest: TomlManifest,
}

impl ConfigManager {
    pub const fn new(manifest: TomlManifest) -> Self {
        Self { manifest }
    }

    pub fn configure_rustc_lints<'toml, 'src>(
        &'toml self,
        rustc_lints_grouped: Vec<GroupedRustcLint<'src>>,
    ) -> Result<
        (
            Vec<ConfiguredRustcLint<'src>>,
            HashMap<RustcGroup, &'toml TomlLint>,
        ),
        Box<dyn Error>,
    > {
        let manifest_lints = self.manifest.normalized_lints()?.unwrap();

        let rustc_lint_configs = manifest_lints.get("rust").unwrap();

        let mut configured_groups = HashMap::new();
        let mut configured_lints = HashMap::new();

        for (identifier, config) in rustc_lint_configs {
            let identifier = identifier.replace('_', "-");
            if let Ok(group) = RustcGroup::try_from(identifier.as_str()) {
                let _: Option<&TomlLint> =
                    configured_groups.insert(group, config);
            } else {
                let _: Option<&TomlLint> =
                    configured_lints.insert(identifier, config);
            }
        }

        let final_rustc_lint_vec: Vec<_> = rustc_lints_grouped
            .into_iter()
            .map(|curr_lint| {
                let mut highest_piority = None;
                let mut configured_level = None;
                if let Some(group) = curr_lint.group()
                    && let Some(config) = configured_groups.get(&group)
                    && Some(config.priority()) > highest_piority
                {
                    configured_level = Some(config.level());
                    highest_piority = Some(config.priority());
                }
                if let Some(lint_config) =
                    configured_lints.remove(curr_lint.name())
                    && Some(lint_config.priority()) > highest_piority
                {
                    configured_level = Some(lint_config.level());
                }
                ConfiguredRustcLint::new(
                    curr_lint,
                    configured_level.map(Into::into),
                )
            })
            .collect();

        Ok((final_rustc_lint_vec, configured_groups))
    }

    pub fn configure_clippy_lints<'toml, 'src>(
        &'toml self,
        clippy_lints_grouped: Vec<GroupedClippyLint<'src>>,
    ) -> Result<
        (
            Vec<ConfiguredClippyLint<'src>>,
            HashMap<ClippyGroup, &'toml TomlLint>,
        ),
        Box<dyn Error>,
    > {
        let manifest_lints = self.manifest.normalized_lints()?.unwrap();

        let clippy_lint_configs = manifest_lints.get("clippy").unwrap();

        let mut configured_groups = HashMap::new();
        let mut configured_lints = HashMap::new();

        let mut clippyall = None;

        for (identifier, config) in clippy_lint_configs {
            let mut identifier = identifier.replace('_', "-");
            identifier.insert_str(0, "clippy::");
            if let Ok(group) = ClippyGroup::try_from(identifier.as_str()) {
                let _: Option<&TomlLint> =
                    configured_groups.insert(group, config);
            } else if "clippy::all" == identifier {
                clippyall = Some(config);
            } else {
                let _: Option<&TomlLint> =
                    configured_lints.insert(identifier, config);
            }
        }

        let final_clippy_lint_vec: Vec<_> = clippy_lints_grouped
            .into_iter()
            .map(|curr_lint| {
                let mut highest_piority = None;
                let mut configured_level = None;
                if let Some(clippyallconf) = clippyall
                    && curr_lint.is_in_clippy_all()
                    && Some(clippyallconf.priority()) > highest_piority
                {
                    configured_level = Some(clippyallconf.level());
                    highest_piority = Some(clippyallconf.priority());
                }
                if let Some(group_config) =
                    configured_groups.get(&curr_lint.primary_group())
                    && Some(group_config.priority()) > highest_piority
                {
                    configured_level = Some(group_config.level());
                    highest_piority = Some(group_config.priority());
                }
                if let Some(lint_config) =
                    configured_lints.remove(curr_lint.name())
                    && Some(lint_config.priority()) > highest_piority
                {
                    configured_level = Some(lint_config.level());
                }

                ConfiguredClippyLint::new(
                    curr_lint,
                    configured_level.map(Into::into),
                )
            })
            .collect();

        Ok((final_clippy_lint_vec, configured_groups))
    }
}
