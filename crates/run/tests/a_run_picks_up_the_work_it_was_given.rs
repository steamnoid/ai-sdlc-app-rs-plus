//! A run picking up the work a filing named, and no other way of doing it.
//!
//! **The work stands at `IDLE` and a work item there has not begun**, however true everything in
//! it is. The table's own edge out of `IDLE` is `AWAITING_AGENT_PICKUP`, and **that edge is what
//! makes the record a piece of work rather than a note about one** — so taking this step is not a
//! convenience, it is the first thing the state machine has to allow.
//!
//! **No other stage is reachable, and the work item is the only door.** A move composed here would
//! be a second transition table, and this repository has one.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_run_picks_up_the_work::a_run_picks_up_the_work_it_was_given;

/// A work item a run was told about, standing where work begins.
fn a_work_item_standing_at_idle() -> WorkItem {
    WorkItem::standing_at(
        WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        WhoCommissioned::try_from("a person").expect("a name"),
        Repository::try_from("steamnoid/one").expect("two halves"),
        Role::try_from("DEV").expect("a discipline"),
        Stage::Idle,
    )
    .expect("a work item may stand at idle")
}

#[test]
fn a_run_picks_up_the_work_and_it_moves_along_the_tables_own_edge() {
    let the_work = a_work_item_standing_at_idle();

    let what_it_became = a_run_picks_up_the_work_it_was_given(&the_work);

    assert_eq!(
        what_it_became
            .as_ref()
            .ok()
            .map(|it| it.the_stage_it_stands_at()),
        Some(Stage::AwaitingAgentPickup),
        "**because `IDLE -> AWAITING_AGENT_PICKUP` is the table's own edge and it is what makes the \
         record a piece of work rather than a note about one** — and a work standing at idle has not \
         begun, however true everything in it is: {what_it_became:?}"
    );
}

#[test]
fn picking_up_work_twice_is_refused_rather_than_silently_accepted() {
    let the_work = a_work_item_standing_at_idle();
    let already_picked_up = a_run_picks_up_the_work_it_was_given(&the_work).expect("one pickup");

    let the_way_out = a_run_picks_up_the_work_it_was_given(&already_picked_up);

    assert!(
        the_way_out.is_err(),
        "**because a second pickup would be a move the table does not have** — and a use case that \
         accepts it is a second transition table, and this repository has one: {the_way_out:?}"
    );
}

#[test]
fn the_work_picked_up_is_the_same_work_and_not_a_copy_with_another_name() {
    let the_work = a_work_item_standing_at_idle();

    let what_it_became = a_run_picks_up_the_work_it_was_given(&the_work).expect("a legal edge");

    assert_eq!(
        what_it_became.the_work_item_id(),
        the_work.the_work_item_id(),
        "**because a work item that changes its name on the way is two records where one was \
         meant** — and the name is the only handle anybody has on it"
    );
}
