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
use aisdlc_outside::a_comment_reaching_github::ACommentReachingGitHub;
use aisdlc_outside::a_filing_is_read::AFilingIsRead;
use aisdlc_run::a_filing_is_answered::a_filing_is_answered;
use aisdlc_run::an_issue_to_read::AnIssueToRead;
use aisdlc_run::what_this_filing_is_about::{
    TheRoleThatAnswers, TheWorkItemsId, WhatThisFilingIsAbout,
};

/// What a run was told this filing is about, and **told rather than guessed in the code.**
fn a_run_that_was_told_what_it_is_about() -> WhatThisFilingIsAbout {
    WhatThisFilingIsAbout {
        which_work_item: TheWorkItemsId::try_from("WELLBEINGT-42").expect("a name"),
        which_role_answers: TheRoleThatAnswers::try_from("DEV").expect("a discipline"),
    }
}

/// The name a deployment writes its own comments under.
fn a_signer() -> aisdlc_run::who_signs_a_comment::WhoSignsAComment {
    aisdlc_run::who_signs_a_comment::WhoSignsAComment::the_name_a_deployment_used("aisdlc-dev")
        .expect("a name")
}

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

#[test]
#[ignore = "reads GitHub and writes to it, and needs gh authenticated"]
fn a_filing_is_read_and_then_answered_on_it() {
    let mut the_reader = AFilingIsRead;
    let what_was_read = the_reader
        .the_filing_is_read(&a_filing(8).expect("eight is a number"))
        .expect("issue 8 exists and was filed by a person");
    let who_signed = a_signer();
    let mut the_writer = ACommentReachingGitHub;

    let how_it_went = a_filing_is_answered(
        &mut the_writer,
        &what_was_read,
        "A run of this system read this filing and is looking at it. No work has been done and \
         none is claimed yet.",
        &who_signed,
        &a_run_that_was_told_what_it_is_about(),
    );

    eprintln!(
        "read {} and answered: {how_it_went:?}",
        what_was_read.where_it_answers()
    );
    assert!(
        how_it_went.is_ok(),
        "**because a filing that can be read and cannot be answered is half a loop** — and the two \
         halves were both proved separately before anything joined them: {how_it_went:?}"
    );
}

#[test]
#[ignore = "reads GitHub over the network, and needs gh authenticated"]
fn a_number_with_nothing_behind_it_is_refused_as_no_such_filing_and_not_as_a_broken_read() {
    let mut the_reader = AFilingIsRead;

    let what_was_read = the_reader.the_filing_is_read(&a_filing(99_999_999).expect("a number"));

    let the_words = what_was_read
        .as_ref()
        .err()
        .map(std::string::ToString::to_string)
        .unwrap_or_default();
    eprintln!("99_999_999 was refused as: {the_words}");
    assert!(
        the_words.contains("no filing") || the_words.contains("Not Found"),
        "**because a person told 'this system cannot read this' goes to look at this system** — \
         while the next thing they should do is check their own number: {the_words}"
    );
}
