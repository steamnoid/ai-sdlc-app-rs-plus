//! What a verification concluded, and the one thing a suite run once cannot be.
//!
//! **A suite is run twice and the two runs are the evidence.** One is the repository before the
//! change and one is the repository after it, **and the difference between them is the only thing a
//! verification report is about** — a report with one side has run a suite once and called the
//! result a verdict.
//!
//! **Both sides are `Option`, because absence has to be representable**, and a report with one side
//! reads as thorough **because it has a real suite result in it.**

use std::fmt;

use crate::the_tests_are_run_in_a_container::WhatTheSuiteSaid;

/// One side of the comparison, **and which commit it stood on is the whole of what it adds.**
///
/// **`WhatTheSuiteSaid` is reused and not redefined.** A suite's answer is one thing in this
/// system, **and a second type with the same two fields would be a second name for it** — which is
/// the defect the naming rules exist to prevent. What is added here is the one thing a comparison
/// needs and a suite cannot know: **which repository it was run against.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASuiteOutcome {
    /// The commit the clone stood on, **and it is what makes the two sides two sides.**
    pub which_commit_it_stood_on: String,
    /// What the suite said, **copied and never summarised.**
    pub what_the_suite_said: WhatTheSuiteSaid,
}

/// What a verification concluded, **and a comparison or nothing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAVerificationConcluded {
    /// The suite on the repository **before** the change, **and absent when it never ran.**
    pub the_suite_before: Option<ASuiteOutcome>,
    /// The suite on the repository **after** the change, **and absent when it never ran.**
    pub the_suite_after: Option<ASuiteOutcome>,
}

/// Why this verification proves nothing, **and each is a different fact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyAVerificationProvesNothing {
    /// One side is missing, **and it is quoted whole so a reader knows which.**
    ASuiteRanOnlyOnce {
        /// Which side is missing, **spelled out because "one of them" is not an answer.**
        which_side_is_missing: WhichSide,
    },
    /// Both sides ran on one commit, **and a pair of identical answers is not a comparison.**
    TheTwoSidesWereTheSameCommit {
        /// The commit both sides stood on, **quoted whole.**
        which_commit: String,
    },
    /// One side's dependencies did not install, **so that side has not run at all.**
    OneSideNeverRan {
        /// Which side, **and what the install said.**
        which_side: WhichSide,
        /// The install's own words, **because they are the only evidence that anything failed.**
        what_the_install_said: String,
    },
}

/// Which of the two sides a refusal is about, **and never a `bool`.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichSide {
    /// The repository before the change.
    Before,
    /// The repository after the change.
    After,
}

impl WhichSide {
    /// The side, **in words, because it goes into a sentence a person reads.**
    pub fn in_words(&self) -> &'static str {
        match self {
            WhichSide::Before => "before the change",
            WhichSide::After => "after the change",
        }
    }
}

impl WhyAVerificationProvesNothing {
    /// One side is missing, **and it says which, because "one of them" is not an answer.**
    pub fn a_suite_ran_only_once(which_side_is_missing: WhichSide) -> Self {
        WhyAVerificationProvesNothing::ASuiteRanOnlyOnce {
            which_side_is_missing,
        }
    }

    /// Both sides stood on one commit.
    pub fn the_two_sides_were_the_same_commit(which_commit: &str) -> Self {
        WhyAVerificationProvesNothing::TheTwoSidesWereTheSameCommit {
            which_commit: which_commit.to_string(),
        }
    }

    /// One side never ran, **and it carries the install's words.**
    pub fn one_side_never_ran(one_side: WhichSide, what_the_install_said: &str) -> Self {
        WhyAVerificationProvesNothing::OneSideNeverRan {
            which_side: one_side,
            what_the_install_said: what_the_install_said.to_string(),
        }
    }
}

impl fmt::Display for WhyAVerificationProvesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            WhyAVerificationProvesNothing::ASuiteRanOnlyOnce {
                which_side_is_missing,
            } => format!(
                "no suite ran {} , so there is nothing to compare the other side with, and a \
                 report with one side reads as thorough because it has a real suite result in it",
                which_side_is_missing.in_words()
            ),
            WhyAVerificationProvesNothing::TheTwoSidesWereTheSameCommit { which_commit } => {
                format!(
                    "both suites stood on {which_commit}, and a pair of identical answers is not a \
                 comparison however many times it was run"
                )
            }
            WhyAVerificationProvesNothing::OneSideNeverRan {
                which_side,
                what_the_install_said,
            } => format!(
                "the suite {} never ran, because its dependencies did not install, and counting it \
                 as a side turns 'the suite could not start' into 'the change broke the suite' — the \
                 install said: {what_the_install_said}",
                which_side.in_words()
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for WhyAVerificationProvesNothing {}

impl WhatAVerificationConcluded {
    /// Whether a gate may be shown this at all, **and every check is about there being two.**
    ///
    /// **A missing side is refused before anything else**, because the two remaining refusals both
    /// need two commits to compare — **and a refusal that came out of a comparison the reader cannot
    /// see would send them looking for a difference that is not there.**
    pub fn what_the_gate_may_be_shown(&self) -> Result<(), WhyAVerificationProvesNothing> {
        let before = self.the_suite_before.as_ref().ok_or(
            WhyAVerificationProvesNothing::a_suite_ran_only_once(WhichSide::Before),
        )?;
        let after = self.the_suite_after.as_ref().ok_or(
            WhyAVerificationProvesNothing::a_suite_ran_only_once(WhichSide::After),
        )?;
        both_sides_really_ran(before, after)?;
        the_two_sides_are_two_commits(before, after)
    }
}

/// Whether both suites installed their dependencies, **because a side that never ran is not a
/// side.**
///
/// **A free function and not a method**, because it reads nothing of this artifact — and a
/// method that took `&self` and used none of it would be a reader's question with no answer.
fn both_sides_really_ran(
    before: &ASuiteOutcome,
    after: &ASuiteOutcome,
) -> Result<(), WhyAVerificationProvesNothing> {
    one_side_really_ran(before, WhichSide::Before)?;
    one_side_really_ran(after, WhichSide::After)
}

/// Whether one side's dependencies installed.
fn one_side_really_ran(
    one_side: &ASuiteOutcome,
    which_side: WhichSide,
) -> Result<(), WhyAVerificationProvesNothing> {
    let did_it_install = one_side.what_the_suite_said.did_the_dependencies_install;
    if did_it_install {
        return Ok(());
    }
    let what_it_said = &one_side.what_the_suite_said.what_it_wrote;
    Err(WhyAVerificationProvesNothing::one_side_never_ran(
        which_side,
        what_it_said,
    ))
}

/// Whether the two sides stand on two commits, **which is the whole of what makes them a
/// comparison.**
///
/// **A free function and not a method, because it reads nothing of this artifact** — and the
/// clippy guard `unused_self` refused the method that did.
///
/// **The first real run on this system reported a green suite before and after a change that
/// was a byte-identical rewrite of a file**, with both sides on the same repository — **so this
/// is refused by name rather than reported as two green suites.**
fn the_two_sides_are_two_commits(
    before: &ASuiteOutcome,
    after: &ASuiteOutcome,
) -> Result<(), WhyAVerificationProvesNothing> {
    let are_two_commits = before.which_commit_it_stood_on != after.which_commit_it_stood_on;
    if are_two_commits {
        return Ok(());
    }
    let which_commit = &before.which_commit_it_stood_on;
    Err(WhyAVerificationProvesNothing::the_two_sides_were_the_same_commit(which_commit))
}
