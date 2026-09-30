//! A filing that was read, and the comment that is written back on it.
//!
//! **Two working ends and no use joining them are two proofs and not a system.** The read exists
//! and the comment exists, and a run that never joins them has demonstrated a repository and a
//! filing rather than a thing that works on a filing.
//!
//! **The comment goes to the address that was read, and never to one composed from the number.**
//! An issue that was transferred answers at an address that does not carry the number asked for —
//! **and a comment composed from a number is a comment on a page that has nothing to do with the
//! one the run read.**

use aisdlc_domain::repository::Repository;
use aisdlc_domain::who_commissioned::WhoCommissioned;

use aisdlc_run::a_filing_is_answered::{
    TheWork, a_filing_is_answered, a_filing_is_answered_into_work,
};
use aisdlc_run::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhatGitHubWasAskedFor};
use aisdlc_run::an_issue_read::AnIssueRead;
use aisdlc_run::what_this_filing_is_about::{
    TheRoleThatAnswers, TheWorkItemsId, WhatThisFilingIsAbout,
};
use aisdlc_run::who_signs_a_comment::WhoSignsAComment;

/// Somewhere that remembers where it was asked to write, and to whom.
struct SomewhereThatRemembers {
    where_it_was_asked: Vec<String>,
}

impl AGitHubWeWriteTo for SomewhereThatRemembers {
    fn it_is_asked_to(
        &mut self,
        what_git_hub_is_asked: &WhatGitHubWasAskedFor,
    ) -> Result<(), String> {
        self.where_it_was_asked.push(
            what_git_hub_is_asked
                .where_it_is_going()
                .expect("a comment has somewhere to go")
                .as_str()
                .to_string(),
        );
        Ok(())
    }
}

/// What GitHub answered for a filing that was **transferred**, and so answers at an address
/// that does not carry the number that was asked for.
fn what_github_answered_for_a_transferred_filing() -> String {
    String::from(
        "{\"number\":8,\"title\":\"the root tsconfig covers the tests\",\"body\":\"a body\",\
         \"html_url\":\"https://github.com/steamnoid/one/issues/301\",\"state\":\"open\"}",
    )
}

/// What a run was told this filing is about, and **told rather than guessed in the code.**
fn a_run_that_was_told_what_it_is_about() -> WhatThisFilingIsAbout {
    WhatThisFilingIsAbout {
        which_work_item: TheWorkItemsId::try_from("WELLBEINGT-42").expect("a name"),
        which_role_answers: TheRoleThatAnswers::try_from("DEV").expect("a discipline"),
        who_asked: WhoCommissioned::try_from("llm agent").expect("a name"),
        which_repository: Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
    }
}

#[test]
fn a_work_item_says_who_asked_and_not_the_name_the_code_carried() {
    let what_was_read =
        AnIssueRead::the_issue_read(&what_github_answered_for_a_transferred_filing())
            .expect("a filing");
    let what_a_run_was_told = a_run_that_was_told_what_it_is_about();

    let the_work = TheWork::a_filing_becomes(&what_was_read, &what_a_run_was_told);

    assert_eq!(
        (
            the_work.work_item.who_asked().to_string(),
            the_work.work_item.the_repository().to_string()
        ),
        (
            "llm agent".to_string(),
            "steamnoid/wellbeing-tracker-public".to_string()
        ),
        "**because the code that builds a work item carried a person and `steamnoid/one` whatever \
         the run was told** — and a run that files its own ideas would have signed somebody's name \
         on every one of them. That is the forgery #104 closed for a comment, in the one place a \
         record of *who asked* is read first: {the_work:?}"
    );
}

/// The name a deployment writes its own comments under.
fn a_signer() -> WhoSignsAComment {
    WhoSignsAComment::the_name_a_deployment_used("aisdlc-dev").expect("a name")
}

#[test]
fn a_comment_goes_to_the_address_that_was_read_and_not_to_one_composed() {
    let what_was_read =
        AnIssueRead::the_issue_read(&what_github_answered_for_a_transferred_filing())
            .expect("a transferred filing is still a filing");
    let mut somewhere = SomewhereThatRemembers {
        where_it_was_asked: Vec::new(),
    };

    let how_it_went = a_filing_is_answered(
        &mut somewhere,
        &what_was_read,
        "a run is looking at this",
        &a_signer(),
        &a_run_that_was_told_what_it_is_about(),
    );

    assert_eq!(
        somewhere.where_it_was_asked.as_slice(),
        ["https://github.com/steamnoid/one/issues/301".to_string()],
        "**because the comment goes to the address that was read** — the filing answers at 301 and \
         was asked for as 8, and **an address composed from the number is a comment on a page that \
         has nothing to do with the one the run read**: {how_it_went:?} / {:?}",
        somewhere.where_it_was_asked
    );
}

#[test]
fn the_words_a_person_filed_are_the_words_the_work_carries() {
    let what_was_read =
        AnIssueRead::the_issue_read(&what_github_answered_for_a_transferred_filing())
            .expect("a transferred filing is still a filing");

    let the_work =
        a_filing_is_answered_into_work(&what_was_read, &a_run_that_was_told_what_it_is_about());

    assert_eq!(
        (
            the_work.requested.title.as_str(),
            the_work.requested.body.as_str(),
            the_work
                .requested
                .where_it_was_filed()
                .map(aisdlc_domain::commissioned_request::AWhereItWasFiled::as_str)
        ),
        (
            "the root tsconfig covers the tests",
            "a body",
            Some("https://github.com/steamnoid/one/issues/301")
        ),
        "**because a person who reads the work later has to see the words they filed** — and a \
         work item that paraphrases a filing is a record of a different request: {the_work:?}"
    );
}
