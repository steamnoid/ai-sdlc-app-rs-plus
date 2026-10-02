//! A model rules at the gate only while a flag says it may, and never before it is asked.
//!
//! **A flag that is off has to mean the model is not consulted at all**, and that is a property of
//! the order of operations rather than of a check: a program that would notice being run, a flag
//! that is off, and no call. **A gate that reaches the model and then ignores the flag has already
//! paid for a ruling nobody authorised.**
//!
//! **The flag is a value here and not a setting**, because a use case that reads its own
//! configuration has moved a decision out of a type and into a line of code.

use aisdlc_domain::approval::{AnArtifactVersion, WhichArtifact, WhoDecided};
use aisdlc_run::a_model_is_asked_how_it_rules::{
    WhereAModelIs, a_model_may_not_rule_without_a_flag, the_model_is_asked_how_it_rules_on,
};
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::whether_a_model_rules_here::WhetherAModelRulesHere;
use aisdlc_run::{ApprovalQuestion, the_question_put_to_somebody};

/// A program that would notice being run, because a flag that is off must not reach one.
struct AProgramThatWouldNotice {
    was_run: std::cell::Cell<bool>,
}

impl AProgramThatWouldNotice {
    fn a_new_one() -> Self {
        Self {
            was_run: std::cell::Cell::new(false),
        }
    }
}

impl AProgramWeRun for AProgramThatWouldNotice {
    fn run(
        &self,
        _what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo,
    ) -> WhatTheProgramSaid {
        self.was_run.set(true);
        let a_ruling = a_ruling_of("APPROVED");
        WhatTheProgramSaid::of(true, &the_answer_a_model_endpoint_gives_for(&a_ruling))
    }
}

/// The answer an OpenAI-compatible endpoint gives, **and not a bare word** — because no endpoint
/// returns a bare word, and a double that does proves a shape nothing produces.
fn the_answer_a_model_endpoint_gives_for(what_the_model_said: &str) -> String {
    serde_json::json!({
        "id": "chatcmpl-a-test",
        "choices": [{"message": {"role": "assistant", "content": what_the_model_said}}],
    })
    .to_string()
}

/// A verdict word and a reason on a second line, **which is the shape a model is held to** — and
/// **`serde_json` escapes the newline for us**, so the envelope stays valid JSON and these tests
/// keep testing the flag rather than the envelope.
fn a_ruling_of(the_word: &str) -> String {
    format!("{the_word}\nthe requirement is clear and the tests exist")
}

fn a_question_about_one_artifact() -> ApprovalQuestion {
    the_question_put_to_somebody(
        &aisdlc_domain::work_item::WorkItem::standing_at(
            aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
            aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name"),
            aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
                .expect("two halves"),
            aisdlc_domain::role::Role::try_from("PO").expect("a discipline"),
            aisdlc_domain::stage::Stage::AwaitingHumanApproval,
        )
        .expect("a work item answering a request can stand at the gate"),
        &WhichArtifact::try_from("ARequirements").expect("a name"),
        AnArtifactVersion::try_from(1).expect("one is a version"),
    )
    .expect("a work item at the gate can be asked about")
}

fn a_configured_model() -> WhereAModelIs {
    WhereAModelIs {
        the_endpoint: "http://localhost:11434/v1".to_string(),
        which_model: "a-model".to_string(),
    }
}

fn the_one_who_ruled() -> WhoDecided {
    WhoDecided::try_from("a model: the configured one").expect("a name")
}

#[test]
fn a_model_that_may_not_rule_is_refused_and_is_never_asked() {
    let a_program = AProgramThatWouldNotice::a_new_one();

    let how_it_went = a_model_may_not_rule_without_a_flag(
        &a_program,
        &a_question_about_one_artifact(),
        &the_one_who_ruled(),
        &a_configured_model(),
        &WhetherAModelRulesHere::No,
    );

    assert!(
        how_it_went.is_err(),
        "**because a gate answered by a model nobody switched on is a gate that ruled without anybody** — and the refusal is the honest answer until a person turns the flag"
    );
    assert!(
        !a_program.was_run.get(),
        "**and the model was not consulted at all, which is the half that costs money** — a refusal that arrives after the call has already paid for a ruling nobody authorised"
    );
}

#[test]
fn a_model_that_may_rule_is_asked_and_its_ruling_names_the_model() {
    let a_program = AProgramThatWouldNotice::a_new_one();

    let the_ruling = a_model_may_not_rule_without_a_flag(
        &a_program,
        &a_question_about_one_artifact(),
        &the_one_who_ruled(),
        &a_configured_model(),
        &WhetherAModelRulesHere::Yes,
    )
    .expect("a flag that is on lets the model rule");

    assert!(
        the_ruling.approved,
        "**and the model said yes** — which is the whole of what the flag is for, and no more"
    );
    assert!(
        a_program.was_run.get(),
        "**and it was actually asked**, because a flag that is on and a model that is never consulted is a switch wired to nothing"
    );
}

#[test]
fn a_flag_is_a_value_and_not_a_setting_this_crate_reads() {
    let gate_is_open = WhetherAModelRulesHere::Yes;
    let gate_is_shut = WhetherAModelRulesHere::No;

    assert_ne!(
        gate_is_open, gate_is_shut,
        "**and the two are different values**"
    );
    assert_eq!(
        the_model_is_asked_how_it_rules_on(
            &AProgramThatWouldNotice::a_new_one(),
            &a_question_about_one_artifact(),
            &the_one_who_ruled(),
            &a_configured_model(),
        )
        .expect("a one-word answer is a ruling")
        .decided_by
        .to_string(),
        "a model: the configured one",
        "**asking the model directly is a different function from asking it only when allowed**, and the second is the one the gate uses — so the first can exist for a caller that has already decided, and a reader can see which is which"
    );
}
