//! A run is put back to work again, and the two ways that stops are not the same event.
//!
//! **A model has decided, and this is the code that obeys.** `WhatToRepeat` says which agent and
//! which of two places; `the_agent_asked_to_do_it_again` is the literal that sends the work back —
//! **and this is where the ceiling and the judgement meet.**
//!
//! **Two ways the loop ends, and conflating them sends whoever reads the trace to the wrong one.** A
//! model that judges the failure will not change has *decided*, and a run that asked for more turns
//! than the ceiling permits has *run out of them* — and those are facts about the model and about
//! the setting respectively.

use aisdlc_run::a_run_is_put_back_to_work_again::{
    ARepeatThatWasNotPermitted, HowManyTimesARunMayBePutBack, TheRepairCouldNotHappen,
    is_this_repeat_permitted, the_repeat_budget,
};

/// A budget a deployment named, **and every deployment may name one.**
fn a_budget(what_a_deployment_named: Option<&str>) -> HowManyTimesARunMayBePutBack {
    the_repeat_budget(what_a_deployment_named).expect("a number is a budget")
}

#[test]
fn a_deployment_that_named_no_budget_gets_a_number_and_not_an_eternity() {
    let the_budget = a_budget(None);

    assert!(
        the_budget.may_be_put_back_this_many_times() > 0,
        "**because a ceiling nobody noticed and an answer nobody asked for are the same failure \
         wearing different names** — so this defaults to a number and not to *unlimited*. It \
         defaulted to: {}",
        the_budget.may_be_put_back_this_many_times()
    );
}

#[test]
fn a_budget_of_zero_refuses_the_first_repeat() {
    let the_budget = a_budget(Some("0"));

    let the_verdict = is_this_repeat_permitted(&the_budget, 1);

    assert_eq!(
        the_verdict,
        Err(ARepeatThatWasNotPermitted::TheRepeatBudgetRanOut {
            how_many_were_asked_for: 1,
            how_many_were_permitted: 0,
        }),
        "**because a budget of zero is a run that may not be put back at all, and that is a \
         deployment saying so** — it is not a failure of anything, and a refusal that says *budget* \
         says to whoever reads the trace where to look. It refused: {the_verdict:?}"
    );
}

#[test]
fn a_repeat_inside_the_budget_is_permitted_and_one_past_it_is_not() {
    let the_budget = a_budget(Some("2"));

    let inside = is_this_repeat_permitted(&the_budget, 2);
    let past = is_this_repeat_permitted(&the_budget, 3);

    assert_eq!(
        (inside.is_ok(), past.is_err()),
        (true, true),
        "**because a budget of two permits two repetitions and not three**, and the boundary is \
         where the arithmetic is — **a check that refused at `>=` would buy one fewer repair than a \
         deployment asked for and would not say so.** Inside: {inside:?} and past it: {past:?}"
    );
}

#[test]
fn a_setting_that_is_not_a_number_is_refused_and_named() {
    let the_refusal = the_repeat_budget(Some("as often as it takes"));

    assert!(
        the_refusal.is_err(),
        "**because a setting that cannot be read is a setting nobody is watching** — and the \
         alternative is a run that never stops and a deployment that believes it set something. \
         What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_run_with_no_supervisor_is_refused_by_the_field_that_is_empty() {
    let the_refusal = TheRepairCouldNotHappen::ThereIsNoSupervisorForThisRun;

    assert!(
        the_refusal.to_string().contains("supervise"),
        "**because a default that refuses is loud when it runs and silent until then** — and this \
         repository's history has a real run that reached a supervisor twenty minutes in, found \
         the field empty, and delivered nothing. The refusal has to say which field. It said: \
         {the_refusal}"
    );
}

#[test]
fn a_judgement_that_it_will_not_repair_and_a_budget_that_ran_out_are_two_refusals() {
    let the_judgement = TheRepairCouldNotHappen::TheSupervisorJudgedThisIsNotRepairing {
        what_has_not_changed: String::from("the suite went red on the same assertion twice"),
    };
    let the_budget = TheRepairCouldNotHappen::TheRepeatBudgetRanOut {
        how_many_were_asked_for: 3,
        how_many_were_permitted: 2,
    };

    let every_refusal = [the_judgement, the_budget];

    assert_ne!(
        (every_refusal[0].clone(), every_refusal[1].clone()),
        (every_refusal[1].clone(), every_refusal[0].clone()),
        "**because they are facts about different things** — one is the model having decided and the
         other is the setting having run out, **and a run that reported both as *the repair failed* \
         sends whoever reads the trace after the wrong one.** The two are distinct values and the
         test says so."
    );
}
