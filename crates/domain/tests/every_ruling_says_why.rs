//! Every ruling says why, and the record carries the reason.
//!
//! **`approved` is a boolean and refusing leaves no record, so refusing is the half that stops
//! work and approving is the half that hands it on.** That asymmetry is why the reason matters
//! more on the approving side: **a record with no reason is a decision somebody else could have
//! made**, and a trace recording *approved* with no grounds is a coin flip written down at the one
//! place a coin flip is worst.
//!
//! **It is required and not optional, because an optional reason is two standards of evidence in
//! the same table — a person might skip it and a policy would never.**

use aisdlc_domain::approval::{
    ADecisionNobodyGaveAReasonFor, AnArtifactVersion, ApprovalDecision, WhichArtifact, WhoDecided,
};
use aisdlc_domain::work_item_id::WorkItemId;

/// A person ruling, **with the reason they gave.**
fn a_ruling(reason: &str) -> ApprovalDecision {
    ApprovalDecision {
        work_item_id: WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        which_artifact: WhichArtifact::try_from("Requirements").expect("an artifact"),
        the_version: AnArtifactVersion::try_from(1).expect("a version"),
        approved: true,
        decided_by: WhoDecided(String::from("steamnoid")),
        reason: String::from(reason),
    }
}

/// The same person refusing, **and refusing is allowed to carry a reason too.**
fn a_refusal(reason: &str) -> ApprovalDecision {
    ApprovalDecision {
        approved: false,
        ..a_ruling(reason)
    }
}

#[test]
fn a_ruling_with_no_reason_leaves_no_record() {
    let a_ruling_with_no_reason = a_ruling("   ");

    let the_refusal = a_ruling_with_no_reason.why_it_carries_no_ground();

    assert_eq!(
        the_refusal,
        Err(ADecisionNobodyGaveAReasonFor::TheReasonWasBlank),
        "**because a record with no reason is a decision somebody else could have made** — and \
         refusing costs nothing to express, **so the half that hands work on is the half where an \
         unreasoned answer would be hidden.** It refused: {the_refusal:?}"
    );
}

#[test]
fn the_record_carries_the_reason_word_for_word() {
    let the_reason_a_person_wrote = "the requirements name one file and one way of knowing it holds";

    let the_record = a_ruling(the_reason_a_person_wrote)
        .what_the_ruling_left_behind()
        .expect("a reason is present and the ruling was yes");

    assert_eq!(
        the_record.reason, the_reason_a_person_wrote,
        "**because the words are copied and never summarised** — a person reading this record wants \
         the reason they gave and not this system's paraphrase of it, **and a paraphrase is where a \
         decision quietly changes.**"
    );
}

#[test]
fn a_refusal_leaves_no_record_and_still_carries_its_reason() {
    let what_the_refusal_left_behind = a_refusal("the plan names a file nobody approved")
        .what_the_ruling_left_behind();

    assert_eq!(
        what_the_refusal_left_behind, None,
        "**because refusing is a decision and it leaves no record of work done** — a refusal that \
         recorded an approval would be a record of a delivery that never happened."
    );
}

#[test]
fn a_refusal_reason_is_still_required() {
    let a_refusal_with_no_reason = a_refusal("  ");

    let the_refusal = a_refusal_with_no_reason.why_it_carries_no_ground();

    assert_eq!(
        the_refusal,
        Err(ADecisionNobodyGaveAReasonFor::TheReasonWasBlank),
        "**because the trace after a refused run is exactly where somebody needs to know why**, and \
         a refusal with no reason is a refusal with nothing to read."
    );
}
