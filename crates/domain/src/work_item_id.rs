//! The name a work item is tracked by, and the one shape of that name which is refused.
//!
//! A work item cannot be approved, persisted or pointed at by an approval without a
//! name, so a blank one is refused where it enters and there is no second door. Everything
//! else is a name this system accepts: a work item is tracked by whatever it is tracked
//! by, and it does not get a vote on whether the name is a sensible one.
//!
//! Nothing here may reach outside this crate. There is no parsing of a file, no reading
//! of a clock and no name of a model, and the crate holding this file has no dependencies
//! at all — which is the rule every other boundary in the repository rests on.

use std::convert::TryFrom;
use std::fmt;

/// A work item whose name is blank.
///
/// **A refusal and not an `Option`, and it carries the name that was looked for.** A
/// caller asking "is this a work item's name" has the same question whichever way the
/// answer came out, and an `Option` would make it choose which of two shapes to hold
/// before it knows anything. The name matters for the same reason a model is corrected
/// better when it is handed its own spelling back — and for the stronger reason that this
/// crate has two refusals, so a caller told "no" has to learn which of them it was.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWorkItemThatNamesNothing {
    what_was_looked_for: String,
}

impl AWorkItemThatNamesNothing {
    /// The refusal for a name that names no work item.
    pub fn for_a_name(name: &str) -> AWorkItemThatNamesNothing {
        AWorkItemThatNamesNothing {
            what_was_looked_for: name.to_string(),
        }
    }

    /// The name that was looked for, and named no work item.
    ///
    /// **Exactly as it was written, and that is the whole of what this promises.** A
    /// caller that sent three spaces gets three spaces back, because a name this system
    /// silently tidies is a name a lookup will miss — and a lookup that cannot miss is the
    /// reason the type exists in the first place.
    pub fn what_was_looked_for(&self) -> &str {
        &self.what_was_looked_for
    }
}

impl fmt::Display for AWorkItemThatNamesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} names no work item at all",
            self.what_was_looked_for
        )
    }
}

impl std::error::Error for AWorkItemThatNamesNothing {}

/// The name a work item is tracked by.
///
/// **A name and not an identifier, and the two types exist side by side because they
/// refuse different things.** An agent is refused a string that only means absent, and
/// this one is not: see `named` below, where the whole of that difference is written out.
/// What the two have in common is the shape, and it is the shape the glossary fixes: a
/// newtype over `String` with a constructor that refuses, a `TryFrom<&str>` and a
/// `Display`.
///
/// **The same six derives a stage has, less `Copy`, and the difference is the text.** A
/// stage is one of six values that own nothing, so copying one is free and means the same
/// stage. A name owns its spelling, so a copy of a name would be two handles onto one
/// string, and two handles are not two names. `Eq` and `Hash` are here because a name is
/// what a work item and a project directory are looked up by, and `Ord` because a listing
/// that reorders itself between two reads is a record nobody can diff. `Copy` is not
/// merely left off: it is impossible here, and pretending otherwise would need a
/// `#[allow]` that this repository refuses.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkItemId(String);

impl WorkItemId {
    /// The work item tracked by this name, or the refusal quoting the name back.
    ///
    /// **A blank is the whole of what is refused, and `"none"` is not one of the blanks.**
    /// A work item nobody has named yet and an agent that does not exist are different
    /// sentences, and this is the first of them. A tracker of this deployment hands out
    /// identifiers for the work it is asked to do, and one of the identifiers it hands out
    /// is the string `none`; refusing it here would refuse a delivery for the name its own
    /// tracker gave it, and a work item is tracked by whatever it is tracked by. The second
    /// sentence is the one the state machine asks about — it decides whether an agent is
    /// present by asking whether there is one — so the refusal of `none` belongs on
    /// `AgentIdentifier` and not here.
    ///
    /// The name is kept exactly as it was written. Nothing is trimmed and nothing is
    /// lowercased, so a name a person typed and a name a run reads back are the same name
    /// and a caller that sent `"  x  "` can look the work item up with `"  x  "`.
    pub fn named(name: &str) -> Result<WorkItemId, AWorkItemThatNamesNothing> {
        if name.trim().is_empty() {
            return Err(AWorkItemThatNamesNothing::for_a_name(name));
        }
        Ok(WorkItemId(name.to_string()))
    }
}

impl TryFrom<&str> for WorkItemId {
    type Error = AWorkItemThatNamesNothing;

    /// The work item tracked by this name, or the refusal quoting the name back.
    ///
    /// **`named` is the one implementation, and this is the door that calls it.** A second
    /// reading of what makes a name would be two answers to one question the day the two
    /// disagreed, and this crate already has to answer "is that a name" in three places.
    fn try_from(name: &str) -> Result<WorkItemId, AWorkItemThatNamesNothing> {
        WorkItemId::named(name)
    }
}

impl fmt::Display for WorkItemId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_written = &self.0;
        write!(formatter, "{as_written}")
    }
}
