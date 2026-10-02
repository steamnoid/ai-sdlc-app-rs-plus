//! What the quality analyst would check, and the one rule a scenario has to obey.
//!
//! **A scenario is not a wish and a requirement is not a test.** A test somebody writes and then
//! watches pass proves nothing about the work; **a test somebody wrote from a requirement is the
//! only thing that turns a requirement into evidence**, and the link between the two is a fact
//! about this artifact and not a matter of taste.
//!
//! **So a scenario says which requirement it proves, and a scenario proving a requirement nobody
//! asked for is refused.** It reads as harmless and is not: it is a test that will keep passing
//! after the work is done wrong, **and it is a line in a pull request somebody has to read.**

use aisdlc_run::what_the_product_analyst_concluded::ARequirement;
use aisdlc_run::what_would_prove_the_work::{
    AScenario, WhatWouldProveNothing, WhatWouldProveTheWork,
};

/// What is being asked, **and how anybody would know it holds.**
fn a_requirement() -> ARequirement {
    ARequirement {
        what_is_asked: String::from("the leftover database sidecar files are not tracked"),
        how_it_is_known_to_hold: vec![String::from("git status reports no e2e.db-wal")],
    }
}

/// One scenario, **proving that requirement and saying what it looks at.**
fn a_scenario() -> AScenario {
    AScenario {
        which_requirement_it_proves: String::from(
            "the leftover database sidecar files are not tracked",
        ),
        what_is_checked: String::from("a fresh clone holds no e2e.db-wal"),
        where_it_runs: String::from("the repository's own suite"),
    }
}

#[test]
fn a_scenario_proving_a_requirement_nobody_asked_for_is_refused() {
    let what_would_prove = WhatWouldProveTheWork {
        requirements: vec![a_requirement()],
        scenarios: vec![AScenario {
            which_requirement_it_proves: String::from("the home page loads in under a second"),
            what_is_checked: String::from("a stopwatch"),
            where_it_runs: String::from("somewhere"),
        }],
    };

    let the_refusal = what_would_prove.what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhatWouldProveNothing::AScenarioAboutSomethingElse { .. })
        ),
        "**because a scenario proving a requirement nobody asked for is a test that will keep \
         passing after the work is done wrong** — and it is a line in a pull request somebody has \
         to read. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_plan_naming_no_scenario_is_refused() {
    let what_would_prove = WhatWouldProveTheWork {
        requirements: vec![a_requirement()],
        scenarios: vec![],
    };

    let the_refusal = what_would_prove.what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhatWouldProveNothing::NothingWouldBeChecked)
        ),
        "**because a plan of requirements with no scenario on any of them approves evidence for \
         work that was never proved** — and it reads as thorough, because the requirement list is \
         right there. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_scenario_naming_nothing_to_check_is_refused() {
    let what_would_prove = WhatWouldProveTheWork {
        requirements: vec![a_requirement()],
        scenarios: vec![AScenario {
            which_requirement_it_proves: a_requirement().what_is_asked,
            what_is_checked: String::from("   "),
            where_it_runs: String::from("the repository's own suite"),
        }],
    };

    let the_refusal = what_would_prove.what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhatWouldProveNothing::AScenarioThatWouldProveNothing { .. })
        ),
        "**because a scenario that names no check is the most convincing line in a plan and the \
         least useful** — it reads as coverage and proves nothing at all. What was refused: \
         {the_refusal:?}"
    );
}

#[test]
fn a_requirement_nothing_checks_is_refused_as_the_thing_it_is() {
    let a_requirement_nobody_could_check = ARequirement {
        what_is_asked: String::from("the sidecars are handled properly"),
        how_it_is_known_to_hold: vec![],
    };
    let a_scenario_proving_it = AScenario {
        which_requirement_it_proves: a_requirement_nobody_could_check.what_is_asked.clone(),
        what_is_checked: String::from("the implementer says so"),
        where_it_runs: String::from("the repository's own suite"),
    };

    let the_refusal = WhatWouldProveTheWork {
        requirements: vec![a_requirement_nobody_could_check],
        scenarios: vec![a_scenario_proving_it],
    }
    .what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhatWouldProveNothing::ARequirementNothingChecks { .. })
        ),
        "**because a requirement with no way of being known to hold is a wish** — and a scenario \
         pointed at one checks a thing nobody could have checked either, **so the refusal names \
         the requirement rather than the scenario**, which is where the fault is. What was \
         refused: {the_refusal:?}"
    );
}

#[test]
fn a_requirement_no_scenario_proves_is_refused() {
    let the_refusal = WhatWouldProveTheWork {
        requirements: vec![a_requirement(), ARequirement {
            what_is_asked: String::from("the home page keeps the emoji it already had"),
            how_it_is_known_to_hold: vec![String::from("the home page still shows it")],
        }],
        scenarios: vec![a_scenario()],
    }
    .what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhatWouldProveNothing::ARequirementNoScenarioProves { .. })
        ),
        "**because one requirement proved out of two is a delivery whose unproved half is the half \
         nobody will notice** — the scenarios all pass, the run reports green, and the requirement \
         the plan forgot to check is the one that was never checked. What was refused: \
         {the_refusal:?}"
    );
}
