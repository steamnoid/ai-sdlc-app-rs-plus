//! The party that asked for the work.
//!
//! **Who asked and which discipline answers are two questions in two fields.** A set one
//! field wide is a set where the two have already been confused, and the confusion is
//! invisible until a run records an answer nobody asked for.

use std::fmt;

/// Whoever asked for the work, and never a bare string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WhoCommissioned(pub String);

/// A work item that says nobody asked for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWorkItemThatNamesNobody(pub String);

impl TryFrom<&str> for WhoCommissioned {
    type Error = AWorkItemThatNamesNobody;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.trim().is_empty() {
            return Err(AWorkItemThatNamesNobody(name.to_string()));
        }
        Ok(WhoCommissioned(name.to_string()))
    }
}

impl fmt::Display for WhoCommissioned {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Display for AWorkItemThatNamesNobody {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} names nobody who asked", self.0)
    }
}

impl std::error::Error for AWorkItemThatNamesNobody {}
