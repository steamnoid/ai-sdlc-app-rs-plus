//! A run goes from a request to a pull request, and the work item is finished.
//!
//! **A whole run stops at the delivery and a delivery finishes a run, and nothing joined the two.**
//! `the_whole_of_a_run` ends standing at the delivery with `aisdlc-pr` holding the work — **because a
//! delivery pushes a branch and opens a pull request, neither of which is a stage a person rules on.**
//! `the_run_is_delivered` takes a run already standing there. **So this repository could walk seven
//! gates and then stop, which is a run that did all the work and delivered none of it.**

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;

mod the_doubles_a_run_is_run_with;
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
use the_doubles_a_run_is_run_with::{AModelAnswering, APersonWhoRules, AProgramAnswering};

#[test]
fn a_run_from_a_request_to_a_pull_request_leaves_the_work_item_finished() {
    let mut a_model = a_model();
    let finished = a_run_that_every_gate_approves(&mut a_model, &a_program_answering(false));
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
fn the_delivery_is_the_eighth_gate_and_not_an_artifact_gate() {
    let (asked_during_the_run, asked_at_the_delivery) = how_many_each_person_was_asked();

    assert_eq!(asked_during_the_run, 7, "{SEVEN_GATES}");
    assert_eq!(asked_at_the_delivery, 1, "{THE_DELIVERY_IS_A_GATE}");
}

/// A whole run every gate approves, **and it hands back how many questions each person was asked** —
/// which is the whole of what this test is about, **so the run itself does not need a body here.**
fn how_many_each_person_was_asked() -> (usize, usize) {
    let mut a_model = a_model();
    let mut during_the_run = APersonWhoRules::approving_everything();
    let mut at_the_delivery = APersonWhoRules::approving_everything();
    a_run_given(
        &a_run_at_its_first_stage(),
        &mut a_model,
        &mut during_the_run,
        &a_program_answering(false),
        &mut at_the_delivery,
    )
    .expect("a run every gate approves finishes");
    (during_the_run.how_often(), at_the_delivery.how_often())
}

/// The sentences the two assertions read, **and they are constants because a four-line message inside
/// a test is four lines of the test that are not the test.**
const SEVEN_GATES: &str = "**because seven artifacts and seven gates is the whole of the pipeline** \
    — and an eighth question during the run would be a gate nobody can name";
const THE_DELIVERY_IS_A_GATE: &str = "**because the delivery is the eighth gate and it is a gate** \
    — a delivery that finished a run without asking **would put a decision in the record that nobody \
    made.**";

#[test]
fn what_a_run_left_names_the_pull_request_github_opened() {
    let mut a_model = a_model();

    let finished = a_run_that_every_gate_approves(&mut a_model, &a_program_answering(false));

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
    let mut at_the_delivery = APersonWhoRules::approving_everything();

    let stopped = a_run_given(
        &a_run_at_its_first_stage(),
        &mut a_model,
        &mut APersonWhoRules::refusing_at_the(3),
        &a_program_answering(false),
        &mut at_the_delivery,
    );

    let where_it_stands = the_stage_the_stopped_run_stands_at(&stopped);

    assert_eq!(
        where_it_stands,
        Some(Stage::Done),
        "**because refusing ends the run and the run is at `DONE` rather than half-delivered** — \
         there is no pull request and the work item is closed. It stands at {where_it_stands:?}"
    );
    assert_eq!(
        at_the_delivery.how_often(),
        0,
        "**because nobody was asked about a delivery that never happened** — a gate asked after a \
         refusal is a gate asked to rule on work a person has already stopped. They were asked {} \
         times",
        at_the_delivery.how_often()
    );
}

#[test]
fn a_delivery_github_refused_stops_the_run_and_leaves_the_work_item_at_the_gate() {
    let mut a_model = a_model();

    let stopped = a_run_that_every_gate_approves(&mut a_model, &a_program_answering(true));

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
    a_model: &mut AModelAnswering,
    during_the_run: &mut APersonWhoRules,
    the_program: &AProgramAnswering,
    at_the_delivery: &mut APersonWhoRules,
) -> WhatARunLeftOrNot {
    a_run_from_a_request_to_a_pull_request(
        the_run,
        a_model,
        WhichOrganisationReasons::Openai,
        during_the_run,
        &what_there_is_to_deliver(),
        the_program,
        at_the_delivery,
    )
}

/// The ordinary run, **and it approves every gate and the delivery too.**
fn a_run_that_every_gate_approves(
    a_model: &mut AModelAnswering,
    the_program: &AProgramAnswering,
) -> WhatARunLeftOrNot {
    a_run_given(
        &a_run_at_its_first_stage(),
        a_model,
        &mut APersonWhoRules::approving_everything(),
        the_program,
        &mut APersonWhoRules::approving_everything(),
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
