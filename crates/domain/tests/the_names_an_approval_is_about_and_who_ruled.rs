//! The names an approval is about, and the name of whoever ruled.
//!
//! **`WhichArtifact` refuses a blank and nothing else, and that is the point.** The eight
//! artifacts are produced by agents that do not exist yet and naming their identifiers is
//! their business. A closed set of eight written here would bake a list into the enum and
//! into every gate, and every agent arriving later would have to match a name it did not
//! choose — which is why there is no `EVERY_ARTIFACT` in this crate and why the tests
//! below check that a name nobody has written down is still a name.
//!
//! **`AnArtifactVersion` counts from one.** There is no zeroth version of anything, and a
//! version of zero is a decision about an artifact that does not exist yet.
//!
//! No file is read here, because a test of the domain may not reach for a filesystem.

use aisdlc_domain::approval::{
    ADecisionThatNamesNobody, AnApprovalThatNamesNothing, AnArtifactVersion,
    AnArtifactVersionBelowOne, WhichArtifact, WhoDecided,
};

#[test]
fn an_artifact_nobody_has_written_down_is_still_a_name() {
    for name in ["QaPlan", "DiscoveryReport", "SomethingNotWrittenYet", "x"] {
        let which = WhichArtifact::try_from(name).expect("a name that is not blank");

        assert_eq!(
            which.to_string(),
            name,
            "{name:?} is not one of the eight this document lists, and listing them is the agents' business rather than this type's"
        );
    }
}

#[test]
fn an_artifact_named_nothing_is_refused() {
    for nothing in ["", "   ", "\t"] {
        let result = WhichArtifact::try_from(nothing);

        assert_eq!(
            result,
            Err(AnApprovalThatNamesNothing(nothing.to_string())),
            "a decision can never be recorded about nothing, and {nothing:?} is nothing"
        );
    }
}

#[test]
fn an_artifact_is_written_exactly_as_it_was_named() {
    let which = WhichArtifact::try_from("  QaPlan  ").expect("a name");

    assert_eq!(
        which.to_string(),
        "  QaPlan  ",
        "an artifact's name is kept as typed"
    );
}

#[test]
fn a_version_counts_from_one_and_there_is_no_zeroth_one() {
    assert_eq!(
        AnArtifactVersion::try_from(0),
        Err(AnArtifactVersionBelowOne(0)),
        "an artifact version counts from one, and zero is not one"
    );
}

#[test]
fn every_version_from_one_up_is_a_version() {
    for number in 1..8u32 {
        let version = AnArtifactVersion::try_from(number).expect("one and up is a version");

        assert_eq!(
            version.to_string(),
            number.to_string(),
            "{number} is a version"
        );
    }
}

#[test]
fn a_refused_version_says_which_number_was_refused_and_where_counting_starts() {
    let said = AnArtifactVersionBelowOne(0).to_string();

    assert!(
        said.contains('0'),
        "a refusal a reader cannot match teaches nothing: {said}"
    );
    assert!(
        said.contains("one"),
        "and it says where counting starts, so the reader learns the rule and not only the verdict: {said}"
    );
}

#[test]
fn a_person_and_a_policy_are_both_valid_and_the_record_keeps_the_difference() {
    for party in ["paligakrzychu", "aisdlc-autopilot:every-gate", "  spaced  "] {
        let who = WhoDecided::try_from(party).expect("a name that is not blank");

        assert_eq!(
            who.to_string(),
            party,
            "{party:?} ruled and the record says so verbatim: a person's name and a policy's name are both real"
        );
    }
}

#[test]
fn a_decision_naming_nobody_is_refused() {
    for nobody in ["", "   ", "\t"] {
        let result = WhoDecided::try_from(nobody);

        assert_eq!(
            result,
            Err(ADecisionThatNamesNobody(nobody.to_string())),
            "a decision that names nobody is a decision whose audit trail is a claim rather than a record, and {nobody:?} names nobody"
        );
    }
}

#[test]
fn one_blank_produces_two_refusals_a_reader_can_tell_apart() {
    let about_an_artifact = WhichArtifact::try_from("   ").expect_err("blank");
    let about_a_decider = WhoDecided::try_from("   ").expect_err("blank");

    assert_ne!(
        about_an_artifact.to_string(),
        about_a_decider.to_string(),
        "one blank, two doors, and a reader told only the message must be able to say which one refused"
    );
    assert!(
        about_an_artifact.to_string().contains("artifact"),
        "and the artifact refusal names the artifact: {about_an_artifact}"
    );
    assert!(
        about_a_decider.to_string().contains("decided"),
        "and the decider refusal names the decision: {about_a_decider}"
    );
}
