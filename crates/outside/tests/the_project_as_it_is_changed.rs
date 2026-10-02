//! A file written into a real clone on a real disk, **and what the real filesystem answers.**
//!
//! **A write proved by a double is a write proved by nothing.** The point of this file is the
//! half the use case's test cannot reach: **that bytes reach a directory and that a directory
//! this system was not given is never written to.**
//!
//! **`..` and an absolute path are refused, and the refusal happens before any byte is written** —
//! because a repository this system writes into is somebody else's working tree, **and a path
//! that steps out of the clone is a file in somebody else's project.**
//!
//! **No credential, no network and no container**, because a local filesystem is the one boundary
//! that needs none of the three — **and that is why the write port is separate from the one that
//! runs a repository's own tests.**

use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_outside::the_project_as_it_is_changed::TheProjectAsItIsChanged;
use aisdlc_run::a_change_is_made_in_the_clone::AFileToWrite;
use aisdlc_run::a_record_of_a_change::AChangeIsRecorded;

/// A directory under the target directory, **and nothing of it left from a previous run.**
fn a_place_named(named: &str) -> std::path::PathBuf {
    let under_the_target = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target")
        .join(named);
    let _ = std::fs::remove_dir_all(&under_the_target);
    std::fs::create_dir_all(&under_the_target).expect("a directory this machine will hold");
    under_the_target
}

/// A clone standing at that place, **and a project opened on it.**
fn a_project_over(a_place: &std::path::Path) -> TheProjectAsItIsChanged {
    let the_repository =
        aisdlc_domain::repository::Repository::try_from("steamnoid/one").expect("two halves");
    let of_which_clone = aisdlc_run::the_repository_to_clone_into::ARepositoryToClone::of(
        the_repository,
        WorkItemId::try_from("WELLBEINGT-42").expect("a name"),
        a_place.to_path_buf(),
    );
    TheProjectAsItIsChanged::the_project_over_the_clone_at(&of_which_clone)
}

#[test]
fn what_a_file_says_is_what_the_file_says_afterwards() {
    let the_place = a_place_named("a-write-really-lands");
    let mut the_project = a_project_over(&the_place);

    the_project
        .one_file_is_written("AGENTS.md", "the gate a document sent the wrong way first")
        .expect("a file inside the clone is written");

    let what_the_file_says = std::fs::read_to_string(the_place.join("AGENTS.md"))
        .expect("the file this system wrote is on the disk");
    assert_eq!(
        what_the_file_says,
        "the gate a document sent the wrong way first",
        "**because a write proved by a double is a write proved by nothing** — the bytes have to \
         reach a directory, and this is the only assertion in this cycle that says they did. The \
         place was {}",
        the_place.display()
    );
}

#[test]
fn a_directory_the_write_lands_in_is_made_because_the_file_had_none() {
    let the_place = a_place_named("a-write-makes-its-directory");
    let mut the_project = a_project_over(&the_place);

    the_project
        .one_file_is_written("tests/a_gate_and_its_breaches.rs", "a test")
        .expect("a test inside the clone is written");

    let what_is_there = std::fs::read_to_string(the_place.join("tests/a_gate_and_its_breaches.rs"))
        .expect("the test the plan named is on the disk");
    assert_eq!(
        what_is_there,
        "a test",
        "**because a plan names a test in a directory that may not exist yet, and a run that \
         refused the plan for a missing directory would be refusing it for the thing the plan is \
         for.** The place was {}",
        the_place.display()
    );
}

#[test]
fn a_path_stepping_out_of_the_clone_is_refused_and_nothing_is_written() {
    let the_place = a_place_named("a-path-may-not-step-out");
    let mut the_project = a_project_over(&the_place);

    let the_refusal = the_project.one_file_is_written("../a-file-in-somebody-elses-project", "no");

    let nothing_was_written = !the_place
        .parent()
        .expect("the place has a parent")
        .join("a-file-in-somebody-elses-project")
        .exists();
    assert!(
        the_refusal.is_err() && nothing_was_written,
        "**because a repository this system writes into is somebody else's working tree** — and a \
         path that steps out of the clone is a file in somebody else's project. What was refused: \
         {the_refusal:?} and nothing was written: {nothing_was_written}"
    );
}

#[test]
fn what_a_file_already_says_is_what_it_says_before_anything_is_written() {
    let the_place = a_place_named("a-file-already-there");
    std::fs::write(the_place.join("AGENTS.md"), "what it already said").expect("a file to hold");
    let the_project = a_project_over(&the_place);

    let what_it_holds = the_project.what_it_holds_of("AGENTS.md");

    assert_eq!(
        what_it_holds.as_deref(),
        Some("what it already said"),
        "**because the refusal for a write that changes nothing reads what is there *before* it \
         writes** — and a reader that read afterwards would compare the file with itself and find \
         every write a change."
    );
}

#[test]
fn a_file_the_clone_never_had_is_held_by_nobody() {
    let the_place = a_place_named("a-file-never-there");
    let the_project = a_project_over(&the_place);

    let what_it_holds = the_project.what_it_holds_of("a-file-never-written");

    assert_eq!(
        what_it_holds, None,
        "**because `None` and not an empty string** — a file nobody wrote says nothing, and a file \
         written empty says nothing **and is a change**, so the two answers cannot be the same \
         value."
    );
}

#[test]
fn a_string_that_is_no_path_at_all_never_reaches_the_disk() {
    let the_refusal = AFileToWrite::the_file_at("AGENTS.md//a-name", "no");

    assert!(
        the_refusal.is_err(),
        "**because the shape rule is applied before the disk is touched, and a path refused here \
         is refused where a person can be told** — not by the operating system, and not by a \
         review of a diff three days later. What was refused: {the_refusal:?}"
    );
}
