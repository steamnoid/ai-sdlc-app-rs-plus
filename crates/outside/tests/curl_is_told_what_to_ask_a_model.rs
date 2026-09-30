//! The arguments a model is asked with, and the refusal when there is no credential behind it.
//!
//! **A program with no credential refuses, and never sends a request without one.** A request that
//! arrives unauthenticated comes back as a refusal in words this system did not write, and the
//! caller has to tell *no credential* from *no such model* from *the network is down* — **so the
//! thing it can most usefully be told is that it never left.**
//!
//! **The credential is on the program and not on the request**, and the arguments are where it
//! finally appears — one argument each, with no shell anywhere to re-split them.

use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

fn a_question_to_a_model() -> WhatWeTellAProgramToDo {
    WhatWeTellAProgramToDo::ask_a_model_how_it_rules(
        "http://localhost:11434/v1/chat/completions",
        "a-model",
        "{\"model\":\"a-model\",\"messages\":[]}",
    )
}

fn a_program(with_a_credential: bool) -> TheProgramAsItRuns {
    let the_credential = with_a_credential
        .then(|| ACredential::the_key_a_deployment_supplied("sk-a-key").expect("a key"));
    TheProgramAsItRuns::as_it_is_on_this_host(the_credential)
}

#[test]
fn a_program_with_no_credential_is_asked_for_nothing() {
    let the_arguments = a_program(false).the_arguments_of(&a_question_to_a_model());

    assert!(
        the_arguments.is_empty(),
        "**because a request that goes out unauthenticated comes back in words this system did not write** — and the one fact the caller can usefully be given is that nothing left. `curl` with no authorization header is a request the far end refuses, and its refusal is not this system's to read."
    );
}

#[test]
fn the_credential_reaches_the_request_only_as_one_argument() {
    let the_arguments = a_program(true).the_arguments_of(&a_question_to_a_model());

    let all_of_them_are_one_thing_each = the_arguments
        .iter()
        .any(|one| one == "http://localhost:11434/v1/chat/completions")
        && the_arguments
            .iter()
            .any(|one| one == "{\"model\":\"a-model\",\"messages\":[]}")
        && the_arguments
            .iter()
            .any(|one| one == "Authorization: Bearer sk-a-key");
    assert!(
        all_of_them_are_one_thing_each,
        "**because the body and the address and the credential are three separate arguments and not one string** — and one string is a shell's opinion about what it means, and there is no shell here to have that opinion: {the_arguments:?}"
    );
}

#[test]
fn the_body_is_one_argument_and_never_split_into_its_pieces() {
    let the_arguments = a_program(true).the_arguments_of(&a_question_to_a_model());

    let the_body = "{\"model\":\"a-model\",\"messages\":[]}";
    let the_body_is_whole = the_arguments.iter().any(|one| one == the_body);
    let braces_are_arguments_of_their_own =
        the_arguments.iter().any(|one| one == "{" || one == "}");
    assert!(
        the_body_is_whole && !braces_are_arguments_of_their_own,
        "**because a body split into its braces is a body no longer being a body** — and a request built from pieces is a request whose pieces somebody else assembles: {the_arguments:?}"
    );
}

#[test]
fn the_program_is_told_to_be_quiet() {
    let the_arguments = a_program(true).the_arguments_of(&a_question_to_a_model());

    let it_was_told_to_be_quiet = the_arguments.iter().any(|one| one == "-s");
    let it_was_told_to_narrate = the_arguments
        .iter()
        .any(|one| one == "--progress-bar" || one == "#");
    assert!(
        it_was_told_to_be_quiet && !it_was_told_to_narrate,
        "**because a program that narrates is a program whose narration can be mistaken for what it \
         found** — the first real run came back with curl's transfer table where the model's words \
         should have been, and the refusal quoted a percentage: {the_arguments:?}"
    );
}

#[test]
fn the_model_question_asks_to_post_and_says_where() {
    let the_arguments = a_program(true).the_arguments_of(&a_question_to_a_model());

    assert!(
        the_arguments.iter().any(|one| one == "-X")
            && the_arguments.iter().any(|one| one == "POST"),
        "**and it says what it is doing and where**, because a curl with no verb and no address is a program run for no reason at all: {the_arguments:?}"
    );
}

/// A question to a model, and a program carrying what a deployment named.
fn a_program_with_what_a_deployment_named(what_it_named: &str) -> TheProgramAsItRuns {
    let the_environment =
        TheEnvironment::of(vec![("LLM_HEADERS".to_string(), what_it_named.to_string())]);
    let the_credential = ACredential::the_key_a_deployment_supplied("sk-a-key").expect("a key");
    TheProgramAsItRuns::as_it_is_on_this_host(Some(the_credential))
        .with_what_a_deployment_named(&the_environment)
}

#[test]
fn a_header_a_deployment_named_is_one_argument_beside_the_others() {
    let the_arguments = a_program_with_what_a_deployment_named("x-opencode-session: aisdlc")
        .the_arguments_of(&a_question_to_a_model());

    let the_header_is_whole = the_arguments
        .iter()
        .any(|one| one == "x-opencode-session: aisdlc");
    let the_name_became_its_own_argument = the_arguments
        .iter()
        .any(|one| one.as_str() == "x-opencode-session" || one.as_str() == "aisdlc");
    assert!(
        the_header_is_whole && !the_name_became_its_own_argument,
        "**because a header is a name and a value and not two arguments** — and a name split from \
         its value is a header `curl` will send to a host it was not addressed to: {the_arguments:?}"
    );
}

#[test]
fn a_deployment_that_named_no_headers_still_asks_the_model_and_names_no_header() {
    let the_arguments =
        a_program_with_what_a_deployment_named("").the_arguments_of(&a_question_to_a_model());

    let it_still_asks = the_arguments.iter().any(|one| one == "POST")
        && the_arguments
            .iter()
            .any(|one| one == "http://localhost:11434/v1/chat/completions");
    let an_empty_header_was_sent = the_arguments.iter().any(String::is_empty);
    assert!(
        it_still_asks && !an_empty_header_was_sent,
        "**because most deployments need no header at all** — and sending an empty one is a header \
         with an empty name, which is a thing no host can be addressed by: {the_arguments:?}"
    );
}

#[test]
fn a_program_with_no_credential_is_asked_for_nothing_even_with_headers() {
    let the_environment = TheEnvironment::of(vec![(
        "LLM_HEADERS".to_string(),
        "x-opencode-session: aisdlc".to_string(),
    )]);
    let program_without_a_credential = TheProgramAsItRuns::as_it_is_on_this_host(None)
        .with_what_a_deployment_named(&the_environment);

    let the_arguments = program_without_a_credential.the_arguments_of(&a_question_to_a_model());

    assert!(
        the_arguments.is_empty(),
        "**because a header is not a credential and cannot stand in for one** — the deployment named \
         a header and this system read it, and the request still goes nowhere: {the_arguments:?}"
    );
}
