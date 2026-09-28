//! The one way a work item enters the lifecycle, and the stage it enters at.
//!
//! **A birth that could be refused would be a birth whose failure a caller could retry into
//! a different stage**, and that is a queue somebody can write to. So this takes no stage:
//! the entry state is `READY`, it is not the caller's to choose, and the call cannot fail.
//!
//! **Nothing in the transition table targets `READY`**, which is the half of the claim that
//! lives in the state machine rather than here — and it is the half that would be lost if
//! only this file existed, because a work item built here at the wrong stage and a work item
//! moved into `READY` by the framework are the same lie told two ways.

use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::{legal_transitions_from, transition, validate_state};
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::a_work_item_commissioned_for_an_agent;
use aisdlc_domain::work_item_id::WorkItemId;

fn a_name() -> WorkItemId {
    WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank")
}

fn whoever_asked() -> WhoCommissioned {
    WhoCommissioned::try_from("paligakrzychu").expect("a name is not blank")
}

fn what_it_is_worked_on() -> Repository {
    Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves")
}

fn what_was_asked() -> CommissionedRequest {
    CommissionedRequest::asked_for(
        "add a health endpoint".to_string(),
        "GET /health should return 200".to_string(),
    )
    .expect("a title says something")
}

fn commissioned(requested: Option<CommissionedRequest>) -> aisdlc_domain::work_item::WorkItem {
    a_work_item_commissioned_for_an_agent(
        a_name(),
        whoever_asked(),
        what_it_is_worked_on(),
        Role::Development,
        requested,
    )
}

#[test]
fn a_commissioned_work_item_enters_at_the_entry_state() {
    let born = commissioned(None);

    assert_eq!(
        born.the_stage_it_stands_at(),
        Stage::Ready,
        "READY is the entry state, and it is not the caller's to choose"
    );
}

#[test]
fn a_commissioned_work_item_is_held_by_nobody() {
    let born = commissioned(None);

    assert_eq!(
        born.the_agent_holding_it(),
        None,
        "a person queued this and nobody holds it yet, and READY is a stage nobody may hold"
    );
    assert_eq!(
        validate_state(&born).ok(),
        Some(()),
        "and the state it is born in holds together"
    );
}

#[test]
fn a_commissioned_work_item_keeps_the_words_it_was_asked_for() {
    let born = commissioned(Some(what_was_asked()));

    assert_eq!(
        born.the_request_it_answers()
            .map(|it| (it.title.clone(), it.body.clone())),
        Some((
            "add a health endpoint".to_string(),
            "GET /health should return 200".to_string(),
        )),
        "a stage that reads them nine reads later reads what the first one read"
    );
}

#[test]
fn a_work_item_may_be_born_before_anybody_has_said_what_they_want() {
    let born = commissioned(None);

    assert_eq!(
        born.the_request_it_answers(),
        None,
        "work can be queued before the words arrive, and a request that says nothing was asked for is refused at its own door rather than being invented here"
    );
}

#[test]
fn a_commissioned_work_item_carries_who_asked_verbatim() {
    let born = commissioned(None);

    assert_eq!(
        born.who_asked().to_string(),
        "paligakrzychu",
        "the work item keeps the wording of who asked, and a policy keeps its own name rather than a person's"
    );
}

#[test]
fn the_entry_state_needs_no_agent_and_so_commissioning_cannot_be_refused() {
    assert!(
        !Stage::Ready.an_agent_must_be_holding_it(),
        "if READY ever began to need an agent this call would start failing, and a birth that can fail is a birth whose failure can be retried into a different stage"
    );
}

#[test]
fn no_move_in_the_table_puts_a_work_item_back_into_the_entry_state() {
    let reaches_ready: Vec<Stage> = [
        Stage::Idle,
        Stage::AwaitingAgentPickup,
        Stage::InProgressByAgent,
        Stage::AwaitingHumanApproval,
        Stage::Ready,
        Stage::Done,
    ]
    .into_iter()
    .flat_map(|from| legal_transitions_from(from).to_vec())
    .filter(|to| *to == Stage::Ready)
    .collect();

    assert_eq!(
        reaches_ready,
        Vec::new(),
        "nothing targets READY, and that is a property rather than a gap: the framework cannot put work in its own queue"
    );
}

#[test]
fn a_commissioned_work_item_can_begin_its_first_move() {
    let born = commissioned(None);

    let moved = transition(
        born,
        Stage::AwaitingAgentPickup,
        Some(
            aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-dev-impl")
                .expect("a name is not an identity"),
        ),
    );

    assert!(
        moved.is_ok(),
        "READY hands to AWAITING_AGENT_PICKUP, which is the first move of every run: {moved:?}"
    );
}
