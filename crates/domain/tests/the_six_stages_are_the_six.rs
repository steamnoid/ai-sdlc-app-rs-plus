//! The six stages, and the two spellings of each.
//!
//! **Nothing here reads a file**, because this is a test of the domain and the domain may
//! not reach for a filesystem — not even a test of it, and least of all to fetch the very
//! list this file would otherwise have to trust itself about. Checking the code against the
//! specification's table is a gate's job, in a package that is allowed to read, and it reads
//! the source rather than calling into a crate it is not allowed to depend on.
//!
//! The two spellings are what is proved here: a stage written in capitals reads back as
//! itself, a name nobody wrote is refused, and the refused name is in the refusal.

use aisdlc_domain::stage::{AStageThatNobodyNamed, EVERY_STAGE, Stage};

#[test]
fn there_are_exactly_six_stages() {
    assert_eq!(
        EVERY_STAGE.len(),
        6,
        "the transition table has six rows, and a seventh is a move it was never asked about"
    );
}

#[test]
fn every_stage_is_named_once() {
    let mut seen = Vec::new();
    for stage in EVERY_STAGE {
        seen.push(stage.to_string());
    }
    seen.sort();
    let before = seen.len();
    seen.dedup();

    assert_eq!(seen.len(), before, "a stage is listed twice: {seen:?}");
}

#[test]
fn writing_a_stage_and_reading_it_back_gives_the_same_stage() {
    for stage in EVERY_STAGE {
        let written = stage.to_string();
        let read = Stage::try_from(written.as_str());

        assert_eq!(
            read.ok(),
            Some(stage),
            "{stage} writes as {written} and does not read back as itself"
        );
    }
}

#[test]
fn a_name_nobody_wrote_is_refused() {
    let result = Stage::try_from("INVENTED");

    assert_eq!(
        result,
        Err(AStageThatNobodyNamed("INVENTED".to_string())),
        "a stage nobody wrote down is not a stage, and the transition table knows six rows"
    );
}

#[test]
fn a_stage_is_refused_in_the_wrong_case() {
    let result = Stage::try_from("ready");

    assert_eq!(
        result,
        Err(AStageThatNobodyNamed("ready".to_string())),
        "the capitals are the writing and accepting both spellings is accepting two"
    );
}

#[test]
fn the_refusal_says_which_name_was_refused() {
    let the_refusal = AStageThatNobodyNamed("INVENTED".to_string());

    let said = the_refusal.to_string();

    assert!(
        said.contains("INVENTED"),
        "a refusal a reader cannot match against what they wrote teaches nothing: {said}"
    );
    assert!(
        said.contains("six"),
        "and it says how many there are, so the reader learns the size of the set: {said}"
    );
}

#[test]
fn a_stage_written_in_capitals_is_the_variant_a_reader_expects() {
    for (written, expected) in [
        ("IDLE", Stage::Idle),
        ("AWAITING_AGENT_PICKUP", Stage::AwaitingAgentPickup),
        ("IN_PROGRESS_BY_AGENT", Stage::InProgressByAgent),
        ("AWAITING_HUMAN_APPROVAL", Stage::AwaitingHumanApproval),
        ("READY", Stage::Ready),
        ("DONE", Stage::Done),
    ] {
        assert_eq!(
            Stage::try_from(written).ok(),
            Some(expected),
            "{written} is {expected} in the specification's table and reads as something else"
        );
    }
}

#[test]
fn the_variant_is_the_idea_and_not_the_acronym() {
    for (stage, expected) in [
        (Stage::Idle, "IDLE"),
        (Stage::AwaitingAgentPickup, "AWAITING_AGENT_PICKUP"),
        (Stage::InProgressByAgent, "IN_PROGRESS_BY_AGENT"),
        (Stage::AwaitingHumanApproval, "AWAITING_HUMAN_APPROVAL"),
        (Stage::Ready, "READY"),
        (Stage::Done, "DONE"),
    ] {
        assert_eq!(
            stage.to_string(),
            expected,
            "the capitals are the writing and the variant is the reading, and this pair disagrees"
        );
    }
}
