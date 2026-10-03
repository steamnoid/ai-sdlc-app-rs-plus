//! A run remembers every ruling, **and a refusal is remembered too.**
//!
//! **A run cannot remember who ruled from an `Approval`, because a refusal leaves no `Approval`.**
//! That is the domain's own asymmetry — *refusing is a decision and not a record* — **and a run that
//! remembered only approvals would have nothing at all to say about the gate a person said no at.**
//!
//! **So what the run keeps is the decision and not the record.** The decision carries `approved`,
//! `decided_by` and `reason` for both answers; the `Approval` stays what it is, the record of what
//! was approved. **A summary written from records would name seven gates of a run that had eight,
//! and the one it left out is the one anybody reads a refused run for.**
//!
//! **The gate asks once and keeps both**, because asking the person twice would be a second answer
//! to one question and a person asked twice may answer twice.

#[path = "doubles/a_model_that_answers.rs"]
mod a_model_that_answers;

use std::cell::RefCell;

use a_model_that_answers::AModelAnswering;
use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::{ApprovalDecision, WhoDecided};
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
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

#[test]
fn a_run_every_gate_approves_remembers_every_ruling_in_order() {
    let mut the_project = AProjectThatWrites::writing_everything("every-ruling", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let finished = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    assert_eq!(
        finished
            .as_ref()
            .map(WhereARunStands::every_ruling_this_run_received)
            .map(every_artifact_every_ruling_was_about),
        Ok(every_artifact_the_pipeline_produces()),
        "**because seven gates and seven rulings is the whole of the pipeline** — and a run that \\
         remembered six would leave a reader unable to say who stood behind one of the artifacts \\
         they were handed. It remembered {:?}",
        finished
            .as_ref()
            .map(WhereARunStands::every_ruling_this_run_received)
    );
}

#[test]
fn a_refusal_is_remembered_and_so_is_who_refused_it() {
    let mut the_project = AProjectThatWrites::writing_everything("a-refusal", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::refusing_at_the(3);

    let stopped = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    let the_rulings = stopped
        .as_ref()
        .map(WhereARunStands::every_ruling_this_run_received)
        .unwrap_or_default();
    assert_eq!(
        the_rulings.len(),
        3,
        "**because the fourth agent was never asked and so nobody ruled on its output** — and four \\
         rulings would name a decision nobody made. It remembered {the_rulings:?}"
    );
    assert!(
        the_rulings
            .iter()
            .all(|one| one.decided_by.0.as_str() == "paligakrzychu"),
        "**because a record that cannot be tied to a name is not a record anybody can audit** — \\
         and a refusal is the ruling a reader most wants to attribute. It remembered {the_rulings:?}"
    );
}

#[test]
fn the_third_ruling_says_no_and_the_first_two_say_yes() {
    let mut the_project = AProjectThatWrites::writing_everything("yes-and-no", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::refusing_at_the(3);

    let stopped = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    let which_gate_said_no = stopped
        .as_ref()
        .map(WhereARunStands::every_ruling_this_run_received)
        .map(|rulings| {
            rulings
                .iter()
                .map(|one| !one.approved)
                .collect::<Vec<bool>>()
        });
    assert_eq!(
        which_gate_said_no,
        Ok(vec![false, false, true]),
        "**because a run that recorded only approvals would hold no ruling at all here** — and a \\
         refusal that leaves no record is exactly what makes a reader unable to say who stopped the \\
         run and why. It remembered {:?}",
        stopped
            .as_ref()
            .map(WhereARunStands::every_ruling_this_run_received)
    );
}

#[test]
fn every_ruling_carries_a_reason_a_reader_can_read() {
    let mut the_project = AProjectThatWrites::writing_everything("with-reasons", "WELLBEINGT-42");
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let finished = the_run_given(&mut the_project, &mut a_model, &mut whoever_answers);

    let the_reasons = finished
        .as_ref()
        .map(WhereARunStands::every_ruling_this_run_received)
        .map(|rulings| {
            rulings
                .iter()
                .map(|one| one.reason.clone())
                .collect::<Vec<String>>()
        });
    assert!(
        the_reasons
            .as_ref()
            .map(|reasons| reasons.iter().all(|one| !one.trim().is_empty()))
            .unwrap_or(false),
        "**because a decision somebody else could have made carries no grounds** — and the trace \\
         after a run is exactly where somebody needs to know why. The reasons were {the_reasons:?}"
    );
}

/// What every ruling was about, **and the artifact alone is the question** — the version is one for
/// every production this run made and is asserted by its own test where it could differ.
fn every_artifact_every_ruling_was_about(
    the_rulings: &[aisdlc_domain::approval::ApprovalDecision],
) -> Vec<String> {
    the_rulings
        .iter()
        .map(|one| one.which_artifact.0.clone())
        .collect()
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

impl APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let which_gate = self.how_often.borrow().saturating_add(1);
        *self.how_often.borrow_mut() = which_gate;
        let approved = self.refuses_at_the != Some(which_gate);
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

/// A project that writes a real file, **in a directory of its own.**
struct AProjectThatWrites {
    where_it_lives: std::path::PathBuf,
}

impl AProjectThatWrites {
    fn writing_everything(in_a_directory_named: &str, a_work_item_named: &str) -> Self {
        let where_it_is = std::env::temp_dir().join(format!(
            "aisdlc-a-run-remembers-who-{}-{in_a_directory_named}",
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
