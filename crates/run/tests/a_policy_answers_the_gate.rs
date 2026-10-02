//! A policy answers the gate, and it reads the artifact off disk to do it.
//!
//! **A policy that can only approve is a stamp.** `approved` has been a boolean since before
//! anything could answer, and refusing leaves no `Approval` at all — **so refusing is the half that
//! stops work and approving is the half that hands it on.** A policy with no way to refuse is a
//! rubber stamp with a model's name on it.
//!
//! **It reads the same file a person would open**, and that is the whole of its honesty: a policy
//! ruling on a summary written for it **is not ruling on the artifact**, it is ruling on somebody's
//! description of it.

use aisdlc_domain::approval::{AnArtifactVersion, ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_run::a_policy_answers_the_gate::APolicyThatAnswersTheGate;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::who_answers_the_gate::WhoAnswersTheGate;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// An endpoint that answers with these words, **and records what it was asked.**
struct AnEndpointAnswering {
    the_words: String,
    did_the_call_succeed: bool,
    what_it_was_handed: std::cell::RefCell<Vec<String>>,
}

impl AnEndpointAnswering {
    /// A model that rules, **and gives a reason because the field requires one.**
    fn a_model_that_says(verdict: &str, reason: &str) -> Self {
        Self {
            the_words: format!("{verdict}\\n{reason}"),
            did_the_call_succeed: true,
            what_it_was_handed: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AProgramWeRun for AnEndpointAnswering {
    fn run(&self, what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.what_it_was_handed.borrow_mut().push(format!("{what_it_is_asked_to_do:?}"));
        WhatTheProgramSaid::of(self.did_the_call_succeed, &self.the_words)
    }
}

/// The artifact as it sits on disk, **and the policy has to be given it rather than told about it.**
fn the_artifact_on_disk() -> String {
    "A discovery report: two languages, a build system, a runner, and evidence files.".to_string()
}

fn a_question() -> ApprovalQuestion {
    let the_work_item = aisdlc_domain::work_item::WorkItem::standing_at(
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name"),
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        aisdlc_domain::role::Role::try_from("PO").expect("a discipline"),
        aisdlc_domain::stage::Stage::AwaitingHumanApproval,
    )
    .expect("a work item answering a request can stand at the gate");

    aisdlc_run::the_question_put_to_somebody(
        &the_work_item,
        &WhichArtifact::try_from("DiscoveryReport").expect("a name"),
        AnArtifactVersion::try_from(1).expect("a version"),
    )
    .expect("a work item at the gate can be asked about")
}

/// A policy answering under one name, **with a model that says what this test needs it to say.**
fn a_policy_answering(the_model: &AnEndpointAnswering) -> APolicyThatAnswersTheGate {
    APolicyThatAnswersTheGate::the_policy(
        "review-what-the-run-found",
        &the_model,
        || the_artifact_on_disk(),
        WhoAnswersTheGate::ThisPolicyAnswering {
            which_policy: String::from("review-what-the-run-found"),
        },
    )
}

#[test]
fn a_policy_that_approved_is_recorded_under_its_own_namespace() {
    let the_model = AnEndpointAnswering::a_model_that_says(
        "APPROVED",
        "the report names two languages and a build system",
    );
    let mut the_policy = a_policy_answering(&the_model);

    let the_ruling = the_policy.how_they_rule_on(&a_question());

    assert_eq!(
        (the_ruling.approved, the_ruling.decided_by.to_string()),
        (true, "autopilot:review-what-the-run-found"),
        "**because a policy that could be recorded as a person would forge the one thing this \
         record exists to be** — and the name in the record is the evidence, not the verdict. It \
         recorded: {:?} for {:?}",
        the_ruling.decided_by, the_ruling.about_which_artifact
    );
}

#[test]
fn a_policy_may_refuse_and_a_refusal_is_a_decision() {
    let the_model =
        AnEndpointAnswering::a_model_that_says("REFUSED", "the report cites a file that is not there");
    let mut the_policy = a_policy_answering(&the_model);

    let the_ruling = the_policy.how_they_rule_on(&a_question());

    assert!(
        !the_ruling.approved,
        "**because a policy that can only approve is a stamp** — `approved` is a boolean and a \
         refusal leaves no record at all, **so refusing is the half that stops work and approving is \
         the half that hands it on.** It refused, with the reason {:?}",
        the_ruling.reason
    );
}

#[test]
fn a_policy_is_handed_the_artifact_and_not_a_description_of_it() {
    let the_model = AnEndpointAnswering::a_model_that_says("APPROVED", "the report names its build system");
    let mut the_policy = a_policy_answering(&the_model);

    let _ = the_policy.how_they_rule_on(&a_question());

    let every_call = the_policy.what_it_was_handed.borrow().join("\n");
    assert!(
        every_call.contains("two languages") && every_call.contains("DiscoveryReport"),
        "**because a policy ruling on a summary written for it is not ruling on the artifact** — it \
         is ruling on somebody's description of it, **and the description is a lossy thing produced \
         by the very run being judged.** What the model was handed: {every_call}"
    );
}

#[test]
fn an_answer_that_is_no_ruling_leaves_nothing_and_is_not_an_approval() {
    let the_model = AnEndpointAnswering::a_model_that_says("looks good to me", "no reason at all");
    let mut the_policy = a_policy_answering(&the_model);

    let the_ruling = the_policy.how_they_rule_on(&a_question());
    let what_it_left_behind =
        aisdlc_domain::approval::Approval::what_the_ruling_left_behind(&the_ruling);

    assert_eq!(
        what_it_left_behind, None,
        "**because a guess at what a model meant is a decision nobody made** — and this gate records \
         decisions, so an answer with no shape in it is a refusal rather than a ruling with `true` \
         somewhere in it. It was read as: {:?}",
        the_ruling.decided_by
    );
}

#[test]
fn an_endpoint_that_could_not_be_reached_leaves_nothing_behind() {
    let a_broken_endpoint = AnEndpointAnswering {
        the_words: String::from("the endpoint was down"),
        did_the_call_succeed: false,
        what_it_was_handed: std::cell::RefCell::new(Vec::new()),
    };
    let mut the_policy = a_policy_answering(&a_broken_endpoint);

    let the_ruling = the_policy.how_they_rule_on(&a_question());
    let what_it_left_behind =
        aisdlc_domain::approval::Approval::what_the_ruling_left_behind(&the_ruling);

    assert_eq!(
        what_it_left_behind, None,
        "**because the direction of this failure is the whole point** — an answer that cannot be \
         read must fail toward a refusal, **and a reader that fails the other way puts a model at a \
         human's gate on the strength of a bug.**"
    );
}
