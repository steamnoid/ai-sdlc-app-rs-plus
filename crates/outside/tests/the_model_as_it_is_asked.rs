//! The model a deployment configured, asked over a program, and what a caller gets back.
//!
//! **The port exists and its only implementor is a test double.** `AReasoningModel` is declared
//! in `crates/run/src/lib.rs` for `the_agent_asks_for_its_artifact`, and every answer that use
//! case has had came from `AModelThatAnswers` — **so nothing in `crates/outside` names a model
//! at all**, and the two paths that do ask one go around the port through `AProgramWeRun`.
//!
//! **This is the second time this repository has been caught by that shape.** `README.md`
//! records `AGitHubWeWriteTo` having exactly one implementor and it being a test double, so
//! seven cycles of the creative path were proved against a place that accepts anything. The
//! arguments `curl` is run with are already covered by
//! `curl_is_told_what_to_ask_a_model.rs`; **what an adapter makes of the words that come back
//! is not covered, and that is the half that decides whether a model ever rules.**
//!
//! **A program and not a socket, because the seam is the port this adapter sits on.** The real
//! `curl` is `TheProgramAsItRuns` and it runs on a host in the tests that are `#[ignore]`d for
//! the network. What is unproved is not that a process can be started — it is that the words a
//! process wrote are read as a model's words and as nothing else.

use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_outside::the_model_as_it_is_asked::TheModelAsItIsAsked;
use aisdlc_run::AReasoningModel;
use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A program that answers with fixed words and keeps the one request it was given.
///
/// **The request is shared and not owned, because the program is moved into the adapter under
/// test** — and a test that cannot read what its own double was asked has proved that the
/// adapter asked for nothing.
struct AProgramAnsweringWith {
    what_it_would_answer_with: String,
    the_request_it_was_given: std::rc::Rc<std::cell::RefCell<Option<WhatWeTellAProgramToDo>>>,
}

impl AProgramAnsweringWith {
    /// A program that answers with these words, whatever it is asked to do.
    fn answering(what_it_would_answer_with: &str) -> Self {
        Self {
            what_it_would_answer_with: what_it_would_answer_with.to_string(),
            the_request_it_was_given: std::rc::Rc::new(std::cell::RefCell::new(None)),
        }
    }
}

impl AProgramWeRun for AProgramAnsweringWith {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        *self.the_request_it_was_given.borrow_mut() = Some(what_it_is_asked_to_do.clone());
        WhatTheProgramSaid::of(true, &self.what_it_would_answer_with)
    }
}

/// Where a deployment said its model answers, and which one answers there.
fn where_a_deployment_put_its_model() -> WhereAModelIs {
    WhereAModelIs::where_a_deployment_put_its_model(
        Some("http://localhost:11434/v1/chat/completions"),
        Some("a-model-a-deployment-named"),
    )
    .expect("a deployment named both halves or neither")
}

/// The artifact a caller asked for, by the name the pipeline holds.
fn the_artifact_that_was_asked_for() -> WhichArtifact {
    WhichArtifact::try_from("DiscoveryReport").expect("the pipeline declares that name")
}

/// The company whose model was asked, which is not the model.
fn which_organisation_reasons() -> WhichOrganisationReasons {
    WhichOrganisationReasons::try_from("openai_compatible").expect("four organisations reason")
}

/// The question an agent puts, which reaches the model whole.
fn a_question_about_the_checkout() -> &'static str {
    "What is this repository, and what would you change about it?"
}

/// The adapter and the program under it, and what the program was asked.
fn the_one_request_the_adapter_built(
    what_the_endpoint_would_answer_with: &str,
) -> (String, String) {
    let the_program = AProgramAnsweringWith::answering(what_the_endpoint_would_answer_with);
    let the_request = std::rc::Rc::clone(&the_program.the_request_it_was_given);
    let mut the_model = TheModelAsItIsAsked::the_model_where_a_deployment_put_its_model(
        where_a_deployment_put_its_model(),
        the_program,
    );

    the_model
        .what_it_says_when_asked_for(
            &the_artifact_that_was_asked_for(),
            which_organisation_reasons(),
            a_question_about_the_checkout(),
        )
        .expect("this endpoint answered something readable");

    let the_one_request = the_request.borrow().clone().expect("one request was made");
    the_endpoint_and_the_body_out_of(&the_one_request)
}

/// The address the request names and the body it carries, read out of the request itself.
fn the_endpoint_and_the_body_out_of(one_request: &WhatWeTellAProgramToDo) -> (String, String) {
    match one_request {
        WhatWeTellAProgramToDo::AskAModel {
            the_endpoint,
            the_body,
            ..
        } => (the_endpoint.clone(), the_body.clone()),
        _ => panic!("a model was asked and this request carries no address and no body"),
    }
}

/// What the adapter makes of what an endpoint wrote, through the port and nothing else.
fn what_a_caller_is_given(
    what_the_endpoint_would_answer_with: &str,
) -> Result<String, AnAnswerNobodyCanRead> {
    let mut the_model = TheModelAsItIsAsked::the_model_where_a_deployment_put_its_model(
        where_a_deployment_put_its_model(),
        AProgramAnsweringWith::answering(what_the_endpoint_would_answer_with),
    );
    the_model.what_it_says_when_asked_for(
        &the_artifact_that_was_asked_for(),
        which_organisation_reasons(),
        a_question_about_the_checkout(),
    )
}

/// An envelope in the shape an endpoint wraps a model's words in, built by the JSON library.
fn an_endpoint_answering_with(what_the_model_said: &str) -> String {
    let the_envelope = serde_json::json!({
        "id": "chatcmpl-1",
        "choices": [{"message": {"role": "assistant", "content": what_the_model_said}}]
    });
    serde_json::to_string(&the_envelope).expect("a json value is written")
}

#[test]
fn a_caller_is_given_the_models_own_words_and_not_the_envelope() {
    let what_the_endpoint_sent = an_endpoint_answering_with("APPROVED, decided_by: Alice");

    let what_a_caller_is_given = what_a_caller_is_given(&what_the_endpoint_sent);

    assert_eq!(
        what_a_caller_is_given,
        Ok("APPROVED, decided_by: Alice".to_string()),
        "**because a model asked to say one word answers with JSON around that word** — a reader \
         that handed the whole answer to a caller decides no model can ever rule, and **both \
         failures live in this one string**: the first word is the yes and the rest is the \
         forgery. What the endpoint wrote: {what_the_endpoint_sent:?}"
    );
}

#[test]
fn an_answer_nobody_can_read_is_refused_and_quotes_back_what_came() {
    let what_the_endpoint_said = "Failed to connect to localhost port 11434";

    let the_refusal = what_a_caller_is_given(what_the_endpoint_said);

    assert_eq!(
        the_refusal,
        Err(AnAnswerNobodyCanRead::because(what_the_endpoint_said)),
        "**because *Failed to connect to localhost port 11434* is what the first real run got, and \
         a reader that paraphrased it would have thrown away the only clue there was** — a caller \
         deciding whether to retry needs the words that came back, not a summary of them"
    );
}

#[test]
fn the_request_goes_to_the_address_the_deployment_named() {
    let where_a_deployment_said = where_a_deployment_put_its_model().the_endpoint;

    let (the_endpoint_it_was_asked_at, _) =
        the_one_request_the_adapter_built(&an_endpoint_answering_with("a discovery report"));

    assert_eq!(
        the_endpoint_it_was_asked_at, where_a_deployment_said,
        "**because the first real run fell back to `http://localhost:11434` because nobody had \
         read `LLM_ENDPOINT`, and the message that came back named the fallback** — a deployment \
         that named an address has to be asked at it, or this system is asking a model nobody \
         deployed"
    );
}

#[test]
fn the_body_carries_the_question_and_the_model_the_deployment_named() {
    let which_model_a_deployment_named = where_a_deployment_put_its_model().which_model;
    let the_question = a_question_about_the_checkout();

    let (_, the_body) = the_one_request_the_adapter_built(&an_endpoint_answering_with("a report"));

    let what_it_came_back_as: serde_json::Value =
        serde_json::from_str(&the_body).expect("the body a model is given is JSON");
    let the_content = what_it_came_back_as
        .get("messages")
        .and_then(|every| every.get(0))
        .and_then(|the_first| the_first.get("content"))
        .and_then(serde_json::Value::as_str)
        .expect("the question is the content of the first message")
        .to_string();
    let which_model_the_body_names = what_it_came_back_as["model"]
        .as_str()
        .map(str::to_string)
        .expect("the body names which model answers at that address");

    assert_eq!(
        (the_content.as_str(), which_model_the_body_names.as_str()),
        (the_question, which_model_a_deployment_named.as_str()),
        "**because the body was once written by hand and every request this system ever sent a \
         model was invalid JSON** — the far end answered `400` with zero bytes and the refusal \
         quoted an empty string. A library builds it, and it carries the question whole and names \
         the model the deployment named: {the_body}"
    );
}
