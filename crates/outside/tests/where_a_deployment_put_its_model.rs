//! Where a model is, read out of the deployment, and never invented.
//!
//! **A fallback is a lie with a position.** The first real run of this system fell back to
//! `http://localhost:11434` because nobody had read `LLM_ENDPOINT`, and `curl` answered *Failed to
//! connect to localhost port 11434* — a number this system chose, in a message a person then had to
//! read. **An endpoint nobody said is a refusal, and the refusal names the variable**, because a
//! guess that is wrong costs the same time as a refusal and says less.

use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;

#[test]
fn an_endpoint_nobody_named_is_a_refusal_naming_it() {
    let the_way_out = WhereAModelIs::where_a_deployment_put_its_model(None, Some("a-model"));

    let it_says_which_one = the_way_out
        .as_ref()
        .err()
        .map(|why_not| why_not.contains("LLM_ENDPOINT"));
    assert_eq!(
        it_says_which_one,
        Some(true),
        "**because a guess that is wrong costs the same time as a refusal and says less** — and \
         the first real run answered *Failed to connect to localhost port 11434*, a number this \
         system chose: {the_way_out:?}"
    );
}

#[test]
fn a_model_nobody_named_is_a_refusal_naming_it() {
    let the_way_out = WhereAModelIs::where_a_deployment_put_its_model(
        Some("http://localhost:11434/v1/chat/completions"),
        None,
    );

    let it_says_which_one = the_way_out
        .as_ref()
        .err()
        .map(|why_not| why_not.contains("LLM_MODEL"));
    assert_eq!(
        it_says_which_one,
        Some(true),
        "**because a model this system invented is a model nobody deployed and nobody is \
         responsible for** — and its answer would be recorded against it: {the_way_out:?}"
    );
}

#[test]
fn both_named_and_the_model_can_be_asked() {
    let the_way_out = WhereAModelIs::where_a_deployment_put_its_model(
        Some("http://localhost:11434/v1/chat/completions"),
        Some("a-model"),
    );

    let what_the_deployment_named = the_way_out.as_ref().ok().map(|where_it_is| {
        (
            where_it_is.the_endpoint.as_str(),
            where_it_is.which_model.as_str(),
        )
    });
    assert_eq!(
        what_the_deployment_named,
        Some(("http://localhost:11434/v1/chat/completions", "a-model")),
        "**because two names a deployment set are two facts, and this is the shape they have** — \
         and nothing here may add a third of its own: {the_way_out:?}"
    );
}

/// **Ignored, because the environment this assertion is about is the one the test was started
/// from**, and `cargo test` starts it from a shell that has not read `.env`. Running this with the
/// file loaded is the point; running it without is a skip with the reason printed, and a skip with
/// the reason printed is not a green.
#[test]
#[ignore = "asserts about the environment the process was started from, and needs .env loaded"]
fn the_reader_keeps_what_a_process_was_told_and_asks_for_it_by_name() {
    let what_it_was_told = TheEnvironment::reading_what_the_process_was_told();

    let the_endpoint = what_it_was_told.the_variable_a_deployment_set("LLM_ENDPOINT");
    let endpoint_seen = the_endpoint.map(str::to_string);

    assert!(
        the_endpoint.is_some() && the_endpoint != Some(""),
        "**because this process was started from a file that names an endpoint, and this is the \
         only place in the repository that can read it** — if the reader cannot see it, every \
         caller falls back, and every fallback is a lie with a position: {endpoint_seen:?}"
    );
}
