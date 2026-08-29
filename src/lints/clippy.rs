// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::fmt::{self, Display};

use crate::lints::{BasicLint, LintLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClippyGroup {
    Correctness,
    Suspicious,
    Style,
    Complexity,
    Perf,
    Pedantic,
    Restriction,
    Nursery,
    Cargo,
}

impl Display for ClippyGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Correctness => write!(f, "correctness"),
            Self::Suspicious => write!(f, "suspicious"),
            Self::Style => write!(f, "style"),
            Self::Complexity => write!(f, "complexity"),
            Self::Perf => write!(f, "perf"),
            Self::Pedantic => write!(f, "pedantic"),
            Self::Restriction => write!(f, "restriction"),
            Self::Nursery => write!(f, "nursery"),
            Self::Cargo => write!(f, "cargo"),
        }
    }
}

impl<'src> TryFrom<&'src str> for ClippyGroup {
    type Error = &'src str;

    fn try_from(value: &'src str) -> Result<Self, Self::Error> {
        match value {
            "clippy::correctness" => Ok(Self::Correctness),
            "clippy::suspicious" => Ok(Self::Suspicious),
            "clippy::style" => Ok(Self::Style),
            "clippy::complexity" => Ok(Self::Complexity),
            "clippy::perf" => Ok(Self::Perf),
            "clippy::pedantic" => Ok(Self::Pedantic),
            "clippy::restriction" => Ok(Self::Restriction),
            "clippy::nursery" => Ok(Self::Nursery),
            "clippy::cargo" => Ok(Self::Cargo),
            other => Err(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupedClippyLint<'src> {
    name: &'src str,
    primary_group: ClippyGroup,
    is_in_clippy_all: bool,
    default_level: LintLevel,
    meaning: &'src str,
}

impl<'src> GroupedClippyLint<'src> {
    pub const fn new(
        base: BasicLint<'src>,
        primary_group: ClippyGroup,
        is_in_clippy_all: bool,
    ) -> Self {
        let BasicLint {
            name,
            default_level,
            meaning,
        } = base;
        Self {
            name,
            primary_group,
            is_in_clippy_all,
            default_level,
            meaning,
        }
    }

    pub const fn name(self) -> &'src str {
        self.name
    }

    pub const fn primary_group(self) -> ClippyGroup {
        self.primary_group
    }

    pub const fn is_in_clippy_all(self) -> bool {
        self.is_in_clippy_all
    }

    pub const fn default_level(&self) -> LintLevel {
        self.default_level
    }

    pub const fn meaning(self) -> &'src str {
        self.meaning
    }
}
