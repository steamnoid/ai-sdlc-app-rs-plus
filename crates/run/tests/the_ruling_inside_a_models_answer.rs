//! A model's answer arrives wrapped in the shape its endpoint chose, and the ruling is inside.
//!
//! **The endpoint's envelope is not the ruling, and the ruling is not the envelope.** A model
//! asked to say one word answers with JSON around that word, and a reader that wants the whole
//! answer to *be* the word finds nothing — **so the word is taken out of the envelope and the
//! envelope decides nothing.**
//!
//! **An answer nobody can read is never an approval.** A parse that fails, an envelope with no
//! content, a content that is empty — all of them are a refusal to rule, **because the direction
//! of that failure is the whole point**: reading a broken answer as an approval puts a model at a
//! human's gate on the strength of a bug.

use aisdlc_run::a_model_is_asked_how_it_rules::the_ruling_a_model_gave;

/// The answer an OpenAI-compatible endpoint gives for a model that said the given word.
fn an_answer_carrying(what_the_model_said: &str) -> String {
    format!(
        "{{\"id\":\"chatcmpl-1\",\"choices\":[{{\"message\":{{\"role\":\"assistant\",\"content\":\"{what_the_model_said}\"}}}}]}}"
    )
}

#[test]
fn a_ruling_inside_an_answer_is_read() {
    let approved = the_ruling_a_model_gave(&an_answer_carrying("APPROVED"));
    let refused = the_ruling_a_model_gave(&an_answer_carrying("REFUSED"));

    assert!(
        approved.as_ref().is_ok_and(|it_approved| *it_approved) && !refused.expect("a word"),
        "**because the envelope is the endpoint's and not the ruling** — a model that said APPROVED in a JSON body has approved, and a reader that wants the whole answer to *be* the word has decided no model can ever rule: {approved:?}"
    );
}

#[test]
fn an_answer_nobody_can_read_is_never_an_approval() {
    let the_broken_ones = [
        String::new(),
        "not json at all".to_string(),
        "{}".to_string(),
        an_answer_carrying(""),
        "{\"choices\":[]}".to_string(),
    ];

    let any_was_approved = the_broken_ones
        .iter()
        .any(|one| the_ruling_a_model_gave(one).is_ok_and(|approved| approved));

    assert!(
        !any_was_approved,
        "**because the direction of this failure is the whole point** — an answer that cannot be read must fail toward a refusal, and a reader that fails the other way puts a model at a human's gate on the strength of a bug: {:?}",
        the_broken_ones
            .iter()
            .map(|one| the_ruling_a_model_gave(one).is_ok())
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_model_that_explained_itself_has_not_ruled() {
    let with_a_reason = the_ruling_a_model_gave(&an_answer_carrying(
        "APPROVED, because the requirement is clear and the tests exist",
    ));

    assert!(
        with_a_reason.is_err(),
        "**because a model that added a sentence has not said the fixed form** — and a reader that accepted the first word it found would be accepting a model that opens a gate and then explains itself afterwards, which is not a decision anybody recorded"
    );
}

#[test]
fn what_could_not_be_read_comes_back_whole() {
    let what_came = "the endpoint was down";
    let the_refusal = the_ruling_a_model_gave(what_came);

    assert_eq!(
        the_refusal
            .as_ref()
            .err()
            .map(|one| one.what_it_said().to_string()),
        Some(what_came.to_string()),
        "**and quoted and not summarised** — because a caller deciding whether to retry needs to know what the model actually said, and a paraphrase of it is one more layer to be wrong in"
    );
}
