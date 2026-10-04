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
#[path = "doubles/a_model_that_answers.rs"]
mod a_model_that_answers;
use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhoDecided};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_whole_of_a_run::{AWholeRunThatCouldNotBeHad, the_whole_of_a_run};
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

use a_model_that_answers::AModelAnswering;

/// A project that writes real files, **and counts what it wrote and refuses one when told to** —
/// both of which are its own business rather than the shared project's, **because a double asked for
/// nothing is a double every test could use and a double asked for everything is a double only one
/// test does.**
struct AProjectThatWrites {
    where_it_lives: std::path::PathBuf,
    every_file_it_wrote: RefCell<Vec<std::path::PathBuf>>,
    refuses_the_nth_write: Option<usize>,
}

impl AProjectThatWrites {
    /// A project that writes everything, **in a directory of its own.**
    fn writing_everything(in_a_directory_named: &str) -> Self {
        Self::new(in_a_directory_named, None)
    }

    /// A project that refuses one write, **and the refusal is the disk saying no** — a run is only
    /// refused for a file already being there if it wrote one twice, **which is a different fault.**
    fn refusing_the(nth: usize, in_a_directory_named: &str) -> Self {
        Self::new(in_a_directory_named, Some(nth))
    }

    /// The work item's own directory is created here, **because a run that made a clone created it**
    /// — and a fixture that left it out would be testing the path's refusal rather than the write.
    fn new(in_a_directory_named: &str, refuses_the_nth_write: Option<usize>) -> Self {
        let where_the_run_writes = where_this_process_keeps_its_projects(in_a_directory_named);
        let _ = std::fs::remove_dir_all(&where_the_run_writes);
        std::fs::create_dir_all(the_directory_this_run_writes_into(in_a_directory_named))
            .expect("a directory this machine will hold");
        Self {
            where_it_lives: where_the_run_writes,
            every_file_it_wrote: RefCell::new(Vec::new()),
            refuses_the_nth_write,
        }
    }

    /// Every path this project wrote, **in the order it wrote them.**
    fn every_file_it_wrote(&self) -> Vec<std::path::PathBuf> {
        self.every_file_it_wrote.borrow().clone()
    }
}

impl TheProjectOnDisk for AProjectThatWrites {
    /// Whether anything stands at a path, **and this double answers about the real disk** — it
    /// exists so a test reads real files, **and a double that answered about its own memory would be
    /// a double that could not be wrong about the disk.**
    fn does_anything_stand_at(&self, a_path: &std::path::Path) -> bool {
        a_path.exists()
    }

    fn where_it_lives(&self) -> &std::path::Path {
        self.where_it_lives.as_path()
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        Vec::new()
    }

    fn the_text_of(&mut self, _a_path: &ThePathInsideTheProject) -> Option<String> {
        None
    }

    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        let the_path = std::path::PathBuf::from(a_path.as_str());
        let how_many_are_there = self.every_file_it_wrote.borrow().len() + 1;
        std::fs::write(&the_path, what_it_says).expect("a file this machine will hold");
        if self.refuses_the_nth_write == Some(how_many_are_there) {
            return Err(TheProjectRefusedToWrite::there_was_no_room_for_it(
                a_path,
                "this project is a double told to refuse one write",
            ));
        }
        self.every_file_it_wrote.borrow_mut().push(the_path);
        Ok(())
    }
}

/// Where this process keeps its projects, **and it is named for one caller so two tests cannot
/// collide.**
fn where_this_process_keeps_its_projects(in_a_directory_named: &str) -> std::path::PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-the-whole-run-{}-{in_a_directory_named}",
        std::process::id()
    ));
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is
}

/// The directory this run writes into, **and it is the work item's own directory under the root.**
fn the_directory_this_run_writes_into(in_a_directory_named: &str) -> std::path::PathBuf {
    where_this_process_keeps_its_projects(in_a_directory_named)
        .join("steamnoid/wellbeing-tracker-public")
        .join("WELLBEINGT-42")
}

/// What a gate could open, **and it opens the file the way a person would** — from the disk, and not
/// from whatever a double happens to be holding.
fn what_a_gate_could_open(
    in_a_directory_named: &str,
    which_artifact: &str,
    the_version: u32,
) -> Option<String> {
    let the_file = the_directory_this_run_writes_into(in_a_directory_named)
        .join(format!("{which_artifact}.v{the_version}.json"));
    std::fs::read_to_string(the_file).ok()
}

/// A person who opens the artifact they are ruling on, **and records what they could open.**
struct APersonWhoOpensThenRules {
    in_a_directory_named: String,
    approved: bool,
    refuses_at_the: Option<usize>,
    how_many_these_people_have_been_asked: RefCell<usize>,
    what_each_of_them_could_open: RefCell<Vec<Option<String>>>,
}

impl APersonWhoOpensThenRules {
    fn approving_everything(in_a_directory_named: &str) -> Self {
        Self {
            in_a_directory_named: String::from(in_a_directory_named),
            approved: true,
            refuses_at_the: None,
            how_many_these_people_have_been_asked: RefCell::new(0),
            what_each_of_them_could_open: RefCell::new(Vec::new()),
        }
    }

    fn refusing_at_the(nth: usize, in_a_directory_named: &str) -> Self {
        Self {
            in_a_directory_named: String::from(in_a_directory_named),
            approved: true,
            refuses_at_the: Some(nth),
            how_many_these_people_have_been_asked: RefCell::new(0),
            what_each_of_them_could_open: RefCell::new(Vec::new()),
        }
    }

    fn what_each_of_them_could_open(&self) -> Vec<Option<String>> {
        self.what_each_of_them_could_open.borrow().clone()
    }

    /// Which gate this is, **and the count is what "the third gate" means** — a person who refuses
    /// at the third is refusing at one number, not at a document.
    fn which_gate_are_they_at(&self) -> usize {
        self.how_many_these_people_have_been_asked
            .borrow()
            .saturating_add(1)
    }

    /// What this gate could open, **and it opens it from the disk** — a double that keeps its files
    /// in memory would make this whole file pass while nothing a person could open exists.
    fn they_open_what_they_are_asked_about(&mut self, a_question: &ApprovalQuestion) {
        let the_artifact = a_question.about_which_artifact().0.clone();
        let the_version = a_question.about_which_version().0;
        let what_it_says =
            what_a_gate_could_open(&self.in_a_directory_named, &the_artifact, the_version);
        self.what_each_of_them_could_open
            .borrow_mut()
            .push(what_it_says);
    }
}

impl APersonToDecide for APersonWhoOpensThenRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let which_gate = self.which_gate_are_they_at();
        *self.how_many_these_people_have_been_asked.borrow_mut() = which_gate;
        self.they_open_what_they_are_asked_about(a_question);
        let approved = self.approved && self.refuses_at_the != Some(which_gate);
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

/// Every gate opened a document that was really there, **and this is the whole of what the run is
/// for** — an approval naming an artifact no reader can reach is an approval of a claim.
fn every_gate_opened_a_document_that_is_there(whoever_answers: &APersonWhoOpensThenRules) {
    let what_each_of_them_read = whoever_answers.what_each_of_them_could_open();
    let every_gate_read_something = what_each_of_them_read.iter().all(Option::is_some);
    assert!(
        every_gate_read_something,
        "**because a person who approves an artifact must be able to open it** — and an approval \
         naming an artifact no reader can reach is an approval of a claim. They read \
         {what_each_of_them_read:?}"
    );
}

#[test]
fn every_gate_of_a_whole_run_is_asked_about_a_document_it_can_open() {
    let mut the_project = AProjectThatWrites::writing_everything("a-whole-run");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoOpensThenRules::approving_everything("a-whole-run");

    let ran = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    every_gate_opened_a_document_that_is_there(&whoever_answers);
    let why_it_stopped = ran.as_ref().err();
    assert_eq!(
        the_project.every_file_it_wrote().len(),
        7,
        "**because seven artifacts and seven gates is the whole of the pipeline** — and six files \
         would be one gate asked about a document nobody wrote. The project wrote {:?} and the run \
         stopped with {:?}",
        the_project.every_file_it_wrote(),
        why_it_stopped
    );
}

#[test]
fn a_run_refused_in_the_middle_leaves_the_refused_artifact_on_disk() {
    let mut the_project = AProjectThatWrites::writing_everything("a-refused-gate");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoOpensThenRules::refusing_at_the(3, "a-refused-gate");

    let ran = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);
    let where_it_stands = the_stage_the_run_stands_at(&ran);

    assert_eq!(
        where_it_stands,
        Some(Stage::Done),
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
    let mut the_project = AProjectThatWrites::refusing_the(2, "a-refused-write");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoOpensThenRules::approving_everything("a-refused-write");

    let refused = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    assert!(
        matches!(
            refused,
            Err(AWholeRunThatCouldNotBeHad::NothingWasWritten { .. })
        ),
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

/// The stage a run stands at, **and it is one function so the assertion does not move the answer.**
fn the_stage_the_run_stands_at(
    ran: &Result<WhereARunStands, AWholeRunThatCouldNotBeHad>,
) -> Option<Stage> {
    let the_run = ran.as_ref().ok()?;
    Some(the_run.the_stage_it_stands_at())
}

/// A run standing at the stage its first agent is working on, **and on the artifact it produces.**
fn a_run_at_its_first_stage() -> WhereARunStands {
    WhereARunStands::at_the_first_stage(
        a_work_item_the_discovering_agent_holds(),
        "DiscoveryReport",
    )
}

/// Somebody's request, **and it is a real one rather than a placeholder** — the body is what a
/// person would have written.
fn the_request() -> aisdlc_domain::commissioned_request::CommissionedRequest {
    aisdlc_domain::commissioned_request::CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    }
}

/// A work item held by the discovering agent, **and it stands at the stage that stage is held at.**
fn a_work_item_the_discovering_agent_holds() -> WorkItem {
    let the_id = WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("DEV").expect("a discipline");
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("an agent name");
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
