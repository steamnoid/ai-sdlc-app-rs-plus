//! What this test hands a run, **and it is one file because a file at the top of `tests/` is itself
//! compiled as a test target with no tests in it.**
//!
//! **Four collaborators, and each is the least its own test needs** — a double asked for more than
//! its test uses is reported dead in every binary that has it.

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A person who says yes at every gate, **and it is here rather than the shared one because that one
/// is asked for a refusal this file never makes.**
pub struct APersonWhoApprovesEverything;

impl APersonWhoApprovesEverything {
    pub fn new() -> Self {
        Self
    }
}

impl APersonToDecide for APersonWhoApprovesEverything {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
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

/// A project a run writes real files into, **in a directory of its own.**
pub struct AProjectThatWrites {
    where_it_lives: std::path::PathBuf,
}

impl AProjectThatWrites {
    pub fn writing_everything(in_a_directory_named: &str, a_work_item_named: &str) -> Self {
        let where_it_is = std::env::temp_dir().join(format!(
            "aisdlc-a-run-its-checkout-{}-{in_a_directory_named}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&where_it_is);
        let the_project_directory = where_it_is
            .join("steamnoid/wellbeing-tracker-public")
            .join(a_work_item_named);
        std::fs::create_dir_all(the_project_directory).expect("a directory this machine will hold");
        Self {
            where_it_lives: where_it_is,
        }
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
        std::fs::write(std::path::PathBuf::from(a_path.as_str()), what_it_says)
            .expect("a file this machine will hold");
        Ok(())
    }
}

/// What happened, **in the order it happened** — and it is one log shared by two collaborators
/// because **"before" is a claim about two things at once and two logs cannot say it.**
#[derive(Clone)]
pub struct WhatHappenedInOrder(std::rc::Rc<RefCell<Vec<String>>>);

impl WhatHappenedInOrder {
    pub fn shared() -> Self {
        Self(std::rc::Rc::new(RefCell::new(Vec::new())))
    }

    pub fn it_happened(&self, what: &str) {
        self.0.borrow_mut().push(String::from(what));
    }

    pub fn what_happened(&self) -> Vec<String> {
        self.0.borrow().clone()
    }
}

/// A model that answers everything **and writes down that it was asked**, because the only claim
/// this test makes about a model is *when* it was asked.
pub struct AModelRecordingWhatHappened {
    every_artifact_it_was_asked_for: RefCell<Vec<String>>,
    what_happened_in_order: WhatHappenedInOrder,
}

impl AModelRecordingWhatHappened {
    pub fn answering_everything() -> Self {
        Self {
            every_artifact_it_was_asked_for: RefCell::new(Vec::new()),
            what_happened_in_order: WhatHappenedInOrder::shared(),
        }
    }

    /// Point this model at the log, **and it is a call because the model is built before the test
    /// decides what else is watching.**
    pub fn it_writes_to(&mut self, the_log: WhatHappenedInOrder) {
        self.what_happened_in_order = the_log;
    }

    pub fn every_artifact_it_was_asked_for(&self) -> Vec<String> {
        self.every_artifact_it_was_asked_for.borrow().clone()
    }
}

impl AReasoningModel for AModelRecordingWhatHappened {
    fn what_it_says_when_asked_for(
        &mut self,
        which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        self.every_artifact_it_was_asked_for
            .borrow_mut()
            .push(which_artifact.0.clone());
        let what_happened = format!("an agent was asked for {}", which_artifact.0);
        self.what_happened_in_order.it_happened(&what_happened);
        Ok(String::from("an artifact, of the length a real one is"))
    }
}

/// A program that answers `git`, **and it records the clone it was asked about because a delivery
/// that pushed a different one would be a delivery of something else.**
pub struct AProgramAnswering {
    refuses_the_clone: Option<String>,
    every_clone_named: RefCell<Vec<String>>,
    what_happened_in_order: WhatHappenedInOrder,
}

impl AProgramAnswering {
    pub fn answering_everything(what_happened_in_order: WhatHappenedInOrder) -> Self {
        Self {
            refuses_the_clone: None,
            every_clone_named: RefCell::new(Vec::new()),
            what_happened_in_order,
        }
    }

    pub fn refusing_the_clone(what_git_said: &str) -> Self {
        Self {
            refuses_the_clone: Some(String::from(what_git_said)),
            every_clone_named: RefCell::new(Vec::new()),
            what_happened_in_order: WhatHappenedInOrder::shared(),
        }
    }

    pub fn every_clone_it_was_asked_about(&self) -> Vec<String> {
        self.every_clone_named.borrow().clone()
    }
}
impl AProgramWeRun for AProgramAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.they_remember_the_clone(what_it_is_asked_to_do);
        what_git_said_in_reply(what_it_is_asked_to_do, self)
    }
}

impl AProgramAnswering {
    /// The clone this question is about, **remembered as a path** — and a delivery that pushed a
    /// different one would be a delivery of something else.
    fn they_remember_the_clone(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) {
        let where_it_lands = what_it_is_asked_to_do.the_clone_it_is_about();
        self.every_clone_named
            .borrow_mut()
            .extend(where_it_lands.map(|one| one.where_it_lands().display().to_string()));
    }

    /// What the program answers, **and the refusal comes first because a run that was refused has
    fn what_it_answers(&self) -> WhatTheProgramSaid {
        match &self.refuses_the_clone {
            Some(what_git_said) => WhatTheProgramSaid::of(false, what_git_said),
            None => WhatTheProgramSaid::of(true, ""),
        }
    }

    /// The copy is the only moment worth a line, **and git is asked four questions about one copy —
    /// so this is a question rather than a statement every time the program is run.**
    fn they_write_down_the_copy(&self, is_the_copy_itself: bool) {
        if !is_the_copy_itself {
            return;
        }
        self.what_happened_in_order
            .it_happened("the repository was copied");
    }
}

/// What git said in reply, **and only the copy itself is a moment worth writing down** — git is
/// asked four questions about one copy and one of them is the copy.
fn what_git_said_in_reply(
    what_it_is_asked_to_do: &WhatWeTellAProgramToDo,
    the_program: &AProgramAnswering,
) -> WhatTheProgramSaid {
    let is_the_copy_itself = matches!(
        what_it_is_asked_to_do,
        WhatWeTellAProgramToDo::CloneTheRepository { .. }
    );
    let was_the_clone_asked_about = what_it_is_asked_to_do.the_clone_it_is_about().is_some();
    if !was_the_clone_asked_about {
        return WhatTheProgramSaid::of(true, "");
    }
    the_program.they_write_down_the_copy(is_the_copy_itself);
    the_program.what_it_answers()
}
