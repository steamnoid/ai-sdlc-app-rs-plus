//! What the product analyst concluded, in two lists.
//!
//! **The rule that is not in the type is the one this module exists for.** An ambiguity says who
//! can answer it, **and this system carries it rather than obeying it** — because a run that
//! refuses plannable work delivers nothing, and every gate passes while it does.
//!
//! **Six real runs are the evidence for that reversal**, and they are recorded in this
//! repository's own history. An approved requirement held three things nobody had settled, the
//! analyst said a person could answer them, **and a run stopped and asked.** Then a run refused a
//! request that named the file to add — *"the inspection does not show whether the existing
//! `.gitignore` already ignores `e2e.db-wal`"*, **a true sentence and not a reason, because the
//! implementer opens the file** — and the last of the six refused a request to change **one word
//! on a home page** because nobody had said whether to keep the emoji.
//!
//! **What replaced the refusal is the gate this system already has**: the person ruling on the
//! plan reads the ambiguity there, **with better evidence than the analyst had.**

use std::fmt;

/// Who can settle something nobody had settled, and never a bare string.
///
/// **`WhatTheCodeShows` and not `TheRepository`.** A name may not end on `Repository` — that word
/// is this system's own public GitHub repository, **and the question here is not which repository
/// but whether the answer is already written down.** `the_ambiguities_only_a_person_can_settle`
/// is the list the planner's question is marked from, and this is what keeps it out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoCanAnswerThisAmbiguity {
    /// Only a person, **because no file in the checkout says so.**
    APerson,
    /// The answer is in the code, **because the implementer opens the file.**
    WhatTheCodeShows,
}

/// One thing that is asked, **and the ways of knowing that it holds.**
///
/// **The criteria are inside the requirement and not beside it**, because a criterion that could
/// belong to either of two requirements is a criterion that will.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARequirement {
    /// What is being asked of the repository, **in the words somebody used.**
    pub what_is_asked: String,
    /// What a person looks at to know it holds, **and never nothing.**
    pub how_it_is_known_to_hold: Vec<String>,
}

/// One thing nobody had settled, **and who could settle it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAmbiguity {
    /// What is not settled, **and never a blank.**
    pub what_is_unsettled: String,
    /// Who can settle it, **and never nothing.**
    pub who_can_answer_it: WhoCanAnswerThisAmbiguity,
}

/// What the product analyst concluded about the work, **and nothing it was not asked for.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheProductAnalystConcluded {
    /// What is being asked, **at least one and every one of them checkable.**
    pub requirements: Vec<ARequirement>,
    /// What nobody had settled, **carried to the plan and not obeyed here.**
    pub ambiguities: Vec<AnAmbiguity>,
}

/// Why these requirements cannot be put in front of a person, **and each is a different fact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementsThatAskForNothing {
    /// No requirement at all, **so there is nothing to approve.**
    NothingWasAsked,
    /// A requirement with no observable, **and it is quoted whole.**
    ARequirementNobodyCouldCheck {
        /// What was asked, **word for word, because a person has to find which of their own words
        /// became unwatchable.**
        what_was_asked: String,
    },
    /// An ambiguity that does not say what is unsettled, **and it is quoted whole.**
    AnAmbiguityNamingNothing {
        /// Who was asked to settle it, **because the blank was in the middle of the entry.**
        who_was_asked: WhoCanAnswerThisAmbiguity,
    },
}

impl fmt::Display for RequirementsThatAskForNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            RequirementsThatAskForNothing::NothingWasAsked => String::from(
                "no requirement at all, and silence reaching a gate carries the authority of a \
                 finished analysis",
            ),
            RequirementsThatAskForNothing::ARequirementNobodyCouldCheck { what_was_asked } => {
                format!(
                    "{what_was_asked} names no way of being known to hold, and a requirement nobody \
                 could check is a wish delivered as a requirement"
                )
            }
            RequirementsThatAskForNothing::AnAmbiguityNamingNothing { who_was_asked } => format!(
                "an ambiguity nobody named what is unsettled is not an ambiguity, and a blank on a \
                 person's copy of the question is a question with no question in it — this one was \
                 handed to {who_was_asked:?}"
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for RequirementsThatAskForNothing {}

/// Whether one requirement names a way of being known to hold, **and a blank is not one.**
fn names_a_way_of_being_known_to_hold(requirement: &ARequirement) -> bool {
    requirement
        .how_it_is_known_to_hold
        .iter()
        .any(|one_way| !one_way.trim().is_empty())
}

/// One requirement with no observable, **refused and quoted whole rather than by its position.**
pub fn a_requirement_with_no_way_of_being_known_to_hold(
    requirement: &ARequirement,
) -> RequirementsThatAskForNothing {
    RequirementsThatAskForNothing::ARequirementNobodyCouldCheck {
        what_was_asked: requirement.what_is_asked.clone(),
    }
}

/// One ambiguity with no question in it, **refused and naming who it was handed to.**
fn an_ambiguity_naming_nothing(ambiguity: &AnAmbiguity) -> RequirementsThatAskForNothing {
    RequirementsThatAskForNothing::AnAmbiguityNamingNothing {
        who_was_asked: ambiguity.who_can_answer_it,
    }
}

impl WhatTheProductAnalystConcluded {
    /// Whether a gate may be shown this at all, **and the one rule an ambiguity never breaks.**
    ///
    /// **An ambiguity does not stop anything, and it is checked last on purpose.** An ambiguity
    /// only a person can settle still reaches the gate beside the requirements, **because the
    /// person ruling on the plan is the one who reads it and this analyst was working from an
    /// inspection** — a summary written for a person is the thing this mechanism exists to avoid.
    pub fn what_the_gate_may_be_shown(&self) -> Result<(), RequirementsThatAskForNothing> {
        if self.requirements.is_empty() {
            return Err(RequirementsThatAskForNothing::NothingWasAsked);
        }
        for one_requirement in &self.requirements {
            if !names_a_way_of_being_known_to_hold(one_requirement) {
                return Err(a_requirement_with_no_way_of_being_known_to_hold(
                    one_requirement,
                ));
            }
        }
        for one_ambiguity in &self.ambiguities {
            if one_ambiguity.what_is_unsettled.trim().is_empty() {
                return Err(an_ambiguity_naming_nothing(one_ambiguity));
            }
        }
        Ok(())
    }

    /// The ambiguities only a person can settle, **and this is what the planner's question is
    /// marked from.**
    ///
    /// **`WhatTheCodeShows` is not in it, because the implementer opens the file.** An ambiguity
    /// the checkout already settles is handed on as settled, **and marking it as a person's
    /// question sends a run back to somebody for an answer `.gitignore` already holds.**
    pub fn the_ambiguities_only_a_person_can_settle(&self) -> Vec<&AnAmbiguity> {
        self.ambiguities
            .iter()
            .filter(|one| one.who_can_answer_it == WhoCanAnswerThisAmbiguity::APerson)
            .collect()
    }
}
