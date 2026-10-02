//! A stage is worked and then a person rules on what it produced.
//!
//! **The walk moves the work and nobody was asked.** Cycle 181's `a_run_is_walked_forward` takes a
//! ruling per stage as an argument, **so it can walk a run whose artifacts do not exist** — and this
//! repository has no function that asks an agent for what it is to produce.
//!
//! **A stage does three things and the walk does not do any of them**: the agent holding the work is
//! asked, the work is presented for a ruling, and somebody answers. **Only the fourth — whether the
//! answer was yes — is what the walk consumes**, and that is deliberately the smallest thing this
//! function hands back.

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::a_stage_is_worked_and_ruled_on::{
    AStageThatCouldNotBeWorked, WhatTheStageLeft, a_stage_is_worked_and_ruled_on,
};
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A model that answers, **and records what it was asked for.**
struct AModelThatAnswers {
    what_it_says: Option<String>,
    what_it_was_asked_for: RefCell<Vec<String>>,
}

impl AModelThatAnswers {
    fn saying(what: &str) -> Self {
        Self {
            what_it_says: Some(String::from(what)),
            what_it_was_asked_for: RefCell::new(Vec::new()),
        }
    }

    fn answering_with_nothing() -> Self {
        Self {
            what_it_says: None,
            what_it_was_asked_for: RefCell::new(Vec::new()),
        }
    }
}

impl AReasoningModel for AModelThatAnswers {
    fn what_it_says_when_asked_for(
        &mut self,
        which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        self.what_it_was_asked_for
            .borrow_mut()
            .push(which_artifact.0.clone());
        self.what_it_says
            .clone()
            .ok_or_else(|| AnAnswerNobodyCanRead::because("there is no endpoint"))
    }
}

/// A person who rules the way this says, **and records what they were asked about.**
struct APersonWhoRules {
    approved: bool,
    what_they_were_asked: RefCell<Vec<(String, u32)>>,
}

impl APersonWhoRules {
    fn who_approves() -> Self {
        Self {
            approved: true,
            what_they_were_asked: RefCell::new(Vec::new()),
        }
    }

    fn who_refuses() -> Self {
        Self {
            approved: false,
            what_they_were_asked: RefCell::new(Vec::new()),
        }
    }

    fn asked_about(&self) -> Vec<(String, u32)> {
        self.what_they_were_asked.borrow().clone()
    }
}

impl APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let the_version = a_question.about_which_version().0;
        self.what_they_were_asked
            .borrow_mut()
            .push((a_question.about_which_artifact().0.clone(), the_version));
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            self.approved,
            WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
            "the artifact answers the request",
        )
        .expect("a person may rule")
    }
}

#[test]
fn the_agent_holding_the_work_is_asked_for_the_artifact_the_run_is_on() {
    let mut a_model = AModelThatAnswers::saying("a discovery report");

    let worked = a_stage_is_worked_and_ruled_on(
        &a_run_in_the_middle_of_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::who_approves(),
    );

    assert_eq!(
        worked.as_ref().err(),
        None,
        "**because a stage that was never asked produces nothing to rule on** — and the walk would \
         happily have carried on. It said: {worked:?}"
    );
    assert_eq!(
        a_model.what_it_was_asked_for.borrow().clone(),
        vec![String::from("DiscoveryReport")],
        "**because the artifact comes from the run and not from the caller** — a caller naming it \
         could ask for the artifact a stage is not producing, **and the run would then be ruled on \
         something it never wrote.** It was asked for {:?}",
        a_model.what_it_was_asked_for.borrow()
    );
}

#[test]
fn a_person_is_asked_about_the_artifact_and_the_version_it_is_on() {
    let mut a_model = AModelThatAnswers::saying("a discovery report");
    let mut whoever_answers = APersonWhoRules::who_approves();

    a_stage_is_worked_and_ruled_on(
        &a_run_in_the_middle_of_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut whoever_answers,
    )
    .expect("a stage that was asked can be ruled on");

    assert_eq!(
        whoever_answers.asked_about(),
        vec![(String::from("DiscoveryReport"), 1)],
        "**because an approval naming a version is invalidated when that artifact changes** — and \
         a gate that asked for no version could approve a document it never saw. It was asked \
         about {:?}",
        whoever_answers.asked_about()
    );
}

#[test]
fn a_person_who_approves_leaves_an_approval_naming_them_verbatim() {
    let mut a_model = AModelThatAnswers::saying("a discovery report");

    let worked = a_stage_is_worked_and_ruled_on(
        &a_run_in_the_middle_of_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::who_approves(),
    );

    assert_eq!(
        worked
            .as_ref()
            .map(WhatTheStageLeft::was_the_artifact_approved),
        Ok(true),
        "**because a ruling nobody answered leaves the run where it was** — and a walk handed a \
         `true` it invented is a run approving its own work. It said: {worked:?}"
    );
}

#[test]
fn a_person_who_refuses_leaves_no_approval_and_the_stage_says_so() {
    let mut a_model = AModelThatAnswers::saying("a discovery report");

    let worked = a_stage_is_worked_and_ruled_on(
        &a_run_in_the_middle_of_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::who_refuses(),
    );

    assert_eq!(
        worked
            .as_ref()
            .map(WhatTheStageLeft::was_the_artifact_approved),
        Ok(false),
        "**because refusing is the half that stops work** — and a stage that reported a refusal as \
         a delay would be a run treating a decision as a delay, which is the defect behind three \
         delivered pull requests whose diff was two test files and no route. It said: {worked:?}"
    );
}

#[test]
fn a_model_that_could_not_be_reached_stops_the_stage_and_names_the_artifact() {
    let mut a_model = AModelThatAnswers::answering_with_nothing();

    let refused = a_stage_is_worked_and_ruled_on(
        &a_run_in_the_middle_of_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::who_approves(),
    );

    assert!(
        matches!(
            refused,
            Err(AStageThatCouldNotBeWorked::TheAgentCouldNotBeAsked { .. })
        ),
        "**because nobody was asked and therefore nothing exists to rule on** — and a stage that \
         asked somebody to rule on an artifact that was never written is a gate approving a \
         document nobody produced. It refused: {refused:?}"
    );
}

/// A run standing at the stage its first agent is working on, **and on the artifact that agent
/// produces.**
fn a_run_in_the_middle_of_its_first_stage() -> WhereARunStands {
    let the_request = aisdlc_domain::commissioned_request::CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    };
    let the_work_item = a_work_item_held_by_the_discovering_agent();
    let answering = the_work_item.answering(the_request);
    WhereARunStands::at_the_first_stage(
        answering.expect("a work item answering a request may stand where it was put"),
        "DiscoveryReport",
    )
}

/// The work item the discovery agent is holding, **and a request is added to it afterwards** — the
/// builder takes the request rather than the constructor, **so the two facts are separate and a
/// caller that forgot the request is refused by the model rather than silently asked nothing.**
fn a_work_item_held_by_the_discovering_agent() -> aisdlc_domain::work_item::WorkItem {
    let the_id = aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves");
    let the_discipline = Role::try_from("PO").expect("a discipline");
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("an agent name");
    aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::InProgressByAgent,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one")
}
