// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::fmt::{self, Display};

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
