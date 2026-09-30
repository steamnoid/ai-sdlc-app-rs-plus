//! Each filing a creative run made becoming a work the product analyst answers.
//!
//! **One filing is one work, and not one work with a list in it.** A work item answers one
//! `CommissionedRequest` — **and each issue the run filed is its own request, with its own words, its
//! own evidence and its own address.** Two issues in one work item would be two requests and one
//! answer, and a gate asking about "the requirements" of both could not say which it meant.
//!
//! **The work is named after where it was filed, because that is the only name it has.** A filing a
//! model made has no ticket, and `OFFER-1` would collide with the next run's `OFFER-1` while meaning
//! nothing to the person who has to judge it.
//!
//! **The work answers the filing's own words, and the filing is read rather than remembered.** A run
//! that composed the request from what it had just written would put text in the work item that
//! GitHub never had — **and the two would part company the moment a person edited one word.**

use aisdlc_run::an_issue_read::AnIssueRead;
use aisdlc_run::the_work_each_filing_becomes::the_work_each_filing_becomes;

/// The repository offers are filed in.
fn the_repository_offered_in() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// What GitHub answered for one filing, at one address.
fn what_github_answered_at(its_address: &str) -> String {
    format!(
        "{{\"number\":10,\"title\":\"Stop E2E runs leaving WAL state\",\"body\":\"the ask\",\
         \"html_url\":\"{its_address}\",\"state\":\"open\"}}"
    )
}

/// The addresses two filings went to, **and the two filings read back at them.**
fn two_filings_read_back_from_where_they_went() -> (Vec<String>, Vec<AnIssueRead>) {
    let where_they_went = vec![
        String::from("https://github.com/steamnoid/wellbeing-tracker-public/issues/10"),
        String::from("https://github.com/steamnoid/wellbeing-tracker-public/issues/11"),
    ];
    let what_was_read = where_they_went
        .iter()
        .map(|one_address| {
            AnIssueRead::the_issue_read(&what_github_answered_at(one_address))
                .expect("a filing GitHub composed is a filing")
        })
        .collect();
    (where_they_went, what_was_read)
}

/// What the run will work on, from two filings read back.
fn the_run_will_work_on_two_filings()
-> Vec<aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn> {
    let (where_they_went, what_was_read) = two_filings_read_back_from_where_they_went();
    the_work_each_filing_becomes(
        &the_repository_offered_in(),
        &what_was_read,
        &where_they_went,
    )
    .expect("two filings became two works the product analyst answers")
}

#[test]
fn a_filing_a_model_made_becomes_a_work_the_product_analyst_answers() {
    let what_the_run_will_work_on = the_run_will_work_on_two_filings();

    assert_eq!(
        what_the_run_will_work_on.len(),
        2,
        "**because one filing is one request and a work item answers exactly one** — two issues in \
         one work item would be two requests and one answer, and a gate asking about *the \
         requirements* of both could not say which it meant: {what_the_run_will_work_on:?}"
    );
}

#[test]
fn every_work_is_the_product_analysts_and_says_a_model_asked_for_it() {
    let what_the_run_will_work_on = the_run_will_work_on_two_filings();
    assert_eq!(
        what_the_run_will_work_on.len(),
        2,
        "**because a loop over nothing asserts nothing** — and this test asserts about every work \
         and would have passed on an empty list, which is the assertion that cannot fail: \
         {what_the_run_will_work_on:?}"
    );

    for one_work in &what_the_run_will_work_on {
        assert_eq!(
            (one_work.who_asked(), one_work.role_answers()),
            ("llm agent".to_string(), String::from("PO")),
            "**because a filing this system wrote says a model wrote it, and it is the product \
             analyst who answers it** — an offer is a proposal about what to work on, and that is the \
             analyst's question and not the planner's: {one_work:?}"
        );
    }
}

#[test]
fn a_work_is_named_after_where_it_was_filed_and_never_shares_its_name() {
    let what_the_run_will_work_on = the_run_will_work_on_two_filings();

    let the_names: Vec<String> = what_the_run_will_work_on
        .iter()
        .map(|one_work| one_work.the_work_items_name().to_string())
        .collect();

    assert_eq!(
        (
            the_names[0].as_str(),
            the_names[1].as_str(),
            the_names[0] == the_names[1]
        ),
        (
            "steamnoid/wellbeing-tracker-public#10",
            "steamnoid/wellbeing-tracker-public#11",
            false
        ),
        "**because the address is the only name a self-filed issue has** — and `OFFER-1` collides \
         with the next run's `OFFER-1` while telling the person judging it nothing. Two works with \
         one name is one work found twice: {the_names:?}"
    );
}

#[test]
fn a_work_answers_the_words_the_filing_carries() {
    let what_the_run_will_work_on = the_run_will_work_on_two_filings();

    let what_the_work_answers = what_the_run_will_work_on[0].what_it_answers();
    assert_eq!(
        (
            what_the_work_answers.what_was_asked(),
            what_the_work_answers
                .where_it_was_filed()
                .expect("a filing this system made was filed somewhere")
                .as_str()
        ),
        (
            "Stop E2E runs leaving WAL state",
            "https://github.com/steamnoid/wellbeing-tracker-public/issues/10"
        ),
        "**because the filing is read back rather than remembered** — a run that composed the \
         request from what it had just written would put text in the work item that GitHub never \
         had, and the two would part company the moment a person edited one word"
    );
}

#[test]
fn a_filing_read_at_another_address_is_refused() {
    let (where_they_went, mut what_was_read) = two_filings_read_back_from_where_they_went();
    what_was_read.swap(0, 1);

    let what_the_run_will_work_on = the_work_each_filing_becomes(
        &the_repository_offered_in(),
        &what_was_read,
        &where_they_went,
    );

    assert!(
        what_the_run_will_work_on.is_err(),
        "**because a read filing paired with the wrong address is a work item about the wrong \
         issue** — and both halves are real, so nothing downstream could tell: \
         {what_the_run_will_work_on:?}"
    );
}
