//! A model asked to rule at the gate, and what it is not allowed to say about who decided.
//!
//! **A record naming a person for a decision a model made is a forged audit trail**, and it would
//! look exactly like the real thing at the moment somebody needs to tell them apart. So the name
//! in the record comes from our own configuration and **never from the model's answer**, however
//! confidently the answer offers one.
//!
//! **The model is asked in a fixed form, and anything else is a refusal.** A free-text ruling has
//! no shape to validate, and a contract a model can miss is not a contract.

use aisdlc_domain::approval::{AnArtifactVersion, ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_model_is_asked_how_it_rules::{
    WhereAModelIs, the_model_is_asked_how_it_rules_on,
};
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::{ApprovalQuestion, the_question_put_to_somebody};

/// A model that answers with fixed words and records what it was asked.
struct AModelThatAnswersWith {
    what_it_says: String,
    what_it_was_asked: std::cell::RefCell<Vec<String>>,
}

impl AModelThatAnswersWith {
    /// **A model that says the given words inside the envelope its endpoint answers with.**
    ///
    /// This double used to hand back the bare word, and the whole suite was green — **and no
    /// endpoint returns a bare word.** A double that answers in a shape nothing produces is a
    /// double that proves the plumbing and not the contract, so it now says what an
    /// OpenAI-compatible endpoint says, and the escaping is `serde_json`'s because a model that
    /// explains itself puts newlines and quotes in there.
    /// The verdict word and a reason on a second line, **which is the shape a model is held to.**
    fn ruling(with_the_word: &str) -> Self {
        let the_reason = "the requirement is clear and the tests exist";
        Self::saying(&format!("{with_the_word}\n{the_reason}"))
    }

    fn saying(what_it_says: &str) -> Self {
        Self {
            what_it_says: what_it_says.to_string(),
            what_it_was_asked: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AProgramWeRun for AModelThatAnswersWith {
    fn run(
        &self,
        what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo,
    ) -> WhatTheProgramSaid {
        self.what_it_was_asked
            .borrow_mut()
            .push(what_it_is_asked_to_do.what_it_is_asked_about().to_string());
        WhatTheProgramSaid::of(true, &self.answer_the_endpoint_gave())
    }
}

impl AModelThatAnswersWith {
    /// The answer an OpenAI-compatible endpoint gives for a model that said these words.
    fn answer_the_endpoint_gave(&self) -> String {
        serde_json::json!({
            "id": "chatcmpl-a-test",
            "choices": [{"message": {"role": "assistant", "content": self.what_it_says}}],
        })
        .to_string()
    }
}

/// The model this deployment is configured to ask.
fn a_configured_model() -> WhereAModelIs {
    WhereAModelIs {
        the_endpoint: "http://localhost:11434/v1".to_string(),
        which_model: "a-model".to_string(),
    }
}

/// A work item standing at the gate, which is the only place a question can be put.
fn a_work_item_at_the_gate() -> aisdlc_domain::work_item::WorkItem {
    aisdlc_domain::work_item::WorkItem::standing_at(
        WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name"),
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        aisdlc_domain::role::Role::try_from("PO").expect("a discipline"),
        aisdlc_domain::stage::Stage::AwaitingHumanApproval,
    )
    .expect("a work item answering a request can stand at the gate")
}

fn a_question_about_one_artifact() -> ApprovalQuestion {
    the_question_put_to_somebody(
        &a_work_item_at_the_gate(),
        &WhichArtifact::try_from("ARequirements").expect("a name"),
        AnArtifactVersion::try_from(1).expect("one is a version"),
    )
    .expect("a work item at the gate can be asked about")
}

#[test]
fn a_plain_yes_is_a_ruling_naming_whoever_this_system_was_told() {
    let a_model = AModelThatAnswersWith::ruling("APPROVED");

    let the_ruling = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question_about_one_artifact(),
        &WhoDecided::try_from("a model: the configured one").expect("a name"),
        &a_configured_model(),
    )
    .expect("one word in the fixed form is a ruling");

    assert!(the_ruling.approved, "**the model's yes is a yes**");
    assert_eq!(
        the_ruling.decided_by.to_string(),
        "a model: the configured one",
        "**and the name in the record is the one we were given** — a model that rules here is recorded as a model, and the record says so whether the answer was yes or no"
    );
}

/// A model that offers a decider cannot name one.
///
/// **This test used to demand a refusal, and the refusal was an implementation of the rule rather
/// than the rule.** The rule is that **a record must not name a decider nobody appointed**, and it
/// was enforced by noticing that the model had added a line. Now the verdict is one word on a line
/// of its own and everything after it is the reason, **so a model that offers a decider is no
/// longer refused — it is recorded under whoever this system was told, and its offer lands in the
/// reason where a reader sees it.**
///
/// **The protection is now structural rather than inspected.** `decided_by` is an argument the
/// caller passes and **there is no path from the answer into it**, so Alice can appear in the
/// reason and nowhere else. **A record naming Alice for a decision no person made looks exactly
/// like a real one at the moment somebody needs to tell them apart.**
#[test]
fn a_model_that_offers_a_decider_cannot_name_one() {
    let a_model = a_model_that_offers_alice();
    let whoever_this_system_was_told = this_system_was_told();

    let the_ruling = the_ruling_this_model_gave(&a_model, &whoever_this_system_was_told);
    let what_the_record_says = (
        the_ruling.decided_by.clone(),
        is_alice_only_in_the_reason(&the_ruling),
    );

    assert_eq!(
        (what_the_record_says.0, what_the_record_says.1),
        (whoever_this_system_was_told, true),
        "**because the model was told it could rule and not who ruled** — and the name it offered \
         has to survive as evidence rather than be lost: {:?}",
        the_ruling.reason
    );
}

#[test]
fn an_answer_that_is_not_the_fixed_form_is_refused_rather_than_guessed_at() {
    let a_model = AModelThatAnswersWith::saying("I think it is probably fine, yes?");

    let how_it_went = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question_about_one_artifact(),
        &WhoDecided::try_from("a model: the configured one").expect("a name"),
        &a_configured_model(),
    );

    assert!(
        how_it_went.is_err(),
        "**because a guess at what a model meant is a decision nobody made** — and this gate records decisions, so an answer with no shape in it is a refusal rather than a ruling with `true` somewhere in it"
    );
}

#[test]
fn the_question_it_is_asked_names_the_artifact_and_its_version() {
    let a_model = AModelThatAnswersWith::ruling("REFUSED");

    let _ = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question_about_one_artifact(),
        &WhoDecided::try_from("a model: the configured one").expect("a name"),
        &a_configured_model(),
    );

    let what_it_was_asked = a_model.what_it_was_asked.borrow();
    assert!(
        what_it_was_asked[0].contains("ARequirements")
            && what_it_was_asked[0].contains("WELLBEINGT-5"),
        "**because a model asked about nothing cannot rule about something** — and the question is built here rather than by the model, so a model cannot ask itself a different question and answer it"
    );
}

/// Whether the name the model offered is in the reason and nowhere else, **and the record is
/// asked separately** — because one helper holding both would let a reader miss which was which.
fn is_alice_only_in_the_reason(the_ruling: &ApprovalDecision) -> bool {
    the_ruling.reason.contains("Alice")
}

/// Whoever this system was told ruled, **and it is a model and not a person** — the whole of what
/// makes the assertion about a model rather than about Alice.
fn this_system_was_told() -> WhoDecided {
    WhoDecided::try_from("a model: the configured one").expect("a name")
}

/// A model that answers yes and then offers to name the decider, **and the offer is on its own
/// line so the verdict is still a line of its own.**
fn a_model_that_offers_alice() -> AModelThatAnswersWith {
    AModelThatAnswersWith::saying("APPROVED\ndecided_by: Alice\nbecause it looks right to me")
}

/// What this model ruled about one artifact.
fn the_ruling_this_model_gave(
    a_model: &AModelThatAnswersWith,
    whoever_this_system_was_told: &WhoDecided,
) -> ApprovalDecision {
    the_model_is_asked_how_it_rules_on(
        a_model,
        &a_question_about_one_artifact(),
        whoever_this_system_was_told,
        &a_configured_model(),
    )
    .expect("one word on the first line is a ruling")
}
