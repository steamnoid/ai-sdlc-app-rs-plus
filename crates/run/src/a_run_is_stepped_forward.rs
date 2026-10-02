//! A run is stepped forward, one stage at a time, and never over one it has not passed.
//!
//! **The order is closed and it is not read out of a set.** A run that could reach the planner
//! without a discovery report is a run that delivers work nobody decided was worth doing, **and a
//! set has no next member** — so an order read off a declaration would have been invented rather
//! than chosen.
//!
//! **A refusal ends the run and is not a step.** Refusing is a decision, **and a run that keeps going
//! after one is a run that treats a decision as a delay.**

use std::fmt;

use aisdlc_domain::stage::Stage;

use crate::where_a_run_stands::WhereARunStands;
use crate::where_an_approved_artifact_leads::{WhereARunGoesNext, where_a_gate_outcome_leads};

/// The agent that answers at each destination, **and it is a table and not a second ordering.**
///
/// **The stage and the agent move together and are written beside each other on purpose.** A run
/// that named the stage and left the caller to guess the agent is a run with two orderings, **and
/// the second one would be the one the caller got right by accident.**
const THE_AGENT_AT_EACH_DESTINATION: [(WhereARunGoesNext, &str); 7] = [
    (WhereARunGoesNext::TheProductAnalysis, "aisdlc-po-analyze"),
    (WhereARunGoesNext::TheQualityAnalysis, "aisdlc-qa-analyze"),
    (
        WhereARunGoesNext::TheArchitectureAnalysis,
        "aisdlc-arch-analyze",
    ),
    (WhereARunGoesNext::TheSecurityAnalysis, "aisdlc-sec-analyze"),
    (WhereARunGoesNext::ThePlanning, "aisdlc-dev-plan"),
    (WhereARunGoesNext::TheImplementation, "aisdlc-dev-impl"),
    (WhereARunGoesNext::TheDelivery, "aisdlc-pr"),
];

/// What one step of a run looks like, **and it is a value so a caller can read it before obeying
/// it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatOneStepLeadsTo {
    where_the_run_goes: WhereARunGoesNext,
    who_answers_at_there: String,
}

impl WhatOneStepLeadsTo {
    /// Where the run goes, **asked by name because the field is a closed set and a caller matching
    /// on it would be matching on a shape.**
    pub fn where_the_run_goes(&self) -> WhereARunGoesNext {
        self.where_the_run_goes
    }

    /// Who answers there, **and it is read out of the table rather than composed here.**
    pub fn who_answers_the_next_stage(&self) -> String {
        self.who_answers_at_there.clone()
    }
}

/// Why a run was not stepped, **and each is a fact about a different thing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheRunWasNotStepped {
    /// The work is finished, **and there is nothing left to approve.**
    TheRunHasReachedTheEnd,
    /// The work stands at a stage no agent may hold, **and handing it to one is the forgery the
    /// approval boundary exists to prevent.**
    TheStageNamesNoAgent,
    /// The router holds no destination, **quoted whole because a name is what a reader needs.**
    NoDestinationForThisArtifact {
        /// What the router refused.
        why: String,
    },
}

impl fmt::Display for TheRunWasNotStepped {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            TheRunWasNotStepped::TheRunHasReachedTheEnd => String::from(
                "this run has reached the end, and a finished work item has nothing left to approve",
            ),
            TheRunWasNotStepped::TheStageNamesNoAgent => String::from(
                "this work stands at a stage no agent may hold, and a work item at a gate belongs to \
                 a person and not to an agent",
            ),
            TheRunWasNotStepped::NoDestinationForThisArtifact { why } => {
                format!("this router holds no destination for that artifact: {why}")
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for TheRunWasNotStepped {}

/// Where a run goes, given how a person ruled on what it produced.
///
/// **The destination is the router's and not this function's**, so there is one ordering rather than
/// two — and a refusal reaches `TheRunIsOver` inside the router, **which is the one place that
/// decision is made.**
///
/// **A run standing at a gate is refused before the router is asked, and that is a finding.** The
/// first version asked the router first, **and it handed a work item standing at
/// `AWAITING_HUMAN_APPROVAL` to the quality analyst** — a step that moves gate work into an agent's
/// hands is the forgery the whole approval boundary exists to prevent, **and the router was right
/// and the caller was the thing that was wrong.**
pub fn where_a_run_goes_from_here(
    the_run: &WhereARunStands,
    was_the_artifact_approved: bool,
) -> Result<WhereARunGoesNext, TheRunWasNotStepped> {
    let where_it_stands = the_run.the_stage_it_stands_at();
    if where_it_stands == Stage::Done {
        return Err(TheRunWasNotStepped::TheRunHasReachedTheEnd);
    }
    if !where_it_stands.an_agent_must_be_holding_it() {
        return Err(TheRunWasNotStepped::TheStageNamesNoAgent);
    }
    let which_artifact = the_run
        .which_artifact_it_is_on()
        .ok_or(TheRunWasNotStepped::TheStageNamesNoAgent)?;
    where_a_gate_outcome_leads(which_artifact, was_the_artifact_approved).map_err(
        |no_destination| TheRunWasNotStepped::NoDestinationForThisArtifact {
            why: no_destination.to_string(),
        },
    )
}

/// One step of the run, **and it is refused at both ends: a finished run and a stage no agent may
/// hold.**
pub fn what_a_step_leads(
    the_run: &WhereARunStands,
    was_the_artifact_approved: bool,
) -> Result<WhatOneStepLeadsTo, TheRunWasNotStepped> {
    let where_the_run_goes = where_a_run_goes_from_here(the_run, was_the_artifact_approved)?;
    let agent =
        the_agent_at(where_the_run_goes).ok_or(TheRunWasNotStepped::TheStageNamesNoAgent)?;
    Ok(WhatOneStepLeadsTo {
        where_the_run_goes,
        who_answers_at_there: String::from(agent),
    })
}

/// The agent at a destination, **and `None` for the one destination that names no agent** — a run
/// that is over is not held by anybody.
fn the_agent_at(where_the_run_goes: WhereARunGoesNext) -> Option<&'static str> {
    THE_AGENT_AT_EACH_DESTINATION
        .iter()
        .find(|(one_destination, _)| *one_destination == where_the_run_goes)
        .map(|(_the_destination, the_agent)| *the_agent)
}
