//! Two of the six stages are stages an agent holds, and four are stages nobody may.
//!
//! **This is the third column of the specification's stage table read out of the code.** The
//! table already says which stages carry an agent and which do not, and until now nothing
//! could ask. That matters because it is the whole basis of the two refusals a work item
//! has: one stage is not held by an agent it needs, and the other is held by an agent at a
//! stage where nobody may hold.
//!
//! The answers are written out here rather than read from a table, because a test of the
//! domain may not read a file — the domain may not reach for a filesystem, and a test of it
//! least of all. That the six answers agree with the specification's column is the drift
//! gate's job, in a package allowed to read.

use aisdlc_domain::stage::{EVERY_STAGE, Stage};

#[test]
fn only_the_two_stages_an_agent_may_hold_say_that_one_is_needed() {
    for stage in EVERY_STAGE {
        let expected = matches!(stage, Stage::AwaitingAgentPickup | Stage::InProgressByAgent);
        assert_eq!(
            stage.an_agent_must_be_holding_it(),
            expected,
            "{stage} says {expected} in the specification's third column and says something else here"
        );
    }
}

#[test]
fn the_two_stages_an_agent_must_hold_are_exactly_those_two() {
    let the_ones: Vec<Stage> = EVERY_STAGE
        .into_iter()
        .filter(|stage| stage.an_agent_must_be_holding_it())
        .collect();

    assert_eq!(
        the_ones,
        vec![Stage::AwaitingAgentPickup, Stage::InProgressByAgent],
        "two of the six, and they are the two the table marks present"
    );
}

#[test]
fn the_four_stages_nobody_may_hold_are_exactly_those_four() {
    let the_ones: Vec<Stage> = EVERY_STAGE
        .into_iter()
        .filter(|stage| !stage.an_agent_must_be_holding_it())
        .collect();

    assert_eq!(
        the_ones,
        vec![
            Stage::Idle,
            Stage::AwaitingHumanApproval,
            Stage::Ready,
            Stage::Done,
        ],
        "four of the six, and they are the four the table marks absent"
    );
}

#[test]
fn the_two_halves_cover_every_stage_and_neither_is_empty() {
    let the_ones_that_must: Vec<Stage> = EVERY_STAGE
        .into_iter()
        .filter(|stage| stage.an_agent_must_be_holding_it())
        .collect();
    let the_ones_that_must_not: Vec<Stage> = EVERY_STAGE
        .into_iter()
        .filter(|stage| !stage.an_agent_must_be_holding_it())
        .collect();

    assert_eq!(
        the_ones_that_must.len() + the_ones_that_must_not.len(),
        EVERY_STAGE.len(),
        "every stage is on one side or the other, or a stage has no answer at all"
    );
    assert_eq!(
        the_ones_that_must.len(),
        2,
        "and the side that is empty is not a side"
    );
    assert_eq!(the_ones_that_must_not.len(), 4, "nor the other");
}
