//! What the product analyst concluded, and the one thing it must never do about an ambiguity.
//!
//! **The rule this file exists for was a reversal, and the reversal cost six real runs.** An
//! approved requirement held three things nobody had settled; the analyst said who could answer
//! each, **and a run stopped and asked when a person had to.** Then a real run refused a request
//! that named the file to add, with *"the inspection does not show whether the existing
//! `.gitignore` already ignores `e2e.db-wal`"* — **a true sentence and not a reason, because the
//! implementer opens the file.** Four attempts to steer it failed, and the sixth refused a request
//! to change **one word on a home page** because nobody had said whether to keep the emoji.
//!
//! **A run that refuses plannable work delivers nothing, and every gate passes while it does.**
//! So the ambiguity is carried, not obeyed: **the person ruling on the plan reads it there, with
//! better evidence than the analyst had.** That is the rule these tests assert.

use aisdlc_run::what_the_product_analyst_concluded::{
    ARequirement, AnAmbiguity, RequirementsThatAskForNothing, WhatTheProductAnalystConcluded,
    WhoCanAnswerThisAmbiguity, a_requirement_with_no_way_of_being_known_to_hold,
};

/// One thing that is asked, **and the ways of knowing it holds.**
fn a_requirement() -> ARequirement {
    ARequirement {
        what_is_asked: String::from("the leftover database sidecar files are not tracked"),
        how_it_is_known_to_hold: vec![
            String::from("git status reports no e2e.db-wal"),
            String::from("a clone contains no e2e.db-wal"),
        ],
    }
}

/// One thing nobody had settled, **and the person is the one who can settle it.**
fn an_ambiguity_only_a_person_can_settle() -> AnAmbiguity {
    AnAmbiguity {
        what_is_unsettled: String::from("whether the emoji on the home page stays"),
        who_can_answer_it: WhoCanAnswerThisAmbiguity::APerson,
    }
}

/// What an analyst concluded, **holding one requirement and no ambiguity at all.**
fn what_the_analyst_concluded() -> WhatTheProductAnalystConcluded {
    WhatTheProductAnalystConcluded {
        requirements: vec![a_requirement()],
        ambiguities: vec![],
    }
}

#[test]
fn requirements_naming_one_thing_may_be_shown_to_a_gate() {
    let may_it_be_shown = what_the_analyst_concluded().what_the_gate_may_be_shown();

    assert!(
        may_it_be_shown.is_ok(),
        "**because the requirement is what the person is being asked about, and a set of empty \
         lists cannot be shown to anybody** — and it names a thing and two ways of knowing it \
         holds. What was refused: {may_it_be_shown:?}"
    );
}

#[test]
fn requirements_asking_for_nothing_are_refused() {
    let the_refusal = WhatTheProductAnalystConcluded {
        requirements: vec![],
        ambiguities: vec![],
    }
    .what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(RequirementsThatAskForNothing::NothingWasAsked)
        ),
        "**because an analysis that asked for nothing delivers nothing, and silence reaching a \
         gate carries the authority of a finished analysis** — a person cannot tell an empty \
         analysis from a real one by looking at it. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_requirement_with_no_way_of_being_known_to_hold_is_refused() {
    let a_requirement_nobody_could_check = ARequirement {
        what_is_asked: String::from("the sidecar files are handled properly"),
        how_it_is_known_to_hold: vec![],
    };
    let what_the_analyst_concluded = WhatTheProductAnalystConcluded {
        requirements: vec![a_requirement_nobody_could_check],
        ambiguities: vec![],
    };

    let the_refusal = what_the_analyst_concluded.what_the_gate_may_be_shown();

    assert!(
        matches!(
            the_refusal,
            Err(RequirementsThatAskForNothing::ARequirementNobodyCouldCheck { .. })
        ),
        "**because a requirement with no way of being known to hold is a wish, and a wish \
         delivered as a requirement is a pull request nobody can check** — `the sidecar files are \
         handled properly` names no observable and settles nothing. What was refused: {the_refusal:?}"
    );
}

#[test]
fn an_ambiguity_only_a_person_can_settle_does_not_stop_the_analysis() {
    let the_analyst_concluded = WhatTheProductAnalystConcluded {
        requirements: vec![a_requirement()],
        ambiguities: vec![an_ambiguity_only_a_person_can_settle()],
    };

    let may_it_be_shown = the_analyst_concluded.what_the_gate_may_be_shown();

    assert!(
        may_it_be_shown.is_ok(),
        "**because a run that refuses plannable work delivers nothing and every gate passes while \
         it does** — this repository's own history records six real runs spent on it, the last \
         refusing a request to change one word on a home page. The gate this system already has \
         reads the ambiguity at the plan, with better evidence than the analyst had. What was \
         refused: {may_it_be_shown:?}"
    );
}

#[test]
fn an_ambiguity_the_code_can_answer_is_marked_as_such_and_not_as_a_person_s() {
    let the_analyst_concluded = WhatTheProductAnalystConcluded {
        requirements: vec![a_requirement()],
        ambiguities: vec![AnAmbiguity {
            what_is_unsettled: String::from("whether e2e.db-wal is already ignored"),
            who_can_answer_it: WhoCanAnswerThisAmbiguity::WhatTheCodeShows,
        }],
    };

    let only_a_person_can_settle = the_analyst_concluded.the_ambiguities_only_a_person_can_settle();

    assert!(
        only_a_person_can_settle.is_empty(),
        "**because the implementer opens the file** — an ambiguity the code already settles is \
         handed to the plan as settled, and marking it as a person's question sends a run back \
         to somebody for an answer `.gitignore` already holds. What was marked as a person's: \
         {only_a_person_can_settle:?}"
    );
}

#[test]
fn an_ambiguity_naming_nobody_to_ask_is_refused_before_a_gate_ever_sees_it() {
    let the_analyst_concluded = WhatTheProductAnalystConcluded {
        requirements: vec![a_requirement()],
        ambiguities: vec![AnAmbiguity {
            what_is_unsettled: String::from("  "),
            who_can_answer_it: WhoCanAnswerThisAmbiguity::APerson,
        }],
    };

    let the_refusal = the_analyst_concluded.what_the_gate_may_be_shown();

    assert!(
        matches!(
            the_refusal,
            Err(RequirementsThatAskForNothing::AnAmbiguityNamingNothing { .. })
        ),
        "**because an ambiguity that does not say what is unsettled is not an ambiguity, it is a \
         field nobody filled in** — and a blank on a person's copy of the question is a question \
         with no question in it. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_requirement_nobody_could_check_is_named_by_a_refusal_of_its_own() {
    let the_refusal = a_requirement_with_no_way_of_being_known_to_hold(&ARequirement {
        what_is_asked: String::from("the sidecar files are handled properly"),
        how_it_is_known_to_hold: vec![],
    });

    assert_eq!(
        the_refusal.to_string(),
        "the sidecar files are handled properly names no way of being known to hold, and a \
         requirement nobody could check is a wish delivered as a requirement",
        "**because the refusal quotes the requirement itself and not its position** — a person \
         reading this is told which of their words became unwatchable, and not which row of a \
         list it was on. What it said: {the_refusal}"
    );
}
