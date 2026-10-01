//! An offer becoming a filing, and what GitHub is asked for when something does not exist yet.
//!
//! **A comment has an address and a filing does not**, because a filing is what gives itself one —
//! so a request carrying an address for a filing would be a request for a page that does not exist,
//! **and this repository deleted exactly that once**: `CommissionedIssue` derived `…/issues/N` for a
//! repository with no issues, and the link resolved to nothing while looking exactly like a link.

use aisdlc_run::a_git_hub_we_write_to::WhatGitHubWasAskedFor;
use aisdlc_run::an_offer_becomes_a_filing::the_filing_an_offer_becomes;
use aisdlc_run::what_an_agent_offered::AWorkAnAgentOffered;
use aisdlc_run::what_discovery_concluded::EvidenceFile;

/// The repository a filing is filed in.
fn the_repository_it_read() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// The address one real filing returned, for a comment that has somewhere to go.
fn an_address_a_filing_returned() -> aisdlc_domain::commissioned_request::AWhereItWasFiled {
    aisdlc_domain::commissioned_request::AWhereItWasFiled::the_address_the_filing_returned(
        "https://github.com/steamnoid/wellbeing-tracker-public/issues/8",
    )
    .expect("an address that was read")
}

/// A comment, which has an address because the thing it comments on already has one.
fn a_comment() -> WhatGitHubWasAskedFor {
    WhatGitHubWasAskedFor::a_comment_at(
        an_address_a_filing_returned(),
        "a comment",
        &aisdlc_run::who_signs_a_comment::WhoSignsAComment::the_name_a_deployment_used(
            "aisdlc-dev",
        )
        .expect("a name"),
    )
}

/// One offer grounded in one file of one repository.
fn an_offer_about_one_file() -> AWorkAnAgentOffered {
    AWorkAnAgentOffered {
        title: "the offer is never checked against the listing".to_string(),
        what_it_asks_for: "compare every cited path with the listing the run showed".to_string(),
        what_it_concluded_from: vec![EvidenceFile {
            path: "crates/run/src/what_an_agent_offered.rs".to_string(),
            link: "https://github.com/steamnoid/wellbeing-tracker-public/blob/main/\
crates/run/src/what_an_agent_offered.rs"
                .to_string(),
        }],
        it_restates: None,
    }
}

/// The address the run derived, spelled out so the tests do not repeat it.
fn the_address_the_run_derived() -> String {
    "https://github.com/steamnoid/wellbeing-tracker-public/blob/main/\
crates/run/src/what_an_agent_offered.rs"
        .to_string()
}

#[test]
fn the_filing_carries_the_evidence_a_reader_needs_to_check_the_claim() {
    let the_filing =
        the_filing_an_offer_becomes(&the_repository_it_read(), &an_offer_about_one_file());

    assert!(
        the_filing
            .what_is_written()
            .contains(&the_address_the_run_derived()),
        "**because the person who reads this issue is the one who has to check it** — and a \
         confident claim with no way to check it is worse than no claim at all. Grounding this system \
         did in code is invisible to a reader unless the address reaches the page: {}",
        the_filing.what_is_written()
    );
}

#[test]
fn a_filing_carries_no_address_because_it_has_not_been_filed_yet() {
    let the_filing =
        the_filing_an_offer_becomes(&the_repository_it_read(), &an_offer_about_one_file());

    assert!(
        the_filing.is_a_filing_not_yet_made(),
        "**because an address composed before the filing exists is an address to a page that does not \
         exist** — and this repository deleted exactly that once. The address comes back from GitHub \
         afterwards and nowhere else: {the_filing:?}"
    );
}

#[test]
fn a_run_can_tell_a_comment_from_a_filing() {
    let the_filing =
        the_filing_an_offer_becomes(&the_repository_it_read(), &an_offer_about_one_file());

    assert!(
        the_filing.is_a_filing_not_yet_made() && !a_comment().is_a_filing_not_yet_made(),
        "**because one port carrying both has to be able to tell them apart** — and a run that filed \
         where it meant to comment has written a filing nobody asked for: {the_filing:?} and {:?}",
        a_comment()
    );
}

#[test]
fn the_filing_says_who_wrote_it_and_not_a_person() {
    let the_filing =
        the_filing_an_offer_becomes(&the_repository_it_read(), &an_offer_about_one_file());

    assert_eq!(
        the_filing.who_wrote_this(),
        "llm agent",
        "**because a reader who cannot weigh a filing will read it as a person's request** — and the \
         same forgery a comment carried is the same forgery a filing carries, on a page with a title \
         and a body and an author's name: {the_filing:?}"
    );
}

#[test]
fn the_filing_asks_for_what_the_offer_asked_for() {
    let the_filing =
        the_filing_an_offer_becomes(&the_repository_it_read(), &an_offer_about_one_file());

    assert!(
        the_filing
            .what_is_written()
            .contains("compare every cited path with the listing the run showed"),
        "**because a title and a link and no words is a filing nobody can rule on** — and this is the \
         one thing a human actually reads before deciding whether the work is worth doing: {}",
        the_filing.what_is_written()
    );
}
