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

impl StateMachineError {
    /// A move the table has no row for.
    pub fn for_a_move_the_table_refuses(from: Stage, to: Stage) -> Self {
        StateMachineError {
            why: format!("{from} may not be moved to {to}"),
        }
    }

    /// An agent given to a stage that does not carry one.
    /// A move the destination refuses on the agent, **and which way it was wrong.**
    ///
    /// **One sentence for both directions, because a refusal that says the opposite of what
    /// happened sends a reader to fix the thing that was already right.** A stage that holds an
    /// agent and was given none has been given no agent where one is required; a stage that holds
    /// no agent and was given one has been handed an agent it cannot have.
    pub fn for_an_agent_the_destination_refuses(to: Stage, one_was_given: bool) -> Self {
        let why = match (to.an_agent_must_be_holding_it(), one_was_given) {
            (true, false) => format!(
                "{to} holds an agent, and this move was given none, so nothing is holding the work"
            ),
            (false, true) => format!(
                "{to} holds no agent, and this move was given one, so an agent is holding work \
                 that no agent may hold"
            ),
            _ => format!("{to} and the agent it was given do not belong together"),
        };
        StateMachineError { why }
    }
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
    match from {
        Stage::Idle => &[Stage::AwaitingAgentPickup],
        Stage::AwaitingAgentPickup => &[Stage::InProgressByAgent],
        Stage::InProgressByAgent => &[Stage::AwaitingHumanApproval, Stage::AwaitingAgentPickup],
        Stage::AwaitingHumanApproval => &[Stage::AwaitingAgentPickup, Stage::Done],
        Stage::Ready => &[Stage::AwaitingAgentPickup],
        Stage::Done => &[],
    }
}

/// The one place a work item's stage is written, and the only one.
fn moved_to(a_work_item: WorkItem, to: Stage, the_agent: Option<AgentIdentifier>) -> WorkItem {
    let mut moved = a_work_item;
    moved.stage = to;
    moved.agent = the_agent;
    moved
}

/// Move a work item to a legal next stage, and hand back the moved work item.
///
/// **The destination and the agent are two facts, checked against one rule.** The table
/// says where it may go; the biconditional says who may hold it there; and neither is
/// enough alone — which is why the caller states both and the machine refuses if they
/// disagree.
pub fn transition(
    a_work_item: WorkItem,
    to: Stage,
    the_agent: Option<AgentIdentifier>,
) -> Result<WorkItem, StateMachineError> {
    if !legal_transitions_from(a_work_item.stage).contains(&to) {
        return Err(StateMachineError::for_a_move_the_table_refuses(
            a_work_item.stage,
            to,
        ));
    }
    if the_agent.is_some() != to.an_agent_must_be_holding_it() {
        return Err(StateMachineError::for_an_agent_the_destination_refuses(
            to,
            the_agent.is_some(),
        ));
    }
    Ok(moved_to(a_work_item, to, the_agent))
}

/// Assert that a work item satisfies the invariants of its own state.
pub fn validate_state(a_work_item: &WorkItem) -> Result<(), StateMachineError> {
    let one_is_holding = a_work_item.agent.is_some();
    if one_is_holding == a_work_item.stage.an_agent_must_be_holding_it() {
        return Ok(());
    }
    let the_stage = a_work_item.stage;
    let the_agent_the_stage_claims = the_agent_the_stage_claims(the_stage);
    Err(StateMachineError {
        why: format!("{the_stage} says the agent is {the_agent_the_stage_claims}"),
    })
}

/// What the stage says about the agent, in the two words a reader needs.
fn the_agent_the_stage_claims(a_stage: Stage) -> &'static str {
    if a_stage.an_agent_must_be_holding_it() {
        "absent"
    } else {
        "present"
    }
}
