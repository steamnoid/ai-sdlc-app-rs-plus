//! A stage is worked and then a person rules on what it produced.
//!
//! **The walk moves work that nobody produced.** `a_run_is_walked_forward` takes a ruling per stage
//! as an argument, **so it can walk a run whose seven artifacts do not exist** — and this module is
//! the three steps the walk does not do: the agent holding the work is asked, the work is presented
//! for a ruling, and somebody answers.
//!
//! **Only whether the answer was yes is what the walk consumes.** A function returning the whole run
//! would be a second walk, **and a walk that asks a model is a walk whose loop cannot be tested
//! without one.**

use std::fmt;

use aisdlc_domain::approval::AnArtifactVersion;
use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;

use crate::a_model_is_asked_for_an_artifact::{
    AReasoningModel, AnUnanswerableRequest, the_agent_asks_for_its_artifact,
};
use crate::where_a_run_stands::{WhereARunStands, the_work_is_presented_for_a_ruling};
use crate::{
    APersonToDecide, the_gate_records_what_the_human_decided, the_question_put_to_somebody,
};

/// What a stage left behind, **and the approval is part of it rather than a second return value.**
///
/// **A refusal leaves no approval, and that is why this is an `Option` and not a flag.** There was
/// nothing to record, **and a type that carried a refusal with an empty approval beside it would let
/// a caller mistake one for the other** — which is precisely the moment somebody needs to tell an
/// approval from a refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheStageLeft {
    the_ruling_that_was_recorded: Option<aisdlc_domain::approval::Approval>,
    what_the_agent_said: String,
    which_version_was_produced: AnArtifactVersion,
}

impl WhatTheStageLeft {
    /// Whether a person approved, **and a refusal is `false` rather than an error** — refusing is a
    /// decision and not a failure, **and the walk needs the answer and not the reason.**
    pub fn was_the_artifact_approved(&self) -> bool {
        self.the_ruling_that_was_recorded.is_some()
    }

    /// What was recorded at the gate, **and nothing when a person refused.**
    pub fn what_was_recorded(&self) -> Option<&aisdlc_domain::approval::Approval> {
        self.the_ruling_that_was_recorded.as_ref()
    }

    /// Exactly what the agent said, **and never a summary of it.**
    ///
    /// **A summary is prose nobody approved.** The gate rules on a document, **and a document that
    /// is not the one the agent wrote is a document nobody has read** — so the words travel whole and
    /// whoever writes them to disk writes what the agent said.
    pub fn what_the_agent_said(&self) -> String {
        self.what_the_agent_said.clone()
    }

    /// Which version of the artifact this production is, **and it is what the gate was asked about.**
    pub fn which_version_was_produced(&self) -> AnArtifactVersion {
        self.which_version_was_produced
    }
}

/// Why a stage could not be worked, **and each is a fact about a different thing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AStageThatCouldNotBeWorked {
    /// The agent could not be asked, **carrying the artifact that would have been asked for.**
    ///
    /// **The artifact travels whole, because a caller that had to remember which one it was about
    /// would have to know the pipeline's order** — and a refusal that named no artifact sends a
    /// reader to the run's own state to find it.
    TheAgentCouldNotBeAsked {
        /// Which artifact nobody was asked for.
        which_artifact: WhichArtifact,
        /// What stopped the question.
        why_not: String,
    },
    /// The work was not standing where a gate may be held, **and that is the domain's own refusal.**
    NotStandingAtTheGate {
        /// What the gate said.
        why_not: String,
    },
}

impl fmt::Display for AStageThatCouldNotBeWorked {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            AStageThatCouldNotBeWorked::TheAgentCouldNotBeAsked {
                which_artifact,
                why_not,
            } => format!(
                "nobody was asked for {which_artifact}, so nothing exists to rule on: {why_not}"
            ),
            AStageThatCouldNotBeWorked::NotStandingAtTheGate { why_not } => {
                format!("this work is not standing at a gate: {why_not}")
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for AStageThatCouldNotBeWorked {}

/// Work one stage and put it to somebody, **and answer only whether it was approved.**
///
/// **The artifact comes from the run and is not an argument**, because a caller naming it could ask
/// for the artifact a stage is not producing — **and the run would then be ruled on something it
/// never wrote.**
///
/// **The order is ask, present, rule, and it is the only order that works.** Ruling before the
/// presentation would put an approval on a work item not at a gate, and the gate's own check
/// refuses that, **so a reordering is not a rearrangement but a way of producing an approval that
/// names a document nobody has seen.**
pub fn a_stage_is_worked_and_ruled_on(
    the_run: &WhereARunStands,
    a_model: &mut impl AReasoningModel,
    who_reasoned: WhichOrganisationReasons,
    whoever_answers_the_gate: &mut impl APersonToDecide,
) -> Result<WhatTheStageLeft, AStageThatCouldNotBeWorked> {
    let which_artifact = the_artifact_this_run_is_on(the_run)?;
    let what_the_agent_said =
        the_agent_asked_the_model(a_model, the_run, &which_artifact, who_reasoned)?;
    let which_version = the_version_this_production_is(the_run);
    let produced = the_run_stands_on_what_was_just_produced(the_run, which_version);
    let presented = the_work_is_presented_for_a_ruling(&produced)
        .map_err(|_refused| a_run_not_standing_where_a_gate_may_be_held())?;
    the_question_is_asked_and_answered(
        &presented,
        &which_artifact,
        which_version,
        what_the_agent_said,
        whoever_answers_the_gate,
    )
}

/// The artifact this run is on, **and a run on none is refused by name rather than guessed at** —
/// a stage asked for an artifact it was not on **is a stage asked to produce something the pipeline
/// has no row for.**
fn the_artifact_this_run_is_on(
    the_run: &WhereARunStands,
) -> Result<WhichArtifact, AStageThatCouldNotBeWorked> {
    let on_one = the_run.which_artifact_it_is_on();
    let on_one = on_one.ok_or(this_run_is_on_no_artifact())?;
    Ok(on_one.clone())
}

/// The run saying it is on what was just produced, **and a run on no artifact is refused here rather
/// than at the question** — the two are the same fact and the earlier one names the artifact.
fn the_run_stands_on_what_was_just_produced(
    the_run: &WhereARunStands,
    which_version: AnArtifactVersion,
) -> WhereARunStands {
    the_run
        .which_artifact_is_at(which_version)
        .unwrap_or_else(|_no_version| the_run.clone())
}

/// The version this production is, **and it is one after an artifact nobody has produced and one more
/// after the same one again.**
///
/// **A version counts within one artifact and not across the pipeline**, so a run whose `Requirements`
/// reached version three and then moved on still produces `SecurityReport` at version one — **a
/// requirements document at version eight would be a document nobody has seen seven of.** The run is
/// asked rather than a list kept here, **because the run is the thing that knows which artifact it
/// is on.**
fn the_version_this_production_is(the_run: &WhereARunStands) -> AnArtifactVersion {
    let the_last_one = the_run.which_version_of_its_artifact_exists();
    let the_number = the_last_one.map_or(1, |the_last_one| the_last_one.0 + 1);
    AnArtifactVersion::try_from(the_number).expect("a version at or above one")
}

/// The model is asked for the artifact, **and the answer is not kept here.**
///
/// **It is not kept because a caller that had to hold an artifact's words and its gate would be two
/// things to keep in step**, and the one who reads the artifact is the person at the gate. **What
/// this returns is only the proof that somebody was asked**, because a stage that asked nobody has
/// nothing to rule on.
fn the_agent_asked_the_model(
    a_model: &mut impl AReasoningModel,
    the_run: &WhereARunStands,
    which_artifact: &WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
) -> Result<String, AStageThatCouldNotBeWorked> {
    let the_answer = the_agent_asks_for_its_artifact(
        a_model,
        the_run.the_work_item_it_is(),
        which_artifact,
        who_reasoned,
    );
    let the_answer =
        the_answer.map_err(|no_answer| nobody_was_asked(which_artifact, &no_answer))?;
    Ok(the_answer.what_the_model_said())
}

/// The question is asked of somebody and the answer is recorded, **and a refusal records nothing.**
fn the_question_is_asked_and_answered(
    the_presented: &WhereARunStands,
    which_artifact: &WhichArtifact,
    which_version: AnArtifactVersion,
    what_the_agent_said: String,
    whoever_answers_the_gate: &mut impl APersonToDecide,
) -> Result<WhatTheStageLeft, AStageThatCouldNotBeWorked> {
    let the_question = the_question_put_to_somebody(
        the_presented.the_work_item_it_is(),
        which_artifact,
        which_version,
    );
    let the_question =
        the_question.map_err(|_refused| a_run_not_standing_where_a_gate_may_be_held())?;
    let what_was_recorded =
        the_gate_records_what_the_human_decided(whoever_answers_the_gate, &the_question);
    Ok(WhatTheStageLeft {
        the_ruling_that_was_recorded: what_was_recorded.ok(),
        what_the_agent_said,
        which_version_was_produced: which_version,
    })
}

/// The run is on no artifact, **and the refusal says so rather than naming a gate** — a run with
/// nothing to version has nothing to approve either, **and the two are the same fact here.**
fn this_run_is_on_no_artifact() -> AStageThatCouldNotBeWorked {
    AStageThatCouldNotBeWorked::TheAgentCouldNotBeAsked {
        which_artifact: WhichArtifact::try_from("DiscoveryReport").expect("a name is not blank"),
        why_not: String::from("this run is on no artifact, so there is nothing to rule on"),
    }
}

/// The question could not be put, **and the refusal names the gate rather than the stage.**
fn a_run_not_standing_where_a_gate_may_be_held() -> AStageThatCouldNotBeWorked {
    AStageThatCouldNotBeWorked::NotStandingAtTheGate {
        why_not: String::from("the work is not standing at a gate, so there is nothing to rule on"),
    }
}

/// The model could not be asked, **and the artifact it would have been asked for travels whole.**
fn nobody_was_asked(
    which_artifact: &WhichArtifact,
    the_refusal: &AnUnanswerableRequest,
) -> AStageThatCouldNotBeWorked {
    let why_not = the_refusal.why_it_could_not_be_asked().clone();
    AStageThatCouldNotBeWorked::TheAgentCouldNotBeAsked {
        which_artifact: which_artifact.clone(),
        why_not,
    }
}
