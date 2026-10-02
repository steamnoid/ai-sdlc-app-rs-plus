//! A plan naming the files it wants changed, and the refusal for a plan that names nothing.
//!
//! **There is no implementation agent in this repository, and no `ImplementationPlan`.**
//! `WhatAnAgentHasProduced` declares the name and refuses a plan that says nothing, **and there is
//! nothing behind either** — a plan a person approves is a list of files, and a list a person
//! approves without a type behind it is a sentence.
//!
//! **Five lists and not one string, because the five are checked differently.** A file that is to
//! change must exist, a file that is to be added must not, and a test has to land in a directory
//! the runner already collects — **and a plan that carried them as prose could be checked for none
//! of those without parsing an argument out of a sentence.**
//!
//! **A plan that says nothing is a fault and not a plan.** Silence that reached a human at a gate
//! would carry the authority of a finished analysis, **and a person cannot tell an empty plan from
//! a real one by looking at it.**

use aisdlc_run::what_a_planner_planned::{APlanThatSaysNothing, the_plan_a_model_proposed};

/// The answer a model gives, **in the shape it was asked for.**
const A_MODEL_WROTE_A_PLAN: &str = r#"{
    "files_to_change": ["AGENTS.md"],
    "files_to_add": ["docs/a-gate-and-its-breaches.md"],
    "tests_to_add": ["tests/a_gate_and_its_breaches.rs"],
    "dependencies": [],
    "migrations": []
}"#;

#[test]
fn what_a_model_said_becomes_a_plan_naming_every_list() {
    let what_a_planner_concluded = the_plan_a_model_proposed(A_MODEL_WROTE_A_PLAN)
        .expect("a plan in the shape it was asked for");

    assert_eq!(
        (
            what_a_planner_concluded.files_to_change.as_slice(),
            what_a_planner_concluded.files_to_add.as_slice(),
            what_a_planner_concluded.tests_to_add.as_slice(),
        ),
        (
            ["AGENTS.md".to_string()].as_slice(),
            ["docs/a-gate-and-its-breaches.md".to_string()].as_slice(),
            ["tests/a_gate_and_its_breaches.rs".to_string()].as_slice(),
        ),
        "**because a plan is a list of files a person is about to approve, and a plan carried as \
         prose cannot be checked against the listing without parsing a sentence** — every list \
         reaches the gate as a list, in the order the specification lists them"
    );
}

#[test]
fn a_plan_naming_nothing_is_refused_rather_than_approved() {
    let a_model_that_said_nothing = r#"{"files_to_change":[],"files_to_add":[],"tests_to_add":[],"dependencies":[],"migrations":[]}"#;

    let the_refusal = the_plan_a_model_proposed(a_model_that_said_nothing);

    assert_eq!(
        the_refusal,
        Err(APlanThatSaysNothing::of(&[
            "files_to_change",
            "files_to_add",
            "tests_to_add"
        ])),
        "**because a plan naming nothing is a fault and not a plan** — and silence that reached a \
         human at a gate would carry the authority of a finished analysis. The refusal names the \
         lists that were empty, **because 'the plan said nothing' is a sentence and the five lists \
         are the fact**, and a person deciding whether to approve needs to know which half is missing"
    );
}

#[test]
fn a_dependencies_list_alone_is_still_a_plan_that_says_nothing() {
    let a_model_that_named_only_dependencies = r#"{"files_to_change":[],"files_to_add":[],"tests_to_add":[],"dependencies":["serde"],"migrations":[]}"#;

    let the_refusal = the_plan_a_model_proposed(a_model_that_named_only_dependencies);

    assert!(
        matches!(the_refusal, Err(APlanThatSaysNothing { .. })),
        "**because `dependencies` and `migrations` are not files** — a plan that names a crate to \
         add and no file to change is a plan that changes nothing, **and a plan that changes \
         nothing delivers nothing.** A gate that approved it would be approving a delivery with no \
         change in it: {the_refusal:?}"
    );
}

#[test]
fn an_answer_that_is_no_plan_at_all_is_refused_and_quotes_what_came() {
    let what_the_endpoint_said = "I would be happy to help with that!";

    let the_refusal = the_plan_a_model_proposed(what_the_endpoint_said);

    assert_eq!(
        the_refusal,
        Err(APlanThatSaysNothing::quoting(what_the_endpoint_said)),
        "**because a model asked for a plan and answered with a sentence is the case this \
         repository has been burned by** — three real runs gave three different shapes and the \
         parser insisted on one, **and a refusal that paraphrases the answer throws away the only \
         clue there was.** Quoted whole, and not summarised"
    );
}

#[test]
fn a_file_the_plan_names_twice_is_named_once() {
    let a_model_naming_one_file_twice = r#"{
        "files_to_change": ["AGENTS.md", "AGENTS.md"],
        "files_to_add": [],
        "tests_to_add": [],
        "dependencies": [],
        "migrations": []
    }"#;

    let what_a_planner_concluded = the_plan_a_model_proposed(a_model_naming_one_file_twice)
        .expect("a plan that names a file is a plan");

    assert_eq!(
        what_a_planner_concluded.files_to_change,
        vec![String::from("AGENTS.md")],
        "**because the same file written twice is one file to change and not two** — and a \
         downstream list of writes that carries the same path twice is a run that would write it \
         twice and report two changes: {:?}",
        what_a_planner_concluded.files_to_change
    );
}
