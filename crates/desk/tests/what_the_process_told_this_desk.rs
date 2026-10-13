//! What the process told this desk, **and the refusal when it cannot mean anything.**
//!
//! **Cycle 132 wires the front door to the process.** `what_a_run_was_asked_for` reads what a
//! person typed and `TheEnvironment` holds what the process was told — **and nothing joined the
//! two**, so a deployment that configured everything still has no way to start a run without
//! writing code.
//!
//! **The environment travels as a value and never as a reading**, because a function that reads
//! its own configuration has moved a decision out of a type and into a line of code. A model
//! that was never configured travels as nothing — **and the desk still refuses before it copies
//! anything**, because that refusal names what is missing rather than an address nobody read.

use aisdlc_desk::what_the_process_told_this_desk::what_the_process_told_this_desk;
use aisdlc_outside::the_environment::TheEnvironment;

#[test]
fn the_address_a_deployment_named_reaches_the_desk() {
    let told = the_desk_as_it_was_told(&the_environment_where_everything_is_set());

    assert_eq!(
        told.the_endpoint.as_deref(),
        Some("https://a-model.example/v1/chat/completions"),
        "**because a desk that forgot the address would ask a model somewhere else**"
    );
}

#[test]
fn the_model_a_deployment_deployed_reaches_the_desk() {
    let told = the_desk_as_it_was_told(&the_environment_where_everything_is_set());

    assert_eq!(
        told.which_model.as_deref(),
        Some("a-model-a-deployment-deployed"),
        "**because a desk that forgot which model answers there invents one**"
    );
}

#[test]
fn the_key_a_deployment_supplied_reaches_the_desk() {
    let told = the_desk_as_it_was_told(&the_environment_where_everything_is_set());

    assert_eq!(
        told.the_key.as_deref(),
        Some("a-key-not-a-real-one"),
        "**because a delivery without the key is a branch nothing authenticated pushed**"
    );
}

#[test]
fn the_decider_a_deployment_named_reaches_the_desk() {
    let told = the_desk_as_it_was_told(&the_environment_where_everything_is_set());

    assert_eq!(
        told.who_answers.as_str(),
        "paligakrzychu",
        "**because the record of every gate names whoever answered**"
    );
}

#[test]
fn the_repository_a_person_typed_reaches_the_desk() {
    let told = the_desk_as_it_was_told(&the_environment_where_everything_is_set());

    assert_eq!(
        told.what_a_person_was_asked_for
            .which_repository()
            .to_string()
            .as_str(),
        "steamnoid/wellbeing-tracker-public",
        "**because the work a run does is the work a person typed**"
    );
}

#[test]
fn nothing_typed_is_refused_by_naming_what_is_missing() {
    let refused = what_the_process_told_this_desk(&[], &the_environment_where_everything_is_set());

    assert!(
        matches!(
            refused,
            Err(aisdlc_desk::what_a_run_was_asked_for::WhatWasAskedForThatCannotMeanAnything::NothingWasAskedFor { .. })
        ),
        "**because a person who typed nothing is owed every argument at once**"
    );
}

#[test]
fn no_decider_named_is_still_somebody_at_the_keyboard() {
    let told = the_desk_as_it_was_told(&the_environment_where_no_decider_is_named());

    assert_eq!(
        told.who_answers.as_str(),
        "the operator",
        "**because the person who ran the binary is the operator**"
    );
}

#[test]
fn no_model_configured_travels_as_nothing() {
    let told = the_desk_as_it_was_told(&the_environment_where_nothing_is_set());

    assert!(
        told.the_endpoint.is_none(),
        "**because a desk that invented an address would ask a model somewhere else**"
    );
}

/// The desk as it was told, **typed and set together or not at all.**
fn the_desk_as_it_was_told(
    the_environment: &TheEnvironment,
) -> aisdlc_desk::the_run_is_started::WhatThisDeskWasGiven {
    what_the_process_told_this_desk(&what_a_person_typed(), the_environment)
        .expect("three arguments were typed")
}

/// What a person typed, **and it is the same three arguments the front door refuses.**
fn what_a_person_typed() -> Vec<String> {
    vec![
        String::from("--repository"),
        String::from("steamnoid/wellbeing-tracker-public"),
        String::from("--projects-root"),
        where_this_test_keeps_its_projects(),
        String::from("--do"),
        String::from("untrack the sqlite sidecar files"),
    ]
}

/// Where this test keeps its projects, **and it is a directory of this process's own** because a
/// desk that wrote into somebody's home would be the defect this test exists beside.
fn where_this_test_keeps_its_projects() -> String {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-what-the-process-told-this-desk-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&where_it_is);
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is.display().to_string()
}

/// The environment where everything is set, **as a value and never as a reading.**
fn the_environment_where_everything_is_set() -> TheEnvironment {
    TheEnvironment::of(vec![
        (
            String::from("LLM_ENDPOINT"),
            String::from("https://a-model.example/v1/chat/completions"),
        ),
        (
            String::from("LLM_MODEL"),
            String::from("a-model-a-deployment-deployed"),
        ),
        (
            String::from("OPENAI_API_KEY"),
            String::from("a-key-not-a-real-one"),
        ),
        (
            String::from("AISDLC_WHO_DECIDES"),
            String::from("paligakrzychu"),
        ),
    ])
}

/// The environment where no decider is named, **and the one running the binary still is one.**
fn the_environment_where_no_decider_is_named() -> TheEnvironment {
    TheEnvironment::of(vec![
        (
            String::from("LLM_ENDPOINT"),
            String::from("https://a-model.example/v1/chat/completions"),
        ),
        (
            String::from("LLM_MODEL"),
            String::from("a-model-a-deployment-deployed"),
        ),
        (
            String::from("OPENAI_API_KEY"),
            String::from("a-key-not-a-real-one"),
        ),
    ])
}

/// The environment where everything but the organisation is set, **so the run has nobody to ask.**
fn the_environment_where_everything_but_the_organisation_is_set() -> TheEnvironment {
    TheEnvironment::of(vec![
        (
            String::from("LLM_ENDPOINT"),
            String::from("https://a-model.example/v1/chat/completions"),
        ),
        (
            String::from("LLM_MODEL"),
            String::from("a-model-a-deployment-deployed"),
        ),
        (
            String::from("OPENAI_API_KEY"),
            String::from("a-key-not-a-real-one"),
        ),
        (
            String::from("AISDLC_WHO_DECIDES"),
            String::from("paligakrzychu"),
        ),
    ])
}

/// The environment where nothing is set, **and nothing configured travels as nothing.**
fn the_environment_where_nothing_is_set() -> TheEnvironment {
    TheEnvironment::of(vec![])
}

#[test]
fn the_organisation_a_deployment_named_reaches_the_desk() {
    let told = the_desk_as_it_was_told(&the_environment_where_an_organisation_reasons());

    assert_eq!(
        told.which_organisation,
        Some(aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons::Ollama),
        "**because a run that does not know who reasons about it cannot ask anybody**"
    );
}

#[test]
fn an_organisation_nobody_wrote_down_is_refused_where_it_is_read() {
    let refused = what_the_process_told_this_desk(
        &what_a_person_typed(),
        &the_environment_where_no_organisation_reasons(),
    );

    assert!(
        matches!(
            refused,
            Err(aisdlc_desk::what_a_run_was_asked_for::WhatWasAskedForThatCannotMeanAnything::NoOrganisationThatReasons { .. })
        ),
        "**because a run reasoning with the wrong organisation spends money and answers in the \
         wrong shape** — and the refusal names the variable and the four there are."
    );
}

#[test]
fn no_organisation_named_is_refused_before_anything_is_copied() {
    let mut told = what_the_process_told_this_desk(
        &what_a_person_typed(),
        &the_environment_where_everything_but_the_organisation_is_set(),
    )
    .expect("three arguments were typed, and nothing configured travels as nothing");
    let refused = aisdlc_desk::the_run_is_started::the_run_is_started_with(&mut told);

    let why_not = match refused {
        Err(why_not) => why_not.to_string(),
        Ok(said) => format!("a desk that reasoned with nobody said it left a page: {said}"),
    };
    assert!(
        why_not.contains("organisation"),
        "**because a run that does not know who reasons about it has nobody to ask** — and a \
         refusal that names something else sends a reader to configure the wrong thing. It said: \
         {why_not}"
    );
}

/// The environment where an organisation reasons, **in the specification's own spelling.**
fn the_environment_where_an_organisation_reasons() -> TheEnvironment {
    TheEnvironment::of(vec![
        (
            String::from("LLM_ENDPOINT"),
            String::from("https://a-model.example/v1/chat/completions"),
        ),
        (
            String::from("LLM_MODEL"),
            String::from("a-model-a-deployment-deployed"),
        ),
        (
            String::from("OPENAI_API_KEY"),
            String::from("a-key-not-a-real-one"),
        ),
        (String::from("LLM_ORGANISATION"), String::from("ollama")),
    ])
}

/// The environment where no organisation reasons, **and the name it carries is nobody's.**
fn the_environment_where_no_organisation_reasons() -> TheEnvironment {
    TheEnvironment::of(vec![
        (
            String::from("LLM_ENDPOINT"),
            String::from("https://a-model.example/v1/chat/completions"),
        ),
        (
            String::from("LLM_MODEL"),
            String::from("a-model-a-deployment-deployed"),
        ),
        (
            String::from("OPENAI_API_KEY"),
            String::from("a-key-not-a-real-one"),
        ),
        (String::from("LLM_ORGANISATION"), String::from("anthropic")),
    ])
}
