//! A run makes its own checkout before anybody is asked about anything.
//!
//! **Every cycle so far worked on artifacts and rulings, and not one of them had a repository on
//! disk.** `the_run_makes_a_checkout` exists, it is tested, and **nothing in the pipeline calls it**
//! — so a whole run asked seven models about a repository it had never got, and the delivery pushed
//! from a path a *caller* named rather than the copy the run made.
//!
//! **One clone, and it is the one the run made.** `WhatThereIsToDeliver` carries an
//! `ARepositoryToClone` — **a destination the caller wrote down** — and that is a second answer to
//! the question the run already answers. **A delivery that pushes a clone the run never read is a
//! delivery of something else.**
//!
//! **The clone comes first, and the order is the point.** A run that asked a model about a
//! repository before copying it would be a run reasoning about a description, **which is the whole
//! defect this system was built to avoid.**

#[path = "doubles/a_program_a_project_a_directory_and_a_person.rs"]
mod a_program_a_project_a_directory_and_a_person;
use a_program_a_project_a_directory_and_a_person::{
    AModelRecordingWhatHappened, APersonWhoApprovesEverything, AProgramAnswering,
    AProjectThatWrites, WhatHappenedInOrder,
};

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_run::a_branch_goes_to_a_remote::ABranchToPush;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::a_pull_request_is_opened::{
    APullRequestToOpen, the_request_a_run_makes_of_its_own_change,
};
use aisdlc_run::a_run_from_a_request_to_a_pull_request::{
    ARunThatDidNotFinish, WhatARunLeft, WhatThereIsToDeliver,
    a_run_from_a_request_to_a_pull_request,
};
use aisdlc_run::where_a_run_stands::WhereARunStands;

#[test]
fn the_repository_is_copied_before_any_agent_is_asked_anything() {
    let what_happened_in_order = WhatHappenedInOrder::shared();
    let mut what_the_program_was_asked =
        AProgramAnswering::answering_everything(what_happened_in_order.clone());
    let mut a_model = AModelRecordingWhatHappened::answering_everything();
    a_model.it_writes_to(what_happened_in_order.clone());
    let mut whoever_answers = APersonWhoApprovesEverything::new();

    let _finished = a_run_given(
        &mut what_the_program_was_asked,
        &mut a_model,
        &mut whoever_answers,
    );

    assert_eq!(
        what_happened_in_order
            .what_happened()
            .first()
            .map(String::as_str),
        Some("the repository was copied"),
        "**because a run that asked a model about a repository before copying it was reasoning \
         about a description** — which is the whole defect this system was built to avoid, and the \
         agents that follow a copy are named after the artifacts they were asked for. It did \
         {:?}",
        what_happened_in_order.what_happened()
    );
}

#[test]
fn a_run_whose_checkout_failed_asks_nobody_and_delivers_nothing() {
    let mut what_the_program_was_asked =
        AProgramAnswering::refusing_the_clone("could not read repository");
    let mut a_model = AModelRecordingWhatHappened::answering_everything();
    let mut whoever_answers = APersonWhoApprovesEverything::new();

    let refused = a_run_given(
        &mut what_the_program_was_asked,
        &mut a_model,
        &mut whoever_answers,
    );

    assert!(
        matches!(refused, Err(ARunThatDidNotFinish::TheCheckout { .. })),
        "**because seven gates ruled on nothing when the repository was never on disk** — and a \
         gate that approved a report about a repository nobody had is a gate that approved a \
         claim. It refused: {refused:?}"
    );
    assert_eq!(
        a_model.every_artifact_it_was_asked_for().len(),
        0,
        "**because an agent asked about a repository the run does not have produces a document \
         nobody can check** — and a person approving that document approved a claim"
    );
}

#[test]
fn every_question_the_run_asked_about_a_clone_named_the_one_it_made() {
    let what_happened_in_order = WhatHappenedInOrder::shared();
    let mut what_the_program_was_asked =
        AProgramAnswering::answering_everything(what_happened_in_order.clone());
    let mut a_model = AModelRecordingWhatHappened::answering_everything();
    let mut whoever_answers = APersonWhoApprovesEverything::new();

    let _finished = a_run_given(
        &mut what_the_program_was_asked,
        &mut a_model,
        &mut whoever_answers,
    );

    let every_clone_it_named = what_the_program_was_asked.every_clone_it_was_asked_about();
    assert!(
        !every_clone_it_named.is_empty()
            && every_clone_it_named
                .iter()
                .all(|where_it_lands| where_it_lands.ends_with("WELLBEINGT-42/repository")),
        "**because a delivery that pushes a clone the run never read is a delivery of something \
         else** — and `WhatThereIsToDeliver` carried a destination a caller wrote down, which was a \
         second answer to a question the run answers itself. It asked about {every_clone_it_named:?}"
    );
}

/// A whole run given a program, a model and a person, **and it is one call so three tests are about
/// their own arrangement and not about the order of seven collaborators.**
fn a_run_given(
    what_the_program_was_asked: &mut AProgramAnswering,
    a_model: &mut AModelRecordingWhatHappened,
    whoever_answers: &mut APersonWhoApprovesEverything,
) -> Result<WhatARunLeft, ARunThatDidNotFinish> {
    let mut the_project =
        AProjectThatWrites::writing_everything("the-run-its-checkout", "WELLBEINGT-42");
    a_run_from_a_request_to_a_pull_request(
        &a_run_at_its_first_stage(),
        &mut the_project,
        a_model,
        WhichOrganisationReasons::Openai,
        whoever_answers,
        &what_there_is_to_deliver(),
        what_the_program_was_asked,
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

/// What there is to deliver, **and the clone is no longer among the four: the run derived it from its
/// own work item and the project it writes into.**
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
