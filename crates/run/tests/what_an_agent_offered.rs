//! What an agent offers a human: work it concluded is worth filing, and the files it concluded it from.
//!
//! **An offer that names no file is a guess, and a guess that gets filed looks exactly like a
//! finding.** A discovery report describes a repository that exists, so an empty one is a fault; an
//! offer asks for a change to something that does not exist yet, **and asking for a change to a file
//! that is not there is indistinguishable, in an issue, from asking for a real one.** That is the
//! whole reason the evidence is not optional here.

use aisdlc_run::what_an_agent_judged::WhatAnAgentJudged;
use aisdlc_run::what_an_agent_offered::{AWorkAnAgentOffered, WhatAnAgentOffered};
use aisdlc_run::what_discovery_concluded::EvidenceFile;

/// What an agent said about its own reading, for a test that is not about that.
fn what_it_said_about_its_own_work() -> WhatAnAgentJudged {
    WhatAnAgentJudged::what_an_agent_said(
        "a test directory the suite never collects",
        "the run collects tests/ and not the e2e directory",
        "file the offer and let the analyst judge it",
    )
    .expect("three judgements said")
}

/// The repository whose files an offer is concluded from.
fn a_repository_it_read() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// One offer grounded in the evidence behind it, **and `None` for a filing it does not restate.**
fn an_offer_with(what_it_concluded_from: Vec<EvidenceFile>) -> AWorkAnAgentOffered {
    AWorkAnAgentOffered {
        title: "the tests every run collects".to_string(),
        what_it_asks_for: "the suite collects tests/ and leaves the e2e directory alone"
            .to_string(),
        what_it_concluded_from,
        it_restates: None,
    }
}

#[test]
fn an_offer_that_names_no_file_is_refused() {
    let one_offer_with_nothing_behind_it = vec![an_offer_with(Vec::new())];

    let the_offer = WhatAnAgentOffered::an_agent_offered(
        one_offer_with_nothing_behind_it,
        what_it_said_about_its_own_work(),
    );

    assert!(
        the_offer.is_err(),
        "**because an issue asking for a change to a file that is not there reads exactly like an \
         issue asking for a real one** — and a human ruling on a gate cannot tell them apart by \
         looking, which is already why a discovery report that says nothing at all is refused. One \
         offer with nothing behind it is the fault; no offers at all is a different thing, and the \
         two must not be the same test: {the_offer:?}"
    );
}

#[test]
fn an_offer_that_names_where_it_concluded_from_is_kept_whole() {
    let evidence = WhatAnAgentOffered::the_evidence_it_concluded_from(
        &a_repository_it_read(),
        &["crates/run/src/what_an_agent_offered.rs"],
    )
    .expect("a file of the repository");

    let the_offer = WhatAnAgentOffered::an_agent_offered(
        vec![an_offer_with(evidence)],
        what_it_said_about_its_own_work(),
    );

    let what_was_offered = the_offer.expect("it named a file it concluded from");
    assert_eq!(
        what_was_offered.offers.len(),
        1,
        "**because one real proposal is worth filing, and the address behind it is derived from the \
         repository rather than written by the model:** {what_was_offered:?}"
    );
}

#[test]
fn an_agent_that_found_nothing_offers_nothing_and_still_says_so() {
    let the_offer = WhatAnAgentOffered::an_agent_offered(vec![], what_it_said_about_its_own_work());

    let what_was_offered = the_offer.expect("no offers is not a fault");
    assert!(
        what_was_offered.offers.is_empty(),
        "**because a creative agent that files nothing has still told the truth** — the fault is a \
         filing that names no file, and an agent that filed none has committed that fault zero times. \
         A run that finds nothing to do is a run that worked: {what_was_offered:?}"
    );
}
