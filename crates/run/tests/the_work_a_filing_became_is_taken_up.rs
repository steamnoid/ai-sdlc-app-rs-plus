//! What a filing became, taken up to the first stage.
//!
//! **Cycle 137 takes up what filings became.** A filing becomes the product analyst's work
//! standing at `IDLE` — **and nothing takes it up**, so the works a creative run filed stand
//! still while the pipeline that would work them waits for a run that never starts.
//!
//! **The run picks the work up with the agent its own discipline names and takes it into that
//! agent's hands**, because the walk asks agents and an agent that holds nothing answers
//! nothing. The first artifact is the discovery report.

use aisdlc_run::the_work_a_filing_became_is_taken_up::the_work_a_filing_became_is_taken_up;

#[test]
fn what_a_filing_became_is_worked_by_the_agent_its_discipline_names() {
    let where_the_run_stands = the_run_taking_up_what_a_filing_became();

    assert_eq!(
        where_the_run_stands.the_stage_it_stands_at(),
        aisdlc_domain::stage::Stage::InProgressByAgent,
        "**because the walk asks agents and an agent that holds nothing answers nothing** — and \
         a work item standing at pickup has not been worked on at all."
    );
    assert_eq!(
        where_the_run_stands
            .who_holds_it()
            .map(|who| who.to_string()),
        Some(String::from("aisdlc-po-analyze")),
        "**because the discipline accountable for a filing is the product analyst** — and a name \
         this system chose for itself would be a name nothing could be held to."
    );
}

#[test]
fn what_a_filing_became_stands_at_the_first_stage() {
    let where_the_run_stands = the_run_taking_up_what_a_filing_became();

    assert_eq!(
        where_the_run_stands
            .which_artifact_it_is_on()
            .map(ToString::to_string),
        Some(String::from("DiscoveryReport")),
        "**because every run of the pipeline starts by reading the repository** — and a run \
         standing nowhere has no artifact anybody could rule on."
    );
    assert_eq!(
        where_the_run_stands
            .the_work_item_it_is()
            .the_work_item_id()
            .to_string()
            .as_str(),
        "steamnoid/wellbeing-tracker-public#11",
        "**because the work a run does is the work the filing asked for** — and a run working \
         under another name answers another request."
    );
}

/// The run taking up what a filing became.
fn the_run_taking_up_what_a_filing_became() -> aisdlc_run::where_a_run_stands::WhereARunStands {
    let what_the_run_will_work_on = the_work_a_filing_became();
    the_work_a_filing_became_is_taken_up(&what_the_run_will_work_on)
        .expect("a filing becomes work standing where work begins")
}

/// The work a filing became, **read back and not remembered.**
fn the_work_a_filing_became() -> aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn {
    let where_it_was_filed =
        String::from("https://github.com/steamnoid/wellbeing-tracker-public/issues/11");
    let what_was_read =
        aisdlc_run::an_issue_read::AnIssueRead::the_issue_read(&what_github_answered())
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

/// What GitHub answered, **with a title the model did not write.**
fn what_github_answered() -> String {
    String::from(
        "{\"number\":11,\"title\":\"Stop E2E runs leaving WAL state\",\"body\":\"the ask\",\
         \"html_url\":\"https://github.com/steamnoid/wellbeing-tracker-public/issues/11\",\
         \"state\":\"open\"}",
    )
}
