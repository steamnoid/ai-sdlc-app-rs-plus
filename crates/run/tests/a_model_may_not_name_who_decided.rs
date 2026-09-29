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
use aisdlc_run::a_model_is_asked_how_it_rules::the_model_is_asked_how_it_rules_on;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::{ApprovalQuestion, the_question_put_to_somebody};

/// A model that answers with fixed words and records what it was asked.
struct AModelThatAnswersWith {
    what_it_says: String,
    what_it_was_asked: std::cell::RefCell<Vec<String>>,
}

impl AModelThatAnswersWith {
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
        WhatTheProgramSaid::of(true, &self.what_it_says)
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
fn a_models_answer_is_read_as_a_ruling_and_never_as_a_decider() {
    let a_model =
        AModelThatAnswersWith::saying("APPROVED\ndecided_by: Alice\nbecause it looks right to me");

    let the_ruling = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question_about_one_artifact(),
        &WhoDecided::try_from("a model: the configured one").expect("a name"),
    )
    .expect("a fixed form is a ruling");

    assert!(the_ruling.approved, "**the model's yes is a yes**");
    assert_eq!(
        the_ruling.decided_by.to_string(),
        "a model: the configured one",
        "**and the name in the record is the one we were given, not the one the answer offered** — a model that answers `decided_by: Alice` has written a sentence about a person, and taking it would put a person's name on a decision no person made. **A forged audit trail looks exactly like a real one at the moment somebody needs to tell them apart.**"
    );
}

#[test]
fn an_answer_that_is_not_the_fixed_form_is_refused_rather_than_guessed_at() {
    let a_model = AModelThatAnswersWith::saying("I think it is probably fine, yes?");

    let how_it_went = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question_about_one_artifact(),
        &WhoDecided::try_from("a model: the configured one").expect("a name"),
    );

    assert!(
        how_it_went.is_err(),
        "**because a guess at what a model meant is a decision nobody made** — and this gate records decisions, so an answer with no shape in it is a refusal rather than a ruling with `true` somewhere in it"
    );
}

#[test]
fn the_question_it_is_asked_names_the_artifact_and_its_version() {
    let a_model = AModelThatAnswersWith::saying("REFUSED");

    let _ = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question_about_one_artifact(),
        &WhoDecided::try_from("a model: the configured one").expect("a name"),
    );

    let what_it_was_asked = a_model.what_it_was_asked.borrow();
    assert!(
        what_it_was_asked[0].contains("ARequirements")
            && what_it_was_asked[0].contains("WELLBEINGT-5"),
        "**because a model asked about nothing cannot rule about something** — and the question is built here rather than by the model, so a model cannot ask itself a different question and answer it"
    );
}
