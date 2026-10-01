//! The mark every filing this system makes carries, because a reader cannot otherwise tell.
//!
//! **An issue a person asked for and an issue a model invented are the same shape.** Both have a
//! title, a body and an author, **and the only thing that tells them apart is a label** — which is why
//! a filing this system makes **refuses to be made without it** rather than being made unlabelled and
//! marked afterwards.
//!
//! **The mark is the project's full name and not a short namespace**, because **a filing lands in a
//! repository this project does not live in** — and a reader there has never heard of `aisdlc`,
//! cannot find what it refers to, **and is left with an issue they cannot attribute.** A short name
//! saves eleven characters and spends the one thing a mark exists for.

use aisdlc_outside::a_filing_reaching_github::AFilingReachingGitHub;
use aisdlc_run::a_git_hub_we_write_to::WhatGitHubWasAskedFor;

/// The repository a filing is filed in.
fn the_repository_it_is_filed_in() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// One offer grounded in one file, **which is what becomes a filing.**
fn an_offer_about_one_file() -> aisdlc_run::what_an_agent_offered::AWorkAnAgentOffered {
    aisdlc_run::what_an_agent_offered::AWorkAnAgentOffered {
        title: "the listing: why nothing compares a path".to_string(),
        what_it_asks_for: "compare every cited path with the listing".to_string(),
        what_it_concluded_from: Vec::new(),
    }
}

/// The filing this system makes.
fn the_filing_being_asked_for() -> WhatGitHubWasAskedFor {
    aisdlc_run::an_offer_becomes_a_filing::the_filing_an_offer_becomes(
        &the_repository_it_is_filed_in(),
        &an_offer_about_one_file(),
    )
}

#[test]
fn a_filing_this_system_made_carries_the_mark_that_says_who_wrote_it() {
    let the_arguments = AFilingReachingGitHub.the_arguments_of(&the_filing_being_asked_for());

    assert!(
        the_arguments
            .windows(2)
            .any(|two| two == ["--label", "aisdlc"]),
        "**because an issue a person asked for and an issue a model invented are the same shape** — \\
         both have a title, a body and an author, **and a label is the only thing that tells them \\
         apart.** An unlabelled proposal from this system is indistinguishable from a colleague's \\
         request: {the_arguments:?}"
    );
}

#[test]
fn the_mark_is_the_projects_full_name_and_not_a_namespace_nobody_else_could_resolve() {
    let the_arguments = AFilingReachingGitHub.the_arguments_of(&the_filing_being_asked_for());

    let where_the_mark_is = the_arguments
        .windows(2)
        .find(|two| two[0] == "--label")
        .map(|two| two[1].clone());
    assert_eq!(
        where_the_mark_is.as_deref(),
        Some("aisdlc"),
        "**because that is what this system's agents are called** — `aisdlc-discover`, \\
         `aisdlc-offer` — **and a namespace chosen for this purpose alone would be a second name for \\
         one thing:** {the_arguments:?}"
    );
}
