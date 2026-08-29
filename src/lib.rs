// SPDX-FileCopyrightText: © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! reading makes you smart.

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

use core::iter::Map;
use core::str::Lines;

use crate::lints::clippy::{ClippyGroup, GroupedClippyLint};
use crate::lints::rustc::{GroupedRustcLint, RustcGroup};
use crate::lints::{BasicLint, LintLevel};

pub mod lints;

pub fn extract_next_lints<'src>(
    lines: &mut Map<Lines<'src>, fn(&str) -> &str>,
) -> impl Iterator<Item = BasicLint<'src>> {
    lines
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
        .map(|[lint_name, default, meaning]| {
            (
                lint_name,
                LintLevel::try_from(default).expect("TODO"),
                meaning,
            )
        })
        .map(|(name, default_level, meaning)| {
            BasicLint::new(name, default_level, meaning)
        })
}

pub fn extract_next_groups<'src>(
    lines: &mut Map<Lines<'src>, fn(&str) -> &str>,
) -> impl Iterator<Item = (&'src str, Vec<&'src str>)> {
    lines
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
}

#[derive(Debug, Clone)]
pub struct Initialized<'src> {
    state: Map<Lines<'src>, fn(&str) -> &str>,
}

impl<'src> Initialized<'src> {
    pub fn new(command_output: &'src str) -> Self {
        Self {
            state: command_output.lines().map(str::trim),
        }
    }

    pub fn parse_rustc_lints(mut self) -> ParsedRustcLints<'src> {
        let rustc_lints: Vec<_> =
            extract_next_lints(self.state.by_ref()).collect();
        ParsedRustcLints {
            state: self.state,
            rustc_lints,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedRustcLints<'src> {
    state: Map<Lines<'src>, fn(&str) -> &str>,
    rustc_lints: Vec<BasicLint<'src>>,
}

impl<'src> ParsedRustcLints<'src> {
    pub fn parse_rustc_groups(self) -> ParsedRustcGroups<'src> {
        let Self {
            mut state,
            mut rustc_lints,
        } = self;
        let rustc_groups: Vec<_> = extract_next_groups(state.by_ref())
            .filter_map(|(group, members)| {
                RustcGroup::try_from(group)
                    .ok()
                    .map(|group| (group, members))
            })
            .collect();

        rustc_lints.sort();
        let pregroup_count = rustc_lints.len();

        let mut rustc_lints_grouped = Vec::with_capacity(pregroup_count);

        'lints: for lint in rustc_lints {
            for &(group, ref members) in &rustc_groups {
                if members.contains(&lint.name()) {
                    let grouped = GroupedRustcLint::new(lint, Some(group));
                    rustc_lints_grouped.push(grouped);
                    continue 'lints;
                }
            }
            let grouped = GroupedRustcLint::new(lint, None);
            rustc_lints_grouped.push(grouped);
        }

        if pregroup_count != rustc_lints_grouped.len() {
            eprintln!(
                "WARNING: Only {} of {pregroup_count} rustc lints survived group assignment!",
                rustc_lints_grouped.len(),
            );
        }

        ParsedRustcGroups {
            state,
            rustc_lints_grouped,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedRustcGroups<'src> {
    state: Map<Lines<'src>, fn(&str) -> &str>,
    rustc_lints_grouped: Vec<GroupedRustcLint<'src>>,
}
impl<'src> ParsedRustcGroups<'src> {
    pub fn parse_clippy_lints(mut self) -> ParsedClippyLints<'src> {
        let clippy_lints: Vec<_> =
            extract_next_lints(self.state.by_ref()).collect();
        ParsedClippyLints {
            state: self.state,
            rustc_lints_grouped: self.rustc_lints_grouped,
            clippy_lints,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedClippyLints<'src> {
    state: Map<Lines<'src>, fn(&str) -> &str>,
    rustc_lints_grouped: Vec<GroupedRustcLint<'src>>,
    clippy_lints: Vec<BasicLint<'src>>,
}

impl<'src> ParsedClippyLints<'src> {
    pub fn parse_clippy_groups(self) -> ParsedClippyGroups<'src> {
        let Self {
            mut state,
            rustc_lints_grouped,
            mut clippy_lints,
        } = self;
        let mut clippy_all = Vec::new();
        let clippy_groups: Vec<_> = extract_next_groups(state.by_ref())
            .filter_map(|(group, members)| {
                if "clippy::all" == group {
                    clippy_all = members;
                    None
                } else {
                    ClippyGroup::try_from(group)
                        .ok()
                        .map(|group| (group, members))
                }
            })
            .collect();

        clippy_lints.sort();
        let pregroup_count = clippy_lints.len();

        let mut clippy_lints_grouped = Vec::with_capacity(pregroup_count);

        for lint in clippy_lints {
            for &(group, ref members) in &clippy_groups {
                if members.contains(&lint.name()) {
                    let is_in_clippy_all = clippy_all.contains(&lint.name());
                    let grouped =
                        GroupedClippyLint::new(lint, group, is_in_clippy_all);

                    clippy_lints_grouped.push(grouped);
                    break;
                }
            }
        }

        if pregroup_count != clippy_lints_grouped.len() {
            eprintln!(
                "WARNING: Only {} of {pregroup_count} clippy lints survived group assignment!",
                clippy_lints_grouped.len(),
            );
        }

        ParsedClippyGroups {
            rustc_lints_grouped,
            clippy_lints_grouped,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedClippyGroups<'src> {
    rustc_lints_grouped: Vec<GroupedRustcLint<'src>>,
    clippy_lints_grouped: Vec<GroupedClippyLint<'src>>,
}

impl<'src> ParsedClippyGroups<'src> {
    pub fn collect(
        self,
    ) -> (Vec<GroupedRustcLint<'src>>, Vec<GroupedClippyLint<'src>>) {
        let Self {
            rustc_lints_grouped,
            clippy_lints_grouped,
        } = self;
        (rustc_lints_grouped, clippy_lints_grouped)
    }
}
