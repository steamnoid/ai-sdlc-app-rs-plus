//! What a model said it would file, read out of the envelope it arrived in.
//!
//! **The listing goes into the question and the refusal is not the model's to decide.** A creative
//! agent asked to improve a project will happily invent one that does not exist, **so the run hands
//! it the names of the files there are** and then checks the answer against that same list — **and
//! the check is code, not the sentence in the prompt.**

use aisdlc_run::an_agent_is_asked_what_it_would_file::{
    the_question_an_agent_is_asked, what_a_model_offered,
};

/// The repository whose work is being offered.
fn the_repository_it_read() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// The files a run really did show the model.
fn what_the_run_actually_saw() -> Vec<String> {
    vec![
        "crates/run/src/what_an_agent_offered.rs".to_string(),
        "crates/run/tests/a_model_is_asked_what_it_would_file.rs".to_string(),
    ]
}

/// What an endpoint said, wrapping content the way this gateway does.
fn what_the_endpoint_said(its_content: &str) -> String {
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":{}}}}}]}}",
        serde_json::to_string(its_content).expect("a string is JSON")
    )
}

#[test]
fn the_question_hands_the_model_the_listing_it_may_cite() {
    let the_question =
        the_question_an_agent_is_asked(&the_repository_it_read(), &what_the_run_actually_saw());

    for one_path in what_the_run_actually_saw() {
        assert!(
            the_question.contains(&one_path),
            "**because a model asked what to improve and not told what exists will invent a file** — \\
             and an invented file is an offer that reads exactly like a real one. Every name it is \\
             allowed to cite has to be in the question: {the_question}"
        );
    }
}

#[test]
fn the_answer_is_read_out_of_the_envelope_and_becomes_offers() {
    let its_content = r#"{"offers":[{"title":"the offers are not read by any test","what_it_asks_for":"name the refusal in a test","paths":["crates/run/src/what_an_agent_offered.rs"]}]}"#;

    let what_it_said = what_a_model_offered(&what_the_endpoint_said(its_content))
        .expect("an answer in the shape it was asked for");

    assert_eq!(
        what_it_said.len(),
        1,
        "**because the envelope is the endpoint's and the offer is the model's** — and a reader that \\
         demanded the whole answer be the offer would decide no model can ever file anything: \\
         {what_it_said:?}"
    );
    assert_eq!(
        what_it_said[0].paths,
        vec!["crates/run/src/what_an_agent_offered.rs".to_string()],
        "**because the path is the model's and only the address is ours** — a path parsed into \\
         something else before it can be checked is a path that was never checked: {what_it_said:?}"
    );
}

#[test]
fn an_answer_nobody_can_read_is_refused_and_quoted() {
    let what_a_gateway_said_when_it_failed = "Failed to connect to localhost port 11434";

    let refusal = what_a_model_offered(what_a_gateway_said_when_it_failed);

    assert!(
        refusal.is_err(),
        "**because that sentence is what the very first real run got back** — a fallback address this \\
         system chose, and the whole run bought to find out. Quoted, not summarised: \\
         {refusal:?}"
    );
}

#[test]
fn an_answer_with_no_offers_is_no_offers_and_not_a_refusal() {
    let what_it_said = what_a_model_offered(&what_the_endpoint_said(r#"{"offers":[]}"#))
        .expect("a model that found nothing to file said so");

    assert!(
        what_it_said.is_empty(),
        "**because a creative agent that files nothing has told the truth** — and the fault is an \\
         offer that names a file that is not there, which this answer did not do: {what_it_said:?}"
    );
}
