//! What every agent must do again, and where the run starts again.
//!
//! **A run that went wrong gets another turn, and a model decides how many and where from.** Three
//! refusals mean *a stage was asked for something it could not do* rather than *the work is wrong*,
//! and a run that stops at any of them delivers nothing.
//!
//! **One verdict per agent and no default.** An agent the model left out is **a missing verdict
//! rather than a stage that quietly does not run** — and a stage that quietly does not run is the
//! failure this whole mechanism exists to prevent.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_run::what_to_repeat::{
    AStageNobodyMayRestartFrom, TheVerdict, WhatToRepeat, WhereTheRunShouldStartAgain,
    a_stage_nobody_may_restart_from, the_agent_asked_to_do_it_again, what_to_repeat,
};

/// What one agent must do again, **and its reason.**
fn a_verdict(what_the_agent_must_do: &str) -> TheVerdict {
    TheVerdict {
        who_must_act: AgentIdentifier::try_from("aisdlc-dev-impl").expect("an agent name"),
        what_they_must_do: String::from(what_the_agent_must_do),
        why_the_last_time_was_not_enough: String::from("the suite went red after the change"),
    }
}

/// Every verdict a supervisor is asked about, **and the set is the closed list it was shown.**
fn every_verdict() -> Vec<TheVerdict> {
    vec![
        a_verdict("write the change again and run the suite"),
        TheVerdict {
            who_must_act: AgentIdentifier::try_from("aisdlc-dev-plan").expect("an agent name"),
            what_they_must_do: String::from("name a product file the test can observe"),
            why_the_last_time_was_not_enough: String::from("the plan's tests_to_add was empty"),
        },
    ]
}

/// A whole answer, **and the ordinary case.**
///
/// **Named `a_whole_answer` and not `what_to_repeat`, because a fixture named after the function it
/// builds shadows it** — and a shadowed function is a reader's question with no answer.
fn a_whole_answer() -> WhatToRepeat {
    what_to_repeat(
        every_verdict(),
        "the change is written again",
        "the suite went red on the same assertion twice",
    )
    .expect("a closed answer with a verdict for every agent")
}

#[test]
fn a_repeat_naming_one_place_and_saying_what_has_not_changed_is_accepted() {
    let the_answer = a_whole_answer();

    assert_eq!(
        (every_verdict().len(), the_answer.where_the_run_starts_again),
        (2, WhereTheRunShouldStartAgain::TheChangeIsWrittenAgain),
        "**because the two things this answer carries are the whole of what a supervisor decides** \
         — what each agent must do, and which stage the run is put back to. Neither is a paragraph \
         and both have to be checked."
    );
}

#[test]
fn a_verdict_the_model_left_out_is_refused_by_name() {
    let a_verdict_for_nobody = Vec::new();

    let the_refusal = what_to_repeat(
        a_verdict_for_nobody,
        "the change is written again",
        "nothing has changed",
    );

    assert!(
        matches!(
            &the_refusal,
            Err(AStageNobodyMayRestartFrom::AVerdictForNobody)
        ),
        "**because an agent the model left out is a missing verdict rather than a stage that \
         quietly does not run** — and a stage that quietly does not run is the failure this whole \
         mechanism exists to prevent. What was refused: {the_refusal:?}"
    );
}

#[test]
fn two_verdicts_for_one_agent_are_refused() {
    let mut two_for_the_same = every_verdict();
    two_for_the_same.push(a_verdict("try something else entirely"));

    let the_refusal = what_to_repeat(two_for_the_same, "the change is written again", "nothing");

    assert!(
        matches!(
            &the_refusal,
            Err(AStageNobodyMayRestartFrom::TwoVerdictsForOneAgent { .. })
        ),
        "**because a second instruction to the same agent is one of two things and a run cannot \
         carry both** — the second would silently win and the first would be a record of something \
         that never happened. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_place_outside_the_closed_set_is_refused_and_the_set_is_named() {
    let the_refusal = what_to_repeat(
        every_verdict(),
        "the requirements are analysed again",
        "the product analyst was wrong",
    );

    let it_names_what_was_offered = matches!(
        &the_refusal,
        Err(AStageNobodyMayRestartFrom::NoSuchPlaceToStartAgain { .. })
    );

    assert!(
        it_names_what_was_offered,
        "**because the product analyst is deliberately not in this set** — nothing observed said \
         the requirements were wrong, and a target for the analyst would be a route back past four \
         gates on a guess. What was refused: {the_refusal:?}"
    );
}

#[test]
fn what_has_not_changed_may_not_be_left_blank() {
    let the_refusal = what_to_repeat(every_verdict(), "the change is written again", "   ");

    assert!(
        matches!(
            &the_refusal,
            Err(AStageNobodyMayRestartFrom::NothingSaidHasNotChanged)
        ),
        "**because a supervisor that cannot say what has not changed will put the same agent back \
         to work for ever** — and what has not changed is the one sentence that tells the model \
         whether another turn is worth anything. What was refused: {the_refusal:?}"
    );
}

#[test]
fn the_place_is_the_models_and_the_agent_beside_it_is_the_code_s() {
    let the_place =
        the_agent_asked_to_do_it_again(WhereTheRunShouldStartAgain::TheChangeIsWrittenAgain);
    let the_plan =
        the_agent_asked_to_do_it_again(WhereTheRunShouldStartAgain::TheChangeIsPlannedAgain);

    assert_eq!(
        (the_place.0.as_str(), the_plan.0.as_str()),
        ("aisdlc-dev-impl", "aisdlc-dev-plan"),
        "**because the model decides and the code routes** — the answer names one of two places and \
         **this literal is what sends the work back**, because a value read out of the model would \
         be a model routing the graph, which is the one thing in this pipeline declared as code \
         rather than reasoned about. Writing again is the implementer and planning again is the \
         planner."
    );
}

#[test]
fn the_two_places_are_exactly_two_and_no_other() {
    let every_place = WhereTheRunShouldStartAgain::every_place();

    assert_eq!(
        every_place,
        vec![
            WhereTheRunShouldStartAgain::TheChangeIsWrittenAgain,
            WhereTheRunShouldStartAgain::TheChangeIsPlannedAgain,
        ],
        "**because the product analyst, the quality analyst, the architecture analyst and the \
         security analyst are deliberately absent** — nothing observed so far has said any of them \
         was wrong, **and a route back past four gates on a guess is not a repair.** Two places."
    );
}

#[test]
fn a_place_written_with_surrounding_whitespace_is_still_the_place() {
    let with_spaces = what_to_repeat(every_verdict(), "  the change is planned again\n", "x");

    assert!(
        with_spaces.is_ok(),
        "**because a model that indents its answer has not answered differently** — and refusing it \
         would send a correction that comes back with the same spacing. It gave: {with_spaces:?}"
    );
}

#[test]
fn a_place_nobody_has_ever_heard_of_is_named_rather_than_guessed_at() {
    let the_refusal = a_stage_nobody_may_restart_from("the security analyst tries again");

    assert!(
        the_refusal
            .to_string()
            .contains("the change is written again, the change is planned again"),
        "**because a model told its answer was wrong and not told what the answers are will send \
         the same sentence again** — this repository has been burned by that three times with an \
         invalid JSON body. The refusal has to carry the set. It said: {the_refusal}"
    );
}
