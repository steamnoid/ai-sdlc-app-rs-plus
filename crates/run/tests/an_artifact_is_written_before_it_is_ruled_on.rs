//! A person is asked about an artifact that is already on disk.
//!
//! **The gate rules on a document, and a document nobody can open is not one.** `a_stage_is_worked_and_ruled_on`
//! asks the model, presents the work, and asks somebody — **and the words stop in memory between the
//! first and the last**, so the person ruling is asked about a document that does not exist yet.
//!
//! **The order is ask, write, rule, and it is the whole of this cycle.** A person who approves
//! `DiscoveryReport` must be able to open it, **and an approval naming an artifact no reader can
//! reach is an approval of a claim.**
//!
//! **A refusal still leaves the artifact on disk, because the person read it in order to refuse it.**
//! Deleting the document of a refused gate would destroy the evidence of what was refused, **and a
//! refusal nobody can check is a decision the reader has to take on trust.**

use std::cell::RefCell;
use std::path::PathBuf;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::a_stage_is_worked_and_ruled_on::{
    AStageThatCouldNotBeWorked, WhatAnAgentSaid, WhatTheStageLeft, the_agent_is_asked,
};
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::what_a_stage_kept_is_written::{
    WhatCouldNotBeWritten, what_a_stage_kept_is_written,
};
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A project that holds whatever it is told to, **and it writes a real file as well as remembering
/// it — and that is the whole point of this double.**
struct AProjectRemembering {
    where_it_lives: PathBuf,
    what_it_holds: RefCell<Vec<(PathBuf, String)>>,
}

impl AProjectRemembering {
    /// A project of its own, **and the name is an argument because three tests share one process.**
    ///
    /// **They shared one directory and raced on one file**: each writes `DiscoveryReport.v1.json`,
    /// and a reader in another test could see the file between a truncation and its write — **which
    /// is a person at a gate reading an empty document**, the exact thing this file exists to make
    /// impossible. It passed for months and then failed under load, **which is what a race looks
    /// like.**
    ///
    /// **Everything the run needs on disk is made here, and the directory is cleared first**, so the
    /// test cannot depend on which of its two calls happened first — **and a fixture whose order
    /// matters is a fixture whose failure looks like the code's.**
    fn empty(in_a_directory_named: &str) -> Self {
        let where_the_run_writes = the_projects_root(in_a_directory_named);
        let _ = std::fs::remove_dir_all(&where_the_run_writes);
        the_work_items_directory(in_a_directory_named);
        Self {
            where_it_lives: where_the_run_writes,
            what_it_holds: RefCell::new(Vec::new()),
        }
    }

    /// What the project holds, **and it answers by path so a test can ask about one file.**
    fn what_it_says_at(&self, where_to_look: &ThePathInsideTheProject) -> Option<String> {
        let asked_for = where_to_look.as_str();
        let held = self.what_it_holds.borrow();
        let found = held
            .iter()
            .find(|(one_path, _)| one_path.to_str() == Some(asked_for));
        found.map(|(_one_path, what_it_says)| what_it_says.clone())
    }

    fn how_many_files_it_holds(&self) -> usize {
        self.what_it_holds.borrow().len()
    }

    /// The file is written to the real disk, **because the person at the gate opens files and not
    /// this double's memory.** A double that kept its files in a `RefCell` would make every reading
    /// test pass while nothing a person could open existed — **and that is the very failure this
    /// file exists to make impossible.**
    fn the_real_file_is_written(a_path: &ThePathInsideTheProject, what_it_says: &str) {
        let where_to_write = PathBuf::from(a_path.as_str());
        if let Some(the_directory) = where_to_write.parent() {
            std::fs::create_dir_all(the_directory).expect("a directory this machine will hold");
        }
        std::fs::write(&where_to_write, what_it_says).expect("a file this machine will hold");
    }
}

impl TheProjectOnDisk for AProjectRemembering {
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

    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String> {
        self.what_it_says_at(a_path)
    }

    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        if self.what_it_says_at(a_path).is_some() {
            return Err(TheProjectRefusedToWrite::there_is_already_such_a_file(
                a_path,
            ));
        }
        AProjectRemembering::the_real_file_is_written(a_path, what_it_says);
        self.what_it_holds
            .borrow_mut()
            .push((PathBuf::from(a_path.as_str()), what_it_says.to_string()));
        Ok(())
    }
}

/// A person who rules, **and who can read what they are ruling on — or not.**
struct APersonWhoReadsThenRules {
    approved: bool,
    the_project: PathBuf,
    what_it_read: RefCell<Vec<Option<String>>>,
}

impl APersonWhoReadsThenRules {
    fn who_approves(the_project: &AProjectRemembering) -> Self {
        Self {
            approved: true,
            the_project: the_project.where_it_lives.clone(),
            what_it_read: RefCell::new(Vec::new()),
        }
    }

    fn who_refuses(the_project: &AProjectRemembering) -> Self {
        Self {
            approved: false,
            the_project: the_project.where_it_lives.clone(),
            what_it_read: RefCell::new(Vec::new()),
        }
    }

    /// What the person could open at each gate, **`None` being a gate asked about nothing.**
    fn what_it_read(&self) -> Vec<Option<String>> {
        self.what_it_read.borrow().clone()
    }

    fn it_read(&self, which_artifact: &str, the_version: u32) -> Option<String> {
        let the_file = self
            .the_project
            .join("steamnoid/wellbeing-tracker-public/WELLBEINGT-42")
            .join(format!("{which_artifact}.v{the_version}.json"));
        std::fs::read_to_string(the_file).ok()
    }
}

impl APersonToDecide for APersonWhoReadsThenRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let what_it_read = self.it_read(
            &a_question.about_which_artifact().0,
            a_question.about_which_version().0,
        );
        self.what_it_read.borrow_mut().push(what_it_read);
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

/// A model that answers everything.
struct AModelAnswering;

impl AReasoningModel for AModelAnswering {
    fn what_it_says_when_asked_for(
        &mut self,
        _which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead>
    {
        Ok(String::from("the repository is a self-hosted Plane"))
    }
}

/// Where this deployment keeps projects, **and it really exists.**
fn the_projects_root(in_a_directory_named: &str) -> PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-ruled-on-what-was-read-{}-{in_a_directory_named}",
        std::process::id()
    ));
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is
}

/// The work item's own directory, **and it exists because a run that made a clone created it.**
fn the_work_items_directory(in_a_directory_named: &str) {
    let the_project_directory = the_projects_root(in_a_directory_named)
        .join("steamnoid/wellbeing-tracker-public")
        .join("WELLBEINGT-42");
    std::fs::create_dir_all(&the_project_directory).expect("a directory this machine will hold");
}

#[test]
fn a_person_is_asked_about_an_artifact_that_is_on_disk_before_they_rule() {
    the_work_items_directory("a-person-is-asked");
    let mut the_project = AProjectRemembering::empty("a-person-is-asked");
    let mut whoever_answers = APersonWhoReadsThenRules::who_approves(&the_project);

    let _ruled = the_gate_is_asked_with(&mut the_project, &mut whoever_answers);

    assert_eq!(
        whoever_answers.what_it_read(),
        vec![Some(String::from("the repository is a self-hosted Plane"))],
        "**because the gate rules on a document and a document nobody can open is not one** — an \
         approval naming an artifact no reader can reach is an approval of a claim. They read \
         {:?}",
        whoever_answers.what_it_read()
    );
}

#[test]
fn the_file_a_person_read_is_the_file_at_the_version_the_approval_names() {
    the_work_items_directory("the-file-is-the-version");
    let mut the_project = AProjectRemembering::empty("the-file-is-the-version");
    let mut whoever_answers = APersonWhoReadsThenRules::who_approves(&the_project);

    let ruled = the_gate_is_asked_with(&mut the_project, &mut whoever_answers);

    assert_eq!(
        which_version_was_produced(&ruled),
        Some(1),
        "**because cycle 122 put the version in the file's name and cycle 120 put it on the run** — and \
         the two have to be the same number, **or the approval names a version of the document and a \
         reader opens another.** It produced version {:?}",
        which_version_was_produced(&ruled)
    );
    assert_eq!(
        the_project.how_many_files_it_holds(),
        1,
        "**because one artifact was produced and one file was written** — two would be a second \
         document under one approval. The project holds {} files",
        the_project.how_many_files_it_holds()
    );
}

#[test]
fn a_refused_gate_still_leaves_the_artifact_on_disk() {
    the_work_items_directory("a-refused-gate");
    let mut the_project = AProjectRemembering::empty("a-refused-gate");
    let mut whoever_answers = APersonWhoReadsThenRules::who_refuses(&the_project);

    let ruled = the_gate_is_asked_with(&mut the_project, &mut whoever_answers);

    assert_eq!(
        was_the_gate_approved(&ruled),
        Some(false),
        "**because refusing is a decision and this person read the document in order to make it** — \
         and a refusal nobody can check is a decision the reader has to take on trust."
    );
    assert_eq!(
        the_project.how_many_files_it_holds(),
        1,
        "**because the evidence of what was refused is the refused document** — deleting it would \
         destroy the only thing a reader could check the refusal against. The project holds {} files",
        the_project.how_many_files_it_holds()
    );
}

/// What one gate left, **and the alias is here because four assertions say the same long type.**
type WhatTheGateLeft = Result<WhatTheStageLeft, AStageThatCouldNotBeWorked>;

/// The gate is asked, **with the artifact already on disk — which is the whole of this file.**
fn the_gate_is_asked_with(
    the_project: &mut AProjectRemembering,
    whoever_answers_the_gate: &mut APersonWhoReadsThenRules,
) -> Result<aisdlc_run::a_stage_is_worked_and_ruled_on::WhatTheStageLeft, AStageThatCouldNotBeWorked>
{
    let mut a_model = AModelAnswering;
    let the_run = a_run_at_its_first_stage();
    let said = the_agent_is_asked(&mut a_model, &the_run, WhichOrganisationReasons::Openai)
        .expect("an agent that answers can be asked");
    what_the_run_said_is_written(the_project, &said)
        .expect("the first write of an artifact is permitted");
    said.is_ruled_on(whoever_answers_the_gate)
}

/// Whether the gate said yes, **and it is one function so the assertion does not move the answer.**
fn was_the_gate_approved(ruled: &WhatTheGateLeft) -> Option<bool> {
    let what_the_gate_left = ruled.as_ref().ok()?;
    Some(what_the_gate_left.was_the_artifact_approved())
}

/// Which version was produced, **and it is one function so the two assertions that read it do not
/// each spell out the same two hops.**
fn which_version_was_produced(ruled: &WhatTheGateLeft) -> Option<u32> {
    let what_the_gate_left = ruled.as_ref().ok()?;
    Some(what_the_gate_left.which_version_was_produced().0)
}

/// What the run said is written, **and the gate is asked after.**
fn what_the_run_said_is_written(
    the_project: &mut AProjectRemembering,
    what_an_agent_said: &WhatAnAgentSaid,
) -> Result<PathBuf, WhatCouldNotBeWritten> {
    let where_it_landed = what_a_stage_kept_is_written(
        the_project,
        what_an_agent_said.the_run_standing_at_what_it_produced(),
        &what_an_agent_said.what_the_agent_said(),
    );
    where_it_landed.map(|landed| PathBuf::from(landed.as_str()))
}

/// A run standing at the stage its first agent is working on.
fn a_run_at_its_first_stage() -> WhereARunStands {
    let the_id = WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("PO").expect("a discipline");
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("an agent name");
    let held = a_work_item_the_discovering_agent_is_holding(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        the_agent,
    );
    WhereARunStands::at_the_first_stage(held, "DiscoveryReport")
}

/// The work item, answering a request, **and the request is built here because a fixture holding both
/// is a fixture a reader has to read all the way through before reaching the stage.**
fn a_work_item_the_discovering_agent_is_holding(
    the_id: WorkItemId,
    who_asked: aisdlc_domain::who_commissioned::WhoCommissioned,
    the_repository: Repository,
    the_discipline: aisdlc_domain::role::Role,
    the_agent: aisdlc_domain::agent_identifier::AgentIdentifier,
) -> WorkItem {
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
    answering.expect("a work item answering a request may stand where it was put")
}
