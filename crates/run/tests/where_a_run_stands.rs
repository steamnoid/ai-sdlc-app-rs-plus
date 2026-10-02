//! Where a run stands, and the one rule about who may move a work item.
//!
//! **A run is a state and a state is one value.** Eight stages, seven gates, a suite that ran twice
//! — **and none of it is any use unless a reader can ask a run where it is and be told one thing.**
//!
//! **One writer of the work item, and that is the whole rule.** The sibling implementation's first
//! version used the idempotent commissioning writer for a move, **so a register froze at
//! `AWAITING_AGENT_PICKUP` while a real run went through eight gates and opened a pull request** —
//! and no test saw it, because the unit tests only made one move and the whole-run test never read
//! the stage back. **Recording a move and commissioning are two functions and one function
//! answering both could only answer one.**

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::where_a_run_stands::{
    WhereARunStands, the_work_is_handed_to_an_agent, the_work_is_presented_for_a_ruling,
};

/// The four facts every work item in this file carries, **as closures so a fixture reads as one
/// line and a reader does not have to hold six arguments.**
fn the_four_facts() -> (WorkItemId, WhoCommissioned, Repository, Role) {
    let the_id = WorkItemId::try_from("WELLBEINGT-5").expect("a name");
    let who_asked = WhoCommissioned::try_from("a person").expect("a name");
    let the_repository = Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves");
    let the_discipline = Role::try_from("PO").expect("a discipline");
    (the_id, who_asked, the_repository, the_discipline)
}

/// A work item standing where it is put, **and held by nobody — so the caller picks a stage no
/// agent may hold.**
fn a_work_item_nobody_holds(where_it_stands: Stage) -> WorkItem {
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    WorkItem::standing_at(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        where_it_stands,
    )
    .expect("a work item answering a request can stand where it is put")
}

/// A work item standing where it is put **and held**, **because `AWAITING_AGENT_PICKUP` requires an
/// agent and the domain refuses the door otherwise.**
fn a_work_item_an_agent_holds(where_it_stands: Stage, who_holds_it: &str) -> WorkItem {
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    let the_agent = AgentIdentifier::try_from(who_holds_it).expect("a name");
    WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        where_it_stands,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one")
}

#[test]
fn a_run_that_has_begun_says_which_artifact_it_is_working_on() {
    let the_run = WhereARunStands::at_the_first_stage(
        a_work_item_an_agent_holds(Stage::AwaitingAgentPickup, "aisdlc-discover"),
        "DiscoveryReport",
    );

    assert_eq!(
        the_run.which_artifact_it_is_on().map(|one| one.0.clone()),
        Some(String::from("DiscoveryReport")),
        "**because the artifact is the only thing that says which gate this run is at** — and a \
         reader asking a run where it is wants one answer, not a stage and a guess. It said: {}",
        the_run
            .which_artifact_it_is_on()
            .map(|one| one.0.clone())
            .unwrap_or_default()
    );
}

#[test]
fn handing_the_work_over_moves_it_and_says_who_holds_it_now() {
    let the_run = WhereARunStands::at_the_first_stage(
        a_work_item_an_agent_holds(Stage::AwaitingAgentPickup, "aisdlc-discover"),
        "DiscoveryReport",
    );
    let an_agent = AgentIdentifier::try_from("aisdlc-discover").expect("a name");

    let after_the_handover = the_work_is_handed_to_an_agent(&the_run, an_agent.clone())
        .expect("the table allows a handover");

    assert_eq!(
        (
            after_the_handover.the_stage_it_stands_at(),
            after_the_handover.who_holds_it()
        ),
        (Stage::InProgressByAgent, Some(an_agent)),
        "**because the handover and the move are one fact and not two** — an agent holding work \
         that is still standing at `AWAITING_AGENT_PICKUP` is a state the table forbids, and \
         checking it in two places would be two places to forget. It stands at {} held by {:?}",
        after_the_handover.the_stage_it_stands_at(),
        after_the_handover.who_holds_it()
    );
}

#[test]
fn the_run_before_its_first_stage_says_so_and_names_no_artifact() {
    let the_run = WhereARunStands::before_anything(a_work_item_nobody_holds(Stage::Idle));

    assert_eq!(
        the_run.which_artifact_it_is_on().map(|one| one.0.clone()),
        None,
        "**because a run that has not reached an artifact has not reached a gate** — and naming \
         one anyway would put a name in a record for work nobody has presented. It said: {:?}",
        the_run.which_artifact_it_is_on().map(|one| one.0.clone())
    );
}

#[test]
fn presenting_the_work_moves_it_to_the_gate_and_lets_go_of_the_agent() {
    let the_run = the_work_is_handed_to_an_agent(
        &WhereARunStands::at_the_first_stage(
            a_work_item_an_agent_holds(Stage::AwaitingAgentPickup, "aisdlc-discover"),
            "DiscoveryReport",
        ),
        AgentIdentifier::try_from("aisdlc-discover").expect("a name"),
    )
    .expect("the table allows a handover");

    let at_the_gate =
        the_work_is_presented_for_a_ruling(&the_run).expect("the table allows a gate");

    assert_eq!(
        (
            at_the_gate.the_stage_it_stands_at(),
            at_the_gate.who_holds_it()
        ),
        (Stage::AwaitingHumanApproval, None),
        "**because the agent has let go the moment the work reaches a person** — a work item at a \
         gate with an agent still holding it is a state the invariant forbids, **and the refusal \
         would come from a check that reads a field nobody set.** It stands at {} held by {:?}",
        at_the_gate.the_stage_it_stands_at(),
        at_the_gate.who_holds_it()
    );
}

#[test]
fn a_handover_is_refused_when_the_table_forbids_that_move() {
    let the_run = WhereARunStands::at_the_first_stage(
        a_work_item_an_agent_holds(Stage::AwaitingAgentPickup, "aisdlc-discover"),
        "DiscoveryReport",
    );

    let the_refusal = the_work_is_handed_to_an_agent(
        &the_run,
        AgentIdentifier::try_from("aisdlc-discover").expect("a name"),
    )
    .and_then(|held| {
        the_work_is_handed_to_an_agent(
            &held,
            AgentIdentifier::try_from("aisdlc-dev-impl").expect("a name"),
        )
    });

    assert!(
        the_refusal.is_err(),
        "**because two agents picking up the same work at once is a state this table does not \
         describe** — and a lifecycle that wrote the field directly would have produced it. It \
         refused: {the_refusal:?}"
    );
}
