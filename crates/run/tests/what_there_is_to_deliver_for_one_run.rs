//! What one filing's run delivers, **derived from what the run works on.**
//!
//! **Cycle 138 names the delivery after the work.** Every filing's run pushes a branch and opens
//! a pull request — **and nothing names either**, so the branch a run pushes and the request it
//! opens are somebody's to invent at the moment they matter most.
//!
//! **The branch is named after the work, the title is the filing's, and the body is the filing's
//! words with the address they were read at**, because a title composed by a model is prose
//! nobody checked on the one page every reviewer reads first.

use aisdlc_run::what_there_is_to_deliver_for_one_run::{
    NothingToDeliverForOneRun, what_there_is_to_deliver_for_one_run,
};

#[test]
fn one_filings_run_delivers_on_a_branch_named_after_its_work() {
    let what_the_run_works_on = the_work_a_filing_became();
    let the_credential = the_key_a_deployment_supplied();

    let what_there_is =
        what_there_is_to_deliver_for_one_run(&what_the_run_works_on, &the_credential)
            .expect("a filing has a title, words and an address");

    assert_eq!(
        what_there_is.the_branch().which_branch(),
        "aisdlc/steamnoid/wellbeing-tracker-public#11",
        "**because two runs over one filing push to one branch** — and a branch named after
         nothing in particular is a branch no run can find again."
    );
    assert_eq!(
        what_there_is.the_request().the_title(),
        "Stop E2E runs leaving WAL state",
        "**because the title is the filing's** — and a title composed by a model is prose nobody
         checked on the one page every reviewer reads first."
    );
    assert_eq!(
        what_there_is.the_request().the_body(),
        "the ask\n\nFiled as https://github.com/steamnoid/wellbeing-tracker-public/issues/11",
        "**because the body is the filing's words with the address they were read at** — and a
         body nobody can check is a change nobody can review."
    );
}

#[test]
fn a_run_answering_nothing_has_nothing_to_deliver() {
    let what_the_run_works_on = the_work_answering_nothing();
    let the_credential = the_key_a_deployment_supplied();

    let refused = what_there_is_to_deliver_for_one_run(&what_the_run_works_on, &the_credential);

    assert!(
        matches!(refused, Err(NothingToDeliverForOneRun::NoRequestToAnswer)),
        "**because a run that does not know what to do has nothing to push and nothing to open** \
         — and a delivery about nothing is about nobody's request."
    );
}

#[test]
fn a_title_nobody_can_find_is_refused_rather_than_invented() {
    let what_the_run_works_on = the_work_answering_without_a_title();
    let the_credential = the_key_a_deployment_supplied();

    let refused = what_there_is_to_deliver_for_one_run(&what_the_run_works_on, &the_credential);

    assert!(
        matches!(
            refused,
            Err(NothingToDeliverForOneRun::RequestIsNotAPullRequest { .. })
        ),
        "**because a title nobody can find in a list of fifty is a title nobody can review** — \
         and an invented title is prose nobody checked."
    );
}

#[test]
fn a_branch_git_cannot_take_is_refused_where_it_is_named() {
    let what_the_run_works_on = the_work_named_with_a_space();
    let the_credential = the_key_a_deployment_supplied();

    let refused = what_there_is_to_deliver_for_one_run(&what_the_run_works_on, &the_credential);

    assert!(
        matches!(
            refused,
            Err(NothingToDeliverForOneRun::BranchCannotBeNamed { .. })
        ),
        "**because a branch git cannot take is a push that never lands** — and the refusal names
         the branch rather than git's complaint afterwards."
    );
}

/// The key a deployment supplied.
fn the_key_a_deployment_supplied() -> aisdlc_run::a_credential::ACredential {
    aisdlc_run::a_credential::ACredential::the_key_a_deployment_supplied("a-key-not-a-real-one")
        .expect("the test key is a key")
}

/// The work a filing became, **read back and not remembered.**
fn the_work_a_filing_became() -> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    let where_it_was_filed =
        String::from("https://github.com/steamnoid/wellbeing-tracker-public/issues/11");
    let what_was_read =
        aisdlc_run::an_issue_read::AnIssueRead::the_issue_read(&what_github_answered())
            .expect("GitHub answered with a filing");
    let mut works = aisdlc_run::the_work_each_filing_becomes::the_work_each_filing_becomes(
        &the_repository(),
        &[what_was_read],
        &[where_it_was_filed],
    )
    .expect("a filing this system made becomes a work");
    works.pop().expect("one filing is one work")
}

/// The work answering nothing, **with a name and a discipline and nothing to do.**
fn the_work_answering_nothing() -> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
        the_work_item: the_work_standing_at("WELLBEINGT-42"),
        the_work_request: None,
    }
}

/// The work answering without a title, **with words nobody can find in a list.**
fn the_work_answering_without_a_title()
-> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    let requested = aisdlc_domain::commissioned_request::CommissionedRequest::asked_for(
        String::from(""),
        String::from("some words"),
    )
    .expect("words are something");
    aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
        the_work_item: the_work_standing_at("WELLBEINGT-42"),
        the_work_request: Some(requested),
    }
}

/// The work named with a space, **which no branch can take.**
fn the_work_named_with_a_space() -> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    let requested = aisdlc_domain::commissioned_request::CommissionedRequest::asked_for(
        String::from("a title"),
        String::from("some words"),
    )
    .expect("a title and words are something");
    aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
        the_work_item: the_work_standing_at("has a space"),
        the_work_request: Some(requested),
    }
}

/// The work standing at a name, **where work begins and with nobody holding it.**
fn the_work_standing_at(a_name: &str) -> aisdlc_domain::work_item::WorkItem {
    aisdlc_domain::work_item::WorkItem::standing_at(
        aisdlc_domain::work_item_id::WorkItemId::try_from(a_name).expect("a name"),
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("somebody"),
        the_repository(),
        aisdlc_domain::role::Role::try_from("DEV").expect("a discipline"),
        aisdlc_domain::stage::Stage::Idle,
    )
    .expect("a work item at a stage")
}

/// The repository filings are filed in.
fn the_repository() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// What GitHub answered, **with a title to deliver.**
fn what_github_answered() -> String {
    String::from(
        "{\"number\":11,\"title\":\"Stop E2E runs leaving WAL state\",\"body\":\"the ask\",\
         \"html_url\":\"https://github.com/steamnoid/wellbeing-tracker-public/issues/11\",\
         \"state\":\"open\"}",
    )
}
