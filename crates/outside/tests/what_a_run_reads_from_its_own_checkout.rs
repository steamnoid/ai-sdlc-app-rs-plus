//! A run that really cloned, really opened the clone as a project, and really listed it.
//!
//! **Two things proved alone, and never in the same room.** A real checkout was made by a real
//! `git`, and a real project really reads a real directory — but the project was only ever
//! pointed at this repository's own working tree, and the checkout was never listed. **A pair
//! that has never met is two halves of a feature, and a feature is the pair.**
//!
//! **A clone is the one directory where `.git` matters most and must still not be listed.** A
//! clone's history is hundreds of thousands of files, and a listing that included them would
//! bury every real file under them — **and a double could never show this**, because a double
//! holds two files by construction.

use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_outside::the_project_on_disk::TheProjectAsItIsOnDisk;
use aisdlc_outside::whether_anything_stands_there::WhetherAnythingStandsOnDisk;
use aisdlc_run::a_run_asks_what_is_in_its_own_checkout::the_run_asks_what_is_in_its_own_checkout;
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_run_makes_a_checkout::the_run_makes_a_checkout;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// One directory under `target/`, named so no two tests can reach the same one.
fn under_the_target_directory(named: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join(named)
}

/// The request to clone this repository into a directory of this name.
fn the_request_into(named: &str) -> WhatWeTellAProgramToDo {
    WhatWeTellAProgramToDo::clone_the_repository(ARepositoryToClone::of(
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        under_the_target_directory(named),
    ))
}

/// A run that really cloned this repository into a directory of this name.
fn a_run_that_really_cloned(named: &str) -> Option<aisdlc_domain::a_clone::ACloneOnDisk> {
    let _ = std::fs::remove_dir_all(under_the_target_directory(named));
    let mut the_program = TheProgramAsItRuns::as_it_is_on_this_host();
    let the_disk = WhetherAnythingStandsOnDisk;
    the_run_makes_a_checkout(&mut the_program, &the_disk, &the_request_into(named))
        .the_clone()
        .cloned()
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn a_run_lists_the_checkout_it_just_made_and_the_listing_is_of_files_that_are_there() {
    let the_checkout = a_run_that_really_cloned("a-checkout-a-run-listed")
        .expect("a run that cloned nothing has no checkout to list");

    let mut the_project = TheProjectAsItIsOnDisk::at(the_checkout.where_it_lands());
    let listing = the_run_asks_what_is_in_its_own_checkout(&the_checkout, &mut the_project)
        .expect("a project opened at the clone's own place is the run's own");

    assert!(
        !listing.is_empty(),
        "**a checkout that lists nothing is a checkout no agent can be asked about**"
    );
    let every_name_is_there = listing
        .iter()
        .take(20)
        .all(|a_name| the_checkout.where_it_lands().join(a_name).exists());
    assert!(
        every_name_is_there,
        "**and every name in the listing is a real file, because that is what a listing is** — a name that resolves to nothing is a link that looks like a link, which is the one failure a caller cannot see"
    );
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn a_clones_history_is_not_listed_even_though_the_clone_is_all_history() {
    let the_checkout = a_run_that_really_cloned("a-checkout-with-a-real-history")
        .expect("a run that cloned nothing has no history to hide");

    let mut the_project = TheProjectAsItIsOnDisk::at(the_checkout.where_it_lands());
    let listing = the_run_asks_what_is_in_its_own_checkout(&the_checkout, &mut the_project)
        .expect("a project opened at the clone's own place is the run's own");

    let is_the_history = listing.iter().any(|a_name| a_name.starts_with(".git/"));
    let is_a_real_directory = listing.iter().any(|a_name| a_name.starts_with(".github/"));
    assert!(
        !is_the_history,
        "**a clone's own history is hundreds of thousands of files, and listing them buries every real file under them** — a double could never show this, because a double holds two files by construction"
    );
    assert!(
        is_a_real_directory,
        "**and `.github` is a directory of the repository, not a prefix of `.git`** — a check on the first four characters of the first component refuses this directory, and that is the bug a prefix check writes"
    );
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn the_project_opened_at_this_repository_is_not_the_run_s_own_checkout() {
    let the_checkout = a_run_that_really_cloned("a-checkout-this-repo-is-not");

    let mut this_repository = TheProjectAsItIsOnDisk::at(&aisdlc_run::the_project_this_run_is_in());
    let a_checkout = the_checkout.expect("a run that cloned nothing proves nothing here");
    let what_happened = the_run_asks_what_is_in_its_own_checkout(&a_checkout, &mut this_repository);

    assert!(
        what_happened.is_err(),
        "**because this repository's own working tree would answer every question perfectly and about the wrong repository** — and on a real disk, with a real clone, that is the mistake waiting to happen rather than a shape waiting to be caught"
    );
}
