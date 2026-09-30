//! An offer grounded in the files the run was shown, and the refusal when it was not.
//!
//! **The address being derived is not enough, and this is the rule that closes the gap.** A path
//! that looks like a path can still name a file the repository does not have, **and a model that
//! invents `src/main.rs` in a project with none produces an offer that reads exactly like a real
//! one.** The run has the list of files it actually showed the model, so an offer may cite nothing
//! else.

use aisdlc_run::what_an_agent_judged::WhatAnAgentJudged;
use aisdlc_run::what_an_agent_offered::{WhatAModelSaidItWouldFile, WhatAnAgentOffered};

/// The files a run really did show the model, from its own checkout.
fn what_the_run_actually_saw() -> Vec<String> {
    vec![
        "crates/run/src/what_an_agent_offered.rs".to_string(),
        "crates/run/tests/offer_grounded_in_what_the_run_saw.rs".to_string(),
    ]
}

/// The repository those files are in.
fn the_repository_it_read() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// What the agent said about its own reading.
fn what_it_said_about_its_own_work() -> WhatAnAgentJudged {
    WhatAnAgentJudged::what_an_agent_said(
        "an offer that cites a file this run never showed the model",
        "whether the repository has an entry point at all",
        "file only what the listing names",
    )
    .expect("three judgements said")
}

#[test]
fn an_offer_citing_a_file_the_run_never_showed_is_refused() {
    let what_the_model_said = vec![WhatAModelSaidItWouldFile {
        title: "an entry point to start from".to_string(),
        what_it_asks_for: "add src/main.rs so the project has somewhere to begin".to_string(),
        paths: vec!["src/main.rs".to_string()],
    }];

    let the_offers = WhatAnAgentOffered::the_offers_a_model_offered(
        &the_repository_it_read(),
        &what_the_run_actually_saw(),
        what_the_model_said,
        what_it_said_about_its_own_work(),
    );

    assert!(
        the_offers.is_err(),
        "**because a path that looks like a path can still name a file that is not there** — and a \
         model that invents `src/main.rs` in a project with none produces an offer that reads \
         exactly like a real one. The run showed the model a listing and holds it: {the_offers:?}"
    );
}

#[test]
fn an_offer_citing_what_the_run_showed_is_kept_with_its_derived_address() {
    let what_the_model_said = vec![WhatAModelSaidItWouldFile {
        title: "the offers are not read by any test".to_string(),
        what_it_asks_for: "the module deciding what an agent offered has no test naming its \
                            refusal"
            .to_string(),
        paths: vec!["crates/run/src/what_an_agent_offered.rs".to_string()],
    }];

    let the_offers = WhatAnAgentOffered::the_offers_a_model_offered(
        &the_repository_it_read(),
        &what_the_run_actually_saw(),
        what_the_model_said,
        what_it_said_about_its_own_work(),
    );

    let what_was_offered = the_offers.expect("it cited a file the run showed");
    let every_address_behind_them: Vec<String> = what_was_offered
        .offers
        .iter()
        .flat_map(|one_offer| one_offer.what_it_concluded_from.iter())
        .map(|one_file| one_file.link.clone())
        .collect();

    assert_eq!(
        every_address_behind_them,
        vec![String::from(
            "https://github.com/steamnoid/wellbeing-tracker-public/blob/main/\
crates/run/src/what_an_agent_offered.rs"
        )],
        "**because the address follows from the repository and the path, and is never written by the \
         model** — an address a model may invent is worse than none, and every file behind every \
         offer is compared as a whole because one address built from a path that was not there is \
         the whole fault: {what_was_offered:?}"
    );
}

#[test]
fn a_model_that_said_nothing_wrote_no_offer_at_all() {
    let the_offers = WhatAnAgentOffered::the_offers_a_model_offered(
        &the_repository_it_read(),
        &what_the_run_actually_saw(),
        Vec::new(),
        what_it_said_about_its_own_work(),
    );

    let what_was_offered = the_offers.expect("nothing said is nothing filed");
    assert!(
        what_was_offered.offers.is_empty(),
        "**because a model that said nothing has filed nothing and that is not a fault** — the fault \
         is an offer that names a file that is not there, and this committed it zero times: \
         {what_was_offered:?}"
    );
}
