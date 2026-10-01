//! A run picking up the work a filing named.
//!
//! **The work stands at `IDLE` and a work item there has not begun**, however true everything in
//! it is. The table's own edge out of `IDLE` is `AWAITING_AGENT_PICKUP`, **and that edge is what
//! makes the record a piece of work rather than a note about one** — so this is not a convenience
//! and it is not a move composed here.
//!
//! **One table and not two.** The move is the domain's `transition`, and a use case that could
//! reach any other stage would be a second transition table — and this repository has one.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::{StateMachineError, transition};
use aisdlc_domain::work_item::WorkItem;

/// The agent that answers for a discipline, **and the name is the specification's.**
///
/// **Transcribed from §10 and not invented**, which is the only reason this can be written at all.
/// The repository discovery agent is deliberately absent: **no role answers for it**, **and naming one
/// of the five would be inventing a discipline the specification does not have.**
///
/// **Refused for now** — the shape is here and the mapping is not.
pub fn the_agent_that_answers_for(the_role: &Role) -> Result<AgentIdentifier, RoleThatHasNoAgent> {
    let _ = the_role;
    Err(RoleThatHasNoAgent(String::new()))
}

/// A discipline no agent answers for, **named so a reader is told which one.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleThatHasNoAgent(pub String);

impl std::fmt::Display for RoleThatHasNoAgent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} is a discipline the specification lists and no agent answers for",
            self.0
        )
    }
}

/// The agent that answers for one work, **read from the discipline the work itself carries.**
///
/// **One place that answers it, because five call sites want the same sentence** — and a caller that
/// wrote the name itself is a caller holding a second copy of the specification's table.
pub fn the_agent_answering_for(the_work: &WorkItem) -> Result<AgentIdentifier, RoleThatHasNoAgent> {
    the_agent_that_answers_for(&the_work.the_role_answering())
}

/// The work with the agent its own discipline names taking it on, **and nothing else.**
///
/// **One function and not four test files each composing a transition**, because a caller that wrote
/// the agent itself is a caller holding a second copy of the specification's table — **and this is
/// the exact shape of the defect this commit removed.**
pub fn the_agent_takes_the_work_into_its_own_hands(
    the_work: &WorkItem,
) -> Result<WorkItem, StateMachineError> {
    let who_is_taking_it = the_agent_answering_for(the_work)
        .map_err(|no_agent| StateMachineError { why: no_agent.0 })?;
    transition(
        the_work.clone(),
        Stage::InProgressByAgent,
        Some(who_is_taking_it),
    )
}

/// A run taking up the work it was given, along the one edge the table allows.
///
/// **The refusal is the domain's and not this file's**, because a move this file composed would be
/// a second table — and a second table is one nobody reads when the first one is the specification.
pub fn a_run_picks_up_the_work_it_was_given(
    the_work: &WorkItem,
) -> Result<WorkItem, StateMachineError> {
    transition(
        the_work.clone(),
        Stage::AwaitingAgentPickup,
        Some(the_agent_that_picks_up_work()),
    )
}

/// The agent that picks up work, **and the name is the specification's and not this file's.**
///
/// §10 lists the first agent as the Repository Discovery agent, and the mother project writes that
/// name as `aisdlc-discover`. **Transcribing it is not inventing it**, which is the whole reason
/// this could be written at all: a name a system chose for itself would be a name nothing could be
/// held to.
pub fn the_agent_that_picks_up_work() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-discover").expect("the specification names it")
}
