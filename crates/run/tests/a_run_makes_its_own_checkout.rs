//! A run makes its own checkout before anybody is asked about anything.
//!
//! **Every cycle so far worked on artifacts and rulings, and not one of them had a repository on
//! disk.** `the_run_makes_a_checkout` exists, it is tested, and **nothing in the pipeline calls it**
//! — so a whole run asked seven models about a repository it had never got, and the delivery pushed
//! from a path a *caller* named rather than the copy the run made.
//!
//! **One clone, and it is the one the run made.** `WhatThereIsToDeliver` carries an
//! `ARepositoryToClone` — **a destination the caller wrote down** — and that is a second answer to
//! the question the run already answers. **A delivery that pushes a clone the run never read is a
//! delivery of something else.**
//!
//! **The clone comes first, and the order is the point.** A run that asked a model about a
//! repository before copying it would be a run reasoning about a description, **which is the whole
//! defect this system was built to avoid.**

use std::cell::RefCell;

#[path = "doubles/a_model_that_answers.rs"]
mod a_model_that_answers;
#[path = "doubles/a_person_who_rules.rs"]
mod a_person_who_rules;

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_branch_goes_to_a_remote::ABranchToPush;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::a_pull_request_is_opened::{
    APullRequestToOpen, the_request_a_run_makes_of_its_own_change,
};
use aisdlc_run::a_run_from_a_request_to_a_pull_request::{
    ARunThatDidNotFinish, WhatARunLeft, WhatThereIsToDeliver,
    a_run_from_a_request_to_a_pull_request,
};
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use aisdlc_run::where_a_run_stands::WhereARunStands;

use a_model_that_answers::AModelAnswering;
use a_person_who_rules::APersonWhoRules;
use aisdlc_run::whether_anything_stands_there::WhetherAnythingStandsThere;

#[test]
fn the_repository_is_copied_before_any_agent_is_asked_anything() {
    let mut what_the_program_was_asked = AProgramAnswering::answering_everything();
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let _finished = a_run_given(
        &mut what_the_program_was_asked,
        &mut a_model,
        &mut whoever_answers,
    );

    assert_eq!(
        what_the_program_was_asked.what_it_was_asked_about(),
        vec!["clone".to_string()],
        "**because a run that asked a model about a repository before copying it was reasoning \
         about a description** — which is the whole defect this system was built to avoid. It asked \
         {:?}",
        what_the_program_was_asked.what_it_was_asked_about()
    );
}

#[test]
fn a_run_whose_checkout_failed_asks_nobody_and_delivers_nothing() {
    let mut what_the_program_was_asked =
        AProgramAnswering::refusing_the_clone("could not read repository");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let refused = a_run_given(
        &mut what_the_program_was_asked,
        &mut a_model,
        &mut whoever_answers,
    );

    assert!(
        matches!(refused, Err(ARunThatDidNotFinish::TheCheckout { .. })),
        "**because seven gates ruled on nothing when the repository was never on disk** — and a \
         gate that approved a report about a repository nobody had is a gate that approved a \
         claim. It refused: {refused:?}"
    );
    assert_eq!(
        how_many_agents_were_asked(&a_model),
        0,
        "**because an agent asked about a repository the run does not have produces a document \
         nobody can check** — and a person approving that document approved a claim"
    );
}

#[test]
fn every_question_the_run_asked_about_a_clone_named_the_one_it_made() {
    let mut what_the_program_was_asked = AProgramAnswering::answering_everything();
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let _finished = a_run_given(
        &mut what_the_program_was_asked,
        &mut a_model,
        &mut whoever_answers,
    );

    let every_clone_it_named = what_the_program_was_asked.every_clone_it_was_asked_about();
    assert!(
        !every_clone_it_named.is_empty()
            && every_clone_it_named
                .iter()
                .all(|where_it_lands| where_it_lands.ends_with("WELLBEINGT-42/repository")),
        "**because a delivery that pushes a clone the run never read is a delivery of something \
         else** — and `WhatThereIsToDeliver` carried a destination a caller wrote down, which was a \
         second answer to a question the run answers itself. It asked about {every_clone_it_named:?}"
    );
}

/// A whole run given a program, a model and a person, **and it is one call so three tests are about
/// their own arrangement and not about the order of seven collaborators.**
fn a_run_given(
    what_the_program_was_asked: &mut AProgramAnswering,
    a_model: &mut AModelAnswering,
    whoever_answers: &mut APersonWhoRules,
) -> Result<WhatARunLeft, ARunThatDidNotFinish> {
    let mut the_project =
        AProjectThatWrites::writing_everything("the-run-its-checkout", "WELLBEINGT-42");
    a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        &mut the_project,
        a_model,
        WhichOrganisationReasons::Openai,
        whoever_answers,
        what_there_is_to_deliver(),
        what_the_program_was_asked,
        ADirectoryHoldingNothing,
    )
}

/// How many agents were asked, **and it is a count because a person approving a document about a
/// repository nobody has is the failure and a number is the cheapest way to see it.**
fn how_many_agents_were_asked(a_model: &AModelAnswering) -> usize {
    a_model.what_it_was_asked_for().len()
}

/// A run standing at the stage its first agent is working on, **and on the artifact it produces.**
fn a_run_at_its_first_stage() -> WhereARunStands {
    WhereARunStands::at_the_first_stage(
        a_work_item_the_discovering_agent_holds(),
        "DiscoveryReport",
    )
}

/// Somebody's request, **and it is a real one rather than a placeholder.**
fn the_request() -> CommissionedRequest {
    CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    }
}

/// A work item held by the discovering agent, **answering a request.**
fn a_work_item_the_discovering_agent_holds() -> WorkItem {
    let the_id = WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name");
    let who_asked = WhoCommissioned::try_from("a person").expect("a name is a name");
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_discipline = Role::try_from("DEV").expect("a discipline");
    let the_agent = AgentIdentifier::try_from("aisdlc-discover").expect("an agent name");
    let held = WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::InProgressByAgent,
        the_agent,
    );
    let held = held.expect("a work item may be held at a stage that requires one");
    held.answering(the_request())
        .expect("a work item answering a request may stand where it was put")
}

/// The clone the run was pointed at, **and the address is one a reader can recognise because a
/// caller named it rather than a fixture.**
fn the_clone_the_run_is_pointed_at() -> ARepositoryToClone {
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    ARepositoryToClone::of(
        the_repository,
        WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        std::path::PathBuf::from("/tmp/a-clone-this-run-made"),
    )
}

/// What there is to deliver, **and the clone is no longer among the four: the run made it.**
fn what_there_is_to_deliver() -> WhatThereIsToDeliver {
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_branch =
        ABranchToPush::a_branch_named("aisdlc/WELLBEINGT-42").expect("a branch may take that name");
    let what_to_open = the_request_this_run_makes(the_repository);
    let the_credential =
        ACredential::the_key_a_deployment_supplied("a-key-not-a-real-one").expect("a key");
    WhatThereIsToDeliver::for_this_run(
        the_clone_the_run_is_pointed_at(),
        the_branch,
        what_to_open,
        the_credential,
    )
}

/// The request this run makes, **and no model reaches the title or the body.**
fn the_request_this_run_makes(where_it_is_offered_to: Repository) -> APullRequestToOpen {
    the_request_a_run_makes_of_its_own_change(
        where_it_is_offered_to.clone(),
        where_it_is_offered_to,
        "aisdlc/WELLBEINGT-42",
        "main",
        "the E2E database leaves artifacts behind",
        "A file that was not there, and a test that says so.",
    )
    .expect("a title and a body were both given")
}

/// A program that answers, **and it records what it was asked about in order because "before" is the
/// whole of the first test.**
struct AProgramAnswering {
    refuses_the_clone: Option<String>,
    every_question: RefCell<Vec<String>>,
    every_clone_named: RefCell<Vec<String>>,
}

impl AProgramAnswering {
    fn answering_everything() -> Self {
        Self {
            refuses_the_clone: None,
            every_question: RefCell::new(Vec::new()),
            every_clone_named: RefCell::new(Vec::new()),
        }
    }

    fn refusing_the_clone(what_git_said: &str) -> Self {
        Self {
            refuses_the_clone: Some(String::from(what_git_said)),
            every_question: RefCell::new(Vec::new()),
            every_clone_named: RefCell::new(Vec::new()),
        }
    }

    fn what_it_was_asked_about(&self) -> Vec<String> {
        self.every_question.borrow().clone()
    }

    fn every_clone_it_was_asked_about(&self) -> Vec<String> {
        self.every_clone_named.borrow().clone()
    }
}

impl AProgramWeRun for AProgramAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        let is_a_question_about_a_clone = what_it_is_asked_to_do.the_clone_it_is_about().is_some();
        self.every_question
            .borrow_mut()
            .push(what_a_question_is_about(what_it_is_asked_to_do));
        self.every_clone_named.borrow_mut().extend(
            what_it_is_asked_to_do
                .the_clone_it_is_about()
                .map(|the_clone| the_clone.where_it_lands().display().to_string()),
        );
        if !is_a_question_about_a_clone {
            return WhatTheProgramSaid::of(true, "");
        }
        match &self.refuses_the_clone {
            Some(what_git_said) => WhatTheProgramSaid::of(false, what_git_said),
            None => WhatTheProgramSaid::of(true, ""),
        }
    }
}

/// What a question is about, **as a word a person would use** — the order is the whole of the first
/// test and a word is what a reader reads.
fn what_a_question_is_about(what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> String {
    if what_it_is_asked_to_do.the_clone_it_is_about().is_some() {
        return String::from("clone");
    }
    if what_it_is_asked_to_do.is_naming_where_the_code_was_written_in() {
        return String::from("name the second address");
    }
    String::from("something else")
}

/// A directory holding nothing, **and it is here because the destination is asked about before git
/// is run — that order is the rule, and a run that skipped it would overwrite a resumed clone.**
struct ADirectoryHoldingNothing;

impl WhetherAnythingStandsThere for ADirectoryHoldingNothing {
    fn does_anything_stand_at(&self, _a_directory: &std::path::Path) -> bool {
        false
    }
}

/// A project a run writes real files into, **in a directory of its own.**
struct AProjectThatWrites {
    where_it_lives: std::path::PathBuf,
}

impl AProjectThatWrites {
    fn writing_everything(in_a_directory_named: &str, a_work_item_named: &str) -> Self {
        let where_it_is = std::env::temp_dir().join(format!(
            "aisdlc-a-run-its-checkout-{}-{in_a_directory_named}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&where_it_is);
        std::fs::create_dir_all(
            where_it_is
                .join("steamnoid/wellbeing-tracker-public")
                .join(a_work_item_named),
        )
        .expect("a directory this machine will hold");
        Self {
            where_it_lives: where_it_is,
        }
    }
}

impl TheProjectOnDisk for AProjectThatWrites {
    fn where_it_lives(&self) -> &std::path::Path {
        self.where_it_lives.as_path()
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        Vec::new()
    }

    fn the_text_of(&mut self, _a_path: &aisdlc_run::ThePathInsideTheProject) -> Option<String> {
        None
    }

    fn a_file_is_written(
        &mut self,
        a_path: &aisdlc_run::ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        std::fs::write(std::path::PathBuf::from(a_path.as_str()), what_it_says)
            .expect("a file this machine will hold");
        Ok(())
    }
}
