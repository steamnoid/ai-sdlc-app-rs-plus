//! A stage keeps what the agent said, and every production of an artifact is a new version of it.
//!
//! **A gate that approves a document nobody wrote is the defect the boundary exists to prevent.**
//! `a_stage_is_worked_and_ruled_on` asks the model and **throws the words away** — which is honest
//! only while no artifact reaches disk, and the moment one does this repository would be approving a
//! claim about a file it never wrote.
//!
//! **A second production of the same artifact is a different document, and it has to be a different
//! version.** An approval names the exact artifact *and version* it ruled on, **and it stops being
//! valid the moment that artifact changes** — so a plan rewritten after a vacuous test must be a
//! later version of the plan, or the approval a person gave the first one would silently stand in
//! for the second.

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::a_stage_is_worked_and_ruled_on::a_stage_is_worked_and_ruled_on;
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A model that answers with these exact words, **and they come back out or the test is green for
/// the wrong reason.**
struct AModelAnsweringWith {
    the_words: String,
    can_it_be_reached: bool,
}

impl AModelAnsweringWith {
    fn saying(what: &str) -> Self {
        Self {
            the_words: what.to_string(),
            can_it_be_reached: true,
        }
    }

    fn unreachable() -> Self {
        Self {
            the_words: String::new(),
            can_it_be_reached: false,
        }
    }
}

impl AReasoningModel for AModelAnsweringWith {
    fn what_it_says_when_asked_for(
        &mut self,
        _which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        if !self.can_it_be_reached {
            return Err(AnAnswerNobodyCanRead::because("there is no endpoint"));
        }
        Ok(self.the_words.clone())
    }
}

/// A person who approves, **and records which version they were asked about.**
struct APersonWhoApproves {
    what_they_were_asked: RefCell<Vec<(String, u32)>>,
}

impl APersonWhoApproves {
    fn approving() -> Self {
        Self {
            what_they_were_asked: RefCell::new(Vec::new()),
        }
    }

    fn asked_about(&self) -> Vec<(String, u32)> {
        self.what_they_were_asked.borrow().clone()
    }
}

impl APersonToDecide for APersonWhoApproves {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        self.what_they_were_asked.borrow_mut().push((
            a_question.about_which_artifact().0.clone(),
            a_question.about_which_version().0,
        ));
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            true,
            WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
            "the artifact answers the request",
        )
        .expect("a person may rule")
    }
}

#[test]
fn a_stage_keeps_exactly_what_the_agent_said_and_not_a_summary_of_it() {
    let mut a_model = AModelAnsweringWith::saying("the repository is a self-hosted Plane");
    let the_words_the_agent_said = "the repository is a self-hosted Plane";

    let worked = the_stage_worked_with(&mut a_model, &mut APersonWhoApproves::approving());

    assert_eq!(
        worked.as_ref().map(WhatTheStageLeft::what_the_agent_said),
        Ok(String::from(the_words_the_agent_said)),
        "**because a summary is prose nobody approved** — the gate rules on the document, and a \
         document that is not the one the agent wrote is a document nobody has read. It kept {:?}",
        worked.as_ref().map(WhatTheStageLeft::what_the_agent_said)
    );
}

#[test]
fn the_first_thing_a_stage_produces_is_version_one_of_its_artifact() {
    let mut a_model = AModelAnsweringWith::saying("a discovery report");

    let worked = the_stage_worked_with(&mut a_model, &mut APersonWhoApproves::approving());

    assert_eq!(
        worked
            .as_ref()
            .map(|left| left.which_version_was_produced().0),
        Ok(1),
        "**because a version counts from one and a stage that has never been revised has one \
         version and not zero** — a gate asked about version zero could not tell a fresh artifact \
         from one that had been rewritten. It produced {:?}",
        worked
            .as_ref()
            .map(|left| left.which_version_was_produced().0)
    );
}

#[test]
fn the_same_artifact_produced_again_is_a_later_version_and_the_gate_is_asked_about_it() {
    let mut a_model = AModelAnsweringWith::saying("a discovery report, the second time");
    let mut whoever_answers = APersonWhoApproves::approving();
    let the_run = a_run_on_this_artifact_at_version(1);

    let worked = the_stage_worked_with_this_run(&the_run, &mut a_model, &mut whoever_answers);
    let the_version = worked
        .as_ref()
        .map(|left| left.which_version_was_produced().0);

    assert_eq!(
        the_version,
        Ok(2),
        "**because the second document is not the first document** — and an approval naming version \
         one standing in for version two **is an approval a person never gave.** It produced \
         version {the_version:?}"
    );
    assert_eq!(
        whoever_answers.asked_about(),
        vec![(String::from("DiscoveryReport"), 2)],
        "**because the gate is asked about the version the stage just produced** — and a gate asked \
         about the old one would be approving a document that has already been replaced. They were \
         asked about {:?}",
        whoever_answers.asked_about()
    );
}

#[test]
fn a_different_artifact_is_version_one_whatever_happened_to_the_last_one() {
    let mut a_model = AModelAnsweringWith::saying("the requirements");
    let the_run = a_run_handed_to_the_next_artifact();

    let worked = the_stage_worked_with_this_run(
        &the_run,
        &mut a_model,
        &mut APersonWhoApproves::approving(),
    );

    assert_eq!(
        worked
            .as_ref()
            .map(|left| left.which_version_was_produced().0),
        Ok(1),
        "**because a version counts within one artifact and not across the pipeline** — a \
         requirements document at version eight would be a document nobody has seen seven of. It \
         produced version {:?}",
        worked
            .as_ref()
            .map(|left| left.which_version_was_produced().0)
    );
}

/// A run handed on to the next artifact the way a run hands work on, **and the version it carries is
/// the one the move gave it — which is none.**
///
/// **The fixture this replaces was wrong in a way worth recording.** It told the run it was on
/// version seven of `DiscoveryReport` and then asserted that a *different* artifact came back at
/// version one — **and the code answered eight, correctly**, because it was the same artifact. The
/// only place a version is reset is the run's own move onto a different one, **so that is what the
/// fixture has to do to ask the question at all.**
fn a_run_handed_to_the_next_artifact() -> WhereARunStands {
    let on_the_first_artifact = a_run_on_this_artifact_at_version(7);
    let the_product_analyst =
        aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-po-analyze")
            .expect("an agent name");
    let handed_on = aisdlc_run::where_a_run_stands::the_work_is_handed_to_the_next_agent(
        &on_the_first_artifact,
        the_product_analyst.clone(),
        aisdlc_domain::approval::WhichArtifact::try_from("Requirements").expect("a name"),
    );
    let at_a_pickup = handed_on.expect("the run may hand its work to the next stage");
    let picked_up = aisdlc_run::where_a_run_stands::the_work_is_handed_to_an_agent(
        &at_a_pickup,
        the_product_analyst,
    );
    picked_up.expect("the next agent may pick the work up")
}

#[test]
fn an_agent_that_could_not_be_reached_leaves_no_document_and_no_version() {
    let mut a_model = AModelAnsweringWith::unreachable();

    let refused = the_stage_worked_with(&mut a_model, &mut APersonWhoApproves::approving());

    assert!(
        refused.is_err(),
        "**because there is no document to keep and a gate asked to approve a version of nothing is \
         a gate approving a claim** — and the version would sit on the run looking like a fact. It \
         said: {refused:?}"
    );
}

/// The stage worked from the ordinary run, **and it is one call so four tests are about their own
/// arrangement.**
fn the_stage_worked_with(
    a_model: &mut AModelAnsweringWith,
    whoever_answers: &mut APersonWhoApproves,
) -> Result<WhatTheStageLeft, aisdlc_run::a_stage_is_worked_and_ruled_on::AStageThatCouldNotBeWorked>
{
    the_stage_worked_with_this_run(
        &a_run_on_this_artifact_at_version(0),
        a_model,
        whoever_answers,
    )
}

/// The stage worked from whatever run it is given, **and the run is an argument because two tests
/// work the same artifact twice and one works a different one.**
fn the_stage_worked_with_this_run(
    the_run: &WhereARunStands,
    a_model: &mut AModelAnsweringWith,
    whoever_answers: &mut APersonWhoApproves,
) -> Result<WhatTheStageLeft, aisdlc_run::a_stage_is_worked_and_ruled_on::AStageThatCouldNotBeWorked>
{
    a_stage_is_worked_and_ruled_on(
        the_run,
        a_model,
        WhichOrganisationReasons::Openai,
        whoever_answers,
    )
}

/// A run standing on `DiscoveryReport` at a version, **and zero means it has never been produced.**
fn a_run_on_this_artifact_at_version(the_version: u32) -> WhereARunStands {
    let the_request = aisdlc_domain::commissioned_request::CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    };
    let answering = a_work_item_held_by_the_discovering_agent().answering(the_request);
    let the_run = WhereARunStands::at_the_first_stage(
        answering.expect("a work item answering a request may stand where it was put"),
        "DiscoveryReport",
    );
    if the_version == 0 {
        return the_run;
    }
    let the_version =
        aisdlc_domain::approval::AnArtifactVersion::try_from(the_version).expect("a version");
    the_run
        .which_artifact_is_at(the_version)
        .expect("the run may say which version of its artifact exists")
}

/// The work item the discovery agent is holding, **and a request is added to it afterwards.**
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

use aisdlc_run::a_stage_is_worked_and_ruled_on::WhatTheStageLeft;
