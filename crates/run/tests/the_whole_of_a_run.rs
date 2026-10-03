//! The whole of a run, from its first agent to the one that delivers.
//!
//! **Every stage asked, every gate ruled on, and the rulings walked in.** A stage can be worked
//! (#183) and a run can be walked (#181), **and nothing joined the two** — so a run could have every
//! one of its seven agents work in perfect isolation and never be asked to finish.
//!
//! **A refusal stops the run where it is, and no later agent is asked.** A person who says no is
//! deciding, **and a run that keeps asking agents after a decision is a run that treats a decision
//! as a delay** — which is the defect behind three delivered pull requests whose diff was two test
//! files and no route.
use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_whole_of_a_run::{AWholeRunThatCouldNotBeHad, the_whole_of_a_run};
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::where_a_run_stands::WhereARunStands;

use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A project that writes a real file, **and it writes real files on purpose** — a double that kept
/// its artifacts in memory would make a reading test pass while nothing a person could open existed.
struct AProjectThatWrites {
    where_it_lives: std::path::PathBuf,
}

impl AProjectThatWrites {
    /// A project that writes everything, **in a directory of its own.**
    ///
    /// **It writes and nothing more here**, because this file's doubles are local already and **a
    /// double asked for more than its own test uses is reported dead in every binary that has it.**
    fn writing_everything(in_a_directory_named: &str, a_work_item_named: &str) -> Self {
        let where_the_run_writes = where_this_process_keeps_its_projects(in_a_directory_named);
        let _ = std::fs::remove_dir_all(&where_the_run_writes);
        let the_work_items_directory = where_the_run_writes
            .join("steamnoid/wellbeing-tracker-public")
            .join(a_work_item_named);
        std::fs::create_dir_all(&the_work_items_directory)
            .expect("a directory this machine will hold");
        Self {
            where_it_lives: where_the_run_writes,
        }
    }
}

/// Where this process keeps its projects, **and it is named for one caller so two tests cannot
/// collide.**
fn where_this_process_keeps_its_projects(in_a_directory_named: &str) -> std::path::PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-{}-{in_a_directory_named}",
        std::process::id()
    ));
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is
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
        let the_path = std::path::PathBuf::from(a_path.as_str());
        std::fs::write(&the_path, what_it_says).expect("a file this machine will hold");
        Ok(())
    }
}

/// A model that answers every question, **and records what it was asked for in order.**
struct AModelThatAnswers {
    stops_answering_at: Option<usize>,
    what_it_was_asked_for: RefCell<Vec<String>>,
}

impl AModelThatAnswers {
    fn answering_everything() -> Self {
        Self {
            stops_answering_at: None,
            what_it_was_asked_for: RefCell::new(Vec::new()),
        }
    }

    fn answering_until_the(nth: usize) -> Self {
        Self {
            stops_answering_at: Some(nth),
            what_it_was_asked_for: RefCell::new(Vec::new()),
        }
    }

    fn what_it_was_asked_for(&self) -> Vec<String> {
        self.what_it_was_asked_for.borrow().clone()
    }
}

impl AReasoningModel for AModelThatAnswers {
    fn what_it_says_when_asked_for(
        &mut self,
        which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        let nth = self.what_it_was_asked_for.borrow().len() + 1;
        self.what_it_was_asked_for
            .borrow_mut()
            .push(which_artifact.0.clone());
        if self.stops_answering_at == Some(nth) {
            return Err(AnAnswerNobodyCanRead::because("there is no endpoint"));
        }
        Ok(format!("{which_artifact} was produced"))
    }
}

/// A person who rules, **and refuses at one gate and nowhere else.**
struct APersonWhoRules {
    refuses_at_the: Option<usize>,
    what_they_were_asked: RefCell<Vec<String>>,
}

impl APersonWhoRules {
    fn who_approves_everything() -> Self {
        Self {
            refuses_at_the: None,
            what_they_were_asked: RefCell::new(Vec::new()),
        }
    }

    fn who_refuses_at_the(nth: usize) -> Self {
        Self {
            refuses_at_the: Some(nth),
            what_they_were_asked: RefCell::new(Vec::new()),
        }
    }

    fn what_they_were_asked(&self) -> Vec<String> {
        self.what_they_were_asked.borrow().clone()
    }
}

impl APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let nth = self.what_they_were_asked.borrow().len() + 1;
        self.what_they_were_asked
            .borrow_mut()
            .push(a_question.about_which_artifact().0.clone());
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

#[test]
fn every_agent_in_the_pipeline_is_asked_and_in_order() {
    let mut a_model = AModelThatAnswers::answering_everything();

    let the_run = the_whole_of_a_run(
        &a_run_standing_at_the_stage_its_first_agent_is_working_on(),
        &mut AProjectThatWrites::writing_everything("every-agent-in-order", "WELLBEINGT-5"),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::who_approves_everything(),
    );

    assert!(
        the_run.is_ok(),
        "**because a run that asked four of its seven agents delivered something nobody asked \
         for** — and a pipeline that quietly skips stages is the router defect that cost the \
         sibling a run while its suite stayed green. It said: {the_run:?}"
    );
    assert_eq!(
        a_model.what_it_was_asked_for(),
        every_artifact_the_pipeline_produces(),
        "**because the order is the pipeline's and not the model's** — a stage asked in another \
         order is a stage asked about a work item that does not exist yet. It was asked for {:?}",
        a_model.what_it_was_asked_for()
    );
}

#[test]
fn every_gate_is_asked_about_a_different_artifact_and_seven_times() {
    let mut a_model = AModelThatAnswers::answering_everything();
    let mut whoever_answers = APersonWhoRules::who_approves_everything();

    the_whole_of_a_run(
        &a_run_standing_at_the_stage_its_first_agent_is_working_on(),
        &mut AProjectThatWrites::writing_everything(
            "every-gate-a-different-artifact",
            "WELLBEINGT-5",
        ),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut whoever_answers,
    )
    .expect("a run every gate approves finishes");

    assert_eq!(
        whoever_answers.what_they_were_asked(),
        every_artifact_the_pipeline_produces(),
        "**because a gate asked about the same artifact twice is a gate that approved a document \
         nobody changed** — and the version an approval names exists for exactly that. They were \
         asked about {:?}",
        whoever_answers.what_they_were_asked()
    );
}

#[test]
fn a_run_every_gate_approves_ends_with_the_delivery_agent_holding_the_work() {
    let mut a_model = AModelThatAnswers::answering_everything();

    let the_run = the_whole_of_a_run(
        &a_run_standing_at_the_stage_its_first_agent_is_working_on(),
        &mut AProjectThatWrites::writing_everything("the-delivery-holds-it", "WELLBEINGT-5"),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::who_approves_everything(),
    );

    let who_holds_it = the_run.as_ref().map(WhereARunStands::who_holds_it);
    assert_eq!(
        who_holds_it,
        Ok(Some(
            aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-pr")
                .expect("an agent name")
        )),
        "**because the seventh gate's whole purpose is to hand the work to the delivery** — a run \
         that stopped at the verification delivered nothing. It is held by {who_holds_it:?}"
    );
}

#[test]
fn a_person_who_refuses_ends_the_run_and_no_later_agent_is_asked() {
    let mut a_model = AModelThatAnswers::answering_everything();

    let mut whoever_answers = APersonWhoRules::who_refuses_at_the(3);
    let the_run = the_run_given("a-refusal-ends-the-run", &mut a_model, &mut whoever_answers);
    let where_it_stands = the_run
        .as_ref()
        .map(WhereARunStands::the_stage_it_stands_at);

    assert_eq!(
        where_it_stands,
        Ok(Stage::Done),
        "**because refusing is a decision and not a delay** — and a run that kept working after one \
         would be a run treating a decision as a delay, **which is the defect behind three \
         delivered pull requests whose diff was two test files and no route.** It stands at \
         {where_it_stands:?}"
    );
    assert_eq!(
        a_model.what_it_was_asked_for().len(),
        3,
        "**because a person said no at the third gate and the fourth agent was never asked** — \
         four artifacts asked for would mean one exists that nobody ruled on. It was asked for {:?}",
        a_model.what_it_was_asked_for()
    );
}

#[test]
fn a_model_that_could_not_be_reached_ends_the_run_and_names_the_artifact() {
    let mut a_model = AModelThatAnswers::answering_until_the(4);

    let refused = the_whole_of_a_run(
        &a_run_standing_at_the_stage_its_first_agent_is_working_on(),
        &mut AProjectThatWrites::writing_everything("a-model-that-cannot-answer", "WELLBEINGT-5"),
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut APersonWhoRules::who_approves_everything(),
    );

    assert!(
        matches!(
            refused,
            Err(AWholeRunThatCouldNotBeHad::NobodyWasAskedFor { .. })
        ),
        "**because a stage whose agent could not be reached produces nothing to rule on** — and \
         walking past it would put the next agent to work on an artifact nobody wrote. It \
         refused: {refused:?}"
    );
}

/// A whole run given a model and a person, **and it is one line so four of the five tests are about
/// their own arrangement rather than about this call.**
fn the_run_given(
    in_a_directory_named: &str,
    a_model: &mut AModelThatAnswers,
    whoever_answers: &mut APersonWhoRules,
) -> Result<WhereARunStands, AWholeRunThatCouldNotBeHad> {
    the_whole_of_a_run(
        &a_run_standing_at_the_stage_its_first_agent_is_working_on(),
        &mut AProjectThatWrites::writing_everything(in_a_directory_named, "WELLBEINGT-5"),
        a_model,
        WhichOrganisationReasons::Openai,
        whoever_answers,
    )
}

/// A run standing at the stage its first agent is working on, **and on the artifact that agent
/// produces.**
fn a_run_standing_at_the_stage_its_first_agent_is_working_on() -> WhereARunStands {
    let the_request = aisdlc_domain::commissioned_request::CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    };
    let answering = a_work_item_held_by_the_discovering_agent().answering(the_request);
    WhereARunStands::at_the_first_stage(
        answering.expect("a work item answering a request may stand where it was put"),
        "DiscoveryReport",
    )
}

/// The work item the discovery agent is holding, **and a request is added to it afterwards** — the
/// builder takes the request rather than the constructor, **so a caller that forgot the request is
/// refused by the model rather than silently asked nothing.**
fn a_work_item_held_by_the_discovering_agent() -> aisdlc_domain::work_item::WorkItem {
    let the_id = aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository =
        aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves");
    let the_discipline = Role::try_from("PO").expect("a discipline");
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-discover")
        .expect("an agent name");
    aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::InProgressByAgent,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one")
}
