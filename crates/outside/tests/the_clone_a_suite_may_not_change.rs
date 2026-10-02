//! The clone is read-only to a suite, and a suite that cannot write cannot lie.
//!
//! **A test that rewrites the file it asserts on can turn red into green.** That is not a
//! hypothetical: this repository's own history is a run that verified green twice and delivered
//! nothing, **and a suite with write access to the checkout could have produced that green
//! itself.** Mounting the clone read-write while the suite runs means the thing being judged is
//! something the judge may edit.
//!
//! **Read-only on the run and not on the install, and that asymmetry is the whole design.** A
//! suite that has never installed its dependencies has never run, **and an editable install
//! writes `.egg-info` into the checkout** — so the install needs the write and the run must not
//! have it. The two steps already differ in one flag for exactly this reason; now they differ in
//! two.

use std::path::Path;

use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_run::a_program_we_run::AProgramWeRun;
use aisdlc_run::the_tests_are_run_in_a_container::TheSuiteARepositoryDeclares;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A Python repository's suite, **which installs with `pip install -e .` and so writes.**
fn a_python_suite_that_installs_editable() -> TheSuiteARepositoryDeclares {
    TheSuiteARepositoryDeclares {
        the_toolchain_image: String::from("python:3.12-slim"),
        how_to_install: vec![
            String::from("pip"),
            String::from("install"),
            String::from("-q"),
            String::from("-e"),
            String::from("."),
        ],
        how_to_run: vec![
            String::from("python"),
            String::from("-m"),
            String::from("unittest"),
            String::from("discover"),
        ],
        who_runs_it: String::from("65534"),
    }
}

/// Where the clone lands.
fn where_the_clone_lands() -> &'static Path {
    Path::new("/projects/steamnoid/ai-sdlc-app-rs-plus/WELLBEINGT-5/repository")
}

/// The `docker run` this system builds to run the suite.
fn the_arguments_running_the_suite() -> Vec<String> {
    let suite = a_python_suite_that_installs_editable();
    let request = WhatWeTellAProgramToDo::the_suite_is_run(&suite, where_the_clone_lands());
    TheProgramAsItRuns::as_it_is_on_this_host(None).the_arguments_of(&request)
}

/// The `docker run` this system builds to install the suite's dependencies.
fn the_arguments_installing_the_dependencies() -> Vec<String> {
    let suite = a_python_suite_that_installs_editable();
    let request = WhatWeTellAProgramToDo::the_suite_is_installed(&suite, where_the_clone_lands());
    TheProgramAsItRuns::as_it_is_on_this_host(None).the_arguments_of(&request)
}

/// The one mount in these arguments, **and it is the `-v` and its value.**
fn the_one_mount(every_argument: &[String]) -> Option<String> {
    let where_it_is = every_argument.iter().position(|one| one == "-v")?;
    every_argument.get(where_it_is + 1).cloned()
}

#[test]
fn a_suite_runs_against_a_clone_it_may_not_change() {
    let the_mount = the_one_mount(&the_arguments_running_the_suite());

    assert_eq!(
        the_mount.as_deref().map(|one| one.ends_with(":ro")),
        Some(true),
        "**because a suite with write access to the checkout may edit the thing it is judging** — \
         a test that rewrites the file it asserts on turns red into green, **and this repository's \
         own history is a run that verified green twice and delivered nothing.** The mount it was \
         given: {the_mount:?}"
    );
}

/// Whether this mount lets the container write, **and `:ro` is the only word that says no.**
fn may_be_written_to(the_mount: &Option<String>) -> bool {
    the_mount
        .as_deref()
        .is_some_and(|one| !one.ends_with(":ro"))
}

#[test]
fn installing_an_editable_package_is_the_one_step_that_may_write() {
    let the_install_mount = the_one_mount(&the_arguments_installing_the_dependencies());
    let the_run_mount = the_one_mount(&the_arguments_running_the_suite());

    let may_the_install_write = may_be_written_to(&the_install_mount);
    let may_the_run_write = may_be_written_to(&the_run_mount);

    assert_eq!(
        (may_the_install_write, may_the_run_write),
        (true, false),
        "**because an editable install writes `.egg-info` into the checkout it installs from** — so \
         refusing the write there would refuse every Python project that uses one, **and refusing \
         it nowhere would let the suite edit the code it is judging.** Two steps, two answers, and \
         the difference is the one line that matters. The install was given: \
         {the_install_mount:?} and the run: {the_run_mount:?}"
    );
}

#[test]
fn no_process_in_the_container_may_gain_privileges_it_did_not_have() {
    let the_run = the_arguments_running_the_suite();
    let the_install = the_arguments_installing_the_dependencies();

    let does_the_run_say_it = the_run
        .iter()
        .any(|one_argument| one_argument == "no-new-privileges");
    let does_the_install_say_it = the_install
        .iter()
        .any(|one_argument| one_argument == "no-new-privileges");

    assert_eq!(
        (does_the_run_say_it, does_the_install_say_it),
        (true, true),
        "**because `--cap-drop=ALL` removes the capabilities a process holds and says nothing \
         about the ones it can gain** — a setuid binary inside the image is the way back in, and \
         `no-new-privileges` is the one flag that closes it. A sibling implementation in this \
         workspace already carries it and this one did not. The run was given: {the_run:?}"
    );
}

#[test]
fn a_suite_that_cannot_write_is_still_given_the_place_to_put_its_own_files() {
    let the_run = the_arguments_running_the_suite();

    let where_home_is = the_run
        .iter()
        .position(|one_argument| one_argument == "-e")
        .map(|where_it_is| the_run[where_it_is + 1].clone());

    assert_eq!(
        where_home_is,
        Some(String::from("HOME=/tmp")),
        "**because a read-only mount plus a home directory inside the checkout is a container that \
         cannot write anywhere at all** — and Python writes `.pyc` files before it runs a single \
         test, so the suite would fail on the first import **and the failure would look like a \
         broken repository rather than a sandbox that was too tight.** `HOME=/tmp` is the cheapest \
         way to give it somewhere. It was given: {the_run:?}"
    );
}
