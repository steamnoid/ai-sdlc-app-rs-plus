//! The arguments a model is asked with, and the refusal when there is no credential behind it.
//!
//! **A program with no credential refuses, and never sends a request without one.** A request that
//! arrives unauthenticated comes back as a refusal in words this system did not write, and the
//! caller has to tell *no credential* from *no such model* from *the network is down* — **so the
//! thing it can most usefully be told is that it never left.**
//!
//! **The credential is on the program and not on the request**, and the arguments are where it
//! finally appears — one argument each, with no shell anywhere to re-split them.

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
