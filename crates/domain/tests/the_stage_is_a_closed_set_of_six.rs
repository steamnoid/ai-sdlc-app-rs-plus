//! The gate for RULE 2.5: a stage is a type, and the set of stages is closed.
//!
//! The six are transcribed here rather than imported, on purpose. A test that reads its
//! own expectation from the code it checks proves nothing, and `EVERY_STAGE` is the thing
//! being checked — so importing it would be a gate that agrees with whatever the code
//! happens to say today.
//!
//! **The property that matters is the one the compiler holds.** A stage is an enum, so a
//! seventh one does not compile until a `match` over it is completed, and a table of legal
//! moves gains a row or the build stops. The runtime assertions below cannot hold that
//! property; they hold the spelling, the count and the refusal, which are the parts a
//! reader would otherwise have to take on trust.

use aisdlc_domain::{EVERY_STAGE, Stage};

/// The six stages, written as the specification writes them, in its order.
///
/// Spelled out here and not read from `Stage`, for the reason in this file's first
/// paragraph. The order is the specification's, and a test that kept its own order could
/// not tell a reordering from a change of membership.
const THE_SIX_STAGES_THE_SPECIFICATION_NAMES: [&str; 6] = [
    "IDLE",
    "AWAITING_AGENT_PICKUP",
    "IN_PROGRESS_BY_AGENT",
    "AWAITING_HUMAN_APPROVAL",
    "READY",
    "DONE",
];

#[test]
fn a_stage_is_written_with_its_own_name_in_capitals() {
    let written: Vec<String> = EVERY_STAGE.iter().map(ToString::to_string).collect();
    assert_eq!(
        written, THE_SIX_STAGES_THE_SPECIFICATION_NAMES,
        "a stage is an enum and it is written in capitals, and a reader reads the written form rather than the variant"
    );
}

#[test]
fn there_are_exactly_six_stages() {
    assert_eq!(
        EVERY_STAGE.len(),
        6,
        "the set is closed and this is its size, and a seventh one is a change to the specification rather than a change to this file"
    );
}

#[test]
fn every_stage_the_specification_names_exists() {
    for name in THE_SIX_STAGES_THE_SPECIFICATION_NAMES {
        assert!(
            Stage::try_from(name).is_ok(),
            "{name} is one of the six, and a name the specification writes must be a name a stage answers to"
        );
    }
}

#[test]
fn a_name_nobody_wrote_is_not_a_stage() {
    for name in [
        "STARTED",
        "IN_REVIEW",
        "",
        "   ",
        "idle",
        "AWAITING_HUMAN_APPROVAL ",
    ] {
        let refused = Stage::try_from(name);
        assert!(
            refused.is_err(),
            "{name:?} is not one of the six, and a name nobody wrote down never becomes one"
        );
    }
}

#[test]
fn a_name_nobody_wrote_says_which_name_it_was_looking_for() {
    let Err(refusal) = Stage::try_from("IN_REVIEW") else {
        panic!("IN_REVIEW is not one of the six, so the refusal is the only answer");
    };
    assert_eq!(
        refusal.what_was_looked_for(),
        "IN_REVIEW",
        "a refusal that does not quote the name back spends a correction and teaches the caller nothing"
    );
    let said = refusal.to_string();
    assert!(
        said.contains("IN_REVIEW"),
        "the written refusal names the name too, and a reader debugging a run reads the written one: {said}"
    );
}
