//! The run in one page, **written from the run's own state and not from a model.**
//!
//! **Cycles 126 and 127 gave a run everything a page needs and nothing writes it down.** Seven
//! artifacts it produced, the gates it stood at, and every ruling with the name of whoever made it
//! — **and the only place that would say so is a trace somebody has to know how to read.**
//!
//! **There is no model in this file at all**, and that is not a preference: **a summary written by a
//! model can flatter the run that produced it**, and the one artefact a person reads after a run is
//! the one that must not. A test that cannot prove the model was not consulted is a test that only
//! proves the model was helpful — **so the run here is built by hand and there is nothing that could
//! have been asked.**
//!
//! **A refused run's page is the one anybody reads**, so it says who said no and why, **which is the
//! fact a refusal leaves nowhere else in this system.**

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::{AnArtifactVersion, ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_path_in_the_project::a_path_in_the_project;
use aisdlc_run::the_path_inside_the_project::{
    ThePathInsideTheProject, the_path_inside_the_project,
};
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_summary_of_a_run_is_written::the_summary_of_a_run_is_written;
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::where_an_artifact_of_this_run_landed::WhereAnArtifactOfThisRunLanded;

#[test]
fn the_page_carries_the_request_in_the_words_somebody_typed() {
    let mut the_project = AProjectOnDisk::in_a_directory_of_its_own("the-request");
    let the_run = a_run_that_produced_seven_artifacts(&the_project);

    let the_page = the_page_written(&mut the_project, &the_run);

    assert!(
        the_page.contains("untrack the sqlite sidecar files"),
        "**because a page about a run that does not say what the run was asked for is a page \
         somebody has to correlate by hand** — and the request is the one thing in this system \
         nobody can derive. The page said: {the_page}"
    );
    assert!(
        the_page.contains("e2e.db-wal and e2e.db-shm are 4.1 MB"),
        "**because the title is a summary somebody wrote and the body is the request** — and a \
         page carrying the one nobody typed loses the words a person did. The page said: {the_page}"
    );
}

#[test]
fn every_artifact_is_listed_with_the_place_it_landed() {
    let mut the_project = AProjectOnDisk::in_a_directory_of_its_own("the-artifacts");
    let the_run = a_run_that_produced_seven_artifacts(&the_project);

    let the_page = the_page_written(&mut the_project, &the_run);

    let every_artifact_is_named = every_artifact_the_pipeline_produces()
        .iter()
        .all(|one| the_page.contains(one.as_str()));
    assert!(
        every_artifact_is_named,
        "**because a person who has to ask where a document is has not been given a page** — and \
         every one of the seven was written beside the clone by this very run. The page said: \
         {the_page}"
    );
}

#[test]
fn every_ruling_is_listed_with_who_decided_and_why() {
    let mut the_project = AProjectOnDisk::in_a_directory_of_its_own("the-rulings");
    let the_run = a_run_that_produced_seven_artifacts(&the_project);

    let the_page = the_page_written(&mut the_project, &the_run);

    assert_eq!(
        the_page.matches("paligakrzychu").count(),
        7,
        "**because a decision that cannot be tied to a name is not a decision anybody can audit** \
         — and every gate was answered by somebody. The page said: {the_page}"
    );
    assert!(
        the_page.contains("the artifact answers the request"),
        "**because a decision somebody else could have made carries no grounds** — and the page \
         after a run is exactly where somebody needs to know why. The page said: {the_page}"
    );
}

#[test]
fn a_refused_gate_is_named_as_a_refusal_and_not_listed_like_an_approval() {
    let mut the_project = AProjectOnDisk::in_a_directory_of_its_own("a-refused-run");
    let the_run = a_run_refused_at_its_third_gate(&the_project);

    let the_page = the_page_written(&mut the_project, &the_run);

    assert_eq!(
        the_page.matches("paligakrzychu").count(),
        3,
        "**because a refusal leaves no `Approval` anywhere else in this system** — and the page is \
         the last place a reader can find out who stopped the run. The page said: {the_page}"
    );
    assert!(
        the_page.contains("refused"),
        "**because a page that lists three rulings and says nothing about them reads as three \
         approvals** — and the reader is entitled to know one of them was not. The page said: \
         {the_page}"
    );
}

/// The page as a person would read it, **and it is read back off the disk rather than off whatever
/// the writer returned** — a page a reader cannot open is not a page.
fn the_page_written(the_project: &mut AProjectOnDisk, the_run: &WhereARunStands) -> String {
    let where_the_page_landed = the_summary_of_a_run_is_written(the_project, the_run)
        .expect("a run's own directory is there to hold its page");
    the_project
        .the_text_of(&where_the_page_landed)
        .expect("the page was just written and is there to be read")
}

/// The seven artifacts of the pipeline, **in the order they are produced.**
fn every_artifact_the_pipeline_produces() -> Vec<String> {
    vec![
        "DiscoveryReport",
        "Requirements",
        "QaPlan",
        "ArchitectureReport",
        "SecurityReport",
        "ImplementationPlan",
        "VerificationReport",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

/// A run that got all the way, **and it is built by hand because this file has no model in it.**
fn a_run_that_produced_seven_artifacts(the_project: &AProjectOnDisk) -> WhereARunStands {
    let mut the_run = a_run_at_its_first_stage();
    for which_artifact in every_artifact_the_pipeline_produces() {
        the_run = the_run.with_what_it_wrote(the_artifact_landed_at(&which_artifact, the_project));
        the_run = the_run.with_the_ruling(a_ruling_about(&which_artifact, true));
    }
    the_run
}

/// A run somebody stopped at its third gate, **and the third ruling is a refusal — which is the whole
/// point of the page.**
fn a_run_refused_at_its_third_gate(the_project: &AProjectOnDisk) -> WhereARunStands {
    let mut the_run = a_run_at_its_first_stage();
    let every_artifact = every_artifact_the_pipeline_produces();
    for (which_gate, which_artifact) in every_artifact.iter().enumerate() {
        let was_it_approved = which_gate != 3;
        the_run = the_run.with_what_it_wrote(the_artifact_landed_at(which_artifact, the_project));
        the_run = the_run.with_the_ruling(a_ruling_about(which_artifact, was_it_approved));
    }
    the_run
}

/// One artifact landing beside the clone, **and the place is resolved the way the writer resolves
/// it — so the page names a file a reader could actually open rather than a path a fixture made
/// up.**
fn the_artifact_landed_at(
    which_artifact: &str,
    the_project: &AProjectOnDisk,
) -> WhereAnArtifactOfThisRunLanded {
    let as_written =
        format!("steamnoid/wellbeing-tracker-public/WELLBEINGT-42/{which_artifact}.v1.json");
    let inside = a_path_in_the_project(&as_written).expect("a path inside the project is a path");
    let where_it_landed = the_path_inside_the_project(the_project.where_it_lives(), &inside)
        .expect("the work item's own directory is there to hold it");
    std::fs::write(
        std::path::PathBuf::from(where_it_landed.as_str()),
        "the artifact itself",
    )
    .expect("a file this machine will hold");
    WhereAnArtifactOfThisRunLanded::an_artifact_landed(
        WhichArtifact::try_from(which_artifact).expect("a name is not blank"),
        AnArtifactVersion(1),
        where_it_landed,
    )
}

/// One gate's answer, **and it names a person every time so a page carrying it can be audited.**
fn a_ruling_about(which_artifact: &str, approved: bool) -> ApprovalDecision {
    ApprovalDecision::the_ruling_about(
        WorkItemId::try_from("WELLBEINGT-42").expect("a name is not blank"),
        WhichArtifact::try_from(which_artifact).expect("a name is not blank"),
        AnArtifactVersion(1),
        approved,
        WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
        "the artifact answers the request",
    )
    .expect("a person may rule")
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
    let who_asked = aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person")
        .expect("a name is a name");
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

/// A project a run writes real files into, **and it can read them back so a test never has to
/// believe what a writer returned.**
struct AProjectOnDisk {
    where_it_lives: std::path::PathBuf,
}

impl AProjectOnDisk {
    fn in_a_directory_of_its_own(in_a_directory_named: &str) -> Self {
        let where_it_is = std::env::temp_dir().join(format!(
            "aisdlc-a-summary-{}-{in_a_directory_named}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&where_it_is);
        std::fs::create_dir_all(
            where_it_is.join("steamnoid/wellbeing-tracker-public/WELLBEINGT-42"),
        )
        .expect("a directory this machine will hold");
        Self {
            where_it_lives: where_it_is,
        }
    }
}

impl TheProjectOnDisk for AProjectOnDisk {
    fn where_it_lives(&self) -> &std::path::Path {
        self.where_it_lives.as_path()
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        Vec::new()
    }

    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String> {
        std::fs::read_to_string(std::path::PathBuf::from(a_path.as_str())).ok()
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
