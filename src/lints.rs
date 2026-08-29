// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::fmt::{self, Display};

pub mod clippy;
pub mod rustc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LintLevel {
    Allow,
    Warn,
    Deny,
    Forbid,
}

impl Display for LintLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Allow => write!(f, "allow"),
            Self::Warn => write!(f, "warn"),
            Self::Deny => write!(f, "deny"),
            Self::Forbid => write!(f, "forbid"),
        }
    }
}

impl<'src> TryFrom<&'src str> for LintLevel {
    type Error = &'src str;

    fn try_from(value: &'src str) -> Result<Self, Self::Error> {
        match value {
            "allow" => Ok(Self::Allow),
            "warn" => Ok(Self::Warn),
            "deny" => Ok(Self::Deny),
            "forbid" => Ok(Self::Forbid),
            unknown => Err(unknown),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasicLint<'src> {
    name: &'src str,
    default_level: LintLevel,
    meaning: &'src str,
}

impl<'src> BasicLint<'src> {
    pub const fn new(
        name: &'src str,
        default_level: LintLevel,
        meaning: &'src str,
    ) -> Self {
        Self {
            name,
            default_level,
            meaning,
        }
    }

    pub const fn name(self) -> &'src str {
        self.name
    }

    pub const fn default_level(self) -> LintLevel {
        self.default_level
    }

    pub const fn meaning(self) -> &'src str {
        self.meaning
    }
}
