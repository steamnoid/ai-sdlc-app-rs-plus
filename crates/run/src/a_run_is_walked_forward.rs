//! A run is walked forward, and the walk stops when there is nowhere left to go.
//!
//! **A step describes a move; a walk makes it.** Cycle 113's stepper answers where a run goes, **and
//! nothing in this repository advanced one by asking it** — so a run could be stepped, described,
//! and never actually walked anywhere.
//!
//! **The walk decides nothing.** Every ruling is an argument, **and a walk that could approve a
//! stage of its own accord would be the run approving its own work**, which is the one thing the
//! approval boundary exists to prevent. One ruling is one step.
//!
//! **The number of stages is closed, so a walk longer than that is a loop and not a run.** The
//! repeat budget bounds the *other* loop; **this one needs no budget because the router has seven
//! destinations and nothing has an eighth.**

use std::fmt;

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::StateMachineError;

use crate::a_run_is_stepped_forward::{
    every_agent_the_pipeline_hands_work_to, the_agent_asked_to_do_it,
};
use crate::where_a_run_stands::{
    WhereARunStands, the_run_ends_on_a_refusal, the_work_is_handed_to_an_agent,
    the_work_is_handed_to_the_next_agent, the_work_is_presented_for_a_ruling,
};
use crate::where_an_approved_artifact_leads::{
    AnArtifactThisRouterHoldsNoDestinationFor, WhereARunGoesNext, where_a_gate_outcome_leads,
    which_a_stage_produces,
};

/// Where a run got to, **and every stage it passed through on the way.**
///
/// **The places and the run are both kept, because the two answer different questions.** A reader
/// asking *what happened* wants the places; **a reader asking *where is the work* wants the run**,
/// and a walk that reported only the first has printed a route without moving anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAWalkReached {
    every_place_it_stopped_at: Vec<WhereARunGoesNext>,
    where_the_work_ended_up: WhereARunStands,
}

impl WhatAWalkReached {
    /// How many stages the walk passed through, **and it is zero after a refusal** — a refusal is a
    /// decision and not a stage.
    pub fn how_far_it_got(&self) -> usize {
        self.every_place_it_stopped_at.len()
    }

    /// Every stage this walk passed through, **in the order it passed through them.**
    pub fn every_place_it_stopped_at(&self) -> Vec<WhereARunGoesNext> {
        self.every_place_it_stopped_at.clone()
    }

    /// The run as it stands at the end of the walk, **and it is the work and not the route.**
    pub fn where_the_work_ended_up(&self) -> &WhereARunStands {
        &self.where_the_work_ended_up
    }
}

/// Why a run was not walked, **and each is a fact about a different thing.**
///
/// **The refusals that come from elsewhere keep the refusal they came from.** A `why: String` would
/// read the same whether the table, the biconditional or the invariant had refused, **and those
/// three send a reader to three different files.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AWalkThatWouldNotEnd {
    /// More rulings than the pipeline has stages, **so this is a loop and not a run.**
    MoreRulingsThanStages {
        /// How many rulings the caller offered.
        how_many_were_given: usize,
        /// How many stages there are.
        how_many_there_are: usize,
    },
    /// The work is standing somewhere no stage may begin, **so a walk of it would be guessing.**
    ///
    /// **A pickup that nobody holds is here too, and that is deliberate.** The invariant refuses to
    /// build such a work item, **so a run that arrives at one has arrived from outside the domain
    /// and the walk has nothing to pick it up with** — a second refusal naming a second cause would
    /// be a reader being sent to a file that has no answer in it.
    NowhereToStartFrom {
        /// Which stage it is standing at.
        where_it_stands: Stage,
    },
    /// The run is on no artifact, **and there is no gate to rule on.**
    TheRunIsOnNoArtifact,
    /// The router holds no destination, **and its own refusal is kept rather than restated.**
    NoDestinationForThisArtifact {
        /// What the router said.
        why: AnArtifactThisRouterHoldsNoDestinationFor,
    },
    /// A stage named no agent to hand the work to, **and a stage that produces no artifact goes with
    /// it** — both are the walk reaching the end of the table rather than a fault in the run.
    AStageAtTheEndOfTheTable {
        /// Which stage it was.
        which_stage: WhereARunGoesNext,
    },
    /// The walk's own move was refused by the domain, **and the domain's own type is kept.**
    TheDomainRefusedTheMove {
        /// What the domain said.
        why: StateMachineError,
    },
}

impl AWalkThatWouldNotEnd {
    /// What this refusal says, **and a refusal the walk did not raise says it in its own voice.**
    ///
    /// **The two that came from elsewhere are not given a sentence of the walk's own**, because
    /// prefixing them would make two different faults read as one — **and the router's and the
    /// domain's send a reader to two different files.**
    fn what_it_says(&self) -> String {
        match self {
            AWalkThatWouldNotEnd::MoreRulingsThanStages {
                how_many_were_given,
                how_many_there_are,
            } => {
                format!("{how_many_were_given} rulings and {how_many_there_are} stages: a loop")
            }
            AWalkThatWouldNotEnd::NowhereToStartFrom { where_it_stands } => {
                format!("at {where_it_stands}, and nobody holds the work")
            }
            AWalkThatWouldNotEnd::TheRunIsOnNoArtifact => {
                String::from("on no artifact, so no gate")
            }
            AWalkThatWouldNotEnd::AStageAtTheEndOfTheTable { which_stage } => {
                format!("{which_stage:?} names no agent or no artifact")
            }
            AWalkThatWouldNotEnd::NoDestinationForThisArtifact { why } => why.to_string(),
            AWalkThatWouldNotEnd::TheDomainRefusedTheMove { why } => why.to_string(),
        }
    }
}

impl fmt::Display for AWalkThatWouldNotEnd {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.what_it_says())
    }
}

impl std::error::Error for AWalkThatWouldNotEnd {}

/// Every stage a person rules on, **and it is the stepper's table read as a list.**
///
/// **The ceiling and not an order.** A walk is driven by the router and needs no sequence of its
/// own; **it needs to know when to stop, and the answer is how many stages there are** — so it
/// reads the count from the one table that holds them.
pub fn every_stage_somebody_ruled_on() -> Vec<WhereARunGoesNext> {
    every_agent_the_pipeline_hands_work_to()
        .iter()
        .map(|(where_the_run_goes, _the_agent)| *where_the_run_goes)
        .collect()
}

/// Walk a run as far as the rulings take it, **and refuse to walk further than the pipeline is
/// long.**
///
/// **One ruling is one step, and the work is actually moved.** The stage and the artifact move
/// together at every step, **because a run left standing on `DiscoveryReport` after two stages
/// routes back to the product analysis on the third** — which is the router defect that cost the
/// sibling a run while its suite stayed green.
pub fn a_run_is_walked_forward(
    the_run: &WhereARunStands,
    every_ruling: &[bool],
) -> Result<WhatAWalkReached, AWalkThatWouldNotEnd> {
    let how_many_there_are = every_agent_the_pipeline_hands_work_to().len();
    if every_ruling.len() > how_many_there_are {
        return Err(a_loop_rather_than_a_run(
            every_ruling.len(),
            how_many_there_are,
        ));
    }
    let mut walking = WalkingTheRun::starting_at(the_run)?;
    walking.walk_through(every_ruling)?;
    Ok(WhatAWalkReached {
        every_place_it_stopped_at: walking.every_place_it_passed_through,
        where_the_work_ended_up: walking.where_it_is_now,
    })
}

/// A walk in progress, **and the run moves through it rather than being rebuilt per step.**
struct WalkingTheRun {
    where_it_is_now: WhereARunStands,
    every_place_it_passed_through: Vec<WhereARunGoesNext>,
}

impl WalkingTheRun {
    /// A walk that has picked the work up and is about to work on it, **and refuses a run standing
    /// somewhere a stage cannot begin.**
    ///
    /// **The pickup happens once and is the table's own first edge.** A run handed to an agent and
    /// left standing at `AWAITING_AGENT_PICKUP` is a state the invariant allows but the table does
    /// not move on from, **so the walk makes the move the table already describes rather than
    /// inventing a second way into `IN_PROGRESS_BY_AGENT`.**
    fn starting_at(the_run: &WhereARunStands) -> Result<Self, AWalkThatWouldNotEnd> {
        let where_it_stands = the_run.the_stage_it_stands_at();
        if where_it_stands == Stage::InProgressByAgent {
            return Ok(Self::at(the_run));
        }
        if where_it_stands != Stage::AwaitingAgentPickup {
            return Err(AWalkThatWouldNotEnd::NowhereToStartFrom { where_it_stands });
        }
        let the_agent = the_run
            .who_holds_it()
            .ok_or(AWalkThatWouldNotEnd::NowhereToStartFrom { where_it_stands })?;
        let picked_up = the_work_is_handed_to_an_agent(the_run, the_agent);
        let where_it_is_now = picked_up.map_err(the_domain_refused_the_move)?;
        Ok(Self::at_the_run_standing_at(where_it_is_now))
    }

    /// A walk that has passed through nowhere, **and the route is the walk's to keep.**
    fn at(the_run: &WhereARunStands) -> Self {
        Self::at_the_run_standing_at(the_run.clone())
    }

    /// A walk at a run, **and the route starts empty** — a walk inherits no stages from a run it has
    /// just been handed, **because a route recorded by something else is not a record of this walk.**
    fn at_the_run_standing_at(the_run: WhereARunStands) -> Self {
        Self {
            where_it_is_now: the_run,
            every_place_it_passed_through: Vec::new(),
        }
    }

    /// Walk one stage per ruling, **and stop the moment the run is over.**
    ///
    /// **The stage is recorded inside `walk_one_stage` and not by the loop**, so the loop has no
    /// branch of its own — **a `for` that both walks and decides where to stop is two things in
    /// one reader's eye**, and the nesting rule refuses to let the two be written that way.
    fn walk_through(&mut self, every_ruling: &[bool]) -> Result<(), AWalkThatWouldNotEnd> {
        for was_this_one_approved in every_ruling {
            self.walk_one_stage(*was_this_one_approved)?;
        }
        Ok(())
    }

    /// Work one stage: the agent works, the work is presented, and the ruling is applied.
    ///
    /// **A refusal records no place and stops the walk, which is why `how_far_it_got` is zero after
    /// one.** The run being over is the loop's signal to stop, **and it is asked of the stage rather
    /// than remembered from the ruling** — so a run that ended any other way stops the walk too.
    fn walk_one_stage(&mut self, was_it_approved: bool) -> Result<(), AWalkThatWouldNotEnd> {
        let presented = the_work_is_presented_for_a_ruling(&self.where_it_is_now)
            .map_err(the_domain_refused_the_move)?;
        if !was_it_approved {
            let ended = the_run_ends_on_a_refusal(&presented);
            self.where_it_is_now = ended.map_err(the_domain_refused_the_move)?;
            return Ok(());
        }
        let where_the_run_goes = where_the_presented_artifact_leads(&presented)?;
        self.hand_the_work_to_the_stage_there(where_the_run_goes)?;
        self.every_place_it_passed_through.push(where_the_run_goes);
        Ok(())
    }

    /// Hand the work to the agent at that stage, **and stop if that stage is at the end of the
    /// table.**
    ///
    /// **The delivery produces a pull request and not an artifact, so the walk ends there.** A walk
    /// that invented an artifact for the last stage would route the run onwards into nothing, **and
    /// the run would be finished with nowhere to go and no reason.**
    fn hand_the_work_to_the_stage_there(
        &mut self,
        where_the_run_goes: WhereARunGoesNext,
    ) -> Result<(), AWalkThatWouldNotEnd> {
        let the_agent = the_agent_at(where_the_run_goes)?;
        let which_artifact = which_a_stage_produces(where_the_run_goes)
            .ok_or(a_stage_at_the_end(where_the_run_goes))?;
        let at_a_pickup = the_work_is_handed_to_the_next_agent(
            &self.where_it_is_now,
            the_agent.clone(),
            which_artifact,
        )
        .map_err(the_domain_refused_the_move)?;
        let in_progress = the_work_is_handed_to_an_agent(&at_a_pickup, the_agent);
        self.where_it_is_now = in_progress.map_err(the_domain_refused_the_move)?;
        Ok(())
    }
}

/// Where the artifact now on the table leads, **and a run on no artifact has no gate to rule on.**
fn where_the_presented_artifact_leads(
    the_run: &WhereARunStands,
) -> Result<WhereARunGoesNext, AWalkThatWouldNotEnd> {
    let which_artifact = the_run
        .which_artifact_it_is_on()
        .ok_or(AWalkThatWouldNotEnd::TheRunIsOnNoArtifact)?;
    where_a_gate_outcome_leads(which_artifact, true).map_err(the_router_holds_no_destination)
}

/// The agent at a stage, **and `None` from the table is refused by name rather than defaulted into a
/// blank identifier** — an empty agent name would fail further down, in the domain, **and the
/// refusal there would say a name was given that nobody here ever wrote.**
fn the_agent_at(
    where_the_run_goes: WhereARunGoesNext,
) -> Result<AgentIdentifier, AWalkThatWouldNotEnd> {
    let the_name = the_agent_asked_to_do_it(where_the_run_goes)
        .ok_or(a_stage_at_the_end(where_the_run_goes))?;
    AgentIdentifier::try_from(the_name)
        .map_err(|_not_a_name| a_stage_at_the_end(where_the_run_goes))
}

/// More rulings than stages, **and the refusal is built here rather than written inside the `if`
/// that returns it** — a struct literal inside a branch is the third level of nesting, **and the
/// rule exists because a reader stops seeing the branch.**
fn a_loop_rather_than_a_run(
    how_many_were_given: usize,
    how_many_there_are: usize,
) -> AWalkThatWouldNotEnd {
    AWalkThatWouldNotEnd::MoreRulingsThanStages {
        how_many_were_given,
        how_many_there_are,
    }
}

/// A stage the walk reached and there was nothing beyond, **and both no agent and no artifact are
/// that stage** — the delivery is the one that produces a pull request rather than an artifact.
fn a_stage_at_the_end(which_stage: WhereARunGoesNext) -> AWalkThatWouldNotEnd {
    AWalkThatWouldNotEnd::AStageAtTheEndOfTheTable { which_stage }
}

/// The router's refusal, **kept as the router wrote it.**
fn the_router_holds_no_destination(
    why: AnArtifactThisRouterHoldsNoDestinationFor,
) -> AWalkThatWouldNotEnd {
    AWalkThatWouldNotEnd::NoDestinationForThisArtifact { why }
}

/// A refusal kept whole, **and the walk adds no text of its own to what the domain said.**
fn the_domain_refused_the_move(the_refusal: StateMachineError) -> AWalkThatWouldNotEnd {
    AWalkThatWouldNotEnd::TheDomainRefusedTheMove { why: the_refusal }
}
