//! The question a creative agent is asked carries the filings already on file.
//!
//! **An agent cannot avoid restating a filing it was never shown.** The creative run listed the
//! repository and said nothing about the tracker, **and five real runs of it filed one SQLite
//! observation three times** — because on the evidence in front of it, every run was finding it for
//! the first time.
//!
//! **The reading happens before the question and not while answering it**, because a run that asked
//! first and read afterwards has already paid for an answer **that cannot have known**.

use aisdlc_domain::repository::Repository;
use aisdlc_run::an_agent_is_asked_what_it_would_file::the_question_an_agent_is_asked;
use aisdlc_run::the_filings_already_on_file::{AFilingAlreadyOnFile, TheFilingsAlreadyOnFile};

fn the_repository() -> Repository {
    Repository {
        owner: String::from("steamnoid"),
        name: String::from("wellbeing-tracker-public"),
    }
}

fn the_filings() -> TheFilingsAlreadyOnFile {
    let mut every_one = TheFilingsAlreadyOnFile::none_yet();
    every_one.and_this_one_was_too(AFilingAlreadyOnFile::that_is_open(
        12,
        "Gitignoring the E2E database artifacts is not the same as deleting them",
    ));
    every_one.and_this_one_was_too(AFilingAlreadyOnFile::that_is_open(
        14,
        "Stop shipping test code that no compiler reads",
    ));
    every_one
}

#[test]
fn the_question_names_every_filing_already_on_file() {
    let the_question = the_question_an_agent_is_asked(
        &the_repository(),
        &["src/db.ts".to_string()],
        &the_filings(),
    );

    assert!(
        the_question.contains("#12 Gitignoring the E2E database artifacts")
            && the_question.contains("#14 Stop shipping test code that no compiler reads"),
        "**because a model asked about a subset repeats the rest** — and the filings on file are the \
         only evidence that says an observation is not new: {the_question}"
    );
}

#[test]
fn a_repository_with_nothing_on_file_says_so_rather_than_showing_nothing() {
    let the_question = the_question_an_agent_is_asked(
        &the_repository(),
        &["src/db.ts".to_string()],
        &TheFilingsAlreadyOnFile::none_yet(),
    );

    assert!(
        the_question.contains("no issues are open"),
        "**because a question that shows an empty list reads as a repository with no issues**, and a \
         model told that files the fourth copy of an observation a person already filed: {the_question}"
    );
}

#[test]
fn a_question_naming_what_is_on_file_says_what_to_do_with_a_duplicate() {
    let the_question = the_question_an_agent_is_asked(
        &the_repository(),
        &["src/db.ts".to_string()],
        &the_filings(),
    );

    assert!(
        the_question.contains("it_restates"),
        "**because naming the filings is not enough — a model asked about existing work and given \
         nowhere to say an offer repeats one will file it**, and the field name is the whole of what \
         lets it: {the_question}"
    );
}
