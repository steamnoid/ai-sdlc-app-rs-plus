//! A run is walked forward, and the walk stops when there is nowhere left to go.
//!
//! **The walk decides nothing.** Every stage's outcome comes from outside it — a person or a policy
//! — **and a walk that could approve a stage of its own accord would be the run approving its own
//! work**, which is the one thing the approval boundary exists to prevent.
//!
//! **The number of stages is closed, so a walk longer than that is a loop and not a run.** A
//! repair budget bounds the *other* loop; **this one needs no budget because the router has seven
//! destinations and nothing has an eighth.**

use aisdlc_domain::stage::Stage;
use aisdlc_run::a_run_is_walked_forward::{
    AWalkThatWouldNotEnd, WhatAWalkReached, a_run_is_walked_forward, every_stage_somebody_ruled_on,
};
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::where_an_approved_artifact_leads::WhereARunGoesNext;

/// A run standing at its first stage, **and the artifact it is on.**
fn a_run_at_its_first_stage() -> WhereARunStands {
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("a name");
    let the_work_item = aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::AwaitingAgentPickup,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one");
    WhereARunStands::at_the_first_stage(the_work_item, "DiscoveryReport")
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
    let the_repository =
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("PO").expect("a discipline");
    (the_id, who_asked, the_repository, the_discipline)
}

#[test]
fn a_run_that_somebody_refused_is_not_walked_again() {
    let the_walk = a_run_is_walked_forward(&a_run_at_its_first_stage(), &[false]);

    assert_eq!(
        the_walk.as_ref().map(WhatAWalkReached::how_far_it_got),
        Ok(0),
        "**because refusing is a decision and the walk stops on the first one** — and a walk that \
         asked again after a refusal is a run treating a decision as a delay. It walked {:?}",
        the_walk.as_ref().map(WhatAWalkReached::how_far_it_got)
    );
}

#[test]
fn a_run_is_walked_once_per_ruling_and_no_further() {
    let the_walk = a_run_is_walked_forward(&a_run_at_its_first_stage(), &[true]);

    let where_it_stopped = the_walk
        .as_ref()
        .map(WhatAWalkReached::every_place_it_stopped_at);
    assert_eq!(
        where_it_stopped,
        Ok(vec![WhereARunGoesNext::TheProductAnalysis]),
        "**because one ruling is one step** — and a walk that kept going on a single yes would have \
         walked a run through seven stages with one person having read one document. It stopped at: \
         {where_it_stopped:?}"
    );
}

#[test]
fn a_walked_run_is_handed_to_the_agent_at_each_stage_in_turn() {
    let the_walk = a_run_is_walked_forward(&a_run_at_its_first_stage(), &[true, true]);

    let how_far_it_got = the_walk.as_ref().map(WhatAWalkReached::how_far_it_got);
    assert_eq!(
        how_far_it_got,
        Ok(2),
        "**because a walk that reports where it went without moving the work is a route printed on \
         paper** — and this is the test that says a walk walks. It reached {how_far_it_got:?} stages"
    );

    let where_the_work_ended_up = the_walk
        .as_ref()
        .map(WhatAWalkReached::where_the_work_ended_up);
    assert_eq!(
        where_the_work_ended_up.map(WhereARunStands::the_stage_it_stands_at),
        Ok(Stage::InProgressByAgent),
        "**because the second stage's agent is holding the work and not merely named** — a run that \
         reported a route and left the work at the first stage has moved nothing. It stands at \
         {where_the_work_ended_up:?}"
    );
}

#[test]
fn the_artifact_a_walked_run_is_on_advances_with_the_stage_that_produced_it() {
    let the_walk = a_run_is_walked_forward(&a_run_at_its_first_stage(), &[true, true]);

    let which_artifact = the_walk.as_ref().map(|walk| {
        walk.where_the_work_ended_up()
            .which_artifact_it_is_on()
            .cloned()
    });
    assert_eq!(
        which_artifact,
        Ok(Some(
            aisdlc_domain::approval::WhichArtifact::try_from("QaPlan")
                .expect("a name is not blank")
        )),
        "**because a run still standing on `DiscoveryReport` after two stages would route back to \
         the product analysis on the third** — and a walk that loops on one artifact is the router \
         defect that cost the sibling a run. It stands on {which_artifact:?}"
    );
}

#[test]
fn more_rulings_than_there_are_stages_is_a_loop_and_is_refused() {
    let too_many = vec![true; 9];

    let the_refusal = a_run_is_walked_forward(&a_run_at_its_first_stage(), &too_many);

    assert!(
        matches!(
            the_refusal,
            Err(AWalkThatWouldNotEnd::MoreRulingsThanStages { .. })
        ),
        "**because the router has seven destinations and nothing has an eighth** — so a ninth \
         ruling is not a run, it is a loop, **and a refusal that named it as one would send whoever \
         reads it after the repeat budget instead of after the router.** It refused: {the_refusal:?}"
    );
}

#[test]
fn every_stage_somebody_ruled_on_is_the_whole_of_the_pipeline_and_is_a_const() {
    assert_eq!(
        every_stage_somebody_ruled_on().len(),
        7,
        "**because seven artifacts and seven gates is the whole of it** — and the number is a \
         constant rather than a count of something at runtime, **so a pipeline that grew a stage \
         would fail this test instead of quietly walking further than the guard expects.**"
    );
}
