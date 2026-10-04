//! A whole run leaves a page, **because a run that delivered a pull request and told nobody has done
//! everything and said nothing.**
//!
//! **Cycle 128 wrote `summary.md` and cycle 129 made a run make its own checkout, and nothing
//! connects the two.** The writer is tested; the writer has no caller — **exactly the defect cycle
//! 125 found for the artifact writer, one slice later and one function further from the reader.**
//!
//! **The page is written on both endings, and the refused one matters more.** A delivered run's page
//! is a convenience; **a refused run's page is the only place in this system that says who stopped
//! the run and why**, because a refusal leaves no `Approval` anywhere else.
//!
//! **A page that could not be written is a refusal and not a page.** A trace claiming a run left a
//! page when the disk refused it is the same defect as the eighth gate being the seventh decision
//! again — **a record that does not agree with the disk.**

#[path = "doubles/a_program_answering.rs"]
mod a_program_answering;

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
use aisdlc_run::ApprovalQuestion;
use aisdlc_run::a_branch_goes_to_a_remote::ABranchToPush;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::a_pull_request_is_opened::{
    APullRequestToOpen, the_request_a_run_makes_of_its_own_change,
};
use aisdlc_run::a_run_from_a_request_to_a_pull_request::{
    ARunThatDidNotFinish, WhatARunLeft, WhatThereIsToDeliver,
    a_run_from_a_request_to_a_pull_request,
};
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::where_a_run_stands::WhereARunStands;

use a_program_answering::AProgramAnswering;

#[test]
fn a_run_that_delivered_leaves_a_page_naming_the_pull_request_github_opened() {
    let mut the_project = AProjectOnDisk::in_a_directory_of_its_own("a-delivered-run");
    let finished = a_run_given(&mut the_project, APersonWhoRules::approving_everything());

    let the_page = finished
        .as_ref()
        .map(WhatARunLeft::the_page_it_wrote)
        .map(|page| page.what_the_page_says().to_string());

    let page_says = the_page.as_ref().map(String::as_str).unwrap_or_default();
    assert!(
        page_says.contains("https://github.com/steamnoid/wellbeing-tracker-public/pull/17"),
        "**because the page is the only artefact a person reads after a run and the link is the \\
         whole of what it is for** — a page without it sends a reader back to a trace. The page \\
         said: {page_says}"
    );
}

#[test]
fn a_run_that_was_refused_leaves_a_page_naming_who_stopped_it() {
    let mut the_project = AProjectOnDisk::in_a_directory_of_its_own("a-refused-run");
    let finished = a_run_given(&mut the_project, APersonWhoRules::refusing_at_the(3));

    let page_says = finished
        .as_ref()
        .map(WhatARunLeft::the_page_it_wrote)
        .map(|page| page.what_the_page_says().to_string())
        .unwrap_or_default();

    assert!(
        page_says.contains("refused"),
        "**because a refusal leaves no `Approval` anywhere else in this system** — so this page is \\
         the last place a reader can find out who stopped the run and why. The page said: \\
         {page_says}"
    );
    assert_eq!(
        page_says.matches("paligakrzychu").count(),
        3,
        "**because a person refused at the third gate and there is no record of it but this** — \\
         four would name a decision nobody made. The page said: {page_says}"
    );
}

#[test]
fn a_page_the_disk_refused_is_a_refusal_and_not_a_page() {
    let mut the_project = AProjectOnDisk::refusing_the_page();
    let finished = a_run_given(&mut the_project, APersonWhoRules::approving_everything());

    assert!(
        matches!(finished, Err(ARunThatDidNotFinish::ThePage { .. })),
        "**because a run that reported a page the disk refused is a record that does not agree with \\
         the disk** — which is the same defect as the eighth gate being the seventh decision again, \\
         and it cost the same. It refused: {finished:?}"
    );
}

/// A whole run given a project and a person, **and it is one call so three tests are about their own
/// arrangement and not about the order of seven collaborators.**
fn a_run_given(
    the_project: &mut AProjectOnDisk,
    whoever_answers: APersonWhoRules,
) -> Result<WhatARunLeft, ARunThatDidNotFinish> {
    let mut a_model = AModelAnswering::answering_everything();
    let mut whoever_answers = whoever_answers;
    a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        the_project,
        &mut a_model,
        WhichOrganisationReasons::Openai,
        &mut whoever_answers,
        &what_there_is_to_deliver(),
        &mut AProgramAnswering::refusing_the_pull_request(false),
    )
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

/// What there is to deliver, **and it is three things because the clone is the run's own now.**
fn what_there_is_to_deliver() -> WhatThereIsToDeliver {
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_branch =
        ABranchToPush::a_branch_named("aisdlc/WELLBEINGT-42").expect("a branch may take that name");
    let what_to_open = the_request_this_run_makes(the_repository);
    let the_credential =
        ACredential::the_key_a_deployment_supplied("a-key-not-a-real-one").expect("a key");
    WhatThereIsToDeliver::for_this_run(the_branch, what_to_open, the_credential)
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

/// A model that answers everything, **and nothing else — this file is about the page and not about a
/// model, and a double asked for more than its own test uses is reported dead.**
struct AModelAnswering;

impl AModelAnswering {
    fn answering_everything() -> Self {
        Self
    }
}

impl AReasoningModel for AModelAnswering {
    fn what_it_says_when_asked_for(
        &mut self,
        _which_artifact: &aisdlc_domain::approval::WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        Ok(String::from("an artifact, of the length a real one is"))
    }
}

/// A project a run writes real files into, **and it can be told to refuse the page.**
struct AProjectOnDisk {
    where_it_lives: std::path::PathBuf,
    refuses_the_page: bool,
}

impl AProjectOnDisk {
    fn in_a_directory_of_its_own(in_a_directory_named: &str) -> Self {
        Self::new(in_a_directory_named, false)
    }

    fn refusing_the_page() -> Self {
        Self::new("a-refused-page", true)
    }

    fn new(in_a_directory_named: &str, refuses_the_page: bool) -> Self {
        let where_it_is = std::env::temp_dir().join(format!(
            "aisdlc-a-run-leaves-a-page-{}-{in_a_directory_named}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&where_it_is);
        let the_project_directory = where_it_is
            .join("steamnoid/wellbeing-tracker-public")
            .join("WELLBEINGT-42");
        std::fs::create_dir_all(the_project_directory).expect("a directory this machine will hold");
        Self {
            where_it_lives: where_it_is,
            refuses_the_page,
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

    fn the_text_of(&mut self, _a_path: &ThePathInsideTheProject) -> Option<String> {
        None
    }

    /// Whether anything stands at a path, **and this double answers about the real disk** so a run
    /// copying into a directory it cleared really is copying into nothing.
    fn does_anything_stand_at(&self, a_path: &std::path::Path) -> bool {
        a_path.exists()
    }

    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        if self.refuses_the_page && a_path.as_str().ends_with("summary.md") {
            return Err(TheProjectRefusedToWrite::there_was_no_room_for_it(
                a_path,
                "this project is a double told to refuse the page",
            ));
        }
        std::fs::write(std::path::PathBuf::from(a_path.as_str()), what_it_says)
            .expect("a file this machine will hold");
        Ok(())
    }
}

/// A person who approves everything up to one gate, **and refuses at that one and nowhere else** —
/// it is local rather than the shared one because that one also counts and records what each gate was
/// asked about, **and this test is about the page and neither of those is in it.**
struct APersonWhoRules {
    refuses_at_the: Option<usize>,
    how_often: std::cell::RefCell<usize>,
}

impl APersonWhoRules {
    fn approving_everything() -> Self {
        Self {
            refuses_at_the: None,
            how_often: std::cell::RefCell::new(0),
        }
    }

    fn refusing_at_the(nth: usize) -> Self {
        Self {
            refuses_at_the: Some(nth),
            how_often: std::cell::RefCell::new(0),
        }
    }
}

impl aisdlc_run::APersonToDecide for APersonWhoRules {
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
