//! The run is delivered, and the work item is finished only when a person says so.
//!
//! **A branch on a remote is not a delivery, and a pull request nobody ruled on is not one
//! either.** Both halves of the delivery exist — `a_branch_is_pushed` and `a_pull_request_is_opened`
//! — **and nothing joined them to a run or moved the work item afterwards.** A run could push a
//! branch, open a pull request, and leave its work item standing at `IN_PROGRESS_BY_AGENT`.
//!
//! **`DONE` is reachable from one stage only, and that stage is a gate.** The transition table has
//! no edge from `IN_PROGRESS_BY_AGENT` into `DONE`, **so a delivery cannot finish a run by itself** —
//! it presents what it delivered, and a person rules on it, **which is the one edge the table allows
//! and the only place a refused delivery stops without being finished.**

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhoDecided};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::stage::Stage;
use aisdlc_run::a_branch_goes_to_a_remote::ABranchToPush;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::a_pull_request_is_opened::{
    APullRequestToOpen, the_request_a_run_makes_of_its_own_change,
};
use aisdlc_run::the_run_is_delivered::{
    ADeliveryThatDidNotHappen, the_run_is_delivered,
};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use aisdlc_run::where_a_run_stands::WhereARunStands;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A credential a deployment supplied, **and a real one is never written down in a test.**
fn a_credential() -> ACredential {
    ACredential::the_key_a_deployment_supplied("a-key-no-real-one-is-here").expect("a key")
}

/// A program that answers `git` and `gh` alike, **and refuses exactly one of them when told to.**
struct AProgramAnswering {
    refuses_the_push: bool,
    refuses_the_pull_request: bool,
}

impl AProgramAnswering {
    fn answering_everything() -> Self {
        Self {
            refuses_the_push: false,
            refuses_the_pull_request: false,
        }
    }

    fn refusing_the_push() -> Self {
        Self {
            refuses_the_push: true,
            refuses_the_pull_request: false,
        }
    }

    fn refusing_the_pull_request() -> Self {
        Self {
            refuses_the_push: false,
            refuses_the_pull_request: true,
        }
    }

    fn refusing_nothing_but_telling_us_about_a_nothing() -> Self {
        Self::answering_everything()
    }
}

impl AProgramWeRun for AProgramAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        WhatTheProgramSaid::of(true, &self.what_it_says_for(what_it_is_asked_to_do))
    }
}

impl AProgramAnswering {
    fn what_it_says_for(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> String {
        match what_it_is_asked_to_do {
            WhatWeTellAProgramToDo::ReadTheRemote { .. } => String::new(),
            WhatWeTellAProgramToDo::PushTheBranch { .. } if self.refuses_the_push => {
                String::from("fatal: the remote end hung up")
            }
            WhatWeTellAProgramToDo::PushTheBranch { .. } => String::from("to ../remote.git"),
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } if self.refuses_the_pull_request => {
                String::from("gh: a pull request is not a filing")
            }
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } => String::from(
                r#"{"number":17,"html_url":"https://github.com/steamnoid/wellbeing-tracker-public/pull/17"}"#,
            ),
            every_other_request => format!("{every_other_request:?}"),
        }
    }
}

/// A person who approves the delivery, **or refuses it.**
struct APersonWhoRules {
    approved: bool,
    what_they_were_asked: RefCell<Vec<String>>,
}

impl APersonWhoRules {
    fn who_approves() -> Self {
        Self {
            approved: true,
            what_they_were_asked: RefCell::new(Vec::new()),
        }
    }

    fn who_refuses() -> Self {
        Self {
            approved: false,
            what_they_were_asked: RefCell::new(Vec::new()),
        }
    }

    fn what_they_were_asked(&self) -> Vec<String> {
        self.what_they_were_asked.borrow().clone()
    }
}

impl APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        self.what_they_were_asked
            .borrow_mut()
            .push(a_question.about_which_artifact().0.clone());
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            self.approved,
            WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
            "the change is what was approved and what was delivered",
        )
        .expect("a person may rule")
    }
}

/// The clone the change was made in, **and it is a name rather than a directory because nothing here
/// runs `git`.**
fn the_clone_the_change_was_made_in() -> ARepositoryToClone {
    ARepositoryToClone::of(
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        std::path::PathBuf::from("/tmp/a-clone-this-test-never-reads"),
    )
}

#[test]
fn a_delivery_pushes_the_branch_opens_the_pull_request_and_finishes_the_work_item() {
    let the_program = AProgramAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::who_approves();

    let delivered = the_run_is_delivered(
        &a_run_standing_at_the_delivery(),
        &the_program,
        &the_clone_the_change_was_made_in(),
        &the_branch_this_run_pushes(),
        &the_request_this_run_makes(),
        Some(&a_credential()),
        &mut whoever_answers,
    );

    assert_eq!(
        delivered
            .as_ref()
            .map(|what| what.where_the_work_stands().the_stage_it_stands_at()),
        Ok(Stage::Done),
        "**because `DONE` is reachable from one stage and that stage is a gate** — a work item \
         left standing at `IN_PROGRESS_BY_AGENT` after a pull request exists **is the `DONE` column \
         of the board that nothing can ever fill.** It stands at {:?}",
        delivered
            .as_ref()
            .map(|what| what.where_the_work_stands().the_stage_it_stands_at())
    );
}

#[test]
fn a_delivery_comes_back_with_the_number_and_the_address_github_composed() {
    let the_program = AProgramAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::who_approves();

    let delivered = the_run_is_delivered(
        &a_run_standing_at_the_delivery(),
        &the_program,
        &the_clone_the_change_was_made_in(),
        &the_branch_this_run_pushes(),
        &the_request_this_run_makes(),
        Some(&a_credential()),
        &mut whoever_answers,
    );

    let where_it_answers = delivered
        .as_ref()
        .map(|what| what.which_pull_request().where_it_answers());
    assert_eq!(
        where_it_answers,
        Ok("https://github.com/steamnoid/wellbeing-tracker-public/pull/17"),
        "**because a delivery that recorded an address it composed would record a link to a page \
         that does not exist** — which is the exact defect `CommissionedIssue` had and why \
         `AWhereItWasFiled` is the only door an address comes through. It answers at {where_it_answers:?}"
    );
}

#[test]
fn a_person_who_refuses_the_delivery_leaves_the_work_item_at_the_gate_and_not_finished() {
    let the_program = AProgramAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::who_refuses();

    let delivered = the_run_is_delivered(
        &a_run_standing_at_the_delivery(),
        &the_program,
        &the_clone_the_change_was_made_in(),
        &the_branch_this_run_pushes(),
        &the_request_this_run_makes(),
        Some(&a_credential()),
        &mut whoever_answers,
    );

    let where_it_stands = delivered
        .as_ref()
        .map(|what| what.where_the_work_stands().the_stage_it_stands_at());
    assert_eq!(
        where_it_stands,
        Ok(Stage::AwaitingHumanApproval),
        "**because a refused delivery is not a delivered work item** — and reaching `DONE` anyway \
         **would put a refused delivery in the board's finished column, which is a decision \
         recorded as a delivery.** It stands at {where_it_stands:?}"
    );
    assert_eq!(
        delivered.as_ref().ok().map(|what| what.was_it_approved()),
        Ok(false),
        "**because a refusal is a decision and a delivery that reported one as a yes is a run \
         approving its own work** — the one thing the gate exists to prevent. It said \
         {delivered:?}"
    );
}

#[test]
fn a_branch_that_did_not_go_leaves_the_work_item_where_it_was_and_opens_nothing() {
    let the_program = AProgramAnswering::refusing_the_push();
    let mut whoever_answers = APersonWhoRules::who_approves();

    let refused = the_run_is_delivered(
        &a_run_standing_at_the_delivery(),
        &the_program,
        &the_clone_the_change_was_made_in(),
        &the_branch_this_run_pushes(),
        &the_request_this_run_makes(),
        Some(&a_credential()),
        &mut whoever_answers,
    );

    assert!(
        matches!(refused, Err(ADeliveryThatDidNotHappen::TheBranchDidNotGo { .. })),
        "**because a branch that did not go is not a delivery** — and a run that moved on to the \
         pull request would be offering GitHub a head that does not exist. It refused: {refused:?}"
    );
    assert_eq!(
        whoever_answers.what_they_were_asked(),
        Vec::<String>::new(),
        "**because nobody ruled on anything** — a gate asked after a failed push is a gate asked \
         about a delivery that never happened. They were asked about {:?}",
        whoever_answers.what_they_were_asked()
    );
}

#[test]
fn a_pull_request_that_did_not_open_leaves_the_work_item_where_it_was() {
    let the_program = AProgramAnswering::refusing_the_pull_request();
    let mut whoever_answers = APersonWhoRules::who_approves();

    let refused = the_run_is_delivered(
        &a_run_standing_at_the_delivery(),
        &the_program,
        &the_clone_the_change_was_made_in(),
        &the_branch_this_run_pushes(),
        &the_request_this_run_makes(),
        Some(&a_credential()),
        &mut whoever_answers,
    );

    assert!(
        matches!(
            refused,
            Err(ADeliveryThatDidNotHappen::ThePullRequestDidNotOpen { .. })
        ),
        "**because a branch on a remote is not a delivery** — it is something nobody can see, and \
         a work item finished on it would be finished on a claim. It refused: {refused:?}"
    );
}

#[test]
fn a_run_standing_nowhere_near_the_delivery_is_refused_rather_than_delivered() {
    let the_program = AProgramAnswering::answering_everything();
    let mut whoever_answers = APersonWhoRules::who_approves();
    let not_at_the_delivery = WhereARunStands::before_anything(
        a_work_item_nobody_holds(Stage::AwaitingAgentPickup),
    );

    let refused = the_run_is_delivered(
        &not_at_the_delivery,
        &the_program,
        &the_clone_the_change_was_made_in(),
        &the_branch_this_run_pushes(),
        &the_request_this_run_makes(),
        Some(&a_credential()),
        &mut whoever_answers,
    );

    assert!(
        matches!(
            refused,
            Err(ADeliveryThatDidNotHappen::NotStandingAtTheDelivery { .. })
        ),
        "**because a work item at a pickup belongs to an agent and not to a delivery** — and \
         pushing a branch for work nobody has verified **is how a repository ends up with branches \
         nobody can account for.** It refused: {refused:?}"
    );
}

/// The branch this run pushes, **and it is the run's own name for it.**
fn the_branch_this_run_pushes() -> ABranchToPush {
    ABranchToPush::a_branch_named("aisdlc/WELLBEINGT-42").expect("a branch may take that name")
}

/// The request this run makes, **and no model reaches the title or the body.**
fn the_request_this_run_makes() -> APullRequestToOpen {
    the_request_a_run_makes_of_its_own_change(
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
        "aisdlc/WELLBEINGT-42",
        "main",
        "the E2E database leaves artifacts behind",
        "A file that was not there, and a test that says so.",
    )
    .expect("a title and a body were both given")
}

/// A run standing at the delivery, **holding the work the delivery agent is about to deliver.**
fn a_run_standing_at_the_delivery() -> WhereARunStands {
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    let the_agent = aisdlc_domain::agent_identifier::AgentIdentifier::try_from("aisdlc-pr")
        .expect("an agent name");
    let the_work_item = aisdlc_domain::work_item::WorkItem::held_by(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        Stage::InProgressByAgent,
        the_agent,
    )
    .expect("a work item may be held at a stage that requires one");
    WhereARunStands::at_the_first_stage(the_work_item, "VerificationReport")
}

/// A work item nobody holds, **and only the stages that allow it will build one.**
fn a_work_item_nobody_holds(
    where_it_stands: Stage,
) -> aisdlc_domain::work_item::WorkItem {
    let (the_id, who_asked, the_repository, the_discipline) = the_four_facts();
    aisdlc_domain::work_item::WorkItem::standing_at(
        the_id,
        who_asked,
        the_repository,
        the_discipline,
        where_it_stands,
    )
    .expect("a work item answering a request can stand where it is put")
}

fn the_four_facts() -> (
    aisdlc_domain::work_item_id::WorkItemId,
    aisdlc_domain::who_commissioned::WhoCommissioned,
    Repository,
    aisdlc_domain::role::Role,
) {
    let the_id = aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-42").expect("a name");
    let who_asked =
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name");
    let the_repository = Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves");
    let the_discipline = aisdlc_domain::role::Role::try_from("DEV").expect("a discipline");
    (the_id, who_asked, the_repository, the_discipline)
}
