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

use aisdlc_run::a_model_is_asked_how_it_rules::the_model_is_asked_how_it_rules_on;
use aisdlc_run::a_ruling_is_recorded::the_gate_records_the_ruling_it_was_given;
use aisdlc_run::an_issue_read::AnIssueRead;
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
    aisdlc_domain::state_machine::transition(
        picked_up,
        aisdlc_domain::stage::Stage::InProgressByAgent,
        Some(aisdlc_run::a_run_picks_up_the_work::the_agent_that_picks_up_work()),
    )
    .expect("the table allows pickup to become work in progress")
}

#[test]
fn a_models_ruling_that_reached_the_gate_is_recorded_under_its_own_name() {
    let the_work = a_work_an_agent_is_working_on();
    let what_it_produced =
        WhatAnAgentHasProduced::new("DiscoveryReport", 1).expect("declared here");
    let a_question =
        an_agent_presents_what_it_produced_and_the_question_is_put(&the_work, &what_it_produced)
            .expect("an agent working on a work may present it");
    let who_ruled = aisdlc_domain::approval::WhoDecided::try_from("a model: the configured one")
        .expect("a name");
    let a_model = AModelThatAnswers::saying("APPROVED");

    let the_ruling = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question,
        &who_ruled,
        &aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs {
            the_endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
            which_model: "a-model".to_string(),
        },
    )
    .expect("a model that answers with one word has ruled");

    let the_record = the_gate_records_the_ruling_it_was_given(&the_ruling);

    assert_eq!(
        the_record
            .as_ref()
            .ok()
            .map(|what_it_left| what_it_left.approved_by().who_decided()),
        Some("a model: the configured one"),
        "**because a model can rule at a gate and the ruling must go somewhere** — and a record that \
         said 'approved' and left the name off would be evidence with no author: {the_record:?}"
    );
}

#[test]
fn a_ruling_that_refused_leaves_no_record_because_a_refusal_is_a_decision_and_not_an_approval() {
    let the_work = a_work_an_agent_is_working_on();
    let what_it_produced =
        WhatAnAgentHasProduced::new("DiscoveryReport", 1).expect("declared here");
    let a_question =
        an_agent_presents_what_it_produced_and_the_question_is_put(&the_work, &what_it_produced)
            .expect("an agent working on a work may present it");
    let who_ruled = aisdlc_domain::approval::WhoDecided::try_from("a model: the configured one")
        .expect("a name");
    let a_model = AModelThatAnswers::saying("REFUSED");

    let the_ruling = the_model_is_asked_how_it_rules_on(
        &a_model,
        &a_question,
        &who_ruled,
        &aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs {
            the_endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
            which_model: "a-model".to_string(),
        },
    )
    .expect("a model that answers with one word has ruled");

    let the_record = the_gate_records_the_ruling_it_was_given(&the_ruling);

    assert_eq!(
        the_record,
        Ok(None),
        "**because a refusal is a decision and not an approval** — and a record of a refusal would be \
         a record of something that was approved, which is the one thing a record must not be: \
         {the_record:?}"
    );
}
