//! What every agent must do again, and where the run starts again.
//!
//! **A run that went wrong gets another turn, and a model decides how many and where from.** Three
//! refusals mean *a stage was asked for something it could not do* rather than *the work is wrong* —
//! **and a run that stops at any of them delivers nothing.**
//!
//! **The model decides and the code routes.** The answer names one of two places and a literal here
//! sends the work back, **because a value read out of the model would be a model routing the
//! graph**, which is the one thing in this pipeline declared as code rather than reasoned about.

use std::fmt;

use aisdlc_domain::agent_identifier::AgentIdentifier;

/// Where a run may be put back to, **and it is two and not five on purpose.**
///
/// **The four analysts are deliberately absent.** Nothing observed so far has said the requirements,
/// the scenarios, the architecture or the risk were wrong, **and a route back past four gates on a
/// guess is not a repair.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheRunShouldStartAgain {
    /// The implementer writes the change again, **which is what a red suite after the change is.**
    TheChangeIsWrittenAgain,
    /// The planner writes the plan again, **which is what a vacuous test is** — a test that passed
    /// without the change is a plan naming nothing to observe.
    TheChangeIsPlannedAgain,
}

impl WhereTheRunShouldStartAgain {
    /// Every place, **and there are two.**
    pub fn every_place() -> Vec<Self> {
        vec![Self::TheChangeIsWrittenAgain, Self::TheChangeIsPlannedAgain]
    }

    /// The place as a model writes it, **and it is the whole of the wire.**
    pub fn as_the_model_writes_it(&self) -> &'static str {
        match self {
            Self::TheChangeIsWrittenAgain => "the change is written again",
            Self::TheChangeIsPlannedAgain => "the change is planned again",
        }
    }

    /// The place a model named, **and nothing for a name outside the set.**
    pub fn named(what_the_model_wrote: &str) -> Option<Self> {
        let the_answer = what_the_model_wrote.trim();
        Self::every_place()
            .into_iter()
            .find(|one_place| one_place.as_the_model_writes_it() == the_answer)
    }
}

/// What one agent must do again, **and why the last time was not enough.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheVerdict {
    /// Which agent, **and two verdicts for one agent is refused.**
    pub who_must_act: AgentIdentifier,
    /// What they must do, **in the words the model used.**
    pub what_they_must_do: String,
    /// Why the last time was not enough, **and it is the sentence that stops a loop.**
    pub why_the_last_time_was_not_enough: String,
}

/// What every agent in this run must do again, and where the run starts again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatToRepeat {
    /// One verdict per agent, **and an agent the model left out is a missing verdict rather than a
    /// stage that quietly does not run.**
    pub what_each_agent_must_do: Vec<TheVerdict>,
    /// Which of the two places, **and never one of the four that are not in the set.**
    pub where_the_run_starts_again: WhereTheRunShouldStartAgain,
    /// What has not changed between attempts, **and never a blank.**
    pub what_has_not_changed: String,
}

/// Why this answer cannot be carried out, **and each is a different fact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AStageNobodyMayRestartFrom {
    /// No verdict at all, **quoted whole because an empty list is the shape of a refusal.**
    AVerdictForNobody,
    /// Two verdicts for one agent, **and the agent is named so a reader can see the clash.**
    TwoVerdictsForOneAgent {
        /// Which agent was told two things.
        who_was_told_twice: AgentIdentifier,
    },
    /// A place outside the set, **and the set travels with the refusal.**
    NoSuchPlaceToStartAgain {
        /// What the model wrote, verbatim.
        what_the_model_wrote: String,
        /// Every place that is in the set, **so a correction can repeat them.**
        which_places_were_offered: Vec<String>,
    },
    /// A blank where what has not changed should be.
    NothingSaidHasNotChanged,
}

impl AStageNobodyMayRestartFrom {
    /// Two verdicts for one agent, **and it names the agent rather than an index into a list.**
    pub fn two_verdicts_for_one_agent(who_was_told_twice: &AgentIdentifier) -> Self {
        AStageNobodyMayRestartFrom::TwoVerdictsForOneAgent {
            who_was_told_twice: who_was_told_twice.clone(),
        }
    }

    /// A place outside the set, **and it carries the set.**
    pub fn no_such_place_to_start_from(what_the_model_wrote: &str) -> Self {
        AStageNobodyMayRestartFrom::NoSuchPlaceToStartAgain {
            what_the_model_wrote: what_the_model_wrote.to_string(),
            which_places_were_offered: the_places_as_they_are_offered(),
        }
    }
}

impl fmt::Display for AStageNobodyMayRestartFrom {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            AStageNobodyMayRestartFrom::AVerdictForNobody => String::from(
                "no agent was told what to do, and an agent the model left out is a missing verdict \
                 rather than a stage that quietly does not run",
            ),
            AStageNobodyMayRestartFrom::TwoVerdictsForOneAgent { who_was_told_twice } => format!(
                "{who_was_told_twice} was told two things, and the second would silently win while \
                 the first became a record of something that never happened"
            ),
            AStageNobodyMayRestartFrom::NoSuchPlaceToStartAgain {
                what_the_model_wrote,
                which_places_were_offered,
            } => format!(
                "{what_the_model_wrote} is not a place a run may start again, and the four analysts \
                 are deliberately absent — nothing observed has said any of them was wrong. Answer \
                 with exactly one of: {}",
                which_places_were_offered.join(", ")
            ),
            AStageNobodyMayRestartFrom::NothingSaidHasNotChanged => String::from(
                "what has not changed was left blank, and a supervisor that cannot say what has not \
                 changed will put the same agent back to work for ever",
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for AStageNobodyMayRestartFrom {}

/// Every place, **as the strings a model is offered — which is the whole of the wire.**
fn the_places_as_they_are_offered() -> Vec<String> {
    WhereTheRunShouldStartAgain::every_place()
        .iter()
        .map(|one_place| one_place.as_the_model_writes_it().to_string())
        .collect()
}

/// The answer a model gave, **read out of what it wrote, or the one reason it cannot be carried.**
pub fn what_to_repeat(
    what_each_agent_must_do: Vec<TheVerdict>,
    where_the_run_starts_again: &str,
    what_has_not_changed: &str,
) -> Result<WhatToRepeat, AStageNobodyMayRestartFrom> {
    let the_place = a_place_out_of(where_the_run_starts_again)?;
    each_agent_is_told_once(&what_each_agent_must_do)?;
    if what_has_not_changed.trim().is_empty() {
        return Err(AStageNobodyMayRestartFrom::NothingSaidHasNotChanged);
    }
    Ok(WhatToRepeat {
        what_each_agent_must_do,
        where_the_run_starts_again: the_place,
        what_has_not_changed: what_has_not_changed.to_string(),
    })
}

/// A refusal for a place nobody may start again, **and it is a function and not only the variant
/// so a caller naming one does not have to spell the offered set out by hand.**
pub fn a_stage_nobody_may_restart_from(what_the_model_wrote: &str) -> AStageNobodyMayRestartFrom {
    AStageNobodyMayRestartFrom::no_such_place_to_start_from(what_the_model_wrote)
}

/// The place a model named, **and the refusal carries the set because a model told its answer was
/// wrong and not told what the answers are will send the same sentence again.**
fn a_place_out_of(
    what_the_model_wrote: &str,
) -> Result<WhereTheRunShouldStartAgain, AStageNobodyMayRestartFrom> {
    match WhereTheRunShouldStartAgain::named(what_the_model_wrote) {
        Some(the_place) => Ok(the_place),
        None => Err(AStageNobodyMayRestartFrom::no_such_place_to_start_from(
            what_the_model_wrote,
        )),
    }
}

/// Whether every agent is told once, **and an empty list is the same shape as a duplicate.**
fn each_agent_is_told_once(
    what_each_agent_must_do: &[TheVerdict],
) -> Result<(), AStageNobodyMayRestartFrom> {
    if what_each_agent_must_do.is_empty() {
        return Err(AStageNobodyMayRestartFrom::AVerdictForNobody);
    }
    for one_verdict in what_each_agent_must_do {
        no_agent_is_told_twice(one_verdict, what_each_agent_must_do)?;
    }
    Ok(())
}

/// Whether this agent was told something twice, **and it is one agent per call so a reader of the
/// loop above does not have to hold a count and a list at the same time.**
fn no_agent_is_told_twice(
    one_verdict: &TheVerdict,
    every_verdict: &[TheVerdict],
) -> Result<(), AStageNobodyMayRestartFrom> {
    let how_many_times = how_many_times(one_verdict, every_verdict);
    if how_many_times > 1 {
        return Err(AStageNobodyMayRestartFrom::two_verdicts_for_one_agent(
            &one_verdict.who_must_act,
        ));
    }
    Ok(())
}

/// How many times this agent was told something, **and this is the whole of the duplicate rule.**
fn how_many_times(one_verdict: &TheVerdict, every_verdict: &[TheVerdict]) -> usize {
    every_verdict
        .iter()
        .filter(|every_one| every_one.who_must_act == one_verdict.who_must_act)
        .count()
}

/// The agent this place hands the work back to, **and it is a literal and never read out of the
/// model.**
///
/// **This is the whole of "the model decides, the code routes."** The answer names one of two
/// places, **and this table is what sends the work** — a value read out of the model would be a
/// model choosing its own destination, which is the one thing in this pipeline declared as code
/// rather than reasoned about.
pub fn the_agent_asked_to_do_it_again(
    from_which_place: WhereTheRunShouldStartAgain,
) -> AgentIdentifier {
    match from_which_place {
        WhereTheRunShouldStartAgain::TheChangeIsWrittenAgain => the_agent_named("aisdlc-dev-impl"),
        WhereTheRunShouldStartAgain::TheChangeIsPlannedAgain => the_agent_named("aisdlc-dev-plan"),
    }
}

/// The agent with this name, **and it is refused rather than a blank if the name is wrong** — so a
/// typo in this table is a test failure and not an agent nobody answers to.
fn the_agent_named(what_it_is_called: &str) -> AgentIdentifier {
    AgentIdentifier::try_from(what_it_is_called).expect("an agent name this table is written with")
}
