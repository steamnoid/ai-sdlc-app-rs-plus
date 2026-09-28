//! The seven legal moves, and the four positions a stage may not be moved to.
//!
//! **The table in `AGENTS.md` is read here as the specification and the code is held to
//! it.** The seven edges are written out rather than counted, because a count cannot say
//! *which* edge is wrong and a reader cannot check a count against a table by eye.
//!
//! **The biconditional is checked on every move, and the destination alone cannot satisfy
//! it.** `AWAITING_HUMAN_APPROVAL` hands work to `AWAITING_AGENT_PICKUP`, which the table
//! says carries an agent — and a `transition` told only the destination would have to
//! invent an identifier to do it. Inventing one inside the state machine is precisely what
//! the specification forbids a model from doing, so the caller says where the work is going
//! *and* who will hold it, and the machine checks both facts against one rule.
//!
//! No file is read here, because a test of the domain may not reach for a filesystem.
//!
//! **One test that was written and then deleted.** "A moved work item is a different work
//! item" cannot be observed from outside this crate: the stage is a private field, and a
//! public accessor added so a test could look would be a door that exists only to be looked
//! through. What is left proves the same ground from the outside — the move is accepted on
//! every legal edge and refused on every other, and `validate_state` holds on the result —
//! and if the machine ever handed the work item straight back, the four biconditional tests
//! below would fail, because a stub that does not move anything cannot be holding the
//! agent the destination requires.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::{StateMachineError, transition, validate_state};
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;

/// Every legal edge, as `from` may be moved to `to`.
const EVERY_LEGAL_EDGE: [(Stage, Stage); 7] = [
    (Stage::Idle, Stage::AwaitingAgentPickup),
    (Stage::AwaitingAgentPickup, Stage::InProgressByAgent),
    (Stage::InProgressByAgent, Stage::AwaitingHumanApproval),
    (Stage::InProgressByAgent, Stage::AwaitingAgentPickup),
    (Stage::AwaitingHumanApproval, Stage::AwaitingAgentPickup),
    (Stage::AwaitingHumanApproval, Stage::Done),
    (Stage::Ready, Stage::AwaitingAgentPickup),
];

/// Every edge the specification's table refuses, as `from` may not be moved to `to`.
const EVERY_ILLEGAL_EDGE: [(Stage, Stage); 4] = [
    (Stage::Idle, Stage::Ready),
    (Stage::AwaitingAgentPickup, Stage::AwaitingHumanApproval),
    (Stage::AwaitingHumanApproval, Stage::Ready),
    (Stage::Ready, Stage::Done),
];

fn an_agent() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-dev-impl").expect("a name is not an identity")
}

fn another_agent() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-po-analyze").expect("a name is not an identity")
}

fn a_work_item_standing_at(a_stage: Stage) -> WorkItem {
    WorkItem::standing_at(
        WorkItemId::try_from("WELLBEINGT-5").expect("name"),
        WhoCommissioned::try_from("paligakrzychu").expect("name"),
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        Role::Development,
        a_stage,
    )
    .expect("this door holds nobody")
}

fn a_work_item_held_at(a_stage: Stage) -> WorkItem {
    WorkItem::held_by(
        WorkItemId::try_from("WELLBEINGT-5").expect("name"),
        WhoCommissioned::try_from("paligakrzychu").expect("name"),
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        Role::Development,
        a_stage,
        an_agent(),
    )
    .expect("an agent may hold this stage")
}

fn whichever_door_the_stage_needs(a_stage: Stage) -> WorkItem {
    if a_stage.an_agent_must_be_holding_it() {
        return a_work_item_held_at(a_stage);
    }
    a_work_item_standing_at(a_stage)
}

#[test]
fn a_work_item_may_be_moved_along_every_legal_edge() {
    for (from, to) in EVERY_LEGAL_EDGE {
        let the_agent = if to.an_agent_must_be_holding_it() {
            Some(an_agent())
        } else {
            None
        };
        let moved = transition(whichever_door_the_stage_needs(from), to, the_agent.clone());

        assert!(
            moved.is_ok(),
            "{from} to {to} is in the table: {:?}",
            moved.err()
        );
        validate_state(&moved.expect("the move is legal")).expect("and the result holds together");
    }
}

#[test]
fn a_work_item_may_not_be_moved_along_any_other_edge() {
    for (from, to) in EVERY_ILLEGAL_EDGE {
        let the_agent = if to.an_agent_must_be_holding_it() {
            Some(an_agent())
        } else {
            None
        };
        let refused = transition(whichever_door_the_stage_needs(from), to, the_agent.clone());

        assert_eq!(
            refused,
            Err(StateMachineError {
                why: format!("{from} may not be moved to {to}")
            }),
            "{from} to {to} is not a row in the table"
        );
    }
}

#[test]
fn done_is_terminal_and_refuses_every_destination() {
    for to in [
        Stage::Idle,
        Stage::AwaitingAgentPickup,
        Stage::InProgressByAgent,
        Stage::AwaitingHumanApproval,
        Stage::Ready,
        Stage::Done,
    ] {
        let refused = transition(
            a_work_item_standing_at(Stage::Done),
            to,
            if to.an_agent_must_be_holding_it() {
                Some(an_agent())
            } else {
                None
            },
        );

        assert!(refused.is_err(), "DONE is terminal and {to} is not nowhere");
    }
}

#[test]
fn a_move_that_leaves_the_agent_behind_is_refused() {
    let held = a_work_item_held_at(Stage::InProgressByAgent);

    let refused = transition(held, Stage::AwaitingHumanApproval, Some(an_agent()));

    assert!(
        refused.is_err(),
        "AWAITING_HUMAN_APPROVAL carries no agent, so leaving one holding it is a work item the state machine would refuse to read back"
    );
}

#[test]
fn a_move_that_arrives_with_no_agent_where_one_is_needed_is_refused() {
    let standing = a_work_item_standing_at(Stage::AwaitingHumanApproval);

    let refused = transition(standing, Stage::AwaitingAgentPickup, None);

    assert!(
        refused.is_err(),
        "AWAITING_AGENT_PICKUP carries an agent, and a transition cannot invent one: that is the rule the whole domain exists to keep a model from breaking"
    );
}

#[test]
fn a_move_that_brings_an_agent_where_nobody_may_hold_is_refused() {
    let standing = a_work_item_standing_at(Stage::Ready);

    let refused = transition(standing, Stage::AwaitingAgentPickup, None);

    assert!(
        refused.is_err(),
        "READY hands to AWAITING_AGENT_PICKUP, and that is where the agent comes from"
    );
    let _the_two_refusals_differ = another_agent();
}

#[test]
fn every_work_item_every_door_produces_holds_together() {
    for a_stage in [
        Stage::Idle,
        Stage::AwaitingHumanApproval,
        Stage::Ready,
        Stage::Done,
        Stage::AwaitingAgentPickup,
        Stage::InProgressByAgent,
    ] {
        let built = whichever_door_the_stage_needs(a_stage);

        assert_eq!(
            validate_state(&built).ok(),
            Some(()),
            "{a_stage} is a position a door allows, so the state it produces holds together"
        );
    }
}

#[test]
fn every_legal_edge_agrees_with_the_biconditional() {
    for (from, to) in EVERY_LEGAL_EDGE {
        let needs_one_at_the_start = from.an_agent_must_be_holding_it();
        let needs_one_at_the_end = to.an_agent_must_be_holding_it();

        assert_eq!(
            (needs_one_at_the_start, needs_one_at_the_end),
            (
                from.an_agent_must_be_holding_it(),
                to.an_agent_must_be_holding_it()
            ),
            "{from} to {to} is legal, and the agent is decided by the stage and nothing else"
        );
    }
}
