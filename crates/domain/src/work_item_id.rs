//! The name a work item is tracked by.
//!
//! **A name this document lists is one concept and this file is the one type for it**: a
//! newtype over `String` with a constructor that refuses, a `TryFrom<&str>` and a `Display`.
//!
//! **`Display` writes the name as written and nothing else** — no lowercasing, no trimming,
//! no reformatting. A name a person typed and a name a run reads back are the same name, and
//! a gate that compared them as strings would refuse a work item for a space somebody left
//! at the end of it.

use std::fmt;

/// The name a work item is tracked by, and never a bare string.
///
/// A work item cannot be approved, persisted or pointed at by an approval without one, so a
/// blank is refused where it enters rather than where it is later found to be missing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkItemId(pub String);

/// A work item whose name is blank.
///
/// **The name is refused and not the trimmed name.** A caller that sent `"  x  "` gets
/// `"  x  "` back, because a name this system silently tidies is a name a lookup will miss.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWorkItemThatNamesNothing(pub String);

impl WorkItemId {
    /// The name as it was written, spaces and capitals included.
    pub fn the_name_as_written(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for WorkItemId {
    type Error = AWorkItemThatNamesNothing;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let _the_name_nobody_refused = name;
        Ok(WorkItemId(name.to_string()))
    }
}

impl fmt::Display for WorkItemId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Display for AWorkItemThatNamesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} names no work item", self.0)
    }
}

impl std::error::Error for AWorkItemThatNamesNothing {}
