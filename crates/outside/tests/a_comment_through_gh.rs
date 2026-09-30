//! A comment, turned into what `gh` is run with, so that one really is left.
//!
//! **The port existed with no answer.** `AGitHubWeWriteTo` had exactly one implementor and it was a
//! test double, so a comment had never been written anywhere — the use case was proved against a
//! place that accepts anything, which is the shape that hides a broken translation.
//!
//! **The address is one argument and the body is one argument, and there is no shell.** A comment
//! body is prose with a colon, a dash and a backtick in it, and a body re-split by something is a
//! body somebody else has assembled.

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_outside::a_comment_reaching_github::ACommentReachingGitHub;
use aisdlc_run::a_git_hub_we_write_to::WhatGitHubWasAskedFor;
use aisdlc_run::who_signs_a_comment::WhoSignsAComment;

/// One comment, as GitHub is asked for it, signed by a deployment.
fn a_comment_with_a_body_saying(the_body: &str) -> WhatGitHubWasAskedFor {
    WhatGitHubWasAskedFor::a_comment_at(
        AWhereItWasFiled::the_address_the_filing_returned(
            "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/7",
        )
        .expect("an address GitHub returned is an address"),
        the_body,
        &WhoSignsAComment::the_name_a_deployment_used("aisdlc-dev")
            .expect("a deployment named one"),
    )
}

#[test]
fn a_comment_is_asked_of_gh_and_where_it_was_filed_is_one_argument() {
    let the_arguments =
        ACommentReachingGitHub.the_arguments_of(&a_comment_with_a_body_saying("hi"));

    let the_address_is_one = the_arguments
        .iter()
        .any(|one| one == "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/7");
    assert!(
        the_address_is_one,
        "**because the address is what the filing returned and not a fragment this system put \
         together** — and an address cut into an owner, a name and a number is three chances to be \
         wrong and one page that does not exist: {the_arguments:?}"
    );
}

#[test]
fn a_comment_body_is_one_argument_and_is_never_split_into_its_pieces() {
    let a_body =
        "a run looked at this.\n\n- it found `e2e.db-wal` tracked\n- it is fixing that: see --flag";
    let the_arguments =
        ACommentReachingGitHub.the_arguments_of(&a_comment_with_a_body_saying(a_body));

    let the_body_is_whole = the_arguments.iter().any(|one| one == a_body);
    let pieces_became_arguments = the_arguments
        .iter()
        .any(|one| one == "- it found `e2e.db-wal` tracked");
    assert!(
        the_body_is_whole && !pieces_became_arguments,
        "**because a comment body is prose, and prose has dashes and colons and backticks in it** — \
         and a body re-split into its lines is a body somebody else has assembled: {the_arguments:?}"
    );
}
