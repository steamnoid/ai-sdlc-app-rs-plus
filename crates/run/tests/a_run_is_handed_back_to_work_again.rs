//! A run is handed back to work again, because a model said so and the ceiling allowed it.
//!
//! **The model decides and the code routes.** The answer names one of two places; **the table that
//! sends the work is a literal here**, because a value read out of the model would be a model
//! choosing its own destination — the one thing in this pipeline declared as code rather than
//! reasoned about.
//!
//! **The budget is checked against the setting and not against the model, and its refusal is its
//! own name.** A run that reported both this and a judgement as *the repair failed* sends whoever
//! reads the trace after the wrong one.

use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::stage::Stage;
use aisdlc_run::a_run_is_put_back_to_work_again::{HowManyTimesARunMayBePutBack, the_repeat_budget};
use aisdlc_run::a_run_is_handed_back_to_work_again::{
    TheRepairCouldNotHappen, a_run_is_handed_back_to_work_again,
};
use aisdlc_run::what_to_repeat::{
    TheVerdict, WhatToRepeat, WhereTheRunShouldStartAgain, the_agent_asked_to_do_it_again,
};
use aisdlc_run::where_a_run_stands::WhereARunStands;

/// A run at the stage the failure happened at, **and the artifact it was on.**
fn a_run_where_a_failure_happened() -> WhereARunStands {
    let the_id = aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("DEV").expect("a discipline");
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-dev-impl")
        .expect("an agent name");
    let the_work_item = aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::InProgressByAgent,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one");
    WhereARunStands::at_the_first_stage(the_work_item, "VerificationReport")
}

/// What a supervisor said, **and the ordinary answer.**
fn a_supervisors_answer(from_which_place: WhereTheRunShouldStartAgain) -> WhatToRepeat {
    WhatToRepeat {
        what_each_agent_must_do: vec![TheVerdict {
            who_must_act: the_agent_asked_to_do_it_again(from_which_place),
            what_they_must_do: String::from("write the change again and run the suite"),
            why_the_last_time_was_not_enough: String::from("the suite went red after the change"),
        }],
        where_the_run_starts_again: from_which_place,
        what_has_not_changed: String::from("the same test was never run"),
    }
}

#[test]
fn a_supervisors_answer_hands_the_run_back_to_the_agent_at_the_place_it_named() {
    for from_which_place in WhereTheRunShouldStartAgain::every_place() {
        let handed_back = a_run_is_handed_back_to_work_again(
            &a_run_where_a_failure_happened(),
            1,
            &the_repeat_budget(Some("3")).expect("a number"),
            &a_supervisors_answer(from_which_place),
        );

        assert_eq!(
            handed_back.as_ref().map(WhereARunStands::who_holds_it),
            Ok(Some(the_agent_asked_to_do_it_again(from_which_place))),
            "**because every place in the closed set must route, and not only the first one** — a \
             node proved against one row is a node proved against the row somebody tested. It said: \
             {handed_back:?}"
        );
    }
}

#[test]
fn the_artifact_a_handed_back_run_is_on_is_what_that_stage_produces_again() {
    let handed_back = a_run_is_handed_back_to_work_again(
        &a_run_where_a_failure_happened(),
        1,
        &the_repeat_budget(Some("3")).expect("a number"),
        &a_supervisors_answer(WhereTheRunShouldStartAgain::TheChangeIsPlannedAgain),
    );

    let which_artifact = handed_back
        .as_ref()
        .map(|run| run.which_artifact_it_is_on().cloned());
    assert_eq!(
        which_artifact,
        Ok(Some(WhichArtifact::try_from("ImplementationPlan").expect("a name is not blank"))),
        "**because the planner produces the plan and nothing else** — a run handed back to the \
         planner while still standing on `VerificationReport` would route onwards into the \
         verification it just failed. It stands on {which_artifact:?}"
    );
}

#[test]
fn a_repetition_the_budget_does_not_permit_is_refused_as_the_budget_running_out() {
    let the_budget: HowManyTimesARunMayBePutBack = the_repeat_budget(Some("2")).expect("a number");

    let refused = a_run_is_handed_back_to_work_again(
        &a_run_where_a_failure_happened(),
        3,
        &the_budget,
        &a_supervisors_answer(WhereTheRunShouldStartAgain::TheChangeIsWrittenAgain),
    );

    assert!(
        matches!(refused, Err(TheRepairCouldNotHappen::TheRepeatBudgetRanOut { .. })),
        "**because the ceiling is a fact about the setting and not about the model** — and a run \
         that reported this as *the supervisor judged it will not change* sends whoever reads the \
         trace after a model that was never asked. It refused: {refused:?}"
    );
}

#[test]
fn a_repetition_the_budget_does_permit_is_handed_back_even_at_the_boundary() {
    let the_budget: HowManyTimesARunMayBePutBack = the_repeat_budget(Some("2")).expect("a number");

    let handed_back = a_run_is_handed_back_to_work_again(
        &a_run_where_a_failure_happened(),
        2,
        &the_budget,
        &a_supervisors_answer(WhereTheRunShouldStartAgain::TheChangeIsWrittenAgain),
    );

    assert!(
        handed_back.is_ok(),
        "**because a check refused at `>=` would buy one fewer repair than the deployment asked \
         for and would not say so** — the boundary belongs to the setting and not to this \
         function. It said: {handed_back:?}"
    );
}

#[test]
fn a_run_standing_nowhere_a_failure_happened_is_refused_rather_than_handed_back() {
    let the_id = aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("PO").expect("a discipline");
    let at_a_gate = aisdlc_domain::work_item::WorkItem::standing_at(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::AwaitingHumanApproval,
    )
    .expect("a work item answering a request can stand where it is put");

    let refused = a_run_is_handed_back_to_work_again(
        &WhereARunStands::before_anything(at_a_gate),
        1,
        &the_repeat_budget(Some("3")).expect("a number"),
        &a_supervisors_answer(WhereTheRunShouldStartAgain::TheChangeIsWrittenAgain),
    );

    assert!(
        matches!(
            refused,
            Err(TheRepairCouldNotHappen::TheRunIsNotStandingWhereAFailureHappened { .. })
        ),
        "**because a work item at a gate belongs to a person and not to an agent** — handing it to \
         one is the forgery the whole approval boundary exists to prevent, **and this refusal is \
         the walk's and the domain's said in the walk's own file.** It refused: {refused:?}"
    );
}
