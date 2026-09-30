//! A run picking up the work a filing named.
//!
//! **The work stands at `IDLE` and a work item there has not begun**, however true everything in
//! it is. The table's own edge out of `IDLE` is `AWAITING_AGENT_PICKUP`, **and that edge is what
//! makes the record a piece of work rather than a note about one** — so this is not a convenience
//! and it is not a move composed here.
//!
//! **One table and not two.** The move is the domain's `transition`, and a use case that could
//! reach any other stage would be a second transition table — and this repository has one.

use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::{StateMachineError, transition};
use aisdlc_domain::work_item::WorkItem;

/// A run taking up the work it was given, along the one edge the table allows.
///
/// **The refusal is the domain's and not this file's**, because a move this file composed would be
/// a second table — and a second table is one nobody reads when the first one is the specification.
pub fn a_run_picks_up_the_work_it_was_given(
    the_work: &WorkItem,
) -> Result<WorkItem, StateMachineError> {
    transition(the_work.clone(), Stage::AwaitingAgentPickup, None)
}
