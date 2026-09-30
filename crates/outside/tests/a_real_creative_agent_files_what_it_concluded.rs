//! A real creative agent: a real checkout, a real model, and real issues in a real repository.
//!
//! **Everything else in this repository is a double, and a double is what hid the last two defects
//! this tier exists to find.** A model that cited a file it invented, and a port that could not return
//! an address, **were both provable with a stub and neither was found that way** — one because the
//! stub answered with a shape no endpoint returns, and one because nothing had compared the seam with
//! the thing it stands for.
//!
//! **It writes to a real repository, and it is ignored, and it says so.** A run that filed real issues
//! every time the suite ran is a suite nobody runs, **and a suite nobody runs is a suite that claims
//! nothing.**

use aisdlc_domain::repository::Repository;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_outside::a_filing_reaching_github::AFilingReachingGitHub;
use aisdlc_outside::how_this_deployment_is_set_up::how_this_deployment_is_set_up;
use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_outside::the_project_on_disk::TheProjectAsItIsOnDisk;
use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
use aisdlc_run::a_run_asks_what_is_in_its_own_checkout::the_run_asks_what_is_in_its_own_checkout;
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_run_makes_a_checkout::the_run_makes_a_checkout;
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

/// This deployment's program, **or the reason there is nothing to file with.**
///
/// **Two reasons and both are printed**, because a deployment that will not file and a deployment
/// that cannot reach a model are different facts, **and a tier that skipped without saying which is a
/// green run that called nothing.**
fn a_deployment_that_may_file() -> Option<TheProgramAsItRuns> {
    let what_the_process_was_told = TheEnvironment::reading_what_the_process_was_told();
    let how_it_is_set_up = how_this_deployment_is_set_up(&what_the_process_was_told);
    if how_it_is_set_up.may_an_agent_offer() != WhetherAnAgentOffers::Yes {
        eprintln!("SKIPPED: AISDLC_AN_AGENT_OFFERS is not 'yes' here, so nothing was filed");
        return None;
    }
    let Some(the_credential) = how_it_is_set_up.the_credential().cloned() else {
        eprintln!("SKIPPED: the flag is on and OPENAI_API_KEY is not set, so no model was asked");
        return None;
    };
    Some(
        TheProgramAsItRuns::as_it_is_on_this_host(Some(the_credential))
            .with_what_a_deployment_named(&what_the_process_was_told),
    )
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
use aisdlc_run::whether_anything_stands_there::WhetherAnythingStandsThere;

/// A directory nothing has been put in yet, **and a real clone needs one that is not already there.**
struct NothingStandsThereYet;

impl WhetherAnythingStandsThere for NothingStandsThereYet {
    fn does_anything_stand_at(&self, _a_directory: &std::path::Path) -> bool {
        false
    }
}

/// The files the run will show the model, **read from the checkout the run made itself.**
fn the_listing_a_run_shows(the_program: &mut TheProgramAsItRuns) -> Vec<String> {
    let the_checkout = the_run_makes_a_checkout(
        the_program,
        &NothingStandsThereYet,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone_into()),
    );
    let the_clone = the_checkout
        .the_clone()
        .expect("a real repository either clones, or this tier printed why the clone failed");
    let mut the_project = TheProjectAsItIsOnDisk::at(the_clone.where_it_lands());
    the_run_asks_what_is_in_its_own_checkout(the_clone, &mut the_project)
        .expect("the listing is of the checkout the run made itself")
}

#[test]
#[ignore = "clones a real repository, asks a real model and files real issues in it"]
fn a_real_creative_agent_files_what_it_concluded_and_knows_where() {
    let Some(mut the_program) = a_deployment_that_may_file() else {
        return;
    };
    let Some(where_the_model_is) = where_this_deployments_model_is() else {
        return;
    };
    let the_listing = the_listing_a_run_shows(&mut the_program);
    assert!(
        !the_listing.is_empty(),
        "**because a model asked what to improve and shown an empty listing will invent a project** — \
         and an invented project is a filing about a repository that does not exist"
    );

    let what_it_filed =
        the_run_files_what_it_concluded(&the_program, &the_listing, &where_the_model_is);

    every_filing_went_somewhere_this_deployment_controls(&what_it_filed);
}

/// Every filing went where this deployment controls, **at an address GitHub composed.**
fn every_filing_went_somewhere_this_deployment_controls(
    what_it_filed: &[aisdlc_domain::commissioned_request::AWhereItWasFiled],
) {
    for where_it_went in what_it_filed {
        eprintln!("filed: {}", where_it_went.as_str());
    }
    assert!(
        what_it_filed.iter().all(|where_it_went| {
            where_it_went
                .as_str()
                .starts_with("https://github.com/steamnoid/wellbeing-tracker-public/issues/")
        }),
        "**because the address GitHub composed is the only address** — one this system assembled \
         from a repository and a number resolved to a page that did not exist: {what_it_filed:?}"
    );
}

/// The filings this run makes, **and the addresses it comes back with.**
fn the_run_files_what_it_concluded(
    the_program: &TheProgramAsItRuns,
    the_listing: &[String],
    where_the_model_is: &WhereAModelIs,
) -> Vec<aisdlc_domain::commissioned_request::AWhereItWasFiled> {
    aisdlc_run::a_creative_agent_files_what_it_concluded::a_creative_agent_files_what_it_concluded(
        the_program,
        &mut AFilingReachingGitHub,
        &the_repository_it_files_into(),
        the_listing,
        where_the_model_is,
    )
    .unwrap_or_else(|a_refusal| panic!("the real run refused to file, and said: {:?}", a_refusal.0))
}
