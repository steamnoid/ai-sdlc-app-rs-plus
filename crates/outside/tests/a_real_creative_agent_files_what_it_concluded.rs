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

/// Where a deployment said its model is, or the reason there is nothing to ask.
fn where_a_deployment_put_its_model(
    what_it_was_told: &TheEnvironment,
) -> Result<WhereAModelIs, String> {
    WhereAModelIs::where_a_deployment_put_its_model(
        what_it_was_told.the_variable_a_deployment_set("LLM_ENDPOINT"),
        what_it_was_told.the_variable_a_deployment_set("LLM_MODEL"),
    )
}

/// This deployment, **or the reason there is nothing to file.**
///
/// **Every reason is printed and there are two of them**, because a deployment that will not file and
/// a deployment that cannot reach a model are different facts, **and a tier that skipped without
/// saying which is a green run that called nothing.**
fn a_deployment_that_may_file(
    how_it_is_set_up: &aisdlc_run::how_this_deployment_is_set_up::HowThisDeploymentIsSetUp,
) -> Option<ACredential> {
    if how_it_is_set_up.may_an_agent_offer() != WhetherAnAgentOffers::Yes {
        eprintln!("SKIPPED: AISDLC_AN_AGENT_OFFERS is not 'yes' here, so nothing was filed");
        return None;
    }
    let the_credential = how_it_is_set_up.the_credential().cloned();
    if the_credential.is_none() {
        eprintln!("SKIPPED: the flag is on and OPENAI_API_KEY is not set, so no model was asked");
    }
    the_credential
}

use aisdlc_run::a_credential::ACredential;
use aisdlc_run::whether_anything_stands_there::WhetherAnythingStandsThere;

/// A directory nothing has been put in yet, **and a real clone needs one that is not already there.**
struct NothingStandsThereYet;

impl WhetherAnythingStandsThere for NothingStandsThereYet {
    fn does_anything_stand_at(&self, _a_directory: &std::path::Path) -> bool {
        false
    }
}

#[test]
#[ignore = "clones a real repository, asks a real model and files real issues in it"]
fn a_real_creative_agent_files_what_it_concluded_and_knows_where() {
    let what_the_process_was_told = TheEnvironment::reading_what_the_process_was_told();
    let how_it_is_set_up = how_this_deployment_is_set_up(&what_the_process_was_told);
    let Some(the_credential) = a_deployment_that_may_file(&how_it_is_set_up) else {
        return;
    };
    let named = where_a_deployment_put_its_model(&what_the_process_was_told);
    let Ok(where_the_model_is) = named.as_ref() else {
        eprintln!(
            "SKIPPED: {}",
            named.expect_err("none was found, so this ran")
        );
        return;
    };

    let mut the_program = TheProgramAsItRuns::as_it_is_on_this_host(Some(the_credential))
        .with_what_a_deployment_named(&what_the_process_was_told);
    let the_checkout = the_run_makes_a_checkout(
        &mut the_program,
        &NothingStandsThereYet,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone_into()),
    );
    let the_clone = the_checkout
        .the_clone()
        .expect("a real repository either clones or this tier says why it did not");

    let mut the_project = TheProjectAsItIsOnDisk::at(the_clone.where_it_lands());
    let the_listing = the_run_asks_what_is_in_its_own_checkout(the_clone, &mut the_project)
        .expect("the listing is of the checkout the run made itself");
    assert!(
        !the_listing.is_empty(),
        "**because a model asked what to improve and shown an empty listing will invent a project** — \
         and an invented project is a filing about a repository that does not exist"
    );

    let what_it_filed = aisdlc_run::a_creative_agent_files_what_it_concluded::
        a_creative_agent_files_what_it_concluded(
            &the_program,
            &mut AFilingReachingGitHub,
            &the_repository_it_files_into(),
            &the_listing,
            where_the_model_is,
        )
        .unwrap_or_else(|could_not| panic!("a real run could not file: {:?}", could_not.0));

    for where_it_went in &what_it_filed {
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
