//! The issue GitHub answered with, read out of the address it composed.
//!
//! **A filing run gets an address and the reading port takes a number, and the two have to meet.**
//! Composing an address from a number is what `CommissionedIssue` did and it resolved to a page that
//! did not exist — **but reading a number out of an address GitHub composed is the safe direction of
//! the same pair**, because the address is the primary fact and the number is a reading of it.
//!
//! **An address that is not an issue is refused, and a pull request is the case that matters** —
//! `…/issues/10` and `…/pull/10` are different things, and a work item pointed at the wrong one is a
//! work item about somebody else's pull request.

use aisdlc_run::an_issue_to_read::AnIssueToRead;

/// The repository these addresses are in.
fn the_repository_the_address_is_in() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// The address a real `gh issue create` prints.
fn the_address_an_issue_was_filed_at() -> String {
    String::from("https://github.com/steamnoid/wellbeing-tracker-public/issues/10")
}

#[test]
fn the_number_is_read_out_of_the_address_github_composed() {
    let the_issue = AnIssueToRead::the_issue_an_address_returned(
        &the_repository_the_address_is_in(),
        &the_address_an_issue_was_filed_at(),
    )
    .expect("GitHub composed this address itself");

    assert_eq!(
        the_issue.which_issue(),
        10,
        "**because the address is the primary fact and the number is a reading of it** — the other \\
         direction is the one this repository deleted, because an address composed from a number \\
         resolves to a page that does not exist while looking exactly like a link"
    );
}

#[test]
fn an_address_that_is_a_pull_request_is_not_an_issue() {
    let a_pull_request = "https://github.com/steamnoid/wellbeing-tracker-public/pull/10";

    let the_issue = AnIssueToRead::the_issue_an_address_returned(
        &the_repository_the_address_is_in(),
        a_pull_request,
    );

    assert!(
        the_issue.is_err(),
        "**because `…/issues/10` and `…/pull/10` are different things** — and a work item pointed at \\
         the wrong one is a work item about somebody else's pull request, with a number that was \\
         never wrong and an address that was: {the_issue:?}"
    );
}

#[test]
fn an_address_with_no_number_in_it_is_refused() {
    let not_an_issue_at_all = "https://github.com/steamnoid/wellbeing-tracker-public";

    let the_issue = AnIssueToRead::the_issue_an_address_returned(
        &the_repository_the_address_is_in(),
        not_an_issue_at_all,
    );

    assert!(
        the_issue.is_err(),
        "**because a number read out of nothing is a number this system chose** — and a filing \\
         handed to the product analyst as issue 0 is a work item nobody can find again: \\
         {the_issue:?}"
    );
}
