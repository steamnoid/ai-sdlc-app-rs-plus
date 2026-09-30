//! The body a model is given, and whether the far end can read it.
//!
//! **A body built by hand is a body that is broken by anything the hand did not think of.** The
//! builder wrote the model's question straight into the JSON where a quoted string belongs, with no
//! quotes around it — **so every request this system ever sent was malformed**, and the gateway
//! answered `400` with **zero bytes**, which is why the refusal quoted an empty string while the
//! same curl with a hand-written body answered `APPROVED`.
//!
//! **The far end's own arithmetic is the test.** A body that parses is a body that can be read, and
//! a question is a work item's name and an artifact's name, and both of those are strings that
//! people type.

use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
use aisdlc_run::a_model_is_asked_how_it_rules::the_body_a_model_is_given;

/// Where this deployment says its model is.
fn a_configured_model() -> WhereAModelIs {
    WhereAModelIs {
        the_endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
        which_model: "a-model".to_string(),
    }
}

/// The body, and whether the far end could read it at all.
fn whether_the_far_end_could_read(the_question: &str) -> Result<String, String> {
    let the_body = the_body_a_model_is_given(the_question, &a_configured_model());
    let what_it_came_back_as: Result<serde_json::Value, _> = serde_json::from_str(&the_body);
    if what_it_came_back_as.is_err() {
        return Err(the_body);
    }
    Ok(the_body)
}

#[test]
fn a_question_the_far_end_can_read_produces_a_body_the_far_end_can_read() {
    let the_way_out = whether_the_far_end_could_read("does this work?");

    assert!(
        the_way_out.is_ok(),
        "**because a body that does not parse is a body the far end refused with zero bytes** — and \
         the refusal that reached us was an empty string, which is what a `400` with nothing in it \
         looks like from here: {the_way_out:?}"
    );
}

#[test]
fn a_name_with_a_quote_in_it_cannot_break_the_request() {
    let a_question = "the artifact is named \"ARequirements\" and its version is 1";

    let the_way_out = whether_the_far_end_could_read(a_question);

    assert!(
        the_way_out.is_ok(),
        "**because a name somebody typed cannot be allowed to break the request it is in** — and \
         escaping a quote by hand is how a body stops being a body: {the_way_out:?}"
    );
}

#[test]
fn what_the_far_end_would_read_back_is_exactly_what_was_asked() {
    let a_question = "the artifact is named \"ARequirements\"\nand its version is 1\\two";

    let what_it_would_read = the_body_a_model_is_given(a_question, &a_configured_model());
    let the_content = serde_json::from_str::<serde_json::Value>(&what_it_would_read)
        .expect("a body that parses")
        .pointer("/messages/0/content")
        .and_then(|one| one.as_str())
        .unwrap_or_default()
        .to_string();

    assert_eq!(
        the_content, a_question,
        "**because a question that arrives changed is a question nobody can be held to** — and a \
         hand-rolled escape is a lossy channel in both directions: {the_content:?}"
    );
}
