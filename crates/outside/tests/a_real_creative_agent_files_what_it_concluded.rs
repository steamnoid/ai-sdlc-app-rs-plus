//! A real creative run: a real checkout, a real model, real issues, and the works that follow.
//!
//! **Everything else in this repository is a double, and a double is what hid the last two defects
//! this tier exists to find.** A model that was never asked for the three judgements its parser
//! demanded, **and a port that had no implementation for a filing at all** — neither was findable
//! with a stub, and one of them was found by a real model refusing to answer.
//!
//! **It writes to a real repository, and it is ignored, and it says so.** A suite that filed real
//! issues every time it ran is a suite nobody runs, **and a suite nobody runs claims nothing.**

use aisdlc_domain::repository::Repository;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_outside::a_filing_is_read::AFilingIsRead;
use aisdlc_outside::a_filing_reaching_github::AFilingReachingGitHub;
use aisdlc_outside::how_this_deployment_is_set_up::how_this_deployment_is_set_up;
use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_outside::the_project_on_disk::TheProjectAsItIsOnDisk;
use aisdlc_outside::whether_anything_stands_there::WhetherAnythingStandsOnDisk;
use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
use aisdlc_run::a_run_asks_what_is_in_its_own_checkout::the_run_asks_what_is_in_its_own_checkout;
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_run_makes_a_checkout::the_run_makes_a_checkout;
use aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers;

/// The repository this tier files into, **which is a fork this deployment controls.**
fn the_repository_it_files_into() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

/// Where the checkout of this work item lands.
fn the_clone_into() -> ARepositoryToClone {
    ARepositoryToClone::of(
        the_repository_it_files_into(),
        WorkItemId::try_from("OFFER-1").expect("a name"),
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("target")
            .join("a-real-creative-checkout"),
    )
}

/// Whatever the last run of this tier left behind, **and why a tier deletes it.**
///
/// **A work item reuses its own checkout and this tier must not** — a run that finds yesterday's
/// clone has not proved it can make one, **and the subject of this tier is that the clone happens.**
/// It is `target/`, which is build output, **and deleting it is the only destructive thing here.**
fn let_anything_left_of_the_last_run() {
    let where_it_should_be_empty = the_clone_into().where_it_lands().to_path_buf();
    if std::fs::remove_dir_all(&where_it_should_be_empty).is_ok() {
        eprintln!(
            "cleared {} so this tier proves the clone rather than reusing one",
            where_it_should_be_empty.display()
        );
    }
}

/// The files the run will show the model, **read from the checkout the run made itself.**
fn the_listing_a_run_shows(the_program: &mut TheProgramAsItRuns) -> Vec<String> {
    let the_checkout = the_run_makes_a_checkout(
        the_program,
        &WhetherAnythingStandsOnDisk,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone_into()),
    );
    let the_clone = the_checkout
        .the_clone()
        .expect("a real repository either clones, or this tier printed why the clone failed");
    let mut the_project = TheProjectAsItIsOnDisk::at(the_clone.where_it_lands());
    the_run_asks_what_is_in_its_own_checkout(the_clone, &mut the_project)
        .expect("the listing is of the checkout the run made itself")
}

/// Where this deployment's model is, **and the reason printed when it named none.**
fn where_this_deployments_model_is() -> Option<WhereAModelIs> {
    let what_the_process_was_told = TheEnvironment::reading_what_the_process_was_told();
    let named = WhereAModelIs::where_a_deployment_put_its_model(
        what_the_process_was_told.the_variable_a_deployment_set("LLM_ENDPOINT"),
        what_the_process_was_told.the_variable_a_deployment_set("LLM_MODEL"),
    );
    if let Err(why_it_named_none) = &named {
        eprintln!("SKIPPED: {why_it_named_none}");
    }
    named.ok()
}

/// The flag as this deployment set it, **or the reason printed when nobody asked for one.**
fn whether_this_deployment_asked_for_an_agent(
    what_the_process_was_told: &TheEnvironment,
) -> WhetherAnAgentOffers {
    let how_it_is_set_up = how_this_deployment_is_set_up(what_the_process_was_told);
    if how_it_is_set_up.may_an_agent_offer() != WhetherAnAgentOffers::Yes {
        eprintln!("SKIPPED: AISDLC_AN_AGENT_OFFERS is not 'yes' here, so nothing was filed");
    }
    how_it_is_set_up.may_an_agent_offer()
}

/// This deployment's program, **or the reason there is nothing to file with.**
fn a_deployment_that_may_file(
    what_the_process_was_told: &TheEnvironment,
) -> Option<TheProgramAsItRuns> {
    let how_it_is_set_up = how_this_deployment_is_set_up(what_the_process_was_told);
    let Some(the_credential) = how_it_is_set_up.the_credential().cloned() else {
        eprintln!("SKIPPED: the flag is on and OPENAI_API_KEY is not set, so no model was asked");
        return None;
    };
    Some(
        TheProgramAsItRuns::as_it_is_on_this_host(Some(the_credential))
            .with_what_a_deployment_named(what_the_process_was_told),
    )
}

/// The whole chain, once, on a real repository and a real model.
fn the_whole_creative_run(
    the_program: &TheProgramAsItRuns,
    the_listing: &[String],
    where_the_model_is: &WhereAModelIs,
    may_an_agent_offer: WhetherAnAgentOffers,
) -> Result<Vec<WhatTheRunWillWorkOn>, String> {
    aisdlc_run::a_creative_run_files_what_it_would_file::a_creative_run_files_what_it_would_file(
        the_program,
        &mut AFilingReachingGitHub,
        &mut AFilingIsRead,
        &the_repository_it_files_into(),
        the_listing,
        where_the_model_is,
        may_an_agent_offer,
    )
    .map_err(|the_flag| the_flag.0)
}

#[test]
#[ignore = "clones a real repository, asks a real model, files real issues and reads them back"]
fn a_real_creative_run_files_what_it_concluded_and_works_on_it() {
    let what_the_process_was_told = TheEnvironment::reading_what_the_process_was_told();
    if whether_this_deployment_asked_for_an_agent(&what_the_process_was_told)
        != WhetherAnAgentOffers::Yes
    {
        return;
    }
    let Some(mut the_program) = a_deployment_that_may_file(&what_the_process_was_told) else {
        return;
    };
    let Some(where_the_model_is) = where_this_deployments_model_is() else {
        return;
    };
    let_anything_left_of_the_last_run();
    let the_listing = the_listing_a_run_shows(&mut the_program);

    let what_the_run_will_work_on =
        what_the_run_files_and_works_on(&the_program, &the_listing, &where_the_model_is);

    every_filing_went_somewhere_this_deployment_controls(&what_the_run_will_work_on);
}

/// The chain from the listing to the works, **and the run's own words when it refuses.**
fn what_the_run_files_and_works_on(
    the_program: &TheProgramAsItRuns,
    the_listing: &[String],
    where_the_model_is: &WhereAModelIs,
) -> Vec<WhatTheRunWillWorkOn> {
    the_whole_creative_run(
        the_program,
        the_listing,
        where_the_model_is,
        WhetherAnAgentOffers::Yes,
    )
    .unwrap_or_else(|why_it_refused| panic!("the real run refused, and said: {why_it_refused}"))
}

/// Every work went where this deployment controls, **at an address GitHub composed, and answers the
/// filing read back rather than what the run remembers writing.**
fn every_filing_went_somewhere_this_deployment_controls(
    what_the_run_will_work_on: &[WhatTheRunWillWorkOn],
) {
    for one_work in what_the_run_will_work_on {
        let where_it_answers = one_work
            .what_it_answers()
            .where_it_was_filed()
            .expect("a filing was filed somewhere")
            .as_str();
        eprintln!("filed and worked on: {where_it_answers}");
        assert!(
            where_it_answers
                .starts_with("https://github.com/steamnoid/wellbeing-tracker-public/issues/"),
            "**because the address GitHub composed is the only address** — one this system assembled \
             from a repository and a number resolved to a page that did not exist: {where_it_answers}"
        );
    }
}
