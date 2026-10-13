//! What a filing became, taken up to the first stage.
//!
//! **The run picks the work up with the agent its own discipline names and takes it into that
//! agent's hands**, because the walk asks agents and an agent that holds nothing answers
//! nothing. The first artifact is the discovery report, because every run of the pipeline
//! starts by reading the repository.

use aisdlc_domain::state_machine::StateMachineError;

use crate::a_run_picks_up_the_work::{
    a_run_picks_up_the_work_it_was_given, the_agent_takes_the_work_into_its_own_hands,
};
use crate::the_work_each_filing_becomes::WhatTheRunWillWorkOn;
use crate::where_a_run_stands::WhereARunStands;

/// What a filing became, taken up to the first stage.
pub fn the_work_a_filing_became_is_taken_up(
    what_the_run_will_work_on: &WhatTheRunWillWorkOn,
) -> Result<WhereARunStands, StateMachineError> {
    let picked_up = a_run_picks_up_the_work_it_was_given(&what_the_run_will_work_on.the_work_item)?;
    let in_its_hands = the_agent_takes_the_work_into_its_own_hands(&picked_up)?;
    Ok(WhereARunStands::at_the_first_stage(
        in_its_hands,
        "DiscoveryReport",
    ))
}
