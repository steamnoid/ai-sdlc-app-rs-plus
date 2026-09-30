//! The question put to the gate, and the thing that was presented.
//!
//! **`the_agent_presents_what_it_produced` takes the artifact and throws it away.** It moves the
//! work to the gate and returns a work item that does not say what it is presenting, and
//! `the_question_put_to_somebody` then takes the artifact **again, from the caller** — so the
//! thing approved and the thing ruled on are two arguments a caller has to keep equal by hand, and
//! **nothing checks that they ever were.**
//!
//! **One use case, so there is nothing to keep equal.** Presenting and asking are one act, and the
//! question carries the artifact because the use case was handed the same value it presented —
//! **a gate that can be asked about something other than what was put in front of it is a gate
//! whose approval is about an artifact nobody saw.**

use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_run_picks_up_the_work::a_run_picks_up_the_work_it_was_given;
use aisdlc_run::what_an_agent_has_produced::WhatAnAgentHasProduced;
use aisdlc_run::what_the_agent_presents::an_agent_presents_what_it_produced_and_the_question_is_put;

/// A work item a run picked up and is working on.
fn a_work_an_agent_is_working_on() -> WorkItem {
    let the_work = WorkItem::standing_at(
        WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        WhoCommissioned::try_from("a person").expect("a name"),
        Repository::try_from("steamnoid/one").expect("two halves"),
        Role::try_from("PO").expect("a discipline"),
        Stage::Idle,
    )
    .expect("a work item may stand at idle");
    let the_work =
        a_run_picks_up_the_work_it_was_given(&the_work).expect("the table allows this edge");
    aisdlc_domain::state_machine::transition(
        the_work,
        aisdlc_domain::stage::Stage::InProgressByAgent,
        Some(aisdlc_run::a_run_picks_up_the_work::the_agent_that_picks_up_work()),
    )
    .expect("the table allows pickup to become work in progress")
}

#[test]
fn what_is_presented_is_exactly_what_the_gate_is_asked_about() {
    let the_work = a_work_an_agent_is_working_on();
    let what_it_produced =
        WhatAnAgentHasProduced::new("DiscoveryReport", 1).expect("declared here");

    let the_question = an_agent_presents_what_it_produced_and_the_question_is_put(
        &the_work,
        &what_it_produced,
    )
    .expect(
        "**and the table does not have an edge out of `AWAITING_AGENT_PICKUP` straight to the \
         gate** — an agent presents from `IN_PROGRESS_BY_AGENT`, and a work standing at pickup has \
         not been worked on at all",
    );

    assert_eq!(
        (
            the_question.about_which_artifact().0.as_str(),
            the_question.about_which_version().0
        ),
        ("DiscoveryReport", 1),
        "**because presenting and asking are one act** — and the artifact the gate is asked about \
         came out of the same value that was presented, so **a gate cannot be asked about something \
         other than what was put in front of it**: {the_question:?}"
    );
}

#[test]
fn a_second_version_is_a_second_thing_on_the_table_and_not_the_first_one() {
    let the_work = a_work_an_agent_is_working_on();
    let the_first = WhatAnAgentHasProduced::new("DiscoveryReport", 1).expect("declared here");
    let the_second = WhatAnAgentHasProduced::new("DiscoveryReport", 2).expect("declared here");

    let asked_about_the_first =
        an_agent_presents_what_it_produced_and_the_question_is_put(&the_work, &the_first)
            .expect("an agent working on a work may present it");
    let asked_about_the_second =
        an_agent_presents_what_it_produced_and_the_question_is_put(&the_work, &the_second)
            .expect("an agent working on a work may present it");

    assert_ne!(
        (
            asked_about_the_first.about_which_version().0,
            asked_about_the_second.about_which_version().0
        ),
        (0, 0),
        "**because a version is what makes an approval go stale** — and a gate that cannot tell \
         two versions of the same artifact apart cannot tell that the thing approved has changed: \
         {asked_about_the_first:?} / {asked_about_the_second:?}"
    );
}

#[test]
fn a_work_not_picked_up_cannot_present_anything() {
    let the_work = WorkItem::standing_at(
        WorkItemId::try_from("WELLBEINGT-43").expect("a name"),
        WhoCommissioned::try_from("a person").expect("a name"),
        Repository::try_from("steamnoid/one").expect("two halves"),
        Role::try_from("PO").expect("a discipline"),
        Stage::Idle,
    )
    .expect("a work item may stand at idle");
    let what_it_produced =
        WhatAnAgentHasProduced::new("DiscoveryReport", 1).expect("declared here");

    let the_way_out =
        an_agent_presents_what_it_produced_and_the_question_is_put(&the_work, &what_it_produced);

    assert!(
        the_way_out.is_err(),
        "**because a work standing at `IDLE` has not begun, however true everything in it is** — \
         and the table does not have an edge out of `IDLE` straight to the gate, so presenting from \
         there is a move this system will not make: {the_way_out:?}"
    );
}
