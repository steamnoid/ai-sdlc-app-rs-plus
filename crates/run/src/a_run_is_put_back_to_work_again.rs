//! A run is put back to work again, and the two ways that stops are not the same event.
//!
//! **A model has decided, and this module is where the code obeys.** `WhatToRepeat` says which agent
//! and which of two places; `the_agent_asked_to_do_it_again` is the literal that sends the work —
//! **and this is where the ceiling and the judgement meet.**
//!
//! **Two ways the loop ends and they are not the same event.** A model that judges the failure will
//! not change has *decided*, and a run that asked for more turns than the ceiling permits has *run
//! out of them*. **Conflating them sends whoever reads the trace after the wrong one.**

use std::fmt;

use crate::what_to_repeat::WhatToRepeat;

/// What a deployment gets when it names no budget, **and it is a number and not *unlimited*.**
///
/// **A ceiling nobody noticed and an answer nobody asked for are the same failure wearing different
/// names.**
const THE_REPEAT_BUDGET_A_DEPLOYMENT_GETS: u32 = 3;

/// How many times a run may be put back, **and it counts repetitions and not attempts.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HowManyTimesARunMayBePutBack(u32);

impl HowManyTimesARunMayBePutBack {
    /// The number, **and it is asked for by name so a caller is not reaching into a field.**
    pub fn may_be_put_back_this_many_times(&self) -> u32 {
        self.0
    }
}

/// A setting that is not a number, **quoted whole because a person has to see what was read.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABudgetThatIsNotANumber(pub String);

impl fmt::Display for ABudgetThatIsNotANumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} is not a number of repetitions, and a setting nobody can read is a setting nobody \
             is watching — the alternative is a run that never stops and a deployment that believes \
             it set something",
            self.0
        )
    }
}

impl std::error::Error for ABudgetThatIsNotANumber {}

/// The budget a deployment named, **and a number when it named nothing.**
pub fn the_repeat_budget(
    what_a_deployment_named: Option<&str>,
) -> Result<HowManyTimesARunMayBePutBack, ABudgetThatIsNotANumber> {
    let Some(what_was_named) = what_a_deployment_named else {
        return Ok(HowManyTimesARunMayBePutBack(
            THE_REPEAT_BUDGET_A_DEPLOYMENT_GETS,
        ));
    };
    match what_was_named.trim().parse::<u32>() {
        Ok(the_number) => Ok(HowManyTimesARunMayBePutBack(the_number)),
        Err(_) => Err(ABudgetThatIsNotANumber(what_was_named.to_string())),
    }
}

/// A repetition the budget does not permit, **and both numbers are carried because *ran out* is
/// only useful to a reader who can see what was asked for.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ARepeatThatWasNotPermitted {
    /// The budget was spent.
    TheRepeatBudgetRanOut {
        /// Which repetition this was, **counting from one.**
        how_many_were_asked_for: u32,
        /// How many the budget permitted.
        how_many_were_permitted: u32,
    },
}

impl ARepeatThatWasNotPermitted {
    /// The budget was spent, **and it takes both numbers rather than computing one of them here** —
    /// the permitted count is the budget's, and a function that recomputed it could disagree.
    pub fn the_repeat_budget_ran_out(
        how_many_were_asked_for: u32,
        how_many_permitted: u32,
    ) -> Self {
        ARepeatThatWasNotPermitted::TheRepeatBudgetRanOut {
            how_many_were_asked_for,
            how_many_were_permitted: how_many_permitted,
        }
    }
}

impl fmt::Display for ARepeatThatWasNotPermitted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            ARepeatThatWasNotPermitted::TheRepeatBudgetRanOut {
                how_many_were_asked_for,
                how_many_were_permitted,
            } => format!(
                "repetition {how_many_were_asked_for} was asked for and the budget permitted \
                 {how_many_were_permitted}, so the ceiling stopped this and not the model — and \
                 **those are different facts and a trace that says *repair failed* sends its reader \
                 after the wrong one.**"
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for ARepeatThatWasNotPermitted {}

/// Whether this repetition is inside the budget, **and the boundary is where the arithmetic is.**
///
/// **Permitted while the repetition is at or below the budget**, and **a check refused at `>=`
/// would buy one fewer repair than a deployment asked for and would not say so.**
pub fn is_this_repeat_permitted(
    the_budget: &HowManyTimesARunMayBePutBack,
    which_repetition: u32,
) -> Result<(), ARepeatThatWasNotPermitted> {
    let how_many_were_permitted = the_budget.may_be_put_back_this_many_times();
    let is_inside = which_repetition <= how_many_were_permitted;
    if is_inside {
        return Ok(());
    }
    Err(ARepeatThatWasNotPermitted::the_repeat_budget_ran_out(
        which_repetition,
        how_many_were_permitted,
    ))
}

/// Why a run could not be put back to work again, **and each is a fact about a different thing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheRepairCouldNotHappen {
    /// Nothing was given to ask.
    ThereIsNoSupervisorForThisRun,
    /// The model judged that the failure will not change, **carrying what has not changed.**
    TheSupervisorJudgedThisIsNotRepairing {
        /// The model's own sentence, **quoted whole because it is the only evidence for the call.**
        what_has_not_changed: String,
    },
    /// The ceiling stopped it, **which is a fact about the setting and not about the model.**
    TheRepeatBudgetRanOut {
        /// Which repetition this was.
        how_many_were_asked_for: u32,
        /// How many the budget permitted.
        how_many_were_permitted: u32,
    },
}

impl fmt::Display for TheRepairCouldNotHappen {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            TheRepairCouldNotHappen::ThereIsNoSupervisorForThisRun => String::from(
                "no `supervise` was given to this run — **a default that refuses is loud when it \
                 runs and silent until then**, and this repository's history has a real run that \
                 reached a supervisor twenty minutes in and delivered nothing",
            ),
            TheRepairCouldNotHappen::TheSupervisorJudgedThisIsNotRepairing {
                what_has_not_changed,
            } => format!(
                "a supervisor judged the failure will not change, and its reason is: \
                 {what_has_not_changed}"
            ),
            TheRepairCouldNotHappen::TheRepeatBudgetRanOut { .. } => String::from(
                "the repeat budget ran out, which is a fact about the setting and not about the \
                 model — **a run that reported both this and a judgement as *the repair failed* \
                 sends whoever reads the trace after the wrong one**",
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for TheRepairCouldNotHappen {}

impl TheRepairCouldNotHappen {
    /// A model judged it will not change, **and the sentence it judged on travels with the
    /// refusal** — because a refusal that does not carry its reason is a refusal nobody can argue
    /// with or check.
    pub fn the_supervisor_judged_this_is_not_repairing(the_answer: &WhatToRepeat) -> Self {
        TheRepairCouldNotHappen::TheSupervisorJudgedThisIsNotRepairing {
            what_has_not_changed: the_answer.what_has_not_changed.clone(),
        }
    }
}
