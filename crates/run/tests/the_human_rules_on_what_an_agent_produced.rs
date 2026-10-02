//! A person rules on an exact artifact at an exact version, and the record says who.
//!
//! **This is the boundary the specification calls hard, and it is hard here for one reason: an
//! agent may not rule.** The domain refuses a decider in the `aisdlc-` namespace at the moment
//! the ruling is built, so an agent approving its own output is not a convention this system
//! keeps — it is a type that cannot be constructed.
//!
//! **The question names nobody, because a question does not know who will be asked.** The
//! decider is in the ruling and nowhere else, so a question cannot be handed round with a name
//! already written on it.
//!
//! **The gate refuses a work item that is not standing at it.** `DONE` is the case that
//! matters: a finished work item has nothing left to approve, and the record would be evidence
//! that a person ruled on work that was already over.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::{AnArtifactVersion, ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::{WorkItem, a_work_item_commissioned_for_an_agent};
use aisdlc_domain::work_item_id::WorkItemId;

/// A person who rules, and records what they were asked about.
struct APersonWhoRules {
    the_questions_they_were_asked: Vec<WhichArtifact>,
}

impl APersonWhoRules {
    fn who_approves() -> Self {
        Self {
            the_questions_they_were_asked: Vec::new(),
        }
    }
}

impl aisdlc_run::APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &aisdlc_run::ApprovalQuestion) -> ApprovalDecision {
        self.the_questions_they_were_asked
            .push(a_question.about_which_artifact());
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            true,
            WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
            THE_REASON_THE_DECIDER_GAVE,
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

fn a_work_item_this_agent_holds() -> WorkItem {
    let picked_up = aisdlc_run::the_framework_picks_the_work_up(
        a_work_item_being_worked_on(),
        the_agent_who_did_the_work(),
    )
    .expect("a work item standing at `READY` may be picked up");
    aisdlc_run::the_agent_takes_the_work_item_into_its_own_hands(
        picked_up,
        the_agent_who_did_the_work(),
    )
    .expect("an agent may start work it has been handed")
}

fn a_work_item_waiting_to_be_ruled_on() -> WorkItem {
    let started = a_work_item_this_agent_holds();
    aisdlc_run::the_agent_presents_what_it_produced(
        started,
        the_agent_who_did_the_work(),
        &the_artifact_being_produced(),
    )
    .expect("an agent may present what it produced")
}

fn the_question_about_what_was_produced() -> aisdlc_run::ApprovalQuestion {
    let the_work_item = a_work_item_waiting_to_be_ruled_on();
    aisdlc_run::the_question_put_to_somebody(
        &the_work_item,
        &the_artifact_being_produced(),
        AnArtifactVersion::try_from(1u32).expect("a version is at least one"),
    )
    .expect("a work item standing at the gate may be put to somebody")
}

/// The reason every ruling in this file carries, **and it says something so the
/// new rule about grounds is never what these tests are about.**
const THE_REASON_THE_DECIDER_GAVE: &str =
    "the discovery report names two languages and a build system";

#[test]
fn the_gate_moves_a_work_item_from_being_worked_on_to_waiting_for_a_person() {
    let being_worked_on = a_work_item_this_agent_holds();

    let presented = aisdlc_run::the_agent_presents_what_it_produced(
        being_worked_on,
        the_agent_who_did_the_work(),
        &the_artifact_being_produced(),
    )
    .expect("an agent may present what it produced");

    assert_eq!(
        presented.the_stage_it_stands_at(),
        Stage::AwaitingHumanApproval,
        "and that is the one move the table allows out of `IN_PROGRESS_BY_AGENT` when it is not a hand-back"
    );
}

#[test]
fn a_person_asked_to_rule_is_asked_about_an_exact_artifact_and_an_exact_version() {
    let a_question = the_question_about_what_was_produced();

    assert_eq!(
        a_question.about_which_version(),
        AnArtifactVersion::try_from(1u32).expect("a version is at least one"),
        "**an approval that names no version still covers the artifact after it is rewritten**, which makes the record a claim rather than a decision"
    );
    assert_eq!(
        a_question.about_which_artifact(),
        the_artifact_being_produced()
    );
}

#[test]
fn the_question_names_the_work_item_and_never_a_decider() {
    let a_question = the_question_about_what_was_produced();

    assert_eq!(
        a_question.about_which_work_item(),
        WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank"),
        "**read from the work item itself**, because a record whose subject is supplied by whoever writes it is a record about a guess — and the gate could not name it at all until the domain gained a reader"
    );
}

#[test]
fn a_ruling_leaves_a_record_naming_who_ruled_verbatim() {
    let mut the_person = APersonWhoRules::who_approves();
    let a_question = the_question_about_what_was_produced();

    let the_record =
        aisdlc_run::the_gate_records_what_the_human_decided(&mut the_person, &a_question)
            .expect("a person who approves leaves a record");

    assert_eq!(
        the_record.decided_by,
        WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
        "**a person's name and a policy's name are both valid and the record keeps the difference** — a record naming a person for a decision a policy made would be a forged audit trail, and it would look exactly like the real thing at the moment somebody needs to tell them apart"
    );
}

#[test]
fn the_gate_asks_about_the_artifact_the_question_carries() {
    let mut the_person = APersonWhoRules::who_approves();
    let a_question = the_question_about_what_was_produced();

    aisdlc_run::the_gate_records_what_the_human_decided(&mut the_person, &a_question)
        .expect("a person who approves leaves a record");

    assert_eq!(
        the_person.the_questions_they_were_asked,
        vec![the_artifact_being_produced()],
        "so the person was asked about this artifact and not about whatever the run was carrying"
    );
}

#[test]
fn the_gate_rules_on_a_work_item_that_is_at_it_and_refuses_one_that_is_not() {
    let a_question = the_question_about_what_was_produced();
    let at_the_gate = a_work_item_waiting_to_be_ruled_on();
    let not_at_the_gate = a_work_item_being_worked_on();

    let accepted = aisdlc_run::the_gate_records_what_the_human_decided_at(
        &mut APersonWhoRules::who_approves(),
        &at_the_gate,
        &a_question,
    );
    let refusal = aisdlc_run::the_gate_records_what_the_human_decided_at(
        &mut APersonWhoRules::who_approves(),
        &not_at_the_gate,
        &a_question,
    );

    assert!(
        accepted.is_ok(),
        "**and the same call has to accept the work item that is standing at the gate**, or a gate that refuses everything is a gate nobody can tell from one that refuses the right thing"
    );
    assert!(
        refusal.is_err(),
        "**`DONE` is the case that matters**: a finished work item has nothing left to approve, and the record would be evidence that a person ruled on work that was already over"
    );
}

#[test]
fn a_refusal_says_which_stage_the_work_item_was_standing_at() {
    let a_question = the_question_about_what_was_produced();
    let not_at_the_gate = a_work_item_being_worked_on();

    let refusal = aisdlc_run::the_gate_records_what_the_human_decided_at(
        &mut APersonWhoRules::who_approves(),
        &not_at_the_gate,
        &a_question,
    );

    let why_not = refusal.expect_err("a work item at `READY` is not standing at the gate");
    assert!(
        why_not.why_it_could_not_be_asked().contains("READY"),
        "**an odmowa that cannot be read is a refusal a person cannot act on**, and `Stage` already knew its own name through `Display` — the refusal did not need a second way of asking"
    );
}
