//! A whole run writes what it produced, and every gate opens what it was asked about.
//!
//! **The three steps exist and the whole run does not use them.** Cycle 124 gave
//! `the_agent_is_asked` → the writer → `is_ruled_on`, because **a person who approves an artifact
//! must be able to open it** — and `the_whole_of_a_run` still calls `a_stage_is_worked_and_ruled_on`,
//! **which is the first and third of those composed and writes nothing between them.**
//!
//! **Every gate is asked about a document that is on disk, and the double is a person who opens it.**
//! The record is what each gate could actually read, **so a run that approved seven claims would be
//! seven `None`s** — and that is the whole of what these tests are about.
//!
//! **A refused gate's artifact stays on disk, and a refused *write* stops the run before anybody is
//! asked.** A gate asked about a document nobody could open is the failure this file exists to make
//! impossible.

use std::cell::RefCell;
use std::path::PathBuf;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use std::path::Path;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::the_whole_of_a_run::{AWholeRunThatCouldNotBeHad, the_whole_of_a_run};
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

mod the_doubles_a_run_is_run_with;
use the_doubles_a_run_is_run_with::AModelAnswering;

/// A project that writes a real file, **and counts what it wrote.**
struct AProjectThatWrites {
    where_it_lives: PathBuf,
    every_file_it_wrote: RefCell<Vec<PathBuf>>,
    refuses_the_nth_write: Option<usize>,
}

impl AProjectThatWrites {
    /// A project that writes everything, **and its root really exists.**
    fn writing_everything() -> Self {
        Self::new(None)
    }

    /// A project that refuses one write, **and the refusal is the file being there already** — the
    /// only way a disk refuses a write this run makes.
    fn refusing_the(nth: usize) -> Self {
        Self::new(Some(nth))
    }

    fn new(refuses_the_nth_write: Option<usize>) -> Self {
        let the_directory = the_work_items_directory();
        Self {
            where_it_lives: the_directory,
            every_file_it_wrote: RefCell::new(Vec::new()),
            refuses_the_nth_write,
        }
    }

    /// Every path this project wrote, **in the order it wrote them.**
    fn every_file_it_wrote(&self) -> Vec<PathBuf> {
        self.every_file_it_wrote.borrow().clone()
    }
}

impl aisdlc_run::TheProjectOnDisk for AProjectThatWrites {
    fn where_it_lives(&self) -> &std::path::Path {
        &self.where_it_lives
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
    ) -> Result<(), aisdlc_run::the_project_on_disk::TheProjectRefusedToWrite> {
        let the_path = PathBuf::from(a_path.as_str());
        if the_path.exists() {
            return Err(aisdlc_run::the_project_on_disk::TheProjectRefusedToWrite::there_is_already_such_a_file(
                a_path,
            ));
        }
        if self.refuses_the_nth_write == Some(self.every_file_it_wrote.borrow().len() + 1) {
            std::fs::write(&the_path, what_it_says).expect("a file this machine will hold");
            return Err(aisdlc_run::the_project_on_disk::TheProjectRefusedToWrite::there_was_no_room_for_it(
                a_path,
                "this project is a double told to refuse one write",
            ));
        }
        std::fs::write(&the_path, what_it_says).expect("a file this machine will hold");
        self.every_file_it_wrote.borrow_mut().push(the_path);
        Ok(())
    }
}

/// A person who opens the artifact they are ruling on, **and records what they could open.**
struct APersonWhoOpensThenRules {
    the_projects_root: PathBuf,
    approved: bool,
    refuses_at_the: Option<usize>,
    how_many_these_people_have_been_asked: RefCell<usize>,
    what_each_of_them_could_open: RefCell<Vec<Option<String>>>,
}

impl APersonWhoOpensThenRules {
    fn approving_everything() -> Self {
        Self {
            the_projects_root: the_projects_root(),
            approved: true,
            refuses_at_the: None,
            how_many_these_people_have_been_asked: RefCell::new(0),
            what_each_of_them_could_open: RefCell::new(Vec::new()),
        }
    }

    fn refusing_at_the(nth: usize) -> Self {
        Self {
            the_projects_root: the_projects_root(),
            approved: true,
            refuses_at_the: Some(nth),
            how_many_these_people_have_been_asked: RefCell::new(0),
            what_each_of_them_could_open: RefCell::new(Vec::new()),
        }
    }

    fn what_each_of_them_could_open(&self) -> Vec<Option<String>> {
        self.what_each_of_them_could_open.borrow().clone()
    }
}

impl APersonToDecide for APersonWhoOpensThenRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let nth = self.how_many_these_people_have_been_asked.borrow().saturating_add(1);
        *self.how_many_these_people_have_been_asked.borrow_mut() = nth;
        let the_artifact = a_question.about_which_artifact().0.clone();
        let the_version = a_question.about_which_version().0;
        let what_it_says = it_opens(&self.the_projects_root, &the_artifact, the_version);
        self.what_each_of_them_could_open.borrow_mut().push(what_it_says);
        let approved = self.approved && self.refuses_at_the != Some(nth);
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

/// Where this deployment keeps projects, **and it really exists.**
fn the_projects_root() -> PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-the-whole-run-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is
}

/// The work item's own directory, **and it exists because a run that made a clone created it.**
fn the_work_items_directory() -> PathBuf {
    let the_project_directory =
        the_projects_root().join("steamnoid/wellbeing-tracker-public/WELLBEINGT-42");
    std::fs::create_dir_all(&the_project_directory).expect("a directory this machine will hold");
    the_project_directory
}

#[test]
fn every_gate_of_a_whole_run_is_asked_about_a_document_it_can_open() {
    let mut the_project = AProjectThatWrites::writing_everything();
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoOpensThenRules::approving_everything();

    let _ran = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    let every_gate_read_something = whoever_answers
        .what_each_of_them_could_open()
        .iter()
        .all(|what_it_read| what_it_read.is_some());
    assert!(
        every_gate_read_something,
        "**because a person who approves an artifact must be able to open it** — and an approval \
         naming an artifact no reader can reach is an approval of a claim. They read {:?}",
        whoever_answers.what_each_of_them_could_open()
    );
    assert_eq!(
        the_project.every_file_it_wrote().len(),
        7,
        "**because seven artifacts and seven gates is the whole of the pipeline** — and six files \
         would be one gate asked about a document nobody wrote. The project wrote {:?}",
        the_project.every_file_it_wrote()
    );
}

#[test]
fn a_run_refused_in_the_middle_leaves_the_refused_artifact_on_disk() {
    let mut the_project = AProjectThatWrites::writing_everything();
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoOpensThenRules::refusing_at_the(3);

    let ran = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);
    let where_it_stands = the_stage_the_run_stands_at(&ran);

    assert_eq!(
        where_it_stands,
        Ok(Stage::Done),
        "**because refusing ends the run** — and the run is at `DONE` rather than half-delivered. It \
         stands at {where_it_stands:?}"
    );
    assert_eq!(
        the_project.every_file_it_wrote().len(),
        3,
        "**because the person read the third document in order to refuse it** — deleting it would \
         destroy the evidence of what was refused, and a refusal nobody can check is a decision the \
         reader has to take on trust. The project wrote {:?}",
        the_project.every_file_it_wrote()
    );
}

#[test]
fn a_write_that_is_refused_stops_the_run_before_anybody_is_asked_about_it() {
    let mut the_project = AProjectThatWrites::refusing_the(2);
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoOpensThenRules::approving_everything();

    let refused = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    assert!(
        matches!(refused, Err(AWholeRunThatCouldNotBeHad::NothingWasWritten { .. })),
        "**because a gate asked about a document nobody could open is the failure this run exists to \
         prevent** — and asking anyway would leave an approval naming an artifact no reader can \
         reach. It refused: {refused:?}"
    );
    assert_eq!(
        whoever_answers.what_each_of_them_could_open().len(),
        1,
        "**because the second gate was never asked** — and two questions would mean somebody ruled \
         on a document the run could not write. They were asked about {:?}",
        whoever_answers.what_each_of_them_could_open()
    );
}

/// A whole run given a project, a model and a person, **and it is one call so the three tests are
/// about their own arrangement and not about the order of four arguments.**
fn the_run_given(
    the_project: &mut AProjectThatWrites,
    a_model: &mut AModelAnswering,
    whoever_answers: &mut APersonWhoOpensThenRules,
) -> Result<WhereARunStands, AWholeRunThatCouldNotBeHad> {
    let the_run = a_run_at_its_first_stage();
    the_whole_of_a_run(
        &the_run,
        the_project,
        a_model,
        WhichOrganisationReasons::Openai,
        whoever_answers,
    )
}

/// What a gate could open, **and it opens the file the way a person would** — from the disk, and not
/// from whatever a double happens to be holding.
///
/// **A double that keeps its files in memory makes every reading test pass while nothing a person
/// could open exists**, and that is the failure this file exists to make impossible.
fn it_opens(the_projects_root: &Path, which_artifact: &str, the_version: u32) -> Option<String> {
    let the_file = the_projects_root
        .join("steamnoid/wellbeing-tracker-public/WELLBEINGT-42")
        .join(format!("{which_artifact}.v{the_version}.json"));
    std::fs::read_to_string(the_file).ok()
}

/// The stage a run stands at, **and it is one function so the assertion does not move the answer.**
fn the_stage_the_run_stands_at(ran: &Result<WhereARunStands, AWholeRunThatCouldNotBeHad>) -> Result<Stage, &AWholeRunThatCouldNotBeHad> {
    let the_run = ran.as_ref().map_err(|why_not| why_not);
    the_run.map(WhereARunStands::the_stage_it_stands_at)
}

/// A run standing at the stage its first agent is working on, **and on the artifact it produces.**
fn a_run_at_its_first_stage() -> WhereARunStands {
    let the_id = WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("DEV").expect("a discipline");
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("an agent name");
    let the_request = aisdlc_domain::commissioned_request::CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    };
    let held = WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::InProgressByAgent,
        the_agent,
    );
    let held = held.expect("a work item may be held at a stage that requires one");
    let answering = held.answering(the_request);
    let held = answering.expect("a work item answering a request may stand where it was put");
    WhereARunStands::at_the_first_stage(held, "DiscoveryReport")
}
