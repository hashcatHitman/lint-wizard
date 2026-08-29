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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustcGroup {
    DeprecatedSafe,
    FutureIncompatible,
    KeywordIdents,
    LetUnderscore,
    NonstandardStyle,
    RefiningImplTrait,
    Rust2018Compatibility,
    Rust2018Idioms,
    Rust2021Compatibility,
    Rust2024Compatibility,
    UnknownOrMalformedDiagnosticAttributes,
    Unused,
}

impl Display for RustcGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DeprecatedSafe => write!(f, "deprecated_safe"),
            Self::FutureIncompatible => write!(f, "future_incompatible"),
            Self::KeywordIdents => write!(f, "keyword_idents"),
            Self::LetUnderscore => write!(f, "let_underscore"),
            Self::NonstandardStyle => write!(f, "nonstandard_style"),
            Self::RefiningImplTrait => write!(f, "refining_impl_trait"),
            Self::Rust2018Compatibility => write!(f, "rust_2018_compatibility"),
            Self::Rust2018Idioms => write!(f, "rust_2018_idioms"),
            Self::Rust2021Compatibility => write!(f, "rust_2021_compatibility"),
            Self::Rust2024Compatibility => write!(f, "rust_2024_compatibility"),
            Self::UnknownOrMalformedDiagnosticAttributes => {
                write!(f, "unknown_or_malformed_diagnostic_attributes")
            }
            Self::Unused => write!(f, "unused"),
        }
    }
}

impl<'src> TryFrom<&'src str> for RustcGroup {
    type Error = &'src str;

    fn try_from(value: &'src str) -> Result<Self, Self::Error> {
        match value {
            "deprecated-safe" => Ok(Self::DeprecatedSafe),
            "future-incompatible" => Ok(Self::FutureIncompatible),
            "keyword-idents" => Ok(Self::KeywordIdents),
            "let-underscore" => Ok(Self::LetUnderscore),
            "nonstandard-style" => Ok(Self::NonstandardStyle),
            "refining-impl-trait" => Ok(Self::RefiningImplTrait),
            "rust-2018-compatibility" => Ok(Self::Rust2018Compatibility),
            "rust-2018-idioms" => Ok(Self::Rust2018Idioms),
            "rust-2021-compatibility" => Ok(Self::Rust2021Compatibility),
            "rust-2024-compatibility" => Ok(Self::Rust2024Compatibility),
            "unknown-or-malformed-diagnostic-attributes" => {
                Ok(Self::UnknownOrMalformedDiagnosticAttributes)
            }
            "unused" => Ok(Self::Unused),
            other => Err(other),
        }
    }
}

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
