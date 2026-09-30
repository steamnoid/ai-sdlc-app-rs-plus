//! A pull request at the address a filing was asked for, refused rather than read.
//!
//! **GitHub answers issues and pull requests from one path, so a number handed for an issue may
//! name a pull request** — and that body's description of a change is not a request for one.
//! Reading it as a request hands the pipeline a description to work from, and every artifact after
//! it is about work nobody asked for.
//!
//! **In the repository this system is pointed at, this is the likely reading and not a corner:**
//! six of the seven entries GitHub answers with are pull requests.

use aisdlc_run::an_issue_read::AnIssueRead;

/// What GitHub answers with for a pull request, which is one path and one number as issues.
fn what_github_answered_for_a_pull_request() -> String {
    String::from(
        "{\"number\":113,\"title\":\"docs: what has been proved for real\",\"body\":\"a change\",\
         \"html_url\":\"https://github.com/steamnoid/ai-sdlc-app-rs-plus/pull/113\",\"state\":\
         \"closed\",\"pull_request\":{\"url\":\"https://api.github.com/repos/x/y/pulls/113\"}}",
    )
}

#[test]
fn a_pull_request_is_refused_and_says_which_one_it_was() {
    let what_it_answered = what_github_answered_for_a_pull_request();

    let what_was_read = AnIssueRead::the_issue_read(&what_it_answered);

    assert!(
        what_was_read.is_err(),
        "**because a pull request's body describes a change rather than asking for one** — and \
         reading it as a request hands the pipeline a description to work from: {what_was_read:?}"
    );
}

#[test]
fn the_refusal_names_it_and_what_it_says() {
    let what_it_answered = what_github_answered_for_a_pull_request();

    let the_refusal =
        AnIssueRead::the_issue_read(&what_it_answered).expect_err("a pull request is not a filing");

    let the_words = the_refusal.to_string();
    let it_names_the_address = the_words.contains("pull/113");
    let it_quotes_what_came_back = the_words.contains("docs: what has been proved");
    assert!(
        it_names_the_address && it_quotes_what_came_back,
        "**because a refusal a person cannot act on is not a refusal** — the address is what they \
         can go and look at, and the quoted answer is what tells a typo from a wrong path: \
         {the_words}"
    );
}
