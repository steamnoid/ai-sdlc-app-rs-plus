//! A ruling that reached the gate, and the record left behind.
//!
//! **A model's ruling is a decision and never a record.** `the_model_is_asked_how_it_rules_on`
//! produces an `ApprovalDecision` and the gate's own recorder takes only what a *person* decided —
//! so **a model can rule at a gate and the ruling goes nowhere**, and the run carries on with a
//! decision nobody can point at.
//!
//! **The record names whoever answered, and that is the only difference between a person and a
//! model.** A record that said "approved" and left the name off would be evidence with no author,
//! and a record that put a person's name on a model's ruling would be a forged audit trail.

use aisdlc_domain::approval::{Approval, ApprovalDecision};
use aisdlc_run::a_model_is_asked_how_it_rules::the_model_is_asked_how_it_rules_on;
use aisdlc_run::a_ruling_is_recorded::the_gate_records_the_ruling_it_was_given;
use aisdlc_run::what_an_agent_has_produced::WhatAnAgentHasProduced;
use aisdlc_run::what_the_agent_presents::an_agent_presents_what_it_produced_and_the_question_is_put;

/// A model that answers with a fixed word, and remembers it was asked.
struct AModelThatAnswers {
    what_it_says: String,
}

impl aisdlc_run::a_program_we_run::AProgramWeRun for AModelThatAnswers {
    fn run(
        &self,
        _what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo,
    ) -> aisdlc_run::a_program_we_run::WhatTheProgramSaid {
        aisdlc_run::a_program_we_run::WhatTheProgramSaid::of(true, &self.answer_the_endpoint_gave())
    }
}

impl AModelThatAnswers {
    fn saying(what_it_says: &str) -> Self {
        Self {
            what_it_says: what_it_says.to_string(),
        }
    }

    fn answer_the_endpoint_gave(&self) -> String {
        serde_json::json!({
            "choices": [{"message": {"content": self.what_it_says}}],
        })
        .to_string()
    }
}

/// Where a model that says the given word is, and who the ruling would be recorded under.
fn a_model_that_says(what_it_says: &str) -> AModelThatAnswers {
    AModelThatAnswers::saying(what_it_says)
}

/// The ruling a model gave, and the record it left behind.
fn the_ruling_and_its_record(what_the_model_says: &str) -> (ApprovalDecision, Option<Approval>) {
    let the_work = a_work_an_agent_is_working_on();
    let what_it_produced =
        WhatAnAgentHasProduced::new("DiscoveryReport", 1).expect("declared here");
    let a_question =
        an_agent_presents_what_it_produced_and_the_question_is_put(&the_work, &what_it_produced)
            .expect("an agent working on a work may present it");
    let the_ruling = the_model_is_asked_how_it_rules_on(
        &a_model_that_says(what_the_model_says),
        &a_question,
        &aisdlc_domain::approval::WhoDecided::try_from("a model: the configured one")
            .expect("a name"),
        &aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs {
            the_endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
            which_model: "a-model".to_string(),
        },
    )
    .expect("a model that answers with one word has ruled");
    let what_it_left = the_gate_records_the_ruling_it_was_given(&the_ruling);
    (the_ruling, what_it_left)
}

/// A work an agent is working on, with something to present.
fn a_work_an_agent_is_working_on() -> aisdlc_domain::work_item::WorkItem {
    let the_work = aisdlc_domain::work_item::WorkItem::standing_at(
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name"),
        aisdlc_domain::repository::Repository::try_from("steamnoid/one").expect("two halves"),
        aisdlc_domain::role::Role::try_from("PO").expect("a discipline"),
        aisdlc_domain::stage::Stage::Idle,
    )
    .expect("a work item may stand at idle");
    let picked_up =
        aisdlc_run::a_run_picks_up_the_work::a_run_picks_up_the_work_it_was_given(&the_work)
            .expect("the table allows this edge");
    aisdlc_run::a_run_picks_up_the_work::the_agent_takes_the_work_into_its_own_hands(&picked_up)
        .expect("the table allows pickup to become work in progress")
}

#[test]
fn a_models_ruling_that_reached_the_gate_is_recorded_under_its_own_name() {
    let (_the_ruling, what_it_left) = the_ruling_and_its_record(&a_ruling_of("APPROVED"));

    assert_eq!(
        what_it_left
            .as_ref()
            .map(|what_it_left| what_it_left.decided_by.0.as_str()),
        Some("a model: the configured one"),
        "**because a model can rule at a gate and the ruling must go somewhere** — and a record that \
         said 'approved' and left the name off would be evidence with no author: {what_it_left:?}"
    );
}

#[test]
fn a_ruling_that_refused_leaves_no_record_because_a_refusal_is_a_decision_and_not_an_approval() {
    let (_the_ruling, what_it_left) = the_ruling_and_its_record(&a_ruling_of("REFUSED"));

    assert_eq!(
        what_it_left, None,
        "**because a refusal is a decision and not an approval** — and a record of a refusal would be \
         a record of something that was approved, which is the one thing a record must not be: \
         {what_it_left:?}"
    );
}

/// The verdict word and a reason on a second line, **which is the shape a model is held to, and
/// `serde_json` escapes the newline for us** — so the envelope stays valid and this file keeps
/// testing the record rather than the envelope.
fn a_ruling_of(the_word: &str) -> String {
    format!("{the_word}\nthe requirement is clear and the tests exist")
}
