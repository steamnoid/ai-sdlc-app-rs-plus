//! A number that carries no filing, and an answer this system cannot read — two different facts.
//!
//! **They arrive nearly the same way, which is why they must not be refused the same way.** When
//! GitHub has no filing at a number, `gh` writes `{"message":"Not Found","status":"404"}` and exits
//! non-zero. Read as one stream with its complaint, that is not valid JSON — **so a reader that only
//! knows how to parse would report "I cannot read this", and a person would be told this system is
//! broken rather than that their filing is not there.**
//!
//! **A caller must be able to tell the two apart**, because one is worth retrying and the other is
//! worth reading the number again.

use aisdlc_run::an_issue_read::{AnIssueRead, WhyAnAnswerIsNotAFiling};

/// What `gh` writes on its answer stream when the number carries no filing.
///
/// **The answer alone, and not the answer with the complaint joined to it** — `gh` writes the JSON
/// on one stream and `gh: Not Found (HTTP 404)` on the other, and joining them produces something
/// that is not an answer at all.
fn what_github_answered_for_a_number_with_nothing_behind_it() -> String {
    String::from(
        "{\"message\":\"Not Found\",\"documentation_url\":\"https://docs.github.com/rest/issues\
         /issues#get-an-issue\",\"status\":\"404\"}",
    )
}

/// What a far end wrote that is not an answer at all.
fn what_a_broken_answer_looks_like() -> String {
    String::from("<!DOCTYPE html><html><head><title>502</title></head></html>")
}

#[test]
fn a_number_with_nothing_behind_it_is_refused_as_no_such_filing() {
    let what_it_answered = what_github_answered_for_a_number_with_nothing_behind_it();

    let the_way_out = AnIssueRead::the_issue_read(&what_it_answered);

    let it_is_no_such_filing = the_way_out
        .as_ref()
        .err()
        .map(|why_not| matches!(why_not, WhyAnAnswerIsNotAFiling::NoSuchFiling(_)));
    assert_eq!(
        it_is_no_such_filing,
        Some(true),
        "**because \"there is nothing there\" and \"this system cannot read this\" are different \
         facts** — and a person told the wrong one is sent to look at this system instead of at \
         their own number: {the_way_out:?}"
    );
}

#[test]
fn the_refusal_says_what_was_asked_for_and_not_merely_that_it_failed() {
    let what_it_answered = what_github_answered_for_a_number_with_nothing_behind_it();

    let the_words = AnIssueRead::the_issue_read(&what_it_answered)
        .expect_err("nothing is behind that number")
        .to_string();

    let it_says_there_is_nothing =
        the_words.contains("no filing") || the_words.contains("Not Found");
    assert!(
        it_says_there_is_nothing,
        "**and a refusal a person cannot act on is not a refusal** — the words have to say that \
         there is nothing there, because the next thing a person does is check their own number: \
         {the_words}"
    );
}

#[test]
fn an_answer_this_system_cannot_read_is_never_reported_as_no_such_filing() {
    let a_broken_answer = what_a_broken_answer_looks_like();

    let the_way_out = AnIssueRead::the_issue_read(&a_broken_answer);

    let it_is_merely_unreadable = the_way_out
        .as_ref()
        .err()
        .map(|why_not| !matches!(why_not, WhyAnAnswerIsNotAFiling::NoSuchFiling(_)));
    assert_eq!(
        it_is_merely_unreadable,
        Some(true),
        "**because a far end having a bad minute is not a filing not existing** — and reporting one \
         as the other tells a person their filing is gone when the far end simply answered \
         nonsense: {the_way_out:?}"
    );
}
