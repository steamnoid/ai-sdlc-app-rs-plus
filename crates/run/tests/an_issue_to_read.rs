//! Which issue, of which repository, and the refusals before a request is made.
//!
//! **A number and not an address, because a number is what a person types.** GitHub numbers
//! issues from one, so a number that could not be one has no address — and asking anyway turns a
//! typo into a refusal that reads as a missing request, which is a person being told their filing
//! does not exist when it does.
//!
//! **Refused here rather than sent**, because a mistyped number and a missing issue answer
//! identically, and a run that could not tell them apart would report "no such issue" for somebody
//! who simply typed `7 `.

use aisdlc_domain::repository::Repository;
use aisdlc_run::an_issue_to_read::{ANumberThatIsNotAnIssueNumber, AnIssueToRead};

/// This repository, and one issue of it.
fn an_issue_to_read(number: i64) -> Result<AnIssueToRead, ANumberThatIsNotAnIssueNumber> {
    AnIssueToRead::the_issue_to_read(
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
        number,
    )
}

#[test]
fn what_a_person_named_is_what_will_be_asked_for() {
    let the_request = an_issue_to_read(8).expect("eight is a number");

    assert_eq!(
        the_request.which_issue(),
        8,
        "**because a person types a number and not an address** — and the address is GitHub's to \
         answer with, not ours to compose"
    );
}

#[test]
fn a_number_below_one_is_refused_before_anything_is_asked() {
    let the_way_out: Result<AnIssueToRead, ANumberThatIsNotAnIssueNumber> =
        AnIssueToRead::the_issue_to_read(
            Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
            0,
        );

    assert!(
        the_way_out.is_err(),
        "**because GitHub numbers issues from one** — and a number that is not one has no address, \
         and asking anyway turns a typo into a refusal that reads as a missing request: \
         {the_way_out:?}"
    );
}

#[test]
fn a_number_that_is_not_one_is_refused_by_its_own_type() {
    let the_way_out: Result<AnIssueToRead, ANumberThatIsNotAnIssueNumber> =
        AnIssueToRead::the_issue_to_read(
            Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
            -4,
        );

    let the_refusal = the_way_out.expect_err("-4 is not an issue number");
    assert_eq!(
        std::any::type_name_of_val(&the_refusal),
        "aisdlc_run::an_issue_to_read::ANumberThatIsNotAnIssueNumber",
        "**and the refusal is its own type rather than a string** — because a caller that must tell \
         a typo from a missing filing has nothing to act on when both arrive as text: {the_refusal}"
    );
}
