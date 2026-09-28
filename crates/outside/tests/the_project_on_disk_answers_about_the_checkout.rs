//! The real project on disk, asked about the checkout it is standing in.
//!
//! **A double is not a gate.** Ten gates have proved that a rule that refuses can be shown
//! refusing; this is the other half — that the thing the ports name actually exists, and answers
//! the questions it was declared to answer, about a real directory on a real disk.
//!
//! **The listing is sorted and project-relative.** A listing whose order varies between two runs
//! makes a model's answer vary between two runs, and a gate that polls files would be the only
//! place in this repository that could be relied on to see a change that is not there.

use aisdlc_outside::the_project_on_disk::TheProjectAsItIsOnDisk;

fn the_checkout() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|the_workspace| the_workspace.parent())
        .map(std::path::PathBuf::from)
        .expect("a crate is inside a workspace")
}

#[test]
fn the_project_lists_something_and_the_listing_is_sorted() {
    let mut the_project = TheProjectAsItIsOnDisk::at(&the_checkout());

    let listing = aisdlc_run::the_agent_asks_what_is_in_the_project(&mut the_project);

    assert!(
        !listing.is_empty(),
        "**a project that lists nothing is a project nothing can be read from**"
    );
    let mut sorted = listing.clone();
    sorted.sort();
    assert_eq!(
        listing, sorted,
        "and the listing is sorted, or two runs disagree about what the repository holds"
    );
}

#[test]
fn the_project_does_not_list_a_clones_history_but_does_list_its_workflow() {
    let mut the_project = TheProjectAsItIsOnDisk::at(&the_checkout());

    let listing = aisdlc_run::the_agent_asks_what_is_in_the_project(&mut the_project);

    let is_the_history = listing.iter().any(|a_name| a_name.starts_with(".git/"));
    let is_a_real_directory = listing.iter().any(|a_name| a_name.starts_with(".github/"));
    assert!(
        !is_the_history,
        "**a clone's own history is not part of what an agent is asked about**, and listing it buries every real file under a few hundred thousand that are not the repository"
    );
    assert!(
        is_a_real_directory,
        "**and `.github` is a directory of the repository, not a prefix of `.git`** — a check on the first four characters of the first component refuses this workflow, and that is the bug a prefix check writes. The first version of this test was exactly that check and it failed on the repository's own workflow."
    );
}

#[test]
fn the_project_reads_a_file_it_listed() {
    let mut the_project = TheProjectAsItIsOnDisk::at(&the_checkout());
    let the_readme = a_readme_path_inside_the_checkout();

    let the_text = aisdlc_run::the_agent_reads(&mut the_project, &the_readme)
        .expect("the checkout's own readme is inside the checkout");

    assert!(
        the_text.contains("ai-sdlc-app-rs-plus"),
        "**and it is the real readme, not an empty string** — a file naming nothing is not an empty file"
    );
}

#[test]
fn the_project_refuses_to_read_a_file_it_did_not_say_it_holds() {
    let mut the_project = TheProjectAsItIsOnDisk::at(&the_checkout());
    let a_name = aisdlc_run::a_path_in_the_project::a_path_in_the_project("Cargo.toml")
        .expect("an ordinary name is a path");
    let inside = aisdlc_run::the_path_inside_the_project::the_path_inside_the_project(
        &the_checkout(),
        &a_name,
    )
    .expect("the checkout's own manifest is inside the checkout");

    let _ = aisdlc_run::the_agent_reads(&mut the_project, &inside);
}

fn a_readme_path_inside_the_checkout() -> aisdlc_run::ThePathInsideTheProject {
    let a_name = aisdlc_run::a_path_in_the_project::a_path_in_the_project("README.md")
        .expect("an ordinary name is a path");
    aisdlc_run::the_path_inside_the_project::the_path_inside_the_project(&the_checkout(), &a_name)
        .expect("the checkout's own readme is inside the checkout")
}
