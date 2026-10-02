//! A run goes from a request to a pull request, and the work item is finished.
//!
//! **A whole run stops at the delivery and a delivery finishes a run, and nothing joined the two.**
//! `the_whole_of_a_run` ends standing at the delivery with `aisdlc-pr` holding the work — **because a
//! delivery pushes a branch and opens a pull request, neither of which is a stage a person rules on.**
//! `the_run_is_delivered` takes a run already standing there. **So this repository could walk seven
//! gates and then stop, which is a run that did all the work and delivered none of it.**

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_branch_goes_to_a_remote::ABranchToPush;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::a_pull_request_is_opened::{
    APullRequestToOpen, the_request_a_run_makes_of_its_own_change,
};
use aisdlc_run::a_run_from_a_request_to_a_pull_request::{
    ARunThatDidNotFinish, WhatThereIsToDeliver, a_run_from_a_request_to_a_pull_request,
};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A model that answers everything, **and counts how often it was asked.**
struct AModelAnswering {
    how_often: RefCell<usize>,
}

impl AReasoningModel for AModelAnswering {
    fn what_it_says_when_asked_for(
        &mut self,
        _which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        *self.how_often.borrow_mut() += 1;
        Ok(String::from("an artifact, of the length a real one is"))
    }
}

/// A person who approves everything up to one gate, **and refuses at that one and nowhere else.**
struct APersonWhoRules {
    refuses_at_the: Option<usize>,
    how_often: RefCell<usize>,
}

impl APersonWhoRules {
    fn approving_everything() -> Self {
        Self {
            refuses_at_the: None,
            how_often: RefCell::new(0),
        }
    }

    fn refusing_at_the(nth: usize) -> Self {
        Self {
            refuses_at_the: Some(nth),
            how_often: RefCell::new(0),
        }
    }

    fn how_often_were_they_asked(&self) -> usize {
        *self.how_often.borrow()
    }
}

impl APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let nth = self.how_often.borrow().saturating_add(1);
        *self.how_often.borrow_mut() = nth;
        let approved = self.refuses_at_the != Some(nth);
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            approved,
            WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
            "the artifact answers the request",
        )
        .expect("a person may rule")
    }
}

/// A program that answers `git` and `gh`, **and refuses the pull request when told to.**
struct AProgramAnswering {
    refuses_the_pull_request: bool,
}

impl AProgramWeRun for AProgramAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        let (did_it_work, what_it_says) = self.what_it_says_for(what_it_is_asked_to_do);
        WhatTheProgramSaid::of(did_it_work, &what_it_says)
    }
}

impl AProgramAnswering {
    fn what_it_says_for(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> (bool, String) {
        match what_it_is_asked_to_do {
            WhatWeTellAProgramToDo::ReadTheRemote { .. } => (true, String::new()),
            WhatWeTellAProgramToDo::PushTheBranch { .. } => (true, String::from("to ../remote.git")),
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } if self.refuses_the_pull_request => {
                (false, String::from("gh: no"))
            }
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } => (
                true,
                String::from(
                    r#"{"number":17,"html_url":"https://github.com/steamnoid/wellbeing-tracker-public/pull/17"}"#,
                ),
            ),
            every_other_request => (true, format!("{every_other_request:?}")),
        }
    }
}

#[test]
fn a_run_from_a_request_to_a_pull_request_leaves_the_work_item_finished() {
    let mut a_model = AModelAnswering {
        how_often: RefCell::new(0),
    };
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let finished = a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut whoever_answers,
        &what_there_is_to_deliver(),
        &a_program_answering(false),
        &mut APersonWhoRules::approving_everything(),
    );

    assert_eq!(
        finished
            .as_ref()
            .map(|what| what.where_the_work_stands().the_stage_it_stands_at()),
        Ok(Stage::Done),
        "**because seven gates and a pull request and a work item still standing at a stage is a run \
         that did all the work and delivered none of it** — and `DONE` is the stage the table has \
         exactly one edge into. It stands at {:?}",
        finished
            .as_ref()
            .map(|what| what.where_the_work_stands().the_stage_it_stands_at())
    );
}

#[test]
fn a_run_that_asked_every_gate_asks_the_seventh_person_about_the_delivery_and_not_about_an_artifact() {
    let mut a_model = AModelAnswering {
        how_often: RefCell::new(0),
    };
    let mut during_the_run = APersonWhoRules::approving_everything();
    let mut at_the_delivery = APersonWhoRules::approving_everything();

    a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut during_the_run,
        &what_there_is_to_deliver(),
        &a_program_answering(false),
        &mut at_the_delivery,
    )
    .expect("a run every gate approves finishes");

    assert_eq!(
        during_the_run.how_often_were_they_asked(),
        7,
        "**because seven artifacts and seven gates is the whole of the pipeline** — and an eighth \
         question during the run would be a gate nobody can name. They were asked {} times",
        during_the_run.how_often_were_they_asked()
    );
    assert_eq!(
        at_the_delivery.how_often_were_they_asked(),
        1,
        "**because the delivery is the eighth gate and it is a gate** — a delivery that finished a \
         run without asking would put a decision in the record that nobody made. They were asked {} \
         times",
        at_the_delivery.how_often_were_they_asked()
    );
}

#[test]
fn what_a_run_left_names_the_pull_request_github_opened() {
    let mut a_model = AModelAnswering {
        how_often: RefCell::new(0),
    };

    let finished = a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::approving_everything(),
        &what_there_is_to_deliver(),
        &a_program_answering(false),
        &mut APersonWhoRules::approving_everything(),
    );

    let where_it_answers = finished
        .as_ref()
        .map(|what| what.which_pull_request().where_it_answers.clone());
    assert_eq!(
        where_it_answers,
        Ok(String::from(
            "https://github.com/steamnoid/wellbeing-tracker-public/pull/17"
        )),
        "**because a delivery that recorded an address it composed would record a link to a page \
         that does not exist** — and the run's whole record is that link. It answers at \
         {where_it_answers:?}"
    );
}

#[test]
fn a_run_somebody_refused_in_the_middle_delivers_nothing_and_is_not_finished() {
    let mut a_model = AModelAnswering {
        how_often: RefCell::new(0),
    };
    let mut at_the_delivery = APersonWhoRules::approving_everything();

    let stopped = a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::refusing_at_the(3),
        &what_there_is_to_deliver(),
        &a_program_answering(false),
        &mut at_the_delivery,
    );

    assert_eq!(
        stopped
            .as_ref()
            .map(|what| what.where_the_work_stands().the_stage_it_stands_at()),
        Ok(Stage::Done),
        "**because refusing ends the run and the run is at `DONE` rather than half-delivered** — \
         there is no pull request and the work item is closed. It stands at {:?}",
        stopped
            .as_ref()
            .map(|what| what.where_the_work_stands().the_stage_it_stands_at())
    );
    assert_eq!(
        at_the_delivery.how_often_were_they_asked(),
        0,
        "**because nobody was asked about a delivery that never happened** — a gate asked after a \
         refusal is a gate asked to rule on work a person has already stopped. They were asked {} \
         times",
        at_the_delivery.how_often_were_they_asked()
    );
}

#[test]
fn a_delivery_github_refused_stops_the_run_and_leaves_the_work_item_at_the_gate() {
    let mut a_model = AModelAnswering {
        how_often: RefCell::new(0),
    };

    let stopped = a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::approving_everything(),
        &what_there_is_to_deliver(),
        &a_program_answering(true),
        &mut APersonWhoRules::approving_everything(),
    );

    assert!(
        matches!(stopped, Err(ARunThatDidNotFinish::TheDelivery { .. })),
        "**because a branch on a remote is not a delivery** — it is something nobody can see, and a \
         work item finished on it would be finished on a claim. It refused: {stopped:?}"
    );
}

/// A program answering, **and it is an argument so one test can refuse the pull request.**
fn a_program_answering(refuses_the_pull_request: bool) -> AProgramAnswering {
    AProgramAnswering {
        refuses_the_pull_request,
    }
}

/// What there is to deliver, **and it is one value rather than four arguments** — the four are one
/// fact about one delivery and a reader would otherwise have to hold them in step.
fn what_there_is_to_deliver() -> WhatThereIsToDeliver {
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_clone = ARepositoryToClone::of(
        the_repository.clone(),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        std::path::PathBuf::from("/tmp/a-clone-this-test-never-reads"),
    );
    let the_branch = ABranchToPush::a_branch_named("aisdlc/WELLBEINGT-42")
        .expect("a branch may take that name");
    let what_to_open = the_request_a_run_makes_of_its_own_change(
        the_repository.clone(),
        the_repository,
        "aisdlc/WELLBEINGT-42",
        "main",
        "the E2E database leaves artifacts behind",
        "A file that was not there, and a test that says so.",
    )
    .expect("a title and a body were both given");
    let the_credential = ACredential::the_key_a_deployment_supplied("a-key-not-a-real-one")
        .expect("a key");
    WhatThereIsToDeliver::for_this_run(
        the_clone,
        the_branch,
        what_to_open,
        the_credential,
    )
    .expect("all four are names this run can carry")
}

/// A run standing at the stage its first agent is working on, **and on the artifact it produces.**
fn a_run_at_its_first_stage() -> WhereARunStands {
    let the_request = aisdlc_domain::commissioned_request::CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    };
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("an agent name");
    let the_work_item = aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::InProgressByAgent,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one")
    .answering(the_request)
    .expect("a work item answering a request may stand where it was put");
    WhereARunStands::at_the_first_stage(the_work_item, "DiscoveryReport")
}

fn the_four_facts() -> (
    aisdlc_domain::work_item_id::WorkItemId,
    aisdlc_domain::who_commissioned::WhoCommissioned,
    Repository,
    aisdlc_domain::role::Role,
) {
    let the_id = aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-42").expect("a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("DEV").expect("a discipline");
    (the_id, who_asked, the_repository, the_discipline)
}
