//! An agent presenting what it produced, and the question that follows.
//!
//! **Presenting and asking are one act.** The two functions this replaces took the artifact twice —
//! once to be thrown away and once to be asked about — **so the thing approved and the thing ruled
//! on were two arguments somebody had to keep equal by hand, and nothing checked that they ever
//! were.**
//!
//! **A gate that can be asked about something other than what was put in front of it is a gate
//! whose approval is about an artifact nobody saw** — which is the one thing an approval record
//! exists to prevent. So there is nothing to keep equal: there is one value.

use aisdlc_domain::state_machine::StateMachineError;
use aisdlc_domain::work_item::WorkItem;

use crate::ApprovalQuestion;
use crate::a_run_picks_up_the_work::the_agent_answering_for;
use crate::what_an_agent_has_produced::WhatAnAgentHasProduced;

/// What an agent produced, put in front of the gate, and the question that asks about it.
///
/// **The work must already have been picked up**, because the table has no edge out of `IDLE`
/// straight to the gate — **and a work that has not begun cannot have produced anything**, however
/// true everything in it is. Both moves are the domain's and not this file's.
pub fn an_agent_presents_what_it_produced_and_the_question_is_put(
    the_work: &WorkItem,
    what_it_produced: &WhatAnAgentHasProduced,
) -> Result<ApprovalQuestion, StateMachineError> {
    let standing_at_the_gate = crate::the_agent_presents_what_it_produced(
        the_work.clone(),
        the_agent_answering_for(the_work)
            .map_err(|no_agent| StateMachineError { why: no_agent.0 })?,
        what_it_produced.which_artifact(),
    )?;
    crate::the_question_put_to_somebody(
        &standing_at_the_gate,
        what_it_produced.which_artifact(),
        *what_it_produced.the_version(),
    )
    .map_err(|not_standing| StateMachineError {
        why: format!("{the_work:?} cannot be asked about: {not_standing:?}"),
    })
}
