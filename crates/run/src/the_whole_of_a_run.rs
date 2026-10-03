//! The whole of a run, from its first agent to the one that delivers.
//!
//! **Every stage asked, every gate ruled on, and the rulings walked in.** A stage can be worked and
//! a run can be walked, **and nothing joined the two** — so a run could have every one of its seven
//! agents work in perfect isolation and never be asked to finish.
//!
//! **One stage at a time and one walk per stage, and that is what keeps a refusal a decision.** The
//! run asks its agent, asks its gate, and moves the work — **before** the next agent is asked for
//! anything, **because a run that collected all seven rulings before walking any of them would have
//! asked four agents about a work item a person had already refused.**

use std::fmt;

use aisdlc_domain::approval::ApprovalDecision;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;

use crate::APersonToDecide;
use crate::a_model_is_asked_for_an_artifact::AReasoningModel;
use crate::a_run_is_walked_forward::{AWalkThatWouldNotEnd, a_run_is_walked_forward};
use crate::a_stage_is_worked_and_ruled_on::WhatAnAgentSaid;
use crate::a_stage_is_worked_and_ruled_on::{AStageThatCouldNotBeWorked, WhatTheStageLeft};
use crate::the_path_inside_the_project::ThePathInsideTheProject;
use crate::the_project_on_disk::TheProjectOnDisk;
use crate::what_a_stage_kept_is_written::{WhatCouldNotBeWritten, what_a_stage_kept_is_written};
use crate::where_a_run_stands::WhereARunStands;
use crate::where_an_approved_artifact_leads::WhereARunGoesNext;
use crate::where_an_artifact_of_this_run_landed::WhereAnArtifactOfThisRunLanded;

/// Why a whole run could not be had, **and each is a fact about a different thing.**
///
/// **A refusal that came from somewhere else keeps the type that refused it**, so a reader told
/// "the walk refused" knows it was the routing and not a stage — **and those two send a reader to
/// two different files.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AWholeRunThatCouldNotBeHad {
    /// Nobody was asked for an artifact, **carrying the stage's own refusal whole.**
    ///
    /// **It is the run's own fact and not the stage's**, because the stage never began: there is a
    /// place in the pipeline and no agent to fill it. **The refusal is boxed and keeps the artifact**
    /// — a caller that had to remember which artifact it was about would have to know the
    /// pipeline's order, **and boxing it stops one large refusal from making every refusal large.**
    NobodyWasAskedFor {
        /// What stopped the question, **and which artifact it was about.**
        why_not: Box<AStageThatCouldNotBeWorked>,
    },
    /// The stage could not be worked for some other reason, **and the stage's own refusal travels
    /// whole** with the number of the stage that failed.
    AStageCouldNotBeWorked {
        /// Which stage it was, **counting from one, because a stage has no name of its own.**
        which_stage: usize,
        /// What the stage said, **in the stage's own words.**
        why_not: String,
    },
    /// The walk refused to move the work, **and the walk's own refusal travels whole.**
    TheWalkRefused {
        /// What the walk said.
        why_not: AWalkThatWouldNotEnd,
    },
    /// What an agent said could not be written, **and the writer's own refusal travels whole.**
    ///
    /// **It stops the run before the gate is asked**, because a gate asked about a document nobody
    /// could open **would leave an approval naming an artifact no reader can reach** — which is the
    /// failure the whole approval boundary exists to prevent.
    NothingWasWritten {
        /// What the writer said.
        why_not: Box<WhatCouldNotBeWritten>,
    },
}

impl fmt::Display for AWholeRunThatCouldNotBeHad {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            AWholeRunThatCouldNotBeHad::NobodyWasAskedFor { why_not } => {
                format!("nobody was asked: {why_not}")
            }
            AWholeRunThatCouldNotBeHad::AStageCouldNotBeWorked {
                which_stage,
                why_not,
            } => format!("stage {which_stage} could not be worked: {why_not}"),
            Self::TheWalkRefused { why_not } => {
                format!("the walk refused to move the work on: {why_not}")
            }
            Self::NothingWasWritten { why_not } => {
                format!("nothing was written, so there was nothing to rule on: {why_not}")
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for AWholeRunThatCouldNotBeHad {}

/// Ask every agent of a run in order and rule on what each produced, **until the run is over or the
/// delivery is reached.**
///
/// **The delivery is the end of this function and not an omission.** A delivery pushes a branch and
/// opens a pull request, **and neither of those is a stage a person rules on** — so a whole run
/// that also delivered would be asking somebody to approve a pull request that does not exist yet.
/// **The run ends standing at the delivery with `aisdlc-pr` holding the work**, which is the one
/// place it can honestly stop.
pub fn the_whole_of_a_run(
    the_run: &WhereARunStands,
    the_project: &mut impl TheProjectOnDisk,
    a_model: &mut impl AReasoningModel,
    who_reasoned: WhichOrganisationReasons,
    whoever_answers_the_gates: &mut impl APersonToDecide,
) -> Result<WhereARunStands, AWholeRunThatCouldNotBeHad> {
    let mut walking_from = the_run.clone();
    let mut where_it_last_stopped = None;
    let mut which_stage = 1;
    while !the_run_has_ended(&walking_from, where_it_last_stopped) {
        where_it_last_stopped = the_next_stage_is_worked_written_ruled_and_walked(
            &mut walking_from,
            the_project,
            a_model,
            who_reasoned,
            whoever_answers_the_gates,
            which_stage,
        )?;
        which_stage += 1;
    }
    Ok(walking_from)
}

/// The walked run, **told what this stage wrote** — and the two are one line because a run that
/// walked and a run that wrote are one run.
fn the_walk_is_taken_and_told_what_was_written(
    one_step: &crate::a_run_is_walked_forward::WhatAWalkReached,
    what_this_stage_wrote: WhereAnArtifactOfThisRunLanded,
    what_the_gate_ruled: ApprovalDecision,
) -> WhereARunStands {
    one_step
        .where_the_work_ended_up()
        .clone()
        .with_what_it_wrote(what_this_stage_wrote)
        .with_the_ruling(what_the_gate_ruled)
}

/// What this stage wrote, **named by the artifact and version the gate is about to be asked** — and
/// that is the run standing at the gate rather than the one before or after it, **because that is the
/// run whose artifact the person ruled on and the run that the walk has already handed on.**
fn what_this_stage_wrote_of(
    what_an_agent_said: &WhatAnAgentSaid,
    where_it_landed: ThePathInsideTheProject,
) -> WhereAnArtifactOfThisRunLanded {
    let the_run = what_an_agent_said.the_run_standing_at_what_it_produced();
    let which_artifact = the_run
        .which_artifact_it_is_on()
        .expect("a run standing at a gate is standing at an artifact")
        .clone();
    let the_version = the_run
        .which_version_of_its_artifact_exists()
        .expect("a run standing at a gate is standing at a version");
    WhereAnArtifactOfThisRunLanded::an_artifact_landed(which_artifact, the_version, where_it_landed)
}

/// Whether this run has anywhere left to go, **and it is asked of the run's stage and of the last
/// place it stopped rather than of a list kept here.**
///
/// **A run the delivery is holding has ended**, and the test is the place rather than the agent's
/// name — **because a name is a string and a place is the pipeline's own word for where it got to.**
fn the_run_has_ended(
    the_run: &WhereARunStands,
    where_it_last_stopped: Option<WhereARunGoesNext>,
) -> bool {
    if the_run.the_stage_it_stands_at() == Stage::Done {
        return true;
    }
    where_it_last_stopped == Some(WhereARunGoesNext::TheDelivery)
}

/// The next stage is worked, **what it said is written, and a person rules on it** — and that order
/// is the whole of this function: **the write comes before the ruling so the document the approval
/// names is a document somebody can open.**
fn the_next_stage_is_worked_and_asked_about(
    the_run: &WhereARunStands,
    the_project: &mut impl TheProjectOnDisk,
    a_model: &mut impl AReasoningModel,
    who_reasoned: WhichOrganisationReasons,
    whoever_answers_the_gates: &mut impl APersonToDecide,
    which_stage: usize,
) -> Result<(WhatTheStageLeft, WhereAnArtifactOfThisRunLanded), AWholeRunThatCouldNotBeHad> {
    let what_an_agent_said =
        crate::a_stage_is_worked_and_ruled_on::the_agent_is_asked(a_model, the_run, who_reasoned);
    let refused = what_an_agent_said
        .map_err(|why_not| the_run_reports_its_own_failure(which_stage, &why_not));
    let what_an_agent_said = refused?;
    let where_it_landed = what_an_agent_said_is_written(the_project, &what_an_agent_said)?;
    let what_the_stage_left = what_an_agent_said
        .is_ruled_on(whoever_answers_the_gates)
        .map_err(|why_not| the_run_reports_its_own_failure(which_stage, &why_not))?;
    let what_this_stage_wrote = what_this_stage_wrote_of(&what_an_agent_said, where_it_landed);
    Ok((what_the_stage_left, what_this_stage_wrote))
}

/// What an agent said is written, **and a refused write stops the run before anybody is asked.**
///
/// **The place it landed comes back whole**, because the run is about to be told about it and **a
/// write whose answer was thrown away left seven files on disk that nothing could name.**
fn what_an_agent_said_is_written(
    the_project: &mut impl TheProjectOnDisk,
    what_an_agent_said: &WhatAnAgentSaid,
) -> Result<ThePathInsideTheProject, AWholeRunThatCouldNotBeHad> {
    let the_run = what_an_agent_said.the_run_standing_at_what_it_produced();
    what_a_stage_kept_is_written(
        the_project,
        the_run,
        &what_an_agent_said.what_the_agent_said(),
    )
    .map_err(nothing_was_written)
}

/// Nothing was written, **and the writer's own refusal travels whole.**
fn nothing_was_written(why_not: WhatCouldNotBeWritten) -> AWholeRunThatCouldNotBeHad {
    AWholeRunThatCouldNotBeHad::NothingWasWritten {
        why_not: Box::new(why_not),
    }
}

/// One stage is worked, what it said is written, a person rules on it and the run walks on — **and
/// the four are one moment in a run, which is why this function is the loop's body rather than four
/// calls in it.**
fn the_next_stage_is_worked_written_ruled_and_walked(
    walking_from: &mut WhereARunStands,
    the_project: &mut impl TheProjectOnDisk,
    a_model: &mut impl AReasoningModel,
    who_reasoned: WhichOrganisationReasons,
    whoever_answers_the_gates: &mut impl APersonToDecide,
    which_stage: usize,
) -> Result<Option<WhereARunGoesNext>, AWholeRunThatCouldNotBeHad> {
    let (what_the_stage_left, where_it_landed) = the_next_stage_is_worked_and_asked_about(
        walking_from,
        the_project,
        a_model,
        who_reasoned,
        whoever_answers_the_gates,
        which_stage,
    )?;
    let one_step = the_work_is_walked_on_from(walking_from, &what_the_stage_left)?;
    *walking_from = the_walk_is_taken_and_told_what_was_written(
        &one_step,
        where_it_landed,
        what_the_stage_left.the_ruling().clone(),
    );
    Ok(one_step.every_place_it_stopped_at().last().copied())
}

/// The refusal a stage raised, **and the two kinds are the run's own and the stage's.**
///
/// **Neither arm writes a struct literal**, because a literal inside a match arm is the third level
/// of nesting and **a reader stops seeing the match** — so each arm names a refusal and each refusal
/// is built by a function of its own.
fn the_run_reports_its_own_failure(
    stage: usize,
    why_not: &AStageThatCouldNotBeWorked,
) -> AWholeRunThatCouldNotBeHad {
    match why_not {
        AStageThatCouldNotBeWorked::TheAgentCouldNotBeAsked { .. } => nobody_was_asked(why_not),
        AStageThatCouldNotBeWorked::NotStandingAtTheGate { .. } => a_stage_failed(stage, why_not),
    }
}

/// Nobody was asked, **and the stage's refusal is kept whole so the artifact it names travels with
/// it** — a caller that had to remember which artifact it was about would have to know the order.
fn nobody_was_asked(why_not: &AStageThatCouldNotBeWorked) -> AWholeRunThatCouldNotBeHad {
    let why_not = Box::new(why_not.clone());
    AWholeRunThatCouldNotBeHad::NobodyWasAskedFor { why_not }
}

/// A stage refused for some other reason, **and the number of the stage travels with it** — a stage
/// has no name of its own, and a refusal naming none sends a reader to count the gates themselves.
fn a_stage_failed(
    stage: usize,
    why_not: &AStageThatCouldNotBeWorked,
) -> AWholeRunThatCouldNotBeHad {
    let why_not = why_not.to_string();
    AWholeRunThatCouldNotBeHad::AStageCouldNotBeWorked {
        which_stage: stage,
        why_not,
    }
}

/// The work is walked on from the one ruling this stage produced, **and one walk per stage rather
/// than seven up front.**
///
/// **A walk of every ruling at once would need all seven before the first, which is a run that keeps
/// asking agents after a refusal** — and it is also the loop `a_run_is_walked_forward` already
/// holds, so walking one at a time is the same machinery with a different stopping rule.
fn the_work_is_walked_on_from(
    the_run: &WhereARunStands,
    what_the_stage_left: &WhatTheStageLeft,
) -> Result<crate::a_run_is_walked_forward::WhatAWalkReached, AWholeRunThatCouldNotBeHad> {
    let was_it_approved = what_the_stage_left.was_the_artifact_approved();
    a_run_is_walked_forward(the_run, &[was_it_approved])
        .map_err(|why_not| AWholeRunThatCouldNotBeHad::TheWalkRefused { why_not })
}
