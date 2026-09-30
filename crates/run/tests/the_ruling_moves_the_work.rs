//! A ruling, and the move it makes in the work item's own state.
//!
//! **An approval is the only move that ends the work, and a refusal sends it back.** Both edges are
//! the table's own: `AWAITING_HUMAN_APPROVAL -> DONE` and `AWAITING_HUMAN_APPROVAL ->
//! AWAITING_AGENT_PICKUP`. **A work whose ruling was a refusal is not delivered**, and a refusal
//! that ended the work would record a delivery nobody made.
//!
//! **`DONE` is terminal and holds no agent**, so the move that gets there must leave the agent
//! behind — and a run that carried the agent along would be stopped by the invariant, which is the
//! point of an invariant.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_ruling_moves_the_work::the_ruling_moves_the_work;

/// The work the agent picked up, worked on, and is presenting.
fn an_agent_is_working_on_it(picked_up: WorkItem) -> WorkItem {
    aisdlc_domain::state_machine::transition(
        picked_up,
        Stage::InProgressByAgent,
        Some(aisdlc_run::a_run_picks_up_the_work::the_agent_that_picks_up_work()),
    )
    .expect("the table allows pickup to become work in progress")
}

/// A work item standing at the gate, waiting to be ruled on.
fn a_work_standing_at_the_gate() -> WorkItem {
    let the_work = WorkItem::standing_at(
        WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        WhoCommissioned::try_from("a person").expect("a name"),
        Repository::try_from("steamnoid/one").expect("two halves"),
        Role::try_from("PO").expect("a discipline"),
        Stage::Idle,
    )
    .expect("a work item may stand at idle");
    let picked_up =
        aisdlc_run::a_run_picks_up_the_work::a_run_picks_up_the_work_it_was_given(&the_work)
            .expect("the table allows this edge");
    let in_progress = an_agent_is_working_on_it(picked_up);
    aisdlc_run::the_agent_presents_what_it_produced(
        in_progress,
        aisdlc_run::a_run_picks_up_the_work::the_agent_that_picks_up_work(),
        &aisdlc_domain::approval::WhichArtifact::try_from("DiscoveryReport").expect("a name"),
    )
    .expect("an agent working on a work may present it")
}

#[test]
fn a_work_that_was_approved_is_delivered_and_nothing_holds_it() {
    let the_work = a_work_standing_at_the_gate();

    let what_it_became = the_ruling_moves_the_work(&the_work, true);

    assert_eq!(
        what_it_became
            .as_ref()
            .ok()
            .map(WorkItem::the_stage_it_stands_at),
        Some(Stage::Done),
        "**because an approval is the only move that ends the work** — and `DONE` is the table's \\
         own destination for it, not one this file chose: {what_it_became:?}"
    );
}

#[test]
fn a_work_that_was_refused_goes_back_rather_than_being_delivered() {
    let the_work = a_work_standing_at_the_gate();

    let what_it_became = the_ruling_moves_the_work(&the_work, false);

    assert_eq!(
        what_it_became
            .as_ref()
            .ok()
            .map(WorkItem::the_stage_it_stands_at),
        Some(Stage::AwaitingAgentPickup),
        "**because a work whose ruling was a refusal is not delivered** — and a refusal that ended \\
         the work would record a delivery nobody made: {what_it_became:?}"
    );
}

#[test]
fn a_work_that_stands_nowhere_near_the_gate_cannot_be_ruled_on() {
    let the_work = WorkItem::standing_at(
        WorkItemId::try_from("WELLBEINGT-43").expect("a name"),
        WhoCommissioned::try_from("a person").expect("a name"),
        Repository::try_from("steamnoid/one").expect("two halves"),
        Role::try_from("PO").expect("a discipline"),
        Stage::Idle,
    )
    .expect("a work item may stand at idle");

    let the_way_out = the_ruling_moves_the_work(&the_work, true);

    assert!(
        the_way_out.is_err(),
        "**because the table has no edge out of `IDLE` to anywhere a ruling could move it** — and a \\
         ruling recorded against a work nobody presented is a ruling about nothing: {the_way_out:?}"
    );
}
