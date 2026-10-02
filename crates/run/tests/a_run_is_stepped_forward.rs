//! A run is stepped forward, one stage at a time, and never over one it has not passed.
//!
//! **The order is closed and it is not read out of a set.** A run that could reach the planner
//! without a discovery report is a run that delivers work nobody decided was worth doing — **and a
//! set has no next member**, so an order read off a declaration would have been invented rather than
//! chosen.
//!
//! **A refusal ends the run and is not a step.** Refusing is a decision, **and the run that keeps
//! going after one is a run that treats a decision as a delay.**

use aisdlc_domain::stage::Stage;
use aisdlc_run::a_run_is_stepped_forward::{
    TheRunWasNotStepped, what_a_step_leads, where_a_run_goes_from_here,
};
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::where_an_approved_artifact_leads::WhereARunGoesNext;

/// A run that has just finished the discovery report and is standing at its gate.
fn a_run_at_the_discovery_gate() -> WhereARunStands {
    a_work_item_held_at_the_gate_with(Stage::AwaitingAgentPickup, "DiscoveryReport")
}

/// What the agent already holding the work is, **and it is `Some` only where the stage allows one.**
fn a_work_item_held_at_the_gate_with(
    where_it_stands: Stage,
    which_artifact: &str,
) -> WhereARunStands {
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("a name");
    let the_work_item = aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        where_it_stands,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one");

    WhereARunStands::at_the_first_stage(the_work_item, which_artifact)
}

fn the_four_facts() -> (
    aisdlc_domain::work_item_id::WorkItemId,
    aisdlc_domain::who_commissioned::WhoCommissioned,
    aisdlc_domain::repository::Repository,
    aisdlc_domain::role::Role,
) {
    let the_id = aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository = aisdlc_domain::repository::Repository::try_from(
        "steamnoid/ai-sdlc-app-rs-plus",
    )
    .expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("PO").expect("a discipline");
    (the_id, who_asked, the_repository, the_discipline)
}

#[test]
fn an_approved_artifact_sends_the_run_to_the_stage_the_router_names() {
    let the_run = a_run_at_the_discovery_gate();

    let where_it_goes = where_a_run_goes_from_here(&the_run, true);

    assert_eq!(
        where_it_goes,
        Ok(WhereARunGoesNext::TheProductAnalysis),
        "**because the destination is the router's and not this function's** — and the router is a \
         closed declaration over the artifact names, **so a run that worked the order out here would \
         be a second ordering nobody chose.** It said: {where_it_goes:?}"
    );
}

#[test]
fn a_refused_artifact_ends_the_run_and_is_not_a_step() {
    let the_run = a_run_at_the_discovery_gate();

    let where_it_goes = where_a_run_goes_from_here(&the_run, false);

    assert_eq!(
        where_it_goes,
        Ok(WhereARunGoesNext::TheRunIsOver),
        "**because refusing is a decision and not a delay** — and a run that keeps going after one is \
         a run that treats a decision as a delay. It said: {where_it_goes:?}"
    );
}

#[test]
fn a_step_hands_the_work_to_the_agent_that_answers_at_the_next_stage() {
    let the_run = a_run_at_the_discovery_gate();

    let the_next_step = what_a_step_leads(&the_run, true);

    assert_eq!(
        the_next_step.map(|one| one.who_answers_the_next_stage()),
        Ok(String::from("aisdlc-po-analyze")),
        "**because the agent and the stage move together** — a run that named the stage and left \
         the caller to guess the agent is a run with two orderings. It said: {the_next_step:?}"
    );
}

#[test]
fn a_run_that_has_reached_the_end_has_nowhere_to_go_and_says_so() {
    let the_run = WhereARunStands::before_anything(a_work_item_nobody_holds(Stage::Done));

    let where_it_goes = where_a_run_goes_from_here(&the_run, true);

    assert!(
        matches!(where_it_goes, Err(TheRunWasNotStepped::TheRunHasReachedTheEnd)),
        "**because a finished work item has nothing left to approve** — and asking where it goes next \
         is a question about a run that is over. It said: {where_it_goes:?}"
    );
}

#[test]
fn a_stage_no_agent_may_hold_is_refused_rather_than_handed_over() {
    let the_run = WhereARunStands::at_the_first_stage(
        a_work_item_nobody_holds(Stage::AwaitingHumanApproval),
        "Requirements",
    );

    let the_next_step = what_a_step_leads(&the_run, true);

    assert!(
        matches!(
            the_next_step,
            Err(TheRunWasNotStepped::TheStageNamesNoAgent)
        ),
        "**because a work item at a gate belongs to a person and not to an agent** — and handing \
         that work to an agent is the forgery the whole approval boundary exists to prevent. It \
         refused: {the_next_step:?}"
    );
}

/// A work item nobody holds, **and `DONE` and the gate are the two stages that allow it.**
fn a_work_item_nobody_holds(where_it_stands: Stage) -> aisdlc_domain::work_item::WorkItem {
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    aisdlc_domain::work_item::WorkItem::standing_at(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        where_it_stands,
    )
    .expect("a work item answering a request can stand where it is put")
}
