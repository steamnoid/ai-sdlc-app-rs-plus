//! What a filing is ready to deliver.
//!
//! **Cycle 139 prepares the delivery without running it.** A filing becomes a work, the work is
//! taken up answering, and what it delivers is derived beside it — **and nothing joins the
//! three**, so the works a creative run filed are never taken up answering with their delivery
//! beside them.
//!
//! **Seven collaborators do not fit one signature a reader can hold**, so preparing a delivery
//! and running it are two functions: this one takes the work and the key, and the run takes the
//! rest.

use aisdlc_run::what_a_filing_is_ready_to_deliver::{
    AFilingThatDidNotDeliver, WhatAFilingIsReadyToDeliver, what_a_filing_is_ready_to_deliver,
};

#[test]
fn what_a_filing_became_is_taken_up_answering() {
    let ready = the_filing_ready_to_deliver("Stop E2E runs leaving WAL state");

    assert_eq!(
        ready.the_run().the_stage_it_stands_at(),
        aisdlc_domain::stage::Stage::InProgressByAgent,
        "**because the walk asks agents and an agent that holds nothing answers nothing**"
    );
    assert_eq!(
        ready
            .the_run()
            .the_work_item_it_is()
            .the_request_it_answers()
            .as_ref()
            .expect("a ready run answers its request")
            .what_was_asked(),
        "Stop E2E runs leaving WAL state",
        "**because the run works on the filing's words** — and a run answering another request
         delivers another change."
    );
}

#[test]
fn what_a_filing_became_delivers_on_a_branch_named_after_its_work() {
    let ready = the_filing_ready_to_deliver("Stop E2E runs leaving WAL state");

    assert_eq!(
        ready.what_there_is_to_deliver().the_branch().which_branch(),
        "aisdlc/steamnoid/wellbeing-tracker-public#11",
        "**because two runs over one filing push to one branch** — and a branch named after
         nothing in particular is a branch no run can find again."
    );
    assert_eq!(
        ready.what_there_is_to_deliver().the_request().the_title(),
        "Stop E2E runs leaving WAL state",
        "**because the title is the filing's** — and a title composed by a model is prose nobody
         checked on the one page every reviewer reads first."
    );
    assert_eq!(
        ready.what_there_is_to_deliver().the_request().the_body(),
        "the ask\n\nFiled as https://github.com/steamnoid/wellbeing-tracker-public/issues/11",
        "**because the body is the filing's words with the address they were read at** — and a
         body nobody can check is a change nobody can review."
    );
}

#[test]
fn a_filing_with_no_title_has_nothing_to_deliver() {
    let refused = the_filing_becoming_ready("");

    assert!(
        matches!(
            refused,
            Err(AFilingThatDidNotDeliver::ThereWasNothingToDeliver { .. })
        ),
        "**because a title nobody can find in a list of fifty is a title nobody can review** — \
         and the work was never taken up on words nobody can name."
    );
}

#[test]
fn work_picked_up_already_is_never_taken_up_twice() {
    let picked_up = the_work_picked_up_once();
    let ready = what_a_filing_is_ready_to_deliver(&picked_up, &the_key_a_deployment_supplied());

    assert!(
        matches!(
            ready,
            Err(AFilingThatDidNotDeliver::TheWorkWasNeverTakenUp { .. })
        ),
        "**because a work item awaiting pickup is already somebody's** — and taking it up again
         would hand one piece of work to two agents."
    );
}

#[test]
fn a_run_answering_nothing_has_nothing_to_deliver() {
    let answering_nothing = the_work_answering_nothing();
    let ready =
        what_a_filing_is_ready_to_deliver(&answering_nothing, &the_key_a_deployment_supplied());

    assert!(
        matches!(
            ready,
            Err(AFilingThatDidNotDeliver::ThereWasNothingToDeliver { .. })
        ),
        "**because a run that does not know what to do has nothing to push and nothing to open** \
         — and a delivery about nothing is about nobody's request."
    );
}

/// The filing ready to deliver, **titled as told.**
fn the_filing_ready_to_deliver(with_title: &str) -> WhatAFilingIsReadyToDeliver {
    the_filing_becoming_ready(with_title).expect("a filing has a title, words and an address")
}

/// The filing becoming ready, **or the refusal naming what it cannot name.**
fn the_filing_becoming_ready(
    with_title: &str,
) -> Result<WhatAFilingIsReadyToDeliver, AFilingThatDidNotDeliver> {
    what_a_filing_is_ready_to_deliver(
        &the_work_a_filing_became(with_title),
        &the_key_a_deployment_supplied(),
    )
}

/// The key a deployment supplied.
fn the_key_a_deployment_supplied() -> aisdlc_run::a_credential::ACredential {
    aisdlc_run::a_credential::ACredential::the_key_a_deployment_supplied("a-key-not-a-real-one")
        .expect("the test key is a key")
}

/// The work a filing became, **read back and not remembered.**
fn the_work_a_filing_became(
    with_title: &str,
) -> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    let where_it_was_filed =
        String::from("https://github.com/steamnoid/wellbeing-tracker-public/issues/11");
    let what_was_read =
        aisdlc_run::an_issue_read::AnIssueRead::the_issue_read(&what_github_answered(with_title))
            .expect("GitHub answered with a filing");
    let mut works = aisdlc_run::the_work_each_filing_becomes::the_work_each_filing_becomes(
        &aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
        &[what_was_read],
        &[where_it_was_filed],
    )
    .expect("a filing this system made becomes a work");
    works.pop().expect("one filing is one work")
}

/// The work picked up once, **awaiting an agent that already holds it.**
fn the_work_picked_up_once() -> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    let requested = aisdlc_domain::commissioned_request::CommissionedRequest::asked_for(
        String::from("a title"),
        String::from("some words"),
    )
    .expect("a title and words are something");
    let picked_up = aisdlc_run::a_run_picks_up_the_work::a_run_picks_up_the_work_it_was_given(
        &the_work_standing_at("WELLBEINGT-42"),
    )
    .expect("a work item standing where work begins");
    aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
        the_work_item: picked_up,
        the_work_request: Some(requested),
    }
}

/// The work answering nothing, **with a name and a discipline and nothing to do.**
fn the_work_answering_nothing() -> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
        the_work_item: the_work_standing_at("WELLBEINGT-42"),
        the_work_request: None,
    }
}

/// The work standing at a name, **where work begins and with nobody holding it.**
fn the_work_standing_at(a_name: &str) -> aisdlc_domain::work_item::WorkItem {
    aisdlc_domain::work_item::WorkItem::standing_at(
        aisdlc_domain::work_item_id::WorkItemId::try_from(a_name).expect("a name"),
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("somebody"),
        aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
        aisdlc_domain::role::Role::try_from("DEV").expect("a discipline"),
        aisdlc_domain::stage::Stage::Idle,
    )
    .expect("a work item at a stage")
}

/// What GitHub answered, **titled as told.**
fn what_github_answered(with_title: &str) -> String {
    format!(
        "{{\"number\":11,\"title\":\"{with_title}\",\"body\":\"the ask\",\
         \"html_url\":\"https://github.com/steamnoid/wellbeing-tracker-public/issues/11\",\
         \"state\":\"open\"}}"
    )
}
