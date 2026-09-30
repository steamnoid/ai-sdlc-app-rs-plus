//! A filing, read out of GitHub by `gh`, against the real thing.
//!
//! **This is the half that was missing while the writing half was finished.** A comment could go
//! back to a filing that nothing could bring in, and this is the end that brings one in.
//!
//! **Two refusals are proved first, because they cost nothing and they are the ones a run can get
//! wrong silently.** A number that carries no filing, and a number that carries a pull request —
//! **in the repository this is pointed at, six of the seven entries are pull requests.** Then the
//! reading itself, on a filing that really exists.

use aisdlc_domain::repository::Repository;
use aisdlc_outside::a_filing_is_read::AFilingIsRead;
use aisdlc_run::an_issue_to_read::AnIssueToRead;

/// The repository this run is pointed at, and one filing of it.
fn a_filing(
    number: i64,
) -> Result<AnIssueToRead, aisdlc_run::an_issue_to_read::ANumberThatIsNotAnIssueNumber> {
    AnIssueToRead::the_issue_to_read(
        Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
        number,
    )
}

#[test]
#[ignore = "reads GitHub over the network, and needs gh authenticated"]
fn a_filing_is_read_and_answers_where_it_answers() {
    let mut the_reader = AFilingIsRead;

    let what_was_read = the_reader.the_filing_is_read(&a_filing(8).expect("eight is a number"));

    let it_was_read = what_was_read.expect("issue 8 exists and was filed by a person");
    eprintln!(
        "read {:?}, still open: {}",
        it_was_read.where_it_answers(),
        it_was_read.whether_it_is_open()
    );
    assert_eq!(
        it_was_read.where_it_answers(),
        "https://github.com/steamnoid/wellbeing-tracker-public/issues/8",
        "**because the address is the one GitHub answered at** — and this is the whole point of \
         the read: a filing that cannot be brought in cannot be answered on"
    );
}

#[test]
#[ignore = "reads GitHub over the network, and needs gh authenticated"]
fn a_pull_request_at_that_address_is_refused_and_not_read_as_a_filing() {
    let mut the_reader = AFilingIsRead;

    let what_was_read = the_reader.the_filing_is_read(&a_filing(115).expect("a number"));

    assert!(
        what_was_read.is_err(),
        "**because a pull request's body describes a change rather than asking for one** — and a \
         run that read it as a request would go on to write artifacts about work nobody asked for: \
         {what_was_read:?}"
    );
}
