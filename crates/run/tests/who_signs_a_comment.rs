//! The name on a comment this system wrote, and why a caller does not get to choose it.
//!
//! **A name on a comment is a record of who wrote it, and a comment nobody wrote cannot carry a
//! person's name.** `the_run_leaves_a_comment` took the name as an argument, and a test asserted
//! that the name "Alice" reached GitHub — **so this system held a way to put a person's name on its
//! own writing, and a test called that correct.** Nobody reading the record afterwards could tell a
//! person had not written it.
//!
//! **So the use case takes no name at all.** There is no argument for a caller to sign with, and a
//! forgery primitive is removed by having no door rather than by a check on the way through it.

use aisdlc_run::a_comment_is_left::the_run_leaves_a_comment;
use aisdlc_run::a_git_hub_we_write_to::{
    AGitHubWeWriteTo, WhatGitHubWasAskedFor, WhereGitHubAnswered,
};
use aisdlc_run::who_signs_a_comment::WhoSignsAComment;

/// Somewhere that remembers what it was asked, and takes it.
struct SomewhereThatRemembers {
    what_it_was_asked: Vec<WhatGitHubWasAskedFor>,
}

impl AGitHubWeWriteTo for SomewhereThatRemembers {
    fn it_is_asked_to(
        &mut self,
        what_git_hub_is_asked: &WhatGitHubWasAskedFor,
    ) -> Result<WhereGitHubAnswered, String> {
        self.what_it_was_asked.push(what_git_hub_is_asked.clone());
        Ok(WhereGitHubAnswered::an_address_at(
            "https://github.com/steamnoid/wellbeing-tracker-public/issues/1",
        ))
    }
}

fn a_request_that_was_filed() -> aisdlc_domain::commissioned_request::CommissionedRequest {
    aisdlc_domain::commissioned_request::CommissionedRequest::asked_for_and_filed_at(
        "untrack the sidecar files".to_string(),
        "e2e.db-wal and e2e.db-shm are in version control and should not be".to_string(),
        aisdlc_domain::commissioned_request::AWhereItWasFiled::the_address_the_filing_returned(
            "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/7",
        )
        .expect("an address GitHub returned is an address"),
    )
    .expect("a request somebody filed is a request")
}

#[test]
fn the_use_case_takes_no_name_a_caller_could_sign_with() {
    let the_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/a_comment_is_left.rs"
    ))
    .expect("the file that leaves a comment is readable");

    let the_signature = the_source
        .split("pub fn the_run_leaves_a_comment")
        .nth(1)
        .and_then(|what_follows| what_follows.split(')').next())
        .unwrap_or_default();

    assert!(
        !the_signature.contains("WhoDecided"),
        "**because a name on a comment is a record of who wrote it, and this system wrote it** — a \
         caller who supplies the name can put a person's name on this system's own writing, and \
         nobody reading the record afterwards can tell a person had not written it: the signature is \
         {the_signature}"
    );
}

#[test]
fn a_comment_carries_the_name_a_deployment_used() {
    let a_deployment_named =
        WhoSignsAComment::the_name_a_deployment_used("aisdlc-dev").expect("a deployment named one");
    let mut somewhere = SomewhereThatRemembers {
        what_it_was_asked: Vec::new(),
    };

    let _ = the_run_leaves_a_comment(
        &mut somewhere,
        &a_request_that_was_filed(),
        "a run is opening a pull request for this",
        &a_deployment_named,
    );

    let what_it_was_asked = somewhere
        .what_it_was_asked
        .first()
        .expect("the comment was written somewhere");
    assert_eq!(
        what_it_was_asked.who_wrote_this(),
        "aisdlc-dev",
        "**because the name is a fact about the deployment and not a choice this call made** — and a \
         name that a caller chose is a name a caller could have forged: {what_it_was_asked:?}"
    );
}

#[test]
fn a_deployment_that_named_nobody_cannot_have_a_comment_written() {
    let the_way_out = WhoSignsAComment::the_name_a_deployment_used("   ");

    assert!(
        the_way_out.is_err(),
        "**because a comment with nobody on it is a comment nobody can be asked about** — and an \
         invented name is the forgery this whole type exists to remove: {the_way_out:?}"
    );
}
