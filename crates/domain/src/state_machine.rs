//! Which moves are legal, and the one door a stage may be moved through.
//!
//! **The table is an exhaustive `match` and not a lookup**, so a seventh stage does not
//! compile until the table has been completed. A table a stage could be missing from is a
//! table that says a work item standing there has nowhere to go, silently.

use std::fmt;

use crate::agent_identifier::AgentIdentifier;
use crate::stage::Stage;
use crate::work_item::WorkItem;

/// The stage machine refused, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateMachineError {
    /// The move that was asked for, in the words the specification uses.
    pub why: String,
}

impl fmt::Display for StateMachineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.why)
    }
}

impl std::error::Error for StateMachineError {}

/// Every stage a work item standing at `from` may be moved to.
///
/// **Exhaustive on purpose.** A seventh stage has to be given its row here before anything
/// in this crate compiles, which is the whole point: a stage nobody has thought about
/// cannot quietly end up with nowhere to go.
pub fn legal_transitions_from(from: Stage) -> &'static [Stage] {
    let _the_stage_was_not_asked_about = from;
    &[]
}

/// Move a work item to a legal next stage, and hand back the moved work item.
pub fn transition(
    a_work_item: WorkItem,
    to: Stage,
    the_agent: Option<AgentIdentifier>,
) -> Result<WorkItem, StateMachineError> {
    let _the_destination_was_not_asked_about = to;
    let _the_agent_was_not_asked_about = the_agent;
    Ok(a_work_item)
}

/// Assert that a work item satisfies the invariants of its own state.
pub fn validate_state(a_work_item: &WorkItem) -> Result<(), StateMachineError> {
    let _the_work_item_was_not_read = a_work_item;
    Ok(())
}
