//! A public GitHub repository, named by its owner and its name.
//!
//! **The two halves are two fields and not one string**, because "a half carrying a slash is
//! both halves pasted into one field" — a single `owner/name` would let `owner` be the whole
//! address and the mistake would only show up when something tried to clone it.

use std::fmt;

/// The one slash that stands between an owner and a name.
const THE_SLASH_BETWEEN_THE_TWO_HALVES: char = '/';

/// A repository, and never a bare string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Repository {
    /// Who owns it.
    pub owner: String,
    /// What it is called.
    pub name: String,
}

/// A boundary that names nothing, because half of it is blank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AHalfThatIdentifiesNothing(pub String);

impl TryFrom<&str> for Repository {
    type Error = AHalfThatIdentifiesNothing;

    fn try_from(address: &str) -> Result<Self, Self::Error> {
        let halves: Vec<&str> = address.split(THE_SLASH_BETWEEN_THE_TWO_HALVES).collect();
        if halves.len() != 2 {
            return Err(AHalfThatIdentifiesNothing(address.to_string()));
        }
        if halves[0].trim().is_empty() || halves[1].trim().is_empty() {
            return Err(AHalfThatIdentifiesNothing(address.to_string()));
        }
        Ok(Repository {
            owner: halves[0].to_string(),
            name: halves[1].to_string(),
        })
    }
}

impl fmt::Display for Repository {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.owner, self.name)
    }
}

impl fmt::Display for AHalfThatIdentifiesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} names no repository", self.0)
    }
}

impl std::error::Error for AHalfThatIdentifiesNothing {}
