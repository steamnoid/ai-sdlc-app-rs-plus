//! A creative agent asked over a program, producing offers grounded in what it was shown.
//!
//! **The whole path in one test and the seams beside it**, because the first real run in this
//! repository walked seven gates and six models and was refused by an arity at the last step — **and
//! the seam and the thing it stands for had never been compared.**
//!
//! **What the agent says about its own reading comes from the model too, and it is checked here** —
//! a blank reaching a card is indistinguishable from an agent that was never asked.

use aisdlc_domain::repository::Repository;
use aisdlc_run::an_agent_is_asked_what_it_would_file::an_agent_is_asked_what_it_would_file;
use aisdlc_run::an_agent_is_asked_what_it_would_file::{
    the_body_a_creative_agent_is_given, the_three_judgements_a_model_gave,
};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// Where a deployment said its model is.
fn where_the_model_is() -> aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs {
    aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs::where_a_deployment_put_its_model(
        Some("https://opencode.ai/zen/go/v1/chat/completions"),
        Some("space-bunny-free"),
    )
    .expect("a full address and a model")
}

/// The repository whose work is being offered.
fn the_repository_it_read() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

/// The files a run really did show the model.
fn what_the_run_actually_saw() -> Vec<String> {
    vec!["crates/run/src/what_an_agent_offered.rs".to_string()]
}

/// What an endpoint said, wrapping content the way this gateway does.
fn what_the_endpoint_said(its_content: &str) -> String {
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":{}}}}}]}}",
        serde_json::to_string(its_content).expect("a string is JSON")
    )
}

#[test]
fn what_the_agent_said_about_its_own_work_is_read_and_kept() {
    let its_content = r#"{"offers":[],"what_it_found":"an offer path is never checked against the listing","what_it_could_not_settle":"whether the listing is truncated","what_it_expects_next":"file what the listing names"}"#;

    let what_it_said = the_three_judgements_a_model_gave(&what_the_endpoint_said(its_content))
        .expect("three judgements said");

    assert_eq!(
        what_it_said.what_it_could_not_settle.as_str(),
        "whether the listing is truncated",
        "**because the one worth keeping is the doubt, and it is the only reader that can have it** — \
         a paraphrase of what an agent said is a second thing said in its name: {what_it_said:?}"
    );
}

#[test]
fn an_answer_whose_three_judgements_are_blank_is_refused() {
    let its_content = r#"{"offers":[],"what_it_found":"","what_it_could_not_settle":"","what_it_expects_next":""}"#;

    let what_it_said = the_three_judgements_a_model_gave(&what_the_endpoint_said(its_content));

    assert!(
        what_it_said.is_err(),
        "**because a blank on a card reads as *the agent had nothing to say*** — and that is not a \
         thing anybody can check, the same rule an ignored test follows and the same rule a silent \
         field follows: {what_it_said:?}"
    );
}

#[test]
fn the_body_is_valid_json_however_the_question_reads() {
    let a_question_reading_awkwardly =
        "a \"quoted\" word, a \\ backslash and a newline\nsecond line";

    let the_body =
        the_body_a_creative_agent_is_given(a_question_reading_awkwardly, &where_the_model_is());

    let what_it_came_back_as = serde_json::from_str::<serde_json::Value>(&the_body);
    assert!(
        what_it_came_back_as.is_ok(),
        "**because a hand-rolled escape is a lossy channel in both directions** — and every request \
         this system ever sent a model was invalid JSON until the library built the body, the far end \
         answered 400 with zero bytes, and the refusal reached us as an empty string. The body this \
         run would send is: {the_body}"
    );
    let what_it_came_back_as = what_it_came_back_as.expect("checked one line above");
    assert_eq!(
        what_it_came_back_as
            .get("messages")
            .and_then(|messages| messages.get(0))
            .and_then(|one| one.get("content"))
            .and_then(|it| it.as_str()),
        Some(a_question_reading_awkwardly),
        "**because a hand-rolled escape is a lossy channel in both directions** — and every request \
         this system ever sent a model was invalid JSON until the library built the body, the far \
         end answered 400 with zero bytes, and the refusal reached us as an empty string"
    );
}

#[test]
fn the_request_is_a_question_about_what_the_agent_would_file() {
    let what_the_run_asks_for = WhatWeTellAProgramToDo::ask_a_model_what_it_would_file(
        &where_the_model_is().the_endpoint,
        &where_the_model_is().which_model,
        "a body",
    );

    assert!(
        what_the_run_asks_for.is_a_question_about_what_it_would_file(),
        "**because a run that cannot tell what kind of question it is asking cannot refuse to ask \
         one** — and the refusal is the whole of the flag's meaning: {what_the_run_asks_for:?}"
    );
}

#[test]
fn the_whole_path_from_a_listing_to_grounded_offers() {
    let its_content = r#"{"offers":[{"title":"an offer path is never checked","what_it_asks_for":"compare each cited path with the listing","paths":["crates/run/src/what_an_agent_offered.rs"]}],"what_it_found":"nothing compares the paths","what_it_could_not_settle":"nothing","what_it_expects_next":"add the comparison"}"#;
    let the_program = AProgramThatAnswers(what_the_endpoint_said(its_content));

    let what_was_offered = an_agent_is_asked_what_it_would_file(
        &the_program,
        &the_repository_it_read(),
        &what_the_run_actually_saw(),
        &where_the_model_is(),
    )
    .expect("an answer in the shape it was asked for, grounded in the listing");

    assert_eq!(
        what_was_offered.offers.len(),
        1,
        "**because one real proposal is worth filing and the whole path is one test** — checkout \
         listing, question, body, request, answer, grounding: {what_was_offered:?}"
    );
}

/// A program that answers with a fixed answer, and remembers nothing.
struct AProgramThatAnswers(String);

impl aisdlc_run::a_program_we_run::AProgramWeRun for AProgramThatAnswers {
    fn run(
        &self,
        _what_it_is_asked: &WhatWeTellAProgramToDo,
    ) -> aisdlc_run::a_program_we_run::WhatTheProgramSaid {
        aisdlc_run::a_program_we_run::WhatTheProgramSaid::of(true, &self.0)
    }
}
