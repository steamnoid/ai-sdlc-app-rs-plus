//! A credential a program carries and no request does, and a question that never reveals one.
//!
//! **The key lives on the program and not on the request**, because a request is the thing this
//! system renders, records, quotes into a commit message and shows to a reader — **so a secret on
//! a request is a secret in four places, and a secret on a program is a secret in one.**
//!
//! **A credential that prints itself is a credential one `{}` away from a log.** Every way of
//! turning this value into text prints that it is a credential and not what it is, so the mistake
//! that leaks it produces a sentence with nothing in it.

use aisdlc_run::a_credential::ACredential;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

#[test]
fn a_credential_prints_as_a_credential_and_not_as_its_key() {
    let the_key = "sk-a-real-key-that-must-never-appear-anywhere";

    let what_a_reader_sees = ACredential::the_key_a_deployment_supplied(the_key)
        .expect("a key is a key")
        .to_string();

    assert_eq!(
        what_a_reader_sees, "a credential",
        "**because every way of turning this into text has to be safe, and the unsafe one is a `{{}}`** — so the text is the fact that there is a credential and never the credential"
    );
    assert!(
        !what_a_reader_sees.contains("sk-"),
        "and the check says so in the reader's own terms rather than mine"
    );
}

#[test]
fn a_key_that_is_nothing_is_refused() {
    for nothing in ["", "   ", "\n"] {
        assert!(
            ACredential::the_key_a_deployment_supplied(nothing).is_err(),
            "**because a credential that is blank is a request that will be refused by the far end**, and the far end's refusal is this system's problem to read: {nothing:?}"
        );
    }
}

#[test]
fn a_request_carries_no_credential_and_a_reader_could_print_one_safely() {
    let a_question = WhatWeTellAProgramToDo::ask_a_model_how_it_rules(
        "http://localhost:11434/v1",
        "a-model",
        "a question about an artifact",
    );

    let what_a_reader_could_print = a_question.what_it_is_asked_about().to_string();

    assert!(
        what_a_reader_could_print.contains("a question about an artifact"),
        "**and the question is readable, because a question a reader cannot read is a question nobody can check** — this is the whole reason a credential is not here: everything a request says is safe to show"
    );
}
