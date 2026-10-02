//! What a verification concluded, and the one thing a suite run once cannot be.
//!
//! **A suite is run twice and the two runs are the evidence.** One is the repository before the
//! change and one is the repository after it, **and the difference between them is the only thing a
//! verification report is about** — a report with one side has run a suite once and called the
//! result a verdict.
//!
//! **And both sides must stand on different commits.** The first real run on this system reported
//! a green suite before and after a change that was a byte-identical rewrite of a file, **and the
//! two sides were the same repository twice**, which is a pair of identical answers and not a
//! comparison.

use aisdlc_run::the_tests_are_run_in_a_container::WhatTheSuiteSaid;
use aisdlc_run::what_a_verification_concluded::{
    ASuiteOutcome, WhatAVerificationConcluded, WhyAVerificationProvesNothing,
};

/// What a suite said, **and it installed and passed.**
fn a_suite_that_passed() -> WhatTheSuiteSaid {
    WhatTheSuiteSaid {
        did_the_dependencies_install: true,
        what_it_wrote: String::from("74 passed, 9 skipped"),
    }
}

/// One side of the comparison, **standing on a commit.**
///
/// **It answers the outcome and not an `Option`, and the caller wraps it** — a helper whose return
/// type is `Option` is a helper that cannot say *which* absence it is, **and "absent" is the
/// subject of two of the four tests here.**
fn the_suite_on(before: &str) -> ASuiteOutcome {
    ASuiteOutcome {
        which_commit_it_stood_on: String::from(before),
        what_the_suite_said: a_suite_that_passed(),
    }
}

/// A verification with two sides on two commits, **which is the ordinary case.**
fn what_a_verification_concluded() -> WhatAVerificationConcluded {
    WhatAVerificationConcluded {
        the_suite_before: Some(the_suite_on("bf38697")),
        the_suite_after: Some(the_suite_on("c2a9015")),
    }
}

#[test]
fn two_suites_on_two_commits_is_what_a_gate_may_be_shown() {
    let the_refusal = what_a_verification_concluded().what_the_gate_may_be_shown();

    assert!(
        the_refusal.is_ok(),
        "**because this is the ordinary case and a gate has to be able to see it pass** — a check \
         that refused everything would deliver nothing, and this repository's own history is six \
         real runs spent on a run that refused everything it was asked to do. What was refused: \
         {the_refusal:?}"
    );
}

#[test]
fn a_verification_with_one_side_is_refused() {
    let the_refusal = WhatAVerificationConcluded {
        the_suite_before: Some(the_suite_on("bf38697")),
        the_suite_after: None,
    }
    .what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhyAVerificationProvesNothing::ASuiteRanOnlyOnce { .. })
        ),
        "**because a suite run once has nothing to be compared with** — and a report with one side \
         reads as thorough, because it has a real suite result in it. What was refused: \
         {the_refusal:?}"
    );
}

#[test]
fn two_suites_on_the_same_commit_are_refused_by_name() {
    let the_refusal = WhatAVerificationConcluded {
        the_suite_before: Some(the_suite_on("bf38697")),
        the_suite_after: Some(the_suite_on("bf38697")),
    }
    .what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhyAVerificationProvesNothing::TheTwoSidesWereTheSameCommit { .. })
        ),
        "**because a pair of identical answers is not a comparison** — and the first real run on \
         this system reported a green suite before and after a change that was a byte-identical \
         rewrite of a file, with both sides standing on the same repository. What was refused: \
         {the_refusal:?}"
    );
}

/// A suite whose dependencies did not install, **and it is named here rather than in the test
/// because a struct literal inside a struct literal inside a struct literal is four levels deep
/// and this repository's own guard refuses the third.**
fn a_side_whose_install_never_worked() -> ASuiteOutcome {
    ASuiteOutcome {
        which_commit_it_stood_on: String::from("c2a9015"),
        what_the_suite_said: WhatTheSuiteSaid {
            did_the_dependencies_install: false,
            what_it_wrote: String::from("no matching distribution found"),
        },
    }
}

#[test]
fn a_side_whose_install_never_worked_is_not_a_side() {
    let the_refusal = WhatAVerificationConcluded {
        the_suite_before: Some(the_suite_on("bf38697")),
        the_suite_after: Some(a_side_whose_install_never_worked()),
    }
    .what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhyAVerificationProvesNothing::OneSideNeverRan { .. })
        ),
        "**because a suite whose dependencies did not install has not run** — and counting it as \
         a side turns 'the suite could not start' into 'the change broke the suite', **which is the \
         report this system would hand a person to rule on.** What was refused: {the_refusal:?}"
    );
}
