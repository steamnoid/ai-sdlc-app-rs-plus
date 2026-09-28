//! A run closes a work item by holding the record of a ruling, and by nothing else.
//!
//! **`DONE` is terminal and one edge reaches it, and the edge is `AWAITING_HUMAN_APPROVAL`.**
//! A run that closed a work item without a ruling would be a run deciding for itself that
//! somebody approved, and the prohibition on an agent ruling is the hard boundary this system
//! is built around.
//!
//! **The approval is in the signature and not in a check.** `Approval::what_the_ruling_left_behind`
//! returns nothing after a refusal, so a function that takes an `Approval` cannot be called at
//! all without the record — **a rule a type carries is a rule a caller cannot forget, and a rule
//! a function checks is a rule a caller can pass through by lying about the argument.**
//!
//! **A refusal is a decision and a work item whose delivery was refused is not delivered**, so
//! there is nothing to close: the gate left no record and this has nothing to hand in.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::{AnArtifactVersion, ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::{WorkItem, a_work_item_commissioned_for_an_agent};
use aisdlc_domain::work_item_id::WorkItemId;

/// A person who approves, or who refuses.
struct APersonWhoRules {
    approved: bool,
}

impl APersonWhoRules {
    fn who_approves() -> Self {
        Self { approved: true }
    }

    fn who_refuses() -> Self {
        Self { approved: false }
    }
}

impl aisdlc_run::APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &aisdlc_run::ApprovalQuestion) -> ApprovalDecision {
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            self.approved,
            WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
        )
        .expect("a person may rule")
    }
}

fn the_agent_who_did_the_work() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-po-analyze").expect("a name is not blank")
}

fn the_artifact_being_produced() -> WhichArtifact {
    WhichArtifact::try_from("DiscoveryReport").expect("an artifact has a name")
}

fn a_work_item_being_worked_on() -> WorkItem {
    a_work_item_commissioned_for_an_agent(
        WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank"),
        WhoCommissioned::try_from("paligakrzychu").expect("a name is not blank"),
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        Role::Product,
        Some(
            CommissionedRequest::asked_for(
                "Name the stages a work item may be at".to_string(),
                "A reader should not have to read the code to learn them.".to_string(),
            )
            .expect("a request with a title and a body is a request"),
        ),
    )
}

fn a_work_item_waiting_to_be_ruled_on() -> WorkItem {
    let picked_up = aisdlc_run::the_framework_picks_the_work_up(
        a_work_item_being_worked_on(),
        the_agent_who_did_the_work(),
    )
    .expect("a work item standing at `READY` may be picked up");
    let started = aisdlc_run::the_agent_takes_the_work_item_into_its_own_hands(
        picked_up,
        the_agent_who_did_the_work(),
    )
    .expect("an agent may start work it has been handed");
    aisdlc_run::the_agent_presents_what_it_produced(
        started,
        the_agent_who_did_the_work(),
        &the_artifact_being_produced(),
    )
    .expect("an agent may present what it produced")
}

fn a_question_about_what_was_produced() -> aisdlc_run::ApprovalQuestion {
    let the_work_item = a_work_item_waiting_to_be_ruled_on();
    aisdlc_run::the_question_put_to_somebody(
        &the_work_item,
        &the_artifact_being_produced(),
        AnArtifactVersion::try_from(1u32).expect("a version is at least one"),
    )
    .expect("a work item standing at the gate may be put to somebody")
}

#[test]
fn a_run_holding_a_ruling_closes_the_work_item() {
    let the_work_item = a_work_item_waiting_to_be_ruled_on();
    let the_record = aisdlc_run::the_gate_records_what_the_human_decided_at(
        &mut APersonWhoRules::who_approves(),
        &the_work_item,
        &a_question_about_what_was_produced(),
    )
    .expect("a person who approves leaves a record");

    let closed = aisdlc_run::the_run_closes_the_work_item(the_work_item, &the_record)
        .expect("a run holding a ruling may close the work item");

    assert_eq!(
        closed.the_stage_it_stands_at(),
        Stage::Done,
        "and `DONE` is terminal, so this is the last move there is"
    );
}

#[test]
fn a_closed_work_item_is_held_by_nobody() {
    let the_work_item = a_work_item_waiting_to_be_ruled_on();
    let the_record = aisdlc_run::the_gate_records_what_the_human_decided_at(
        &mut APersonWhoRules::who_approves(),
        &the_work_item,
        &a_question_about_what_was_produced(),
    )
    .expect("a person who approves leaves a record");

    let closed = aisdlc_run::the_run_closes_the_work_item(the_work_item, &the_record)
        .expect("a run holding a ruling may close the work item");

    assert_eq!(
        closed.the_agent_holding_it(),
        None,
        "**the biconditional says an agent is absent at `DONE`**, and a run that closed its work item with an agent still attached would be a work item finished by somebody nobody can name"
    );
}

#[test]
fn a_refusal_leaves_no_record_and_so_nothing_to_close() {
    let the_work_item = a_work_item_waiting_to_be_ruled_on();

    let after_a_refusal = aisdlc_run::the_gate_records_what_the_human_decided_at(
        &mut APersonWhoRules::who_refuses(),
        &the_work_item,
        &a_question_about_what_was_produced(),
    );

    assert!(
        after_a_refusal.is_err(),
        "**a refusal is a decision and a work item whose delivery was refused is not delivered** — the gate left no record, and there is nothing to hand in to close with"
    );
}

#[test]
fn closing_a_work_item_that_is_not_waiting_for_a_ruling_is_refused() {
    let still_being_worked_on = aisdlc_run::the_framework_picks_the_work_up(
        a_work_item_being_worked_on(),
        the_agent_who_did_the_work(),
    )
    .expect("a work item standing at `READY` may be picked up");
    let the_record = aisdlc_run::the_gate_records_what_the_human_decided_at(
        &mut APersonWhoRules::who_approves(),
        &a_work_item_waiting_to_be_ruled_on(),
        &a_question_about_what_was_produced(),
    )
    .expect("a person who approves leaves a record");

    let refusal = aisdlc_run::the_run_closes_the_work_item(still_being_worked_on, &the_record);

    assert!(
        refusal.is_err(),
        "**a record of somebody ruling on a work item is a record about that work item**, and one work item's record must not close another — which the transition table refuses here because the stage is not `AWAITING_HUMAN_APPROVAL`"
    );
}
