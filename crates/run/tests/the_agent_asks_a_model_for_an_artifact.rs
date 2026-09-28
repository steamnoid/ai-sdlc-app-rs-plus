//! An agent asks a model for an artifact and gets back something that is not one yet.
//!
//! **The answer is not yet an artifact, and the type is the point.** Nothing guarantees a
//! model honoured the request, so a type called `QaPlan` on the way out of a model call would
//! be a reader treating a dictionary as a plan. Nothing in this system may hand back an
//! artifact from a call that has not validated one, and the way to make that true is for the
//! return type to have no name that could be mistaken for one.
//!
//! **The first port with a caller, and the caller is the reason it exists.** A port with no
//! implementation behind it is not a port; it is a trait nobody calls. The glossary says one
//! arrives with the first use case that needs it — this is that use case, and without it the
//! port would be a declaration nobody could have pointed at.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::{WorkItem, a_work_item_commissioned_for_an_agent};
use aisdlc_domain::work_item_id::WorkItemId;

/// A model that answers with one fixed sentence, and records what it was asked for.
///
/// **A double that answers is a seam; a double nobody composes is a class.** This one is
/// called by the test below and its question is asserted, so it is composing with something.
struct AModelThatAnswers {
    what_it_was_asked_for: Vec<WhichArtifact>,
    the_questions_it_was_asked: Vec<String>,
    the_answer: String,
}

impl AModelThatAnswers {
    fn answering(the_answer: &str) -> Self {
        Self {
            what_it_was_asked_for: Vec::new(),
            the_questions_it_was_asked: Vec::new(),
            the_answer: the_answer.to_string(),
        }
    }
}

impl aisdlc_run::AReasoningModel for AModelThatAnswers {
    fn what_it_says_when_asked_for(
        &mut self,
        which_artifact: &WhichArtifact,
        the_question: &str,
    ) -> String {
        self.what_it_was_asked_for.push(which_artifact.clone());
        self.the_questions_it_was_asked
            .push(the_question.to_string());
        self.the_answer.clone()
    }
}

fn the_agent_who_will_do_it() -> AgentIdentifier {
    AgentIdentifier::try_from("aisdlc-po-analyze").expect("a name is not blank")
}

fn a_work_item_this_agent_is_working_on() -> WorkItem {
    aisdlc_run::the_agent_takes_the_work_item_into_its_own_hands(
        aisdlc_run::the_framework_picks_the_work_up(
            a_work_item_somebody_is_working_on(),
            the_agent_who_will_do_it(),
        )
        .expect("a work item standing at `READY` may be picked up"),
        the_agent_who_will_do_it(),
    )
    .expect("an agent may start work it has been handed")
}

fn the_artifact_being_asked_for() -> WhichArtifact {
    WhichArtifact::try_from("DiscoveryReport").expect("an artifact has a name")
}

fn a_work_item_somebody_is_working_on() -> WorkItem {
    a_work_item_commissioned_for_an_agent(
        WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank"),
        WhoCommissioned::try_from("paligakrzychu").expect("a name is not blank"),
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        Role::Product,
        Some(
            CommissionedRequest::asked_for(
                "Name the stages a work item may be at".to_string(),
                "A reader should not have to read the code to learn them.".to_string(),
            )
            .expect("a request with a title and a body is a request"),
        ),
    )
}

#[test]
fn an_agent_asking_a_model_asks_for_the_artifact_its_work_item_needs() {
    let mut the_model = AModelThatAnswers::answering("this repository is a wellbeing tracker");
    let the_work_item = a_work_item_this_agent_is_working_on();

    aisdlc_run::the_agent_asks_for_its_artifact(
        &mut the_model,
        &the_work_item,
        &the_artifact_being_asked_for(),
    );

    assert_eq!(
        the_model.what_it_was_asked_for,
        vec![the_artifact_being_asked_for()],
        "and it asked for what it was told to ask for, in one call and not three"
    );
    let the_request = the_work_item
        .the_request_it_answers()
        .expect("a commissioned work item answers a request");
    assert!(
        the_model.the_questions_it_was_asked[0].contains(&the_request.title),
        "**the model is asked the request somebody actually typed**, and a model asked a question nobody asked is a model answering a question of its own"
    );
}

#[test]
fn what_a_model_says_comes_back_as_an_answer_and_not_as_an_artifact() {
    let mut the_model = AModelThatAnswers::answering("this repository is a wellbeing tracker");
    let the_work_item = a_work_item_this_agent_is_working_on();

    let the_answer = aisdlc_run::the_agent_asks_for_its_artifact(
        &mut the_model,
        &the_work_item,
        &the_artifact_being_asked_for(),
    );

    assert_eq!(
        the_answer.what_the_model_said(),
        "this repository is a wellbeing tracker",
        "the text is exactly what came back, because a reader who cannot see the raw words cannot tell a validated artifact from an invention"
    );
    assert_eq!(
        the_answer.which_artifact_was_asked_for(),
        the_artifact_being_asked_for(),
        "and it knows which artifact it is standing in for, because a run asks eight agents for eight artifacts and 'the shape was wrong' without saying which is a question rather than a diagnosis"
    );
}

#[test]
fn the_answer_carries_the_request_it_was_given() {
    let mut the_model = AModelThatAnswers::answering("whatever");
    let the_work_item = a_work_item_this_agent_is_working_on();

    let the_answer = aisdlc_run::the_agent_asks_for_its_artifact(
        &mut the_model,
        &the_work_item,
        &the_artifact_being_asked_for(),
    );

    let the_request = the_work_item
        .the_request_it_answers()
        .expect("a commissioned work item answers a request");

    assert!(
        the_answer
            .the_question_it_was_given()
            .contains(&the_request.title),
        "**the request is what somebody actually typed**, and a model asked a question nobody asked is a model answering a question of its own"
    );
}
