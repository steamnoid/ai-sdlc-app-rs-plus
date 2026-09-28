//! Three doors, and the two positions nobody may put a work item in.
//!
//! **A work item has private fields, so these three are the only ways to build one**, and
//! every one of them validates. An earlier shape had a constructor that did not, which meant
//! a work item could be built at a stage that needs an agent and hold nobody — and a
//! checkpoint carrying exactly that was read back as a real work item.
//!
//! **One refusal type for all three doors**, and these tests are the reason that is not a
//! loss: a caller told "this cannot exist" has one question, and three types would make it
//! choose which one to ask. `the_two_illegal_positions_are_one_refusal_not_three` is the
//! test that would fail if a fourth door grew a type of its own.
//!
//! No file is read here, because a test of the domain may not reach for a filesystem.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::{AWorkItemThatNamesNobody, WhoCommissioned};
use aisdlc_domain::work_item::{AnUncheckedWorkItem, WorkItem};
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
        "GET /health".to_string(),
    )
    .expect("a title says something")
}

fn an_agent() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-dev-impl").expect("a name is not an identity")
}

fn held_at(a_stage: Stage) -> Result<WorkItem, AnUncheckedWorkItem> {
    WorkItem::held_by(
        a_name(),
        whoever_asked(),
        what_it_is_worked_on(),
        Role::Development,
        a_stage,
        an_agent(),
    )
}

fn standing_at(a_stage: Stage) -> Result<WorkItem, AnUncheckedWorkItem> {
    WorkItem::standing_at(
        a_name(),
        whoever_asked(),
        what_it_is_worked_on(),
        Role::Development,
        a_stage,
    )
}

#[test]
fn who_asked_cannot_be_nobody() {
    for nobody in ["", "   ", "\t"] {
        let result = WhoCommissioned::try_from(nobody);

        assert_eq!(
            result,
            Err(AWorkItemThatNamesNobody(nobody.to_string())),
            "work with no origin cannot be traced back to the request that started it, and {nobody:?} is no origin"
        );
    }
}

#[test]
fn whoever_asked_is_kept_exactly_as_it_was_named() {
    let asked = WhoCommissioned::try_from("  paligakrzychu  ").expect("a name");

    assert_eq!(
        asked.to_string(),
        "  paligakrzychu  ",
        "a name is kept as typed"
    );
}

#[test]
fn a_policy_may_ask_on_somebodys_behalf_and_the_name_is_kept_verbatim() {
    let a_policy = WhoCommissioned::try_from("aisdlc-autopilot:every-gate").expect("a name");

    assert_eq!(
        a_policy.to_string(),
        "aisdlc-autopilot:every-gate",
        "an aisdlc- identifier means a policy asked on somebody's behalf, and the name is not rewritten to hide that"
    );
}

#[test]
fn a_work_item_stands_where_nobody_may_hold_it() {
    for a_stage in [
        Stage::Idle,
        Stage::AwaitingHumanApproval,
        Stage::Ready,
        Stage::Done,
    ] {
        let built = standing_at(a_stage);

        assert!(
            built.is_ok(),
            "{a_stage} is a stage nobody may hold: {:?}",
            built.err()
        );
    }
}

#[test]
fn a_work_item_standing_at_a_stage_needing_an_agent_is_refused() {
    for a_stage in [Stage::AwaitingAgentPickup, Stage::InProgressByAgent] {
        let built = standing_at(a_stage);

        assert!(
            built.is_err(),
            "{a_stage} needs an agent and this door holds nobody, so a work item here would be standing where nobody is working"
        );
    }
}

#[test]
fn a_work_item_held_at_a_stage_nobody_may_hold_is_refused() {
    for a_stage in [
        Stage::Idle,
        Stage::AwaitingHumanApproval,
        Stage::Ready,
        Stage::Done,
    ] {
        let held = held_at(a_stage);

        assert!(
            held.is_err(),
            "{a_stage} is a stage nobody may hold, so an agent holding one is a position the table does not have"
        );
    }
}

#[test]
fn a_work_item_is_held_where_the_table_says_an_agent_may_hold_it() {
    for a_stage in [Stage::AwaitingAgentPickup, Stage::InProgressByAgent] {
        let held = held_at(a_stage);

        assert!(
            held.is_ok(),
            "{a_stage} is a stage an agent may hold: {:?}",
            held.err()
        );
    }
}

#[test]
fn a_request_is_a_fact_and_not_a_position_so_it_refuses_nothing() {
    for a_stage in [
        Stage::Idle,
        Stage::AwaitingHumanApproval,
        Stage::Ready,
        Stage::Done,
    ] {
        let answered = standing_at(a_stage)
            .expect("nobody may hold it")
            .answering(what_was_asked());

        assert!(
            answered.is_ok(),
            "{a_stage} takes a request whatever it is: {:?}",
            answered.err()
        );
    }
}

#[test]
fn the_two_illegal_positions_are_one_refusal_not_three() {
    let standing_where_a_stage_needs_one =
        standing_at(Stage::AwaitingAgentPickup).expect_err("this door holds nobody");
    let held_where_nobody_may = held_at(Stage::Done).expect_err("nobody may hold DONE");

    let both_take_the_one_refusal: fn(&AnUncheckedWorkItem) -> String = |one| one.why.clone();

    assert_eq!(
        (
            both_take_the_one_refusal(&standing_where_a_stage_needs_one),
            both_take_the_one_refusal(&held_where_nobody_may),
        ),
        (
            standing_where_a_stage_needs_one.why.clone(),
            held_where_nobody_may.why.clone(),
        ),
        "one refusal type for all three doors, and the compiler holds this one: `expect_err` on a Result<_, AnUncheckedWorkItem> is what let this file bind both of them"
    );
    assert_ne!(
        standing_where_a_stage_needs_one.why, held_where_nobody_may.why,
        "and one type does not mean one message: these are two different positions"
    );
}

#[test]
fn a_refusal_says_which_stage_was_impossible() {
    let refused = standing_at(Stage::AwaitingAgentPickup).expect_err("this door holds nobody");

    let said = refused.to_string();

    assert!(
        said.contains("AWAITING_AGENT_PICKUP"),
        "a refusal a reader cannot match against the stage they named teaches nothing: {said}"
    );
}
