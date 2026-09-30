//! A filing reaching GitHub, and the one answer this port has to hand back.
//!
//! **A comment has nowhere new to point, and a filing has exactly one place to point.** `gh issue
//! create` prints the address of what it made — **and a run that filed an issue into a real
//! repository and threw the address away has done the one thing a person cannot undo and cannot
//! find.** That is the whole reason the port returns GitHub's answer rather than nothing.
//!
//! **A title is a separate argument from a body**, because a title has dashes and colons in it and a
//! body is prose with backticks — **and one string re-split into two is one string somebody else
//! assembled.**

use aisdlc_outside::a_filing_reaching_github::AFilingReachingGitHub;
use aisdlc_run::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhatGitHubWasAskedFor};
use aisdlc_run::what_an_agent_offered::AWorkAnAgentOffered;

/// The repository a filing is filed in.
fn the_repository_it_is_filed_in() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// One offer grounded in one file, with a title that has a dash and a colon in it.
fn an_offer_with_an_awkward_title() -> AWorkAnAgentOffered {
    AWorkAnAgentOffered {
        title: "the listing: why nothing compares a path".to_string(),
        what_it_asks_for: "compare every cited path with the listing".to_string(),
        what_it_concluded_from: Vec::new(),
    }
}

/// What GitHub is asked for when this system files something it concluded.
fn the_filing_being_asked_for() -> WhatGitHubWasAskedFor {
    aisdlc_run::an_offer_becomes_a_filing::the_filing_an_offer_becomes(
        &the_repository_it_is_filed_in(),
        &an_offer_with_an_awkward_title(),
    )
}

#[test]
fn a_filing_carries_the_title_it_was_called_and_not_only_its_words() {
    let the_filing = the_filing_being_asked_for();

    assert_eq!(
        the_filing.what_the_filing_is_called(),
        Some("the listing: why nothing compares a path"),
        "**because a title and a body and no title is a filing nobody can find in a list of fifty** — \
         and an issue titled with the first words of its own paragraph is an issue a person cannot \
         scan: {the_filing:?}"
    );
}

#[test]
fn the_arguments_name_the_repository_the_title_and_the_body_one_each() {
    let the_filing = the_filing_being_asked_for();

    let the_arguments = AFilingReachingGitHub.the_arguments_of(&the_filing);

    assert_eq!(
        the_arguments,
        vec![
            String::from("issue"),
            String::from("create"),
            String::from("--repo"),
            String::from("steamnoid/wellbeing-tracker-public"),
            String::from("--title"),
            String::from("the listing: why nothing compares a path"),
            String::from("--body"),
            the_filing.what_is_written().to_string(),
        ],
        "**because the repository is asked for by name and not by address** — the filing does not \
         exist yet, so there is no address to give — **and the title and the body are one argument \
         each, because prose re-split into two is prose somebody else assembled**"
    );
}

#[test]
fn a_filing_whose_address_github_never_gave_is_refused() {
    let what_github_said = AFilingReachingGitHub.what_the_address_it_returned("");

    assert!(
        what_github_said.is_err(),
        "**because a run that filed an issue into a real repository and threw the address away has done \
         the one thing a person cannot undo and cannot find** — and there is exactly one address of \
         that issue and it is the one `gh` printed: {what_github_said:?}"
    );
}

#[test]
fn the_address_github_returned_is_the_only_address() {
    let what_github_said = AFilingReachingGitHub.what_the_address_it_returned(
        "https://github.com/steamnoid/wellbeing-tracker-public/issues/11\n",
    );

    assert_eq!(
        what_github_said.expect("an address was printed").as_str(),
        "https://github.com/steamnoid/wellbeing-tracker-public/issues/11",
        "**because this is the address GitHub composed, not one this system assembled from a \
         repository and a number** — that composition is the thing this repository deleted once, and \
         it resolved to a page that did not exist while looking exactly like a link"
    );
}

#[test]
fn a_comment_still_answers_with_the_address_it_commented_at() {
    let a_comment = WhatGitHubWasAskedFor::a_comment_at(
        aisdlc_domain::commissioned_request::AWhereItWasFiled::the_address_the_filing_returned(
            "https://github.com/steamnoid/wellbeing-tracker-public/issues/8",
        )
        .expect("an address that was read"),
        "a comment",
        &aisdlc_run::who_signs_a_comment::WhoSignsAComment::the_name_a_deployment_used(
            "aisdlc-dev",
        )
        .expect("a name"),
    );

    assert_eq!(
        a_comment.what_the_filing_is_called(),
        None,
        "**because a comment is not a filing and a title asked of one is a title nobody filed** — \
         and one port carrying both has to be able to say which it was asked for: {a_comment:?}"
    );
}
