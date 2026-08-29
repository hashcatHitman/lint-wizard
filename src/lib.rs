// SPDX-FileCopyrightText: © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! reading makes you smart.

use core::iter::Map;
use core::str::Lines;

pub fn extract_next_lints<'src>(
    lines: &mut Map<Lines<'src>, fn(&str) -> &str>,
) -> Vec<[&'src str; 3]> {
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
        .collect()
}

pub fn extract_next_groups<'src>(
    lines: &mut Map<Lines<'src>, fn(&str) -> &str>,
) -> Vec<(&'src str, Vec<&'src str>)> {
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
        .collect()
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
        let rustc_lints: Vec<_> = extract_next_lints(self.state.by_ref());
        ParsedRustcLints {
            state: self.state,
            rustc_lints,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedRustcLints<'src> {
    state: Map<Lines<'src>, fn(&str) -> &str>,
    rustc_lints: Vec<[&'src str; 3]>,
}

impl<'src> ParsedRustcLints<'src> {
    pub fn parse_rustc_groups(mut self) -> ParsedRustcGroups<'src> {
        let rustc_groups: Vec<_> = extract_next_groups(self.state.by_ref());
        ParsedRustcGroups {
            state: self.state,
            rustc_lints: self.rustc_lints,
            rustc_groups,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedRustcGroups<'src> {
    state: Map<Lines<'src>, fn(&str) -> &str>,
    rustc_lints: Vec<[&'src str; 3]>,
    rustc_groups: Vec<(&'src str, Vec<&'src str>)>,
}
impl<'src> ParsedRustcGroups<'src> {
    pub fn parse_clippy_lints(mut self) -> ParsedClippyLints<'src> {
        let clippy_lints: Vec<_> = extract_next_lints(self.state.by_ref());
        ParsedClippyLints {
            state: self.state,
            rustc_lints: self.rustc_lints,
            rustc_groups: self.rustc_groups,
            clippy_lints,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedClippyLints<'src> {
    state: Map<Lines<'src>, fn(&str) -> &str>,
    rustc_lints: Vec<[&'src str; 3]>,
    rustc_groups: Vec<(&'src str, Vec<&'src str>)>,
    clippy_lints: Vec<[&'src str; 3]>,
}

impl<'src> ParsedClippyLints<'src> {
    pub fn parse_clippy_groups(mut self) -> ParsedClippyGroups<'src> {
        let clippy_groups: Vec<_> = extract_next_groups(self.state.by_ref());
        ParsedClippyGroups {
            rustc_lints: self.rustc_lints,
            rustc_groups: self.rustc_groups,
            clippy_lints: self.clippy_lints,
            clippy_groups,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedClippyGroups<'src> {
    rustc_lints: Vec<[&'src str; 3]>,
    rustc_groups: Vec<(&'src str, Vec<&'src str>)>,
    clippy_lints: Vec<[&'src str; 3]>,
    clippy_groups: Vec<(&'src str, Vec<&'src str>)>,
}

impl<'src> ParsedClippyGroups<'src> {
    pub fn collect(
        self,
    ) -> (
        Vec<[&'src str; 3]>,
        Vec<(&'src str, Vec<&'src str>)>,
        Vec<[&'src str; 3]>,
        Vec<(&'src str, Vec<&'src str>)>,
    ) {
        let Self {
            rustc_lints,
            rustc_groups,
            clippy_lints,
            clippy_groups,
        } = self;
        (rustc_lints, rustc_groups, clippy_lints, clippy_groups)
    }
}
