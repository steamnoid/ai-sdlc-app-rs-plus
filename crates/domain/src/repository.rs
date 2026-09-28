//! A public GitHub repository, named by its owner and its name.
//!
//! **The two halves are two fields and not one string**, because "a half carrying a slash is
//! both halves pasted into one field" — a single `owner/name` would let `owner` be the whole
//! address and the mistake would only show up when something tried to clone it.

use std::fmt;

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

    fn try_from(_address: &str) -> Result<Self, Self::Error> {
        Ok(Repository {
            owner: String::new(),
            name: String::new(),
        })
    }
}

impl fmt::Display for Repository {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _the_two_halves_were_not_asked_about = self;
        formatter.write_str("")
    }
}

impl fmt::Display for AHalfThatIdentifiesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} names no repository", self.0)
    }
}

impl std::error::Error for AHalfThatIdentifiesNothing {}
