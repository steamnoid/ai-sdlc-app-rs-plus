//! A run goes from a request to a pull request, and the work item is finished.
//!
//! **A whole run stops at the delivery and a delivery finishes a run, and nothing joined the two.**
//! `the_whole_of_a_run` ends standing at the delivery with `aisdlc-pr` holding the work — **because a
//! delivery pushes a branch and opens a pull request, neither of which is a stage a person rules on.**
//! `the_run_is_delivered` takes a run already standing there. **So this repository could walk seven
//! gates and then stop, which is a run that did all the work and delivered none of it.**
#[path = "doubles/a_model_that_answers.rs"]
mod a_model_that_answers;
#[path = "doubles/a_person_who_rules.rs"]
mod a_person_who_rules;
#[path = "doubles/a_program_answering.rs"]
mod a_program_answering;
#[path = "doubles/a_project_the_run_writes_to.rs"]
mod a_project_the_run_writes_to;

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;

use a_model_that_answers::AModelAnswering;
use a_person_who_rules::APersonWhoRules;
use a_program_answering::AProgramAnswering;
use a_project_the_run_writes_to::AProjectThatWrites;
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
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::where_a_run_stands::WhereARunStands;

#[test]
fn a_run_from_a_request_to_a_pull_request_leaves_the_work_item_finished() {
    let mut a_model = a_model();
    let finished = a_run_that_every_gate_approves(
        &mut a_model,
        &a_program_answering(false),
        "the-work-item-is-finished",
    );
    let where_it_stands = the_stage_the_stopped_run_stands_at(&finished);

    assert_eq!(
        where_it_stands,
        Some(Stage::Done),
        "**because seven gates and a pull request and a work item still standing at a stage is a run \
         that did all the work and delivered none of it** — and `DONE` is the stage the table has \
         exactly one edge into. It stands at {where_it_stands:?}"
    );
}

#[test]
fn one_person_is_asked_eight_times_and_the_eighth_gate_is_the_delivery() {
    let mut a_model = a_model();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    a_run_given(
        &a_run_at_its_first_stage(),
        "the-delivery-is-the-eighth-gate",
        &mut a_model,
        &mut whoever_answers,
        &a_program_answering(false),
    )
    .expect("a run every gate approves finishes");

    assert_eq!(
        whoever_answers.how_often(),
        8,
        "**because seven artifacts and seven gates and the delivery's own gate is the eighth** — \
         and seven would be a delivery that finished a run without asking, which would put a \
         decision in the record that nobody made. They were asked {} times",
        whoever_answers.how_often()
    );
    each_of_the_seven_gates_was_asked_about_a_different_artifact(&whoever_answers);
}

/// A finished run remembers the delivery's ruling as well, **because the delivery's gate is a gate
/// and not a footnote** — and a run that remembered seven of its eight rulings would leave a summary
/// with nothing at all to say about the decision that let the branch go.
#[test]
fn a_finished_run_remembers_the_deliveries_ruling_as_its_eighth() {
    let mut a_model = a_model();
    let mut whoever_answers = APersonWhoRules::approving_everything();

    let finished = a_run_given(
        &a_run_at_its_first_stage(),
        "the-delivery-ruling-is-eighth",
        &mut a_model,
        &mut whoever_answers,
        &a_program_answering(false),
    );

    assert_eq!(
        finished.as_ref().map(|left| left
            .where_the_work_stands()
            .every_ruling_this_run_received()
            .len()),
        Ok(8),
        "**because the delivery's gate is a gate and not a footnote** — and a run that remembered \
         seven of its eight rulings would leave a summary with nothing to say about the decision \
         that let the branch go. It remembered {:?}",
        finished.as_ref().map(|left| left
            .where_the_work_stands()
            .every_ruling_this_run_received())
    );
}

/// The seven gates named the seven artifacts and no gate named one twice, **and a gate asked twice
/// about one artifact is a gate that approved a document nobody changed** — which is what the
/// version on an approval exists to prevent.
fn each_of_the_seven_gates_was_asked_about_a_different_artifact(whoever_answers: &APersonWhoRules) {
    let what_they_were_asked_about = whoever_answers.what_each_gate_was_asked_about();
    let what_the_seven_were_asked_about = what_they_were_asked_about
        .iter()
        .map(|(artifact, _version)| artifact.clone())
        .take(7)
        .collect::<Vec<String>>();
    assert_eq!(
        what_the_seven_were_asked_about,
        every_artifact_the_pipeline_produces(),
        "**because seven artifacts and seven gates is the whole of the pipeline** — and the eighth \
         question is the delivery's, which is not an eighth artifact. They were asked about \
         {what_they_were_asked_about:?}"
    );
}

#[test]
fn what_a_run_left_names_the_pull_request_github_opened() {
    let mut a_model = a_model();

    let finished = a_run_that_every_gate_approves(
        &mut a_model,
        &a_program_answering(false),
        "what-a-run-left",
    );

    let where_it_answers = finished.as_ref().map(WhatARunLeft::which_pull_request);
    let where_it_answers = where_it_answers.map(|opened| opened.where_it_answers.clone());
    assert_eq!(
        where_it_answers,
        Ok(String::from(
            "https://github.com/steamnoid/wellbeing-tracker-public/pull/17"
        )),
        "**because a delivery that recorded an address it composed would record a link to a page \
         that does not exist** — and the run's whole record is that link. It answers at \
         {where_it_answers:?}"
    );
}

#[test]
fn a_run_somebody_refused_in_the_middle_delivers_nothing_and_is_not_finished() {
    let mut a_model = a_model();
    let mut whoever_answers = APersonWhoRules::refusing_at_the(3);

    let stopped = a_run_given(
        &a_run_at_its_first_stage(),
        "a-refusal-in-the-middle",
        &mut a_model,
        &mut whoever_answers,
        &a_program_answering(false),
    );

    let where_it_stands = the_stage_the_stopped_run_stands_at(&stopped);

    assert_eq!(
        where_it_stands,
        Some(Stage::Done),
        "**because refusing ends the run and the run is at `DONE` rather than half-delivered** — \
         there is no pull request and the work item is closed. It stands at {where_it_stands:?}"
    );
    assert_eq!(
        whoever_answers.how_often(),
        3,
        "**because nobody was asked about a delivery that never happened** — a gate asked after a \
         refusal is a gate asked to rule on work a person has already stopped. They were asked {} \
         times",
        whoever_answers.how_often()
    );
}

#[test]
fn a_delivery_github_refused_stops_the_run_and_leaves_the_work_item_at_the_gate() {
    let mut a_model = a_model();

    let stopped = a_run_that_every_gate_approves(
        &mut a_model,
        &a_program_answering(true),
        "a-refused-delivery",
    );

    assert!(
        matches!(stopped, Err(ARunThatDidNotFinish::TheDelivery { .. })),
        "**because a branch on a remote is not a delivery** — it is something nobody can see, and a \
         work item finished on it would be finished on a claim. It refused: {stopped:?}"
    );
}

/// The stage a stopped or finished run stands at, **and it is one function so four tests do not each
/// spell out the two hops through the run.**
fn the_stage_the_stopped_run_stands_at(stopped: &WhatARunLeftOrNot) -> Option<Stage> {
    let the_run = stopped.as_ref().ok()?;
    Some(the_run.where_the_work_stands().the_stage_it_stands_at())
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

/// A model that answers, **and it is a function because four tests build the same double and a
/// struct literal in each of them is four places to change when it grows a field.**
fn a_model() -> AModelAnswering {
    AModelAnswering::answering_everything()
}

/// What a finished or stopped run is, **and the alias is here because five tests say the same long
/// type and a reader should not have to check it five times.**
type WhatARunLeftOrNot = Result<WhatARunLeft, ARunThatDidNotFinish>;

/// A run from a request to a pull request, **and it is one call so the four tests are about their own
/// arrangement and not about the order of seven arguments.**
fn a_run_given(
    the_run: &WhereARunStands,
    in_a_directory_named: &str,
    a_model: &mut AModelAnswering,
    whoever_answers_the_gates: &mut APersonWhoRules,
    the_program: &AProgramAnswering,
) -> WhatARunLeftOrNot {
    let mut the_project =
        AProjectThatWrites::writing_everything(in_a_directory_named, "WELLBEINGT-42");
    a_run_from_a_request_to_a_pull_request(
        the_run,
        &mut the_project,
        a_model,
        WhichOrganisationReasons::Openai,
        whoever_answers_the_gates,
        &what_there_is_to_deliver(),
        the_program,
    )
}

fn a_run_that_every_gate_approves(
    a_model: &mut AModelAnswering,
    the_program: &AProgramAnswering,
    in_a_directory_named: &str,
) -> WhatARunLeftOrNot {
    a_run_given(
        &a_run_at_its_first_stage(),
        in_a_directory_named,
        a_model,
        &mut APersonWhoRules::approving_everything(),
        the_program,
    )
}

/// A program answering, **and it is an argument so one test can refuse the pull request.**
fn a_program_answering(refuses_the_pull_request: bool) -> AProgramAnswering {
    AProgramAnswering::refusing_the_pull_request(refuses_the_pull_request)
}

/// What there is to deliver, **and it is one value rather than four arguments** — the four are one
/// fact about one delivery and a reader would otherwise have to hold them in step.
fn what_there_is_to_deliver() -> WhatThereIsToDeliver {
    let the_repository =
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves");
    let the_clone = the_clone_the_change_was_made_in(&the_repository);
    let the_branch =
        ABranchToPush::a_branch_named("aisdlc/WELLBEINGT-42").expect("a branch may take that name");
    let what_to_open = the_request_this_run_makes(the_repository);
    let the_credential =
        ACredential::the_key_a_deployment_supplied("a-key-not-a-real-one").expect("a key");
    WhatThereIsToDeliver::for_this_run(the_clone, the_branch, what_to_open, the_credential)
}

/// The clone the change was made in, **and it is a path nobody reads because nothing here runs
/// `git` — a double that cloned would be a test of the machine rather than of the delivery.**
fn the_clone_the_change_was_made_in(where_it_is_offered_to: &Repository) -> ARepositoryToClone {
    ARepositoryToClone::of(
        where_it_is_offered_to.clone(),
        WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        std::path::PathBuf::from("/tmp/a-clone-this-test-never-reads"),
    )
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

/// A run standing at the stage its first agent is working on, **and on the artifact it produces.**
fn a_run_at_its_first_stage() -> WhereARunStands {
    let the_work_item = a_work_item_answering_the_request();
    WhereARunStands::at_the_first_stage(the_work_item, "DiscoveryReport")
}

/// The work item the discovery agent is holding, **answering a request** — and the two are built
/// apart because a fixture holding both is a fixture a reader has to read all the way through before
/// reaching the one line it is for.
fn a_work_item_answering_the_request() -> WorkItem {
    let the_request = CommissionedRequest {
        title: String::from("untrack the sqlite sidecar files"),
        body: String::from("e2e.db-wal and e2e.db-shm are 4.1 MB and are committed"),
        filed_as: None,
    };

    let held_by_the_discovering_agent = a_work_item_held_by_the_discovering_agent();
    let answering = held_by_the_discovering_agent.answering(the_request);
    answering.expect("a work item answering a request may stand where it was put")
}

/// The work item the discovery agent is holding, **and it is in progress on its first artifact.**
fn a_work_item_held_by_the_discovering_agent() -> WorkItem {
    let the_id = WorkItemId::try_from("WELLBEINGT-42").expect("a name");
    let who_asked = WhoCommissioned::try_from("a person").expect("a name");
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
    held.expect("a work item may be held at a stage that requires one")
}
