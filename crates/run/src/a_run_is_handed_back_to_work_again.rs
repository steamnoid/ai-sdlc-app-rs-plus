//! A run is handed back to work again, because a model said so and the ceiling allowed it.
//!
//! **Everything this needs already existed and nothing composed it.** `WhatToRepeat` carries a
//! verdict for every agent and names one of two places; `the_agent_asked_to_do_it_again` is the
//! literal that sends the work; `is_this_repeat_permitted` is the ceiling. **What was missing was
//! the step that asks, checks, and moves — so a run that went wrong had nothing to turn it.**
//!
//! **The model decides and the code routes.** The answer names one of two places, **and the table
//! that sends the work is a literal**, because a value read out of the model would be a model
//! choosing its own destination — the one thing in this pipeline declared as code rather than
//! reasoned about.
//!
//! **The budget is checked against the setting and not against the model**, and its refusal keeps
//! its own name.

use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::StateMachineError;

use crate::a_run_is_put_back_to_work_again::{
    HowManyTimesARunMayBePutBack, TheRepairCouldNotHappen, is_this_repeat_permitted,
};
use crate::what_to_repeat::{
    WhatToRepeat, the_agent_asked_to_do_it_again, what_the_place_produces,
};
use crate::where_a_run_stands::the_work_is_handed_to_the_next_agent;
use crate::where_a_run_stands::{WhereARunStands, the_work_is_handed_to_an_agent};

/// Hand a run back to the agent the supervisor named, **if the ceiling still allows another turn.**
///
/// **A run standing anywhere but where a failure could have happened is refused before the ceiling
/// is even asked**, and that is a finding rather than a precaution: **the transition table has one
/// edge out of `AWAITING_HUMAN_APPROVAL` into `AWAITING_AGENT_PICKUP` and it carries two
/// meanings** — a gate was approved and the run goes on, or a repair was claimed and the work is
/// taken off a gate. Only the first is legitimate, **and the table cannot tell them apart because
/// it was not written to: the two meanings are the same move.** So the stage is asked first, and
/// `Stage::a_failure_could_have_happened_here` is where that answer lives.
///
/// **The ceiling is checked before the work is moved and not after**, so a run past its budget is
/// left exactly where the failure left it — **and a reader finding the work item after a dead run
/// finds the failure and not a half-finished repair.** The order is not an implementation detail:
/// the other order is a run that has been handed to an agent and then refused, which is a state the
/// register cannot describe.
///
/// **The supervisor's answer arrives already validated**, because `what_to_repeat` is the door and it
/// refuses a blank reason, an agent named twice and a place outside the closed set. **A node that
/// re-checked them would be a second place to forget**, and the reason this one does not is that the
/// type carries the answer rather than the words it came from.
pub fn a_run_is_handed_back_to_work_again(
    the_run: &WhereARunStands,
    which_repetition: u32,
    the_budget: &HowManyTimesARunMayBePutBack,
    what_the_supervisor_said: &WhatToRepeat,
) -> Result<WhereARunStands, TheRepairCouldNotHappen> {
    let where_the_work_stands = the_run.the_stage_it_stands_at();
    if !where_the_work_stands.a_failure_could_have_happened_here() {
        return Err(nowhere_a_failure_could_have_happened(where_the_work_stands));
    }
    is_this_repeat_permitted(the_budget, which_repetition)
        .map_err(TheRepairCouldNotHappen::from)?;
    let the_agent =
        the_agent_asked_to_do_it_again(what_the_supervisor_said.where_the_run_starts_again);
    let which_artifact =
        what_the_place_produces(what_the_supervisor_said.where_the_run_starts_again);
    let at_a_pickup =
        the_work_is_handed_to_the_next_agent(the_run, the_agent.clone(), which_artifact)
            .map_err(the_run_is_not_standing_where_a_failure_happened)?;
    the_work_is_handed_to_an_agent(&at_a_pickup, the_agent)
        .map_err(the_run_is_not_standing_where_a_failure_happened)
}

/// A stage where nothing can fail, **and the sentence says the stage and not the table.**
fn nowhere_a_failure_could_have_happened(where_the_work_stands: Stage) -> TheRepairCouldNotHappen {
    let where_the_work_stands = where_the_work_stands.to_string();
    TheRepairCouldNotHappen::TheRunIsNotStandingWhereAFailureHappened {
        where_the_work_stands,
    }
}

/// The table's refusal, **kept as the table wrote it and named as the table's.**
fn the_run_is_not_standing_where_a_failure_happened(
    the_refusal: StateMachineError,
) -> TheRepairCouldNotHappen {
    TheRepairCouldNotHappen::TheDomainRefusedTheMove { why: the_refusal }
}
