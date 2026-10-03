//! A run remembers every artifact it wrote, **so the run can be written down from itself.**
//!
//! **Cycle 125 made a whole run write each artifact between asking for it and ruling on it, and then
//! threw the path away.** `what_an_agent_said_is_written` mapped the writer's answer to `()`,
//! **so seven files landed on disk and the run that produced them could not name one of them.**
//!
//! **A summary is the run in one page, and a page about a run that cannot say where its artifacts are
//! is a page about claims.** The artifacts are already on disk; nothing anywhere holds that.
//!
//! **The record is checked against the disk, not against itself.** A run that remembered the path the
//! writer *intended* rather than the file that *exists* would agree with itself perfectly and be wrong.

#[path = "doubles/a_model_that_answers.rs"]
mod a_model_that_answers;

use std::cell::RefCell;

use a_model_that_answers::AModelAnswering;
use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_whole_of_a_run::the_whole_of_a_run;
use aisdlc_run::where_a_run_stands::{WhereARunStands, the_work_is_handed_to_the_next_agent};
use aisdlc_run::where_an_artifact_of_this_run_landed::WhereAnArtifactOfThisRunLanded;

#[test]
fn a_finished_run_remembers_every_artifact_it_wrote_in_order() {
    let mut the_project =
        AProjectThatWrites::writing_everything("a-run-remembers", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let finished = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    let what_it_wrote = finished
        .as_ref()
        .map(WhereARunStands::everything_this_run_wrote);
    let every_artifact_is_named = what_it_wrote.as_ref().map(|written| {
        written
            .iter()
            .map(|one| one.which_artifact.0.clone())
            .collect::<Vec<String>>()
    });
    assert_eq!(
        every_artifact_is_named,
        Ok(every_artifact_the_pipeline_produces()),
        "**because seven artifacts and seven gates is the whole of the pipeline** — and a run that \
         wrote seven files and named five would leave a reader unable to find two of them. It wrote \
         {what_it_wrote:?}"
    );
}

#[test]
fn a_run_refused_at_the_third_gate_remembers_three_and_no_more() {
    let mut the_project = AProjectThatWrites::writing_everything("a-refused-run", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::refusing_at_the(3);

    let stopped = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    assert_eq!(
        stopped
            .as_ref()
            .map(WhereARunStands::everything_this_run_wrote)
            .map(<[WhereAnArtifactOfThisRunLanded]>::len),
        Ok(3),
        "**because the fourth agent was never asked and so wrote nothing** — and a record claiming a \
         fourth artifact would be naming a document that does not exist. It wrote {:?}",
        stopped
            .as_ref()
            .map(WhereARunStands::everything_this_run_wrote)
    );
}

#[test]
fn the_place_a_run_says_an_artifact_landed_is_the_file_that_is_on_disk() {
    let mut the_project = AProjectThatWrites::writing_everything("on-the-disk", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let finished = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);
    let the_run = finished.expect("a run every gate approves finishes");
    let every_one_is_on_disk = the_run
        .everything_this_run_wrote()
        .iter()
        .all(|one| std::path::Path::new(one.where_it_landed.as_str()).is_file());
    assert!(
        every_one_is_on_disk,
        "**because a run that remembered where it meant to write rather than what is there would \
         agree with itself perfectly and be wrong** — and this is the only check that can tell the \
         two apart. It remembered {:?}",
        the_run.everything_this_run_wrote()
    );
}

#[test]
fn handing_the_work_to_the_next_agent_carries_what_was_already_written() {
    let mut the_project = AProjectThatWrites::writing_everything("a-handover", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let finished = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);
    let the_run = finished.expect("a run every gate approves finishes");
    let handed_on = the_work_is_handed_to_the_next_agent(
        &the_run,
        AgentIdentifier::try_from("aisdlc-po-analyze").expect("an agent name"),
        WhichArtifact::try_from("AChangeToDeliver").expect("a name"),
    );

    assert_eq!(
        handed_on.map(|run| run.everything_this_run_wrote().len()),
        Ok(7),
        "**because the next agent inherits a run and not a blank page** — and a hand-off that \
         dropped the record would leave the summary naming nothing where the run wrote seven"
    );
}

/// A whole run given a project, a model and a person, **and it is one call so four tests are about
/// their own arrangement and not about the order of five collaborators.**
fn the_run_given(
    the_project: &mut AProjectThatWrites,
    a_model: &mut AModelAnswering,
    whoever_answers: &mut APersonWhoRules,
) -> Result<WhereARunStands, aisdlc_run::the_whole_of_a_run::AWholeRunThatCouldNotBeHad> {
    the_whole_of_a_run(
        &a_run_at_its_first_stage(),
        the_project,
        a_model,
        WhichOrganisationReasons::Openai,
        whoever_answers,
    )
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
}

impl aisdlc_run::APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(
        &mut self,
        a_question: &aisdlc_run::ApprovalQuestion,
    ) -> aisdlc_domain::approval::ApprovalDecision {
        let nth = self.how_often.borrow().saturating_add(1);
        *self.how_often.borrow_mut() = nth;
        let approved = self.refuses_at_the != Some(nth);
        aisdlc_domain::approval::ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            approved,
            aisdlc_domain::approval::WhoDecided::try_from("paligakrzychu")
                .expect("a name is not blank"),
            "the artifact answers the request",
        )
        .expect("a person may rule")
    }
}

/// A project that writes a real file, **in a directory of its own.**
struct AProjectThatWrites {
    where_it_lives: std::path::PathBuf,
}

impl AProjectThatWrites {
    fn writing_everything(in_a_directory_named: &str, a_work_item_named: &str) -> Self {
        let where_it_is = std::env::temp_dir().join(format!(
            "aisdlc-a-run-remembers-{}-{in_a_directory_named}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&where_it_is);
        let the_work_items_directory = where_it_is
            .join("steamnoid/wellbeing-tracker-public")
            .join(a_work_item_named);
        std::fs::create_dir_all(&the_work_items_directory)
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
