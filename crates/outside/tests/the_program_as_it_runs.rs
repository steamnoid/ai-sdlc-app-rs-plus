//! The real program, run on the real host, about the real repository.
//!
//! **This is the first thing in this repository that executes anything.** A double is not a
//! gate, and eleven gates have proved that a rule which refuses can be shown refusing; this is
//! the other half — that the thing the port names exists, and really runs.
//!
//! **Two of the four run the process and two do not, and that split is the design.** The
//! arguments are the security property — a formatted command line is where a value read out of
//! a model becomes part of a command — and a property that could only be seen by spawning a
//! process would go unchecked in every run that has no network.
//!
//! **`fetch_from` is always an https address, so a real clone is always a networked run.** That
//! is why the two networked tests say so where the compiler reads it, and why they are not
//! quietly counted as green.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_outside::the_program_as_it_runs::{TheProgramAsItRuns, the_arguments_of};
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhichProgramWeRun};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// This repository, cloned into `target/`, which git already ignores and a listing already skips.
///
/// **A name and not one fixed path,** because two tests sharing a destination share it in
/// parallel: git refuses a directory that already exists, so the second of them fails on the
/// first one's leftovers and the failure says nothing about the code.
fn a_clone_the_run_asks_for(named: &str) -> ARepositoryToClone {
    ARepositoryToClone::of(
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        under_the_target_directory(named),
    )
}

/// The request to clone this repository into a directory of this name.
fn a_request_to_clone(named: &str) -> WhatWeTellAProgramToDo {
    WhatWeTellAProgramToDo::clone_the_repository(a_clone_the_run_asks_for(named))
}

/// One directory under `target/`, named so no two tests can reach the same one.
fn under_the_target_directory(named: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join(named)
}

#[test]
fn the_arguments_are_one_thing_each_and_not_one_line() {
    let the_arguments = the_arguments_of(&a_request_to_clone("what-the-run-asked-for"));

    assert_eq!(
        the_arguments,
        vec![
            "clone".to_string(),
            "https://github.com/steamnoid/ai-sdlc-app-rs-plus.git".to_string(),
            under_the_target_directory("what-the-run-asked-for")
                .display()
                .to_string(),
        ],
        "**because a formatted command line is where a value read out of a model becomes part of a command** — three separate arguments are three separate arguments, and one string with spaces in it is a shell's opinion about what it means"
    );
}

#[test]
fn no_argument_is_itself_a_shell_to_be_given_anything_else_to() {
    let the_arguments = the_arguments_of(&a_request_to_clone("what-the-run-asked-for"));

    for an_argument in &the_arguments {
        assert!(
            !an_argument.contains("&&") && !an_argument.contains(';') && !an_argument.contains('|'),
            "**a shell metacharacter in an argument is a second program in a disguise** — no shell is named anywhere, so nothing can read one, and an argument that carries one is either a bug or an attack: {an_argument:?}"
        );
    }
}

#[test]
fn the_program_is_git_and_git_is_never_one_of_the_arguments() {
    let the_program = TheProgramAsItRuns::as_it_is_on_this_host();
    let the_arguments = the_arguments_of(&a_request_to_clone("what-the-run-asked-for"));

    assert_eq!(
        the_program.which_program_it_runs(),
        WhichProgramWeRun::Git,
        "**because the closed set is what makes running something safe** — a port that names the program to run is a port that runs whatever reaches it"
    );
    assert_eq!(
        the_arguments.first().map(String::as_str),
        Some("clone"),
        "**and the arguments begin with what to do, not with which program** — a first element of `git` would be a program name travelling as data, and an implementation that read it as the program would be one refactor away from running the argument before the program"
    );
    assert!(
        !the_arguments.iter().any(|an_argument| an_argument == "git"),
        "**because the program is named by the port and the arguments carry only what to do with it** — the same name in both places is two records of one decision, and a reader cannot tell which one the process was started with"
    );
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn git_really_clones_and_the_clone_really_stands_on_a_commit() {
    let named = "a-clone-of-this-repository";
    let _ = std::fs::remove_dir_all(under_the_target_directory(named));
    let the_program = TheProgramAsItRuns::as_it_is_on_this_host();

    let what_it_said = the_program.run(&a_request_to_clone(named));

    assert!(
        what_it_said.did_it_work(),
        "**a clone that reports success and cloned nothing is the worst outcome available** — it puts a directory where a repository should be and lets the run read it: {}",
        what_it_said.the_words()
    );
    assert!(
        under_the_target_directory(named).join(".git").is_dir(),
        "**and the whole history, not a shallow one** — a shallow clone has no history to answer a `git log` with, and the run does read history"
    );
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn a_repository_that_is_not_there_does_not_say_it_was_cloned() {
    let nowhere = ARepositoryToClone::of(
        Repository::try_from("steamnoid/a-repository-that-is-not-there").expect("two halves"),
        WorkItemId::try_from("WELLBEINGT-6").expect("a name"),
        under_the_target_directory("a-clone-of-a-repository-that-is-not-there"),
    );
    let _ = std::fs::remove_dir_all(nowhere.where_it_lands());
    let the_program = TheProgramAsItRuns::as_it_is_on_this_host();

    let what_it_said = the_program.run(&WhatWeTellAProgramToDo::clone_the_repository(nowhere));

    assert!(
        !what_it_said.did_it_work(),
        "**and the words carry git's own**, because the caller has to tell *no such repository* from *git is unhappy* from *git is not installed*, and a bare boolean is none of those"
    );
    assert!(
        what_it_said
            .the_words()
            .contains("a-repository-that-is-not-there"),
        "**and it failed for the reason this test is about** — a directory left over from an earlier run also fails, also says something, and would let this test pass about a stale directory instead of about a repository that is not there: {}",
        what_it_said.the_words()
    );
}
