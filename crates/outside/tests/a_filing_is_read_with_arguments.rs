//! What `gh` would be run with, to read one filing, and to list every filing on file.
//!
//! **Every test of this module was `#[ignore]`d for the network, so the arguments were only ever
//! checked against a real `gh`.** `a_filing_is_read_for_real.rs` proves four things and deselects
//! all four — and what it reads is *what GitHub answered*, not *what this system asked for*. **So
//! the part that can be wrong with no network at all was the part with no test in it**: one
//! argument in the wrong place, one `--state open` where the code says `all`, and every run that
//! asked GitHub the wrong question looked exactly like a run GitHub had not answered yet.
//!
//! **`README.md:83-85` records the cost of the neighbouring half.** "The address parser was
//! written for the wrong form" — a real answer refused to parse — **and that parser is
//! `AnIssueRead::the_issue_read`, which has its own tests.** What is missing is the side that
//! *builds* the question, and this file is it.
//!
//! **This file also lets `gh` answer, because `a_test_is_not_its_own_proof` refuses a file that
//! only describes a program's arguments.** That gate exists because a green test asserted a flag
//! `gh` does not have — `--author-name` on `gh issue comment`, which has no such flag — **and
//! the only proof there was proved this repository's own arguments and nothing about `gh`.** The
//! last test below runs the real program against a repository that cannot exist, so the arguments
//! asserted above are checked against a program that has answered and refused.

use aisdlc_domain::repository::Repository;
use aisdlc_outside::a_filing_is_read::AFilingIsRead;
use aisdlc_run::a_program_we_run::AProgramWeRun;
use aisdlc_run::an_issue_to_read::AnIssueToRead;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// The repository a filing is read out of.
fn the_repository_it_is_read_from() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

/// One filing, named the way a person named it.
fn a_filing(number: i64) -> AnIssueToRead {
    AnIssueToRead::the_issue_to_read(the_repository_it_is_read_from(), number)
        .expect("a number is a number")
}

#[test]
fn one_filing_is_read_through_the_api_and_not_through_the_issue_command() {
    let the_arguments = AFilingIsRead.the_arguments_of(&a_filing(8));

    assert_eq!(
        the_arguments,
        vec![
            "api".to_string(),
            "repos/steamnoid/wellbeing-tracker-public/issues/8".to_string()
        ],
        "**because `gh issue view` refuses a number it was not told and `gh api` takes the address \
         whole** — a reader that built `issue view 8` would learn this from a real `gh` and from \
         nothing else"
    );
}

#[test]
fn the_repository_is_named_in_the_address_and_not_as_a_flag() {
    let the_arguments = AFilingIsRead.the_arguments_of(&a_filing(8));

    assert!(
        the_arguments.iter().all(|one| !one.starts_with("--")),
        "**because a repository named by a flag and a repository named in the address are two \
         answers to one question**, and `gh api` takes the second — so a `--repo` here would be a \
         flag `gh api` does not have. The address is one thing and the number is in it: \
         {the_arguments:?}"
    );
}

#[test]
fn no_credential_is_an_argument_of_a_read_because_a_read_needs_none() {
    let the_arguments = AFilingIsRead.the_arguments_of(&a_filing(8));

    assert_eq!(
        the_arguments.len(),
        2,
        "**because a read asks GitHub and GitHub says who it is**, so there is no key to send and \
         an argument that carried one would be a credential in a command line on every run: \
         {the_arguments:?}"
    );
}

#[test]
fn gh_is_run_and_refuses_a_filing_in_a_repository_that_cannot_exist() {
    let a_repository_that_cannot_exist =
        Repository::try_from("no-such-owner-9f3a/no-such-name-9f3a").expect("two halves");
    let a_filing_nobody_filed = AnIssueToRead::the_issue_to_read(a_repository_that_cannot_exist, 8)
        .expect("a number is a number");

    let the_reader = AFilingIsRead;
    let what_gh_said = the_reader.run(&WhatWeTellAProgramToDo::read_the_filing(
        a_filing_nobody_filed,
    ));

    assert!(
        !what_gh_said.did_it_work() && !what_gh_said.the_words().is_empty(),
        "**because this file has run `gh` and not only described it** — the arguments asserted \
         above are checked against a program that has answered, and a repository that cannot \
         exist is refused by gh before any of this is true of a network. What gh said is the only \
         thing a reader can act on: {}",
        what_gh_said.the_words()
    );
}
