//! What a filing is about, and who answers for it — both of which a run is told.
//!
//! **Two values were written into the code.** `A-1` and `PO` sat in a function that builds the
//! work, so every run in history was a work item called `A-1` accountable to the product analyst,
//! whatever was filed. **A record that says the same thing every time is not a record of this run.**
//!
//! **The role is the discipline that answers and not the one that notices**, and the two are
//! different questions: an issue nobody had looked at yet has a discipline waiting, and only the
//! work has one answering.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_run::an_issue_read::AnIssueRead;

use aisdlc_run::what_this_filing_is_about::{
    TheRoleThatAnswers, TheWorkItemsId, WhatThisFilingIsAbout,
};

/// What GitHub answered for a filing somebody wrote.
fn a_filing_somebody_wrote() -> AnIssueRead {
    AnIssueRead::the_issue_read(
        "{\"number\":8,\"title\":\"the root tsconfig covers the tests\",\"body\":\"a body\",\
         \"html_url\":\"https://github.com/steamnoid/wellbeing-tracker-public/issues/8\",\
         \"state\":\"open\"}",
    )
    .expect("that is a filing")
}

#[test]
fn a_run_is_told_which_work_item_it_is_and_nothing_is_written_into_the_code() {
    let what_this_filing_is_about = WhatThisFilingIsAbout {
        which_work_item: TheWorkItemsId::try_from("WELLBEINGT-42").expect("a name"),
        which_role_answers: TheRoleThatAnswers::try_from("DEV").expect("a discipline"),
        who_asked: WhoCommissioned::try_from("llm agent").expect("a name"),
        which_repository: Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
    };

    assert_eq!(
        (
            what_this_filing_is_about.which_work_item.as_str(),
            what_this_filing_is_about.which_role_answers.as_str()
        ),
        ("WELLBEINGT-42", "DEV"),
        "**because a record that says the same thing every time is not a record of this run** — \
         and `A-1` and `PO` sat in the code that builds the work, so every run in history was a work \
         item called `A-1`: {what_this_filing_is_about:?}"
    );
}

#[test]
fn a_filing_and_what_it_is_about_become_a_work_of_the_right_name() {
    let what_a_run_was_told = WhatThisFilingIsAbout {
        which_work_item: TheWorkItemsId::try_from("WELLBEINGT-42").expect("a name"),
        which_role_answers: TheRoleThatAnswers::try_from("DEV").expect("a discipline"),
        who_asked: WhoCommissioned::try_from("llm agent").expect("a name"),
        which_repository: Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
    };
    let the_work = a_filing_somebody_wrote().the_work_it_becomes(&what_a_run_was_told);

    assert_eq!(
        the_work.work_item.the_work_item_id().to_string().as_str(),
        "WELLBEINGT-42",
        "**because the work carries the name a run was told** — and a work item that ignores its \\
         own name is a work item that cannot be found again by anybody: {the_work:?}"
    );
    assert_eq!(
        the_work
            .requested
            .where_it_was_filed()
            .map(aisdlc_domain::commissioned_request::AWhereItWasFiled::as_str),
        Some("https://github.com/steamnoid/wellbeing-tracker-public/issues/8"),
        "**and the filing is still the one that was read** — the address GitHub answered at, and \\
         never one composed from a number: {the_work:?}"
    );
}

#[test]
fn a_work_item_id_that_is_not_one_is_refused_before_anything_is_built() {
    let the_way_out = TheWorkItemsId::try_from("   ");

    assert!(
        the_way_out.is_err(),
        "**because a blank work item id is a work nothing can be found by** — and a run that went \\
         ahead would leave a record nothing points at: {the_way_out:?}"
    );
}
