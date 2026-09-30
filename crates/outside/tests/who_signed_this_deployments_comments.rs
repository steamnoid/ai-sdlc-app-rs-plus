//! The name on a comment, read out of the deployment that is writing it.
//!
//! **A port that answers is not a thing anybody calls.** `ACommentReachingGitHub` exists and
//! `WhoSignsAComment` exists, and until this reader existed there was **no way for a deployment to
//! put itself on a comment** — so the only way to reach the adapter was to invent a name, and
//! inventing a name is the forgery `WhoSignsAComment` was written to remove.
//!
//! **An absent variable is a refusal and not a default, and the refusal comes before the write.** A
//! comment with an invented author is worse than no comment, because it looks like a record.

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_outside::a_comment_reaching_github::ACommentReachingGitHub;
use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_outside::who_signed_this_deployments_comments::who_signed_these_comments;
use aisdlc_run::a_comment_is_left::the_run_leaves_a_comment;
use aisdlc_run::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhatGitHubWasAskedFor};

/// Somewhere that answers, and remembers that it did.
struct SomewhereThatAnswers {
    was_it_asked: bool,
}

impl AGitHubWeWriteTo for SomewhereThatAnswers {
    fn it_is_asked_to(
        &mut self,
        _what_git_hub_is_asked: &WhatGitHubWasAskedFor,
    ) -> Result<(), String> {
        self.was_it_asked = true;
        Ok(())
    }
}

fn a_request_that_was_filed() -> aisdlc_domain::commissioned_request::CommissionedRequest {
    aisdlc_domain::commissioned_request::CommissionedRequest::asked_for_and_filed_at(
        "untrack the sidecar files".to_string(),
        "e2e.db-wal is tracked and should not be".to_string(),
        AWhereItWasFiled::the_address_the_filing_returned(
            "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/7",
        )
        .expect("an address GitHub returned"),
    )
    .expect("a request somebody filed")
}

#[test]
fn a_deployment_that_named_itself_gets_that_name_on_its_comments() {
    let what_it_was_told = TheEnvironment::of(vec![(
        "AISDLC_WHO_SIGNS_A_COMMENT".to_string(),
        "aisdlc-dev".to_string(),
    )]);

    let the_name = who_signed_these_comments(&what_it_was_told);

    assert_eq!(
        the_name.as_ref().ok().map(|who| who.as_str().to_string()),
        Some("aisdlc-dev".to_string()),
        "**because the name on a comment is a fact about the deployment** — and a fact this system \
         chose is a forgery with a nicer name: {the_name:?}"
    );
}

#[test]
fn a_deployment_that_named_nobody_cannot_have_a_comment_written() {
    let what_it_was_told = TheEnvironment::of(vec![]);
    let mut somewhere = SomewhereThatAnswers {
        was_it_asked: false,
    };

    let the_way_out = who_signed_these_comments(&what_it_was_told);
    if let Ok(who_signed) = the_way_out.as_ref() {
        let _ = the_run_leaves_a_comment(
            &mut somewhere,
            &a_request_that_was_filed(),
            "a run",
            who_signed,
        );
    }

    assert!(
        the_way_out.is_err() && !somewhere.was_it_asked,
        "**because the order is the rule** — a comment with an invented author is worse than no \
         comment, because it looks like a record, and a reader is asked whether a person wrote it: \
         {the_way_out:?}"
    );
}

#[test]
fn gh_is_never_told_an_author_it_cannot_set_and_the_name_is_in_the_body() {
    let what_it_was_told = TheEnvironment::of(vec![(
        "AISDLC_WHO_SIGNS_A_COMMENT".to_string(),
        "aisdlc-dev".to_string(),
    )]);
    let who_signed = who_signed_these_comments(&what_it_was_told).expect("a name");
    let mut the_adapter = ACommentReachingGitHub;
    let _ = the_run_leaves_a_comment(
        &mut the_adapter,
        &a_request_that_was_filed(),
        "a run wrote this, and it is signed by aisdlc-dev",
        &who_signed,
    );

    let the_arguments = the_adapter.the_arguments_of(&a_comment_about_the_filing());
    let gh_was_told_an_author = the_arguments.iter().any(|one| one.starts_with("--author"));
    let the_name_is_somewhere_a_reader_sees =
        the_arguments.iter().any(|one| one.contains("aisdlc-dev"));
    assert!(
        !gh_was_told_an_author && the_name_is_somewhere_a_reader_sees,
        "**because `gh issue comment` has no author flag** — a real run proved it, the program \
         answered `unknown flag: --author-name` — and it posts as whoever the token belongs to and \
         offers nothing to change that, **so an author this system sets is an author this system \
         has invented**: {the_arguments:?}"
    );
}

/// The comment as GitHub is asked for it, for the arguments to be read.
fn a_comment_about_the_filing() -> WhatGitHubWasAskedFor {
    let who_signed =
        aisdlc_run::who_signs_a_comment::WhoSignsAComment::the_name_a_deployment_used("aisdlc-dev")
            .expect("a deployment named one");
    WhatGitHubWasAskedFor::a_comment_at(
        AWhereItWasFiled::the_address_the_filing_returned(
            "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/7",
        )
        .expect("an address"),
        "a run is opening a pull request for this",
        &who_signed,
    )
}
