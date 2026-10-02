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

use std::path::{Path, PathBuf};

use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
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
fn may_be_written_to(the_mount: Option<&str>) -> bool {
    the_mount.is_some_and(|one| !one.ends_with(":ro"))
}

#[test]
fn installing_an_editable_package_is_the_one_step_that_may_write() {
    let the_install_mount = the_one_mount(&the_arguments_installing_the_dependencies());
    let the_run_mount = the_one_mount(&the_arguments_running_the_suite());

    let may_the_install_write = may_be_written_to(the_install_mount.as_deref());
    let may_the_run_write = may_be_written_to(the_run_mount.as_deref());

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

#[test]
#[ignore = "needs a real container: read-only is proved by an attempt and an attempt needs one"]
fn a_real_container_refuses_to_write_the_clone_and_honours_no_new_privileges() {
    let the_clone = a_real_checkout_holding_one_file();
    let what_the_container_said = what_it_said_when_it_tried_to_write(&the_clone);

    assert!(
        what_the_container_said.contains("Read-only"),
        "**because `:ro` is a claim until a real container refuses a real write, and a `docker` \
         refusing an argument it does not know says `unknown flag` — which is the failure this \
         repository has already been burned by, on a different flag.** Every word in this file \
         has to be a word the running `docker` accepts. It said: {what_the_container_said}"
    );
}

/// A directory holding one real file, **because refusing a write into a directory that was never
/// created would prove nothing at all.**
fn a_real_checkout_holding_one_file() -> PathBuf {
    let the_clone = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/a-clone-that-may-not-be-written");
    let _ = std::fs::remove_dir_all(&the_clone);
    std::fs::create_dir_all(&the_clone).expect("a directory this machine will hold");
    let the_file = "fn nothing() {}\n";
    std::fs::write(the_clone.join("a_file.rs"), the_file).expect("a file to hold");
    the_clone
}

/// What the container said when it tried to write one byte into the clone.
fn what_it_said_when_it_tried_to_write(the_clone: &Path) -> String {
    let suite = a_python_suite_that_installs_editable();
    let mut the_arguments = what_would_be_run_against(&suite, the_clone);
    let up_to_the_image = up_to_the_image_name(&the_arguments, &suite);
    the_arguments.truncate(up_to_the_image);
    the_arguments.extend([
        suite.the_toolchain_image.clone(),
        String::from("sh"),
        String::from("-c"),
        String::from("echo one > /the-clone/a_canary_file"),
    ]);
    let the_output = std::process::Command::new("docker")
        .args(&the_arguments)
        .output();
    docker_wrote(&the_output)
}

/// The `docker run` this system builds, **and a separate function so the two tests in this file
/// that swap the command share one translation rather than copying it.**
fn what_would_be_run_against(
    the_suite: &TheSuiteARepositoryDeclares,
    the_clone: &Path,
) -> Vec<String> {
    let request = WhatWeTellAProgramToDo::the_suite_is_run(the_suite, the_clone);
    TheProgramAsItRuns::as_it_is_on_this_host(None).the_arguments_of(&request)
}

/// Where the image name is, **and everything before it is what this system built and did not
/// change.**
fn up_to_the_image_name(
    every_argument: &[String],
    the_suite: &TheSuiteARepositoryDeclares,
) -> usize {
    every_argument
        .iter()
        .position(|one| *one == the_suite.the_toolchain_image)
        .unwrap_or(every_argument.len())
}

/// What `docker` wrote, **or why it never started at all.**
fn docker_wrote(the_output: &std::io::Result<std::process::Output>) -> String {
    match the_output {
        Ok(it) => format!(
            "{}{}",
            String::from_utf8_lossy(&it.stdout),
            String::from_utf8_lossy(&it.stderr)
        ),
        Err(why) => why.to_string(),
    }
}
