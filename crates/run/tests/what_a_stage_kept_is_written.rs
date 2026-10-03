//! What a stage kept is written beside the clone, and exactly once.
//!
//! **A stage keeps what the agent said and there was nowhere to put it.** Cycle 120 gave
//! `WhatTheStageLeft::what_the_agent_said` and cycle 122 gave the derived path — **and nothing joined
//! them, so the words still stop at the edge of the process.**
//!
//! **An artifact at one version is one file, written once.** An approval names the exact artifact
//! *and version* it ruled on, **and a second write at that version would replace the document under
//! an approval that still names it** — so the write is refused rather than overwriting.
//!
//! **The file is what the agent said and not a summary of it.** A summary is prose nobody approved:
//! the gate rules on a document, **and a document that is not the one the agent wrote is a document
//! nobody has read.**

use std::cell::RefCell;
use std::path::PathBuf;

use aisdlc_domain::approval::{AnArtifactVersion, WhichArtifact};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::where_an_artifact_lands::where_an_artifact_lands;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::what_a_stage_kept_is_written::{
    WhatCouldNotBeWritten, what_a_stage_kept_is_written,
};
use aisdlc_run::where_a_run_stands::WhereARunStands;

/// Where this deployment keeps projects, **and it really exists.**
///
/// **The confinement rule resolves both sides before comparing them**, so a project that is not there
/// is refused for naming nowhere — **and a test pointing at a fictional root would be testing the
/// refusal rather than the write.** The name carries the process id because two runs of the suite at
/// once would otherwise share one directory, **which is the collision the branch fixtures already
/// hit.**
fn the_projects_root() -> PathBuf {
    let where_it_is =
        std::env::temp_dir().join(format!("aisdlc-what-a-stage-kept-{}", std::process::id()));
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is
}

/// Where the run says `DiscoveryReport` at version one lands, **and it is the domain's own answer**
/// so the test compares the writer against the derivation rather than against a string.
fn where_discovery_report_version_one_lands() -> PathBuf {
    let the_name = WhichArtifact::try_from("DiscoveryReport").expect("a name is not blank");
    let the_version = AnArtifactVersion::try_from(1).expect("a version");
    let a_work_item = WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name");
    where_an_artifact_lands(
        &the_projects_root(),
        &what_this_work_is_about(),
        &a_work_item,
        &the_name,
        &the_version,
    )
    .expect("an ordinary artifact may be held")
}

/// The work item's own directory, **and it exists because a run that made a clone created it.**
///
/// **The door a file is written through resolves the directory rather than the file**, so the
/// directory has to be there — **and a fixture that left it out would be testing the refusal rather
/// than the write.** The clone inside it is never made, because nothing here runs `git`.
fn the_work_items_directory() -> PathBuf {
    let the_project_directory =
        the_projects_root().join("steamnoid/wellbeing-tracker-public/WELLBEINGT-42");
    std::fs::create_dir_all(&the_project_directory).expect("a directory this machine will hold");
    the_project_directory
}

/// The repository this work item is about, **named for what it is for rather than for the type it
/// returns**, because a name after a domain type claims nothing about which one.
fn what_this_work_is_about() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

/// A project that holds whatever it is told to, **and it remembers every path it was handed** — so a
/// test can prove the writer asked for the derived path rather than composing one.
struct AProjectRemembering {
    where_it_lives: PathBuf,
    what_it_holds: RefCell<Vec<(PathBuf, String)>>,
}

impl AProjectRemembering {
    /// A project that holds whatever it is told to, **and it owns its own path** — the port hands
    /// back a reference, **so a double that built its path on the spot would be returning a borrow
    /// of something already gone.**
    fn empty() -> Self {
        let _the_directory_is_there = the_work_items_directory();
        Self {
            where_it_lives: the_projects_root(),
            what_it_holds: RefCell::new(Vec::new()),
        }
    }
}

impl AProjectRemembering {
    fn what_it_now_holds(&self) -> Vec<(PathBuf, String)> {
        self.what_it_holds.borrow().clone()
    }
}

impl TheProjectOnDisk for AProjectRemembering {
    fn where_it_lives(&self) -> &std::path::Path {
        self.where_it_lives.as_path()
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        Vec::new()
    }

    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String> {
        let asked_for = a_path.as_str();
        let what_it_holds = self.what_it_holds.borrow();
        let found = what_it_holds
            .iter()
            .find(|(one_path, _)| one_path.to_str() == Some(asked_for));
        found.map(|(_one_path, what_it_says)| what_it_says.clone())
    }

    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        let already_holds = self.the_text_of(a_path).is_some();
        if already_holds {
            return Err(TheProjectRefusedToWrite::there_is_already_such_a_file(
                a_path,
            ));
        }
        self.what_it_holds
            .borrow_mut()
            .push((PathBuf::from(a_path.as_str()), what_it_says.to_string()));
        Ok(())
    }
}

/// What a stage kept is written, **and the project is the one the run's own clone made.**
fn what_this_stage_kept_is_written(
    the_project: &mut AProjectRemembering,
    what_the_agent_said: &str,
    the_version: u32,
) -> Result<PathBuf, WhatCouldNotBeWritten> {
    let the_run = a_run_on_this_artifact(the_version);
    let written = what_a_stage_kept_is_written(the_project, &the_run, what_the_agent_said);
    written.map(|where_it_landed| PathBuf::from(where_it_landed.as_str()))
}

/// A run standing on `DiscoveryReport` at a version.
fn a_run_on_this_artifact(the_version: u32) -> WhereARunStands {
    let the_id = WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_discipline = aisdlc_domain::role::Role::try_from("PO").expect("a discipline");
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("an agent name");
    let the_work_item = aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        what_this_work_is_about(),
        the_discipline,
        aisdlc_domain::stage::Stage::InProgressByAgent,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one");
    let the_run = WhereARunStands::at_the_first_stage(the_work_item, "DiscoveryReport");
    let at_the_version = AnArtifactVersion::try_from(the_version).expect("a version");
    the_run
        .which_artifact_is_at(at_the_version)
        .expect("the run may say which version of its artifact exists")
}

#[test]
fn what_a_stage_kept_is_written_at_the_path_the_run_derives() {
    let mut the_project = AProjectRemembering::empty();

    let written = what_this_stage_kept_is_written(&mut the_project, "the repository is a Plane", 1);

    let where_the_run_says_it_goes = where_discovery_report_version_one_lands();

    let the_same_file_name = written
        .as_ref()
        .map(|landed| landed.file_name() == where_the_run_says_it_goes.file_name());
    assert_eq!(
        the_same_file_name,
        Ok(true),
        "**because the path is derived and not composed by the writer** — a writer that built the \
         path itself would be a second derivation nobody has to keep in step. It landed at {written:?} \
         and the run says {where_the_run_says_it_goes:?}"
    );

    let the_resolved_project = the_project.where_it_lives().canonicalize();
    let is_it_beside_the_clone = written
        .as_ref()
        .map(|landed| landed.starts_with(the_resolved_project.expect("the project is there")));
    assert_eq!(
        is_it_beside_the_clone,
        Ok(true),
        "**because the file lands inside the project and not merely at a path with the right name** — \
         a file named right and placed elsewhere is a file nobody will find. It landed at {written:?}, \
         **and the project root is resolved here because the confinement rule resolves, so on a \
         machine where `/var` is a symlink the two spellings differ and only the resolved ones \
         compare.**"
    );
}

#[test]
fn the_file_holds_exactly_what_the_agent_said() {
    let mut the_project = AProjectRemembering::empty();

    what_this_stage_kept_is_written(&mut the_project, "the repository is a Plane", 1)
        .expect("the first write of an artifact is permitted");

    assert_eq!(
        the_project.what_it_now_holds()[0].1,
        String::from("the repository is a Plane"),
        "**because a summary is prose nobody approved** — the gate rules on a document and a \
         document that is not the one the agent wrote is a document nobody has read. It holds {:?}",
        the_project.what_it_now_holds()
    );
}

#[test]
fn writing_the_same_artifact_at_the_same_version_twice_is_refused() {
    let mut the_project = AProjectRemembering::empty();
    what_this_stage_kept_is_written(&mut the_project, "the repository is a Plane", 1)
        .expect("the first write of an artifact is permitted");

    let refused = what_this_stage_kept_is_written(&mut the_project, "a different account of it", 1);

    assert!(
        matches!(
            refused,
            Err(WhatCouldNotBeWritten::ThereIsAlreadySuchAFile { .. })
        ),
        "**because an approval names the exact artifact and version it ruled on** — and a second write \
         at that version would replace the document under an approval that still names it, **so the \
         file a person read and the file on disk would be different documents with one name.** It \
         refused: {refused:?}"
    );
}

#[test]
fn the_same_artifact_at_a_later_version_is_a_second_file_and_is_permitted() {
    let mut the_project = AProjectRemembering::empty();
    what_this_stage_kept_is_written(&mut the_project, "the repository is a Plane", 1)
        .expect("the first write of an artifact is permitted");

    let the_second =
        what_this_stage_kept_is_written(&mut the_project, "a different account of it", 2);

    let is_it_the_second_version = the_second
        .as_ref()
        .map(|one_path| one_path.to_string_lossy().contains(".v2.json"));

    assert_eq!(
        is_it_the_second_version,
        Ok(true),
        "**because a second production is a second document and it has to be a second file** — the \
         version is in the name for exactly this. It landed at {the_second:?}"
    );
    assert_eq!(
        the_project.what_it_now_holds().len(),
        2,
        "**because the first file is still there** — and a writer that replaced it would leave the \
         first approval naming something nobody can open. It holds {:?}",
        the_project.what_it_now_holds()
    );
}

#[test]
fn a_refused_write_leaves_the_file_saying_what_it_said_before() {
    let mut the_project = AProjectRemembering::empty();
    what_this_stage_kept_is_written(&mut the_project, "the repository is a Plane", 1)
        .expect("the first write of an artifact is permitted");

    let _refused =
        what_this_stage_kept_is_written(&mut the_project, "a different account of it", 1);

    assert_eq!(
        the_project.what_it_now_holds()[0].1,
        String::from("the repository is a Plane"),
        "**because a refusal that truncated the file would destroy the document an approval names** — \
         and a refusal is not a permission to change something. It holds {:?}",
        the_project.what_it_now_holds()
    );
}
