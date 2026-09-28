//! The framework takes a work item off the queue and puts it in front of an agent.
//!
//! **This is the first move the framework makes, and the table already allows it.** Nothing in
//! the transition table targets `READY` and exactly one thing targets
//! `AWAITING_AGENT_PICKUP` from it, so a work item that stops at `READY` is a work item nobody
//! will ever pick up — which is what twenty gates around a state nobody reaches looks like
//! from the outside.
//!
//! **A second move lives in the same file**: the agent takes the work into its own hands, which
//! is `AWAITING_AGENT_PICKUP -> IN_PROGRESS_BY_AGENT` and the only move out of the stage this
//! file is about. It is here rather than in a file of its own because it is the other half of
//! the same pickup, and a file per move would put two halves of one thing in two places.
//!
//! **The agent is named by the caller and not by the framework, and the stage requires one.**
//! `AWAITING_AGENT_PICKUP` is one of the two stages an agent must be holding, so the move is
//! made with the agent the caller says will do the work. This file's first version said
//! otherwise — that the stage meant "available, not taken" and that the move needed no agent —
//! **and the domain refused all four tests**, because the biconditional says an agent is set
//! exactly when the stage is one of those two. Reading the specification is not optional and
//! a guess about it is a guess that gets caught.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::{WorkItem, a_work_item_commissioned_for_an_agent};
use aisdlc_domain::work_item_id::WorkItemId;

fn the_request() -> CommissionedRequest {
    CommissionedRequest::asked_for(
        "Name the stages a work item may be at".to_string(),
        "A reader of the state machine should not have to read the code to learn them.".to_string(),
    )
    .expect("a request with a title and a body is a request")
}

fn a_work_item_waiting_to_be_picked_up() -> WorkItem {
    a_work_item_commissioned_for_an_agent(
        WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank"),
        WhoCommissioned::try_from("paligakrzychu").expect("a name is not blank"),
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        Role::Product,
        Some(the_request()),
    )
}

fn the_agent_who_will_do_it() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-po-analyze").expect("a name is not blank")
}

fn a_different_agent() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-qa-analyze").expect("a name is not blank")
}

#[test]
fn a_work_item_standing_at_ready_is_picked_up() {
    let the_work_item = a_work_item_waiting_to_be_picked_up();

    let picked_up =
        aisdlc_run::the_framework_picks_the_work_up(the_work_item, the_agent_who_will_do_it())
            .expect("a work item standing at `READY` may be picked up");

    assert_eq!(
        picked_up.the_stage_it_stands_at(),
        Stage::AwaitingAgentPickup,
        "and that is the one move the table allows out of `READY`"
    );
}

#[test]
fn a_picked_up_work_item_is_held_by_the_agent_who_will_do_it() {
    let the_work_item = a_work_item_waiting_to_be_picked_up();

    let picked_up =
        aisdlc_run::the_framework_picks_the_work_up(the_work_item, the_agent_who_will_do_it())
            .expect("a work item standing at `READY` may be picked up");

    assert_eq!(
        picked_up.the_agent_holding_it(),
        Some(the_agent_who_will_do_it()),
        "because `AWAITING_AGENT_PICKUP` is one of the two stages an agent must be holding, and the caller is the one who knows which agent that is"
    );
}

#[test]
fn picking_a_work_item_up_changes_nothing_else_about_it() {
    let the_work_item = a_work_item_waiting_to_be_picked_up();

    let picked_up = aisdlc_run::the_framework_picks_the_work_up(
        the_work_item.clone(),
        the_agent_who_will_do_it(),
    )
    .expect("a work item standing at `READY` may be picked up");

    assert_eq!(picked_up.who_asked(), the_work_item.who_asked());
    assert_eq!(
        picked_up.the_request_it_answers(),
        the_work_item.the_request_it_answers(),
        "a move is a move of a work item and not a new one: who asked and for what travel with it"
    );
}

#[test]
fn a_work_item_the_framework_may_not_pick_up_is_refused() {
    let already_picked_up = aisdlc_run::the_framework_picks_the_work_up(
        a_work_item_waiting_to_be_picked_up(),
        the_agent_who_will_do_it(),
    )
    .expect("a work item standing at `READY` may be picked up");

    let refusal =
        aisdlc_run::the_framework_picks_the_work_up(already_picked_up, the_agent_who_will_do_it());

    assert!(
        refusal.is_err(),
        "and it is refused rather than moved, because a work item already in front of an agent is not on the queue and picking it up again would be the framework pretending to be the queue"
    );
}

#[test]
fn the_agent_takes_the_work_item_into_its_own_hands() {
    let in_front_of_the_agent = aisdlc_run::the_framework_picks_the_work_up(
        a_work_item_waiting_to_be_picked_up(),
        the_agent_who_will_do_it(),
    )
    .expect("a work item standing at `READY` may be picked up");

    let started = aisdlc_run::the_agent_takes_the_work_item_into_its_own_hands(
        in_front_of_the_agent,
        the_agent_who_will_do_it(),
    )
    .expect("an agent may start work it has been handed");

    assert_eq!(
        started.the_stage_it_stands_at(),
        Stage::InProgressByAgent,
        "and that is the one move the table allows out of `AWAITING_AGENT_PICKUP`"
    );
}

#[test]
fn an_agent_may_not_start_work_it_was_not_handed() {
    let in_front_of_another_agent = aisdlc_run::the_framework_picks_the_work_up(
        a_work_item_waiting_to_be_picked_up(),
        the_agent_who_will_do_it(),
    )
    .expect("a work item standing at `READY` may be picked up");

    let refusal = aisdlc_run::the_agent_takes_the_work_item_into_its_own_hands(
        in_front_of_another_agent,
        a_different_agent(),
    );

    assert!(
        refusal.is_err(),
        "**the agent is the one named when the work was handed over**, so a second agent cannot begin work that is not its own — and this is the rule that would be worth a field to break if a stage only said who may hold a work item and not who does"
    );
}

#[test]
fn work_already_started_may_not_be_started_again() {
    let started = aisdlc_run::the_agent_takes_the_work_item_into_its_own_hands(
        aisdlc_run::the_framework_picks_the_work_up(
            a_work_item_waiting_to_be_picked_up(),
            the_agent_who_will_do_it(),
        )
        .expect("a work item standing at `READY` may be picked up"),
        the_agent_who_will_do_it(),
    )
    .expect("an agent may start work it has been handed");

    let refusal = aisdlc_run::the_agent_takes_the_work_item_into_its_own_hands(
        started,
        the_agent_who_will_do_it(),
    );

    assert!(
        refusal.is_err(),
        "and it is refused by the table rather than by a check here, because `IN_PROGRESS_BY_AGENT` is not among the moves out of itself"
    );
}
