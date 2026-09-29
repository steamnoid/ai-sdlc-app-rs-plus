//! The whole run, really composed, on a real disk, with a real `git`.
//!
//! **Every part of this was proved alone and nobody proved they fit together.** `git` really
//! cloned on its own, the real disk really answered on its own, and the use case really asked in
//! the right order with doubles — and a node taking different arguments from the function under
//! it is a defect that walks the whole run before anyone hears about it.
//!
//! **The test is the composition root, and that is said here rather than implied.** A repository
//! that names a concrete organisation in a test and a library that does not are the same decision
//! taken twice, and this one is the only place in the crate where the concrete types are named.
//!
//! **The first test reaches no network at all, and that is the point of it.** The destination is
//! asked about before `git` is started, so a run that finds the directory taken is refused on a
//! machine with no internet — **and a refusal that needed the network to happen would not be a
//! decision, it would be a finding.**

use aisdlc_domain::why_cloning_the_repository_failed::WhichWayItFailed;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_outside::whether_anything_stands_there::WhetherAnythingStandsOnDisk;
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_run_makes_a_checkout::the_run_makes_a_checkout;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A directory under `target/`, which git already ignores and a listing already skips.
fn under_the_target_directory(named: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join(named)
}

/// A directory holding something already, made on purpose and not by a run.
fn a_directory_somebody_else_holds(named: &str) -> std::path::PathBuf {
    let where_it_is = under_the_target_directory(named);
    let _ = std::fs::remove_dir_all(&where_it_is);
    std::fs::create_dir_all(&where_it_is).expect("a directory can be made under the target");
    std::fs::write(where_it_is.join("something-in-it"), "held").expect("a file can be written");
    where_it_is
}

/// The checkout a finished run made, or a panic naming what it refused instead.
fn the_checkout_of(
    how_it_went: &aisdlc_run::the_run_makes_a_checkout::HowCloningWent,
) -> &aisdlc_domain::a_clone::ACloneOnDisk {
    how_it_went.the_clone().unwrap_or_else(|| {
        panic!(
            "the whole run made no checkout at all: {:?}",
            how_it_went.why_it_failed()
        )
    })
}

/// The request to clone a repository nobody ever made, into a directory of this name.
fn a_request_for_one_nobody_made(named: &str) -> WhatWeTellAProgramToDo {
    WhatWeTellAProgramToDo::clone_the_repository(ARepositoryToClone::of(
        aisdlc_domain::repository::Repository::try_from("steamnoid/a-repository-nobody-ever-made")
            .expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-6").expect("a name"),
        under_the_target_directory(named),
    ))
}

/// The request this repository is cloned from, into a directory of this name.
fn the_request_into(named: &str) -> WhatWeTellAProgramToDo {
    WhatWeTellAProgramToDo::clone_the_repository(ARepositoryToClone::of(
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        under_the_target_directory(named),
    ))
}

#[test]
fn a_destination_somebody_else_holds_is_refused_without_reaching_the_network() {
    let _ = a_directory_somebody_else_holds("a-destination-somebody-else-holds");
    let mut the_program = TheProgramAsItRuns::as_it_is_on_this_host();
    let the_disk = WhetherAnythingStandsOnDisk;

    let how_it_went = the_run_makes_a_checkout(
        &mut the_program,
        &the_disk,
        &the_request_into("a-destination-somebody-else-holds"),
    );

    assert_eq!(
        how_it_went.which_way_it_failed(),
        Some(WhichWayItFailed::TheDestinationAlreadyHoldsSomething),
        "**because the real disk really holds something there and the real program really was never asked** — this runs on a machine with no network at all, and a guard that could only refuse after starting `git` would be a guard that needs the network to protect a directory on the disk"
    );
    assert!(
        how_it_went.the_clone().is_none(),
        "**and no checkout came back**, because a run that reports a refusal and a clone at once has said two things that cannot both be true"
    );
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn the_whole_run_clones_for_real_and_reads_the_commit_and_the_ref_back() {
    let named = "the-whole-clone";
    let _ = std::fs::remove_dir_all(under_the_target_directory(named));
    let mut the_program = TheProgramAsItRuns::as_it_is_on_this_host();
    let the_disk = WhetherAnythingStandsOnDisk;

    let how_it_went =
        the_run_makes_a_checkout(&mut the_program, &the_disk, &the_request_into(named));

    let the_clone = the_checkout_of(&how_it_went);
    assert!(
        the_clone.the_commit().as_str().len() >= 7,
        "**because a commit is what the clone is standing on, and a run that finished without one has nothing to report a diff against**"
    );
    assert!(
        !the_clone.ref_name().as_str().is_empty(),
        "**and a ref, which may be `HEAD`** — a checkout on no branch still has to say what it is on"
    );
    assert!(
        the_clone.where_it_lands().join(".git").is_dir(),
        "**and a real `.git` on the real disk**, which is the whole history and not a shallow one"
    );
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn a_repository_nobody_ever_made_is_refused_with_gits_own_words() {
    let named = "a-repository-nobody-ever-made";
    let _ = std::fs::remove_dir_all(under_the_target_directory(named));
    let mut the_program = TheProgramAsItRuns::as_it_is_on_this_host();
    let the_disk = WhetherAnythingStandsOnDisk;

    let how_it_went = the_run_makes_a_checkout(
        &mut the_program,
        &the_disk,
        &a_request_for_one_nobody_made(named),
    );

    assert_eq!(
        how_it_went.which_way_it_failed(),
        Some(WhichWayItFailed::NoSuchRepository),
        "**because somebody named a repository nobody ever made, and the whole run has to say so by name** — a boolean here would have thrown away the only thing the caller can act on"
    );
    assert!(
        how_it_went
            .why_it_failed()
            .expect("a refusal has a reason")
            .what_git_said()
            .contains("a-repository-nobody-ever-made"),
        "**and it failed for this reason rather than for a directory left over** — a stale directory fails too, says something too, and would have let this test pass about the wrong thing"
    );
}
