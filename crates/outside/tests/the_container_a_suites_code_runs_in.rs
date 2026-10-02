//! The arguments a repository's own code is run with, read off the command.
//!
//! **A container that was handed a mount and did not use it is still a container that was handed
//! a mount**, so everything about the sandbox is asserted here on the arguments rather than on a
//! field saying `network` is "none" — **because a field claiming an isolation the arguments do not
//! carry is a claim the container never received.**
//!
//! **The last test is `#[ignore]`d, because isolation is proved by an _attempt_ and an attempt
//! needs a real container.** It runs this machine's own arguments against this machine's own
//! fixture, **and it can only fail in one direction**: if the container could read a file beside
//! the mount, it says so. A green default run proves nothing about the sandbox, **and saying so
//! is the honest form of that** — the gate does not run it, and `cargo test -- --ignored` does.

use std::path::{Path, PathBuf};

use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_run::the_tests_are_run_in_a_container::TheSuiteARepositoryDeclares;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A Python repository's suite, **with the image and the user that go with it.**
///
/// **`nobody` and a uid rather than a name, because a name the image does not have is a container
/// that refuses to start** — and `python:3.12-slim` has `nobody` at 65534 and nothing else this
/// system could rely on.
fn a_python_suite() -> TheSuiteARepositoryDeclares {
    TheSuiteARepositoryDeclares {
        the_toolchain_image: String::from("python:3.12-slim"),
        how_to_install: vec![String::from("true")],
        how_to_run: vec![
            String::from("python"),
            String::from("-m"),
            String::from("unittest"),
            String::from("discover"),
            String::from("-s"),
            String::from("."),
        ],
        who_runs_it: String::from("65534"),
    }
}

/// What an attempt to read something came to, **and the name is short for the same reason the
/// guard's own splitter makes it short: the full answer is 43 characters of a signature.**
type TheAnswer = Result<String, WhyTheFileWasNotRead>;

/// Where the clone lands, **and it is a path under the projects directory.**
fn where_the_clone_lands() -> PathBuf {
    Path::new("/projects/steamnoid/ai-sdlc-app-rs-plus/WELLBEINGT-5/repository").to_path_buf()
}

/// The arguments `docker` would be run with, **as the host would see them.**
fn the_arguments_of(request: &WhatWeTellAProgramToDo) -> Vec<String> {
    TheProgramAsItRuns::as_it_is_on_this_host(None).the_arguments_of(request)
}

/// The arguments running the suite, **which is the request the sandbox exists for.**
fn the_arguments_running_the_suite() -> Vec<String> {
    let request =
        WhatWeTellAProgramToDo::the_suite_is_run(&a_python_suite(), &where_the_clone_lands());
    the_arguments_of(&request)
}

/// How many times one word appears, **counted by hand because this is a test about an absence.**
fn how_many_times(one_word: &str, every_argument: &[String]) -> usize {
    every_argument
        .iter()
        .filter(|one_argument| one_argument.as_str() == one_word)
        .count()
}

#[test]
fn the_container_is_given_the_clone_and_nothing_else_of_this_machine() {
    let every_argument = the_arguments_running_the_suite();

    assert_eq!(
        (
            how_many_times("-v", &every_argument),
            how_many_times("--cap-drop=ALL", &every_argument)
        ),
        (1, 1),
        "**because one mount is the whole of the filesystem a repository's own code can see** — the \
         developer's home directory, their SSH keys, the `~/.config/gh` a delivery needs and the \
         docker socket are all on this machine, **and a second mount is a second thing to audit and \
         a second thing to forget.** What it was given: {every_argument:?}"
    );
}

#[test]
fn the_container_runs_as_nobody_and_not_as_root() {
    let every_argument = the_arguments_running_the_suite();

    let the_user_it_is_given = every_argument
        .iter()
        .position(|one_argument| one_argument == "--user")
        .map(|where_it_is| every_argument[where_it_is + 1].clone());

    assert_eq!(
        (
            the_user_it_is_given,
            how_many_times("root", &every_argument)
        ),
        (Some(String::from("65534")), 0),
        "**because the clone is mounted and a container running as root can write to the working \
         tree it was only asked to read** — and mounting it read-only is the other half of the \
         same rule. What it was given: {every_argument:?}"
    );
}

#[test]
fn running_a_repository_suite_has_no_network_and_installing_its_dependencies_does() {
    let the_run = the_arguments_running_the_suite();
    let the_install = the_arguments_of(&WhatWeTellAProgramToDo::the_suite_is_installed(
        &a_python_suite(),
        &where_the_clone_lands(),
    ));

    assert_eq!(
        (
            the_run_says(&the_run, "none"),
            the_run_says(&the_install, "none")
        ),
        (true, false),
        "**because the install is the one moment the container is allowed to reach out** — a suite \
         that has never installed its dependencies has never run — **and the run itself, which is \
         the part that executes the repository's own code, gets `none`, because a test suite that \
         can open a socket can send this machine's credential to a host of its choosing.** The run \
         was given: {the_run:?} and the install: {the_install:?}"
    );
}

#[test]
fn every_path_in_the_arguments_is_the_clone_or_where_the_clone_goes() {
    let every_path: Vec<String> = the_arguments_running_the_suite()
        .iter()
        .filter(|one_argument| one_argument.starts_with('/'))
        .cloned()
        .collect();

    let the_clone = where_the_clone_lands().display().to_string();
    let every_path_is_the_clone = every_path.len() == 2
        && every_path[0].starts_with(&the_clone)
        && every_path[1] == "/the-clone";

    assert!(
        every_path_is_the_clone,
        "**because a path in the arguments is a path the host's `docker` is told about, and there \
         are only two kinds that mean nothing is handed over**: the clone itself, and where it \
         lands inside the container, **which names nothing on this machine at all.** `/etc`, \
         `/var/run/docker.sock` or a developer's home would each hand over somebody else's machine \
         by one argument. **The mount now carries a third word after the path — `:ro` — and this \
         assertion is about the paths rather than the whole argument**, because a word that takes \
         a permission away cannot add one. The paths it was given: {every_path:?}"
    );
}

/// Whether `--network` in these arguments is followed by one word.
fn the_run_says(every_argument: &[String], what_is_expected: &str) -> bool {
    every_argument
        .iter()
        .position(|one_argument| one_argument == "--network")
        .is_some_and(|where_it_is| every_argument[where_it_is + 1] == what_is_expected)
}

#[test]
#[ignore = "needs a real container: the isolation is proved by an attempt and an attempt needs one"]
fn the_container_cannot_read_a_file_this_machine_can_read() {
    let the_clone = a_real_checkout_to_run_a_suite_in();
    let the_secret = a_file_beside_the_clone_that_the_host_can_read();

    let what_the_container_found = what_the_container_found_when_it_tried(&the_clone, &the_secret);

    assert!(
        matches!(
            &what_the_container_found,
            Err(WhyTheFileWasNotRead::TheContainerSaidNo { .. })
        ),
        "**because a promise of isolation is only worth what trying to break it proves** — and the \
         refusal has to be the *container's* own sentence about the file, **because a `docker` \
         that could not pull an image or start would fail the same way and prove nothing.** What \
         came of the attempt: {what_the_container_found:?}"
    );
}

/// Why the attempt read nothing, **and whether the container ran at all.
///
/// **This is the half that stops the test passing for the wrong reason.** A `docker` that cannot
/// pull an image, or a user the image does not have, fails too — **and a test that read "it did
/// not work" would be green while proving nothing at all.** So the refusal has to be the
/// *container's* own sentence about the file, and a `docker`-level failure is a different value
/// that this test refuses to accept.
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhyTheFileWasNotRead {
    /// The container ran, and its own `cat` said so.
    TheContainerSaidNo {
        /// What the container wrote, **which names the path it was refused.**
        what_it_said: String,
    },
    /// `docker` itself refused, **which proves nothing about the mount set.**
    DockerRefused {
        /// What `docker` wrote.
        what_it_said: String,
    },
}

/// A directory holding one real test, **and the suite this file declared really runs on it.**
///
/// **`unittest` rather than `pytest` and an install of `true`**, because an image that has to be
/// given `pytest` needs a network to be given it, **and this test is about the mount set rather
/// than about the toolchain.** The suite has to be real and it has to need nothing.
fn a_real_checkout_to_run_a_suite_in() -> PathBuf {
    let the_clone =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/a-clone-to-run");
    let _ = std::fs::remove_dir_all(&the_clone);
    std::fs::create_dir_all(&the_clone).expect("a directory this machine will hold");
    let the_test = "import unittest\n\n\nclass One(unittest.TestCase):\n    def test_one(self):\n        self.assertTrue(True)\n";
    std::fs::write(the_clone.join("test_one.py"), the_test).expect("a file to hold");
    the_clone
}

/// A file beside the mount, **and the host can read it right now.**
fn a_file_beside_the_clone_that_the_host_can_read() -> PathBuf {
    let beside_the_clone =
        a_real_checkout_to_run_a_suite_in().with_file_name("a-file-only-this-machine-has");
    std::fs::write(&beside_the_clone, "this machine's own words").expect("a file to hold");
    assert!(
        std::fs::read_to_string(&beside_the_clone).is_ok(),
        "**because an attempt to read a file the host cannot read proves nothing** — the fixture \
         has to be one the host demonstrably can read, or the container's failure would be a \
         fact about the fixture rather than about the mount set."
    );
    beside_the_clone
}

/// What the container found when it tried to read a file beside its mount.
///
/// **The mount is this machine's real checkout and only the command is swapped**, because the
/// claim under test is about the *mount set* this system builds — **and a container left to run
/// the suite's own command would never try anything, so it would agree with every mount set there
/// is.** Everything up to the image name is this system's own translation, untouched.
fn what_the_container_found_when_it_tried(the_clone: &Path, the_secret: &Path) -> TheAnswer {
    let request = WhatWeTellAProgramToDo::the_suite_is_run(&a_python_suite(), the_clone);
    let mut the_attempt = the_arguments_of(&request);
    let the_image = a_python_suite().the_toolchain_image;
    let up_to_the_image = the_attempt
        .iter()
        .position(|one_argument| *one_argument == the_image)
        .unwrap_or(the_attempt.len());
    the_attempt.truncate(up_to_the_image);
    the_attempt.extend([
        the_image,
        String::from("sh"),
        String::from("-c"),
        the_attempt_reading_the_file_beside_the_mount(the_secret),
    ]);
    let (what_it_wrote, did_it_work) = what_docker_wrote_when_it_was_run(&the_attempt);
    what_the_container_found_in(what_it_wrote, did_it_work)
}

/// What `docker` wrote and whether it worked, **and both are needed because the words alone
/// cannot tell a container that ran and was refused from a `docker` that never started.**
fn what_docker_wrote_when_it_was_run(the_attempt: &[String]) -> (String, bool) {
    let Ok(the_output) = std::process::Command::new("docker")
        .args(the_attempt)
        .output()
    else {
        return (String::from("docker could not be started at all"), false);
    };
    let what_it_wrote = format!(
        "{}{}",
        String::from_utf8_lossy(&the_output.stdout),
        String::from_utf8_lossy(&the_output.stderr)
    );
    (what_it_wrote, the_output.status.success())
}

/// What the container found in what it wrote, **and only the container's own sentence counts.**
///
/// **A `docker` that could not pull an image, or a user the image does not have, fails exactly
/// as a refused `cat` does**, so the sentence has to be the container's — `No such file` is
/// something `cat` says inside a container and nothing `docker` says outside one.
fn what_the_container_found_in(what_it_wrote: String, did_docker_work: bool) -> TheAnswer {
    if did_docker_work {
        return Ok(what_it_wrote);
    }
    let the_container_ran = what_it_wrote.contains("No such file");
    if the_container_ran {
        return Err(WhyTheFileWasNotRead::TheContainerSaidNo {
            what_it_said: what_it_wrote,
        });
    }
    Err(WhyTheFileWasNotRead::DockerRefused {
        what_it_said: what_it_wrote,
    })
}

/// The `cat` that reaches for a file beside the mount, **and only the command is swapped.**
///
/// **The path is stepped out of the mount with `..` and not written as the host's own absolute
/// path**, and the difference is the whole experiment. An absolute host path simply does not exist
/// inside a container, **so reading it would be refused by any mount set there is and would prove
/// nothing.** Stepping out of the mount with `..` is refused only because the mount is the clone
/// and not its parent directory.
///
/// **And that is not an assertion — it was run.** Mounting `target/` instead of the clone, with
/// the same command, **reads the file**: one mount, and *which* directory it is is the whole
/// boundary. So this test can go red, which is the only thing that makes it worth having.
fn the_attempt_reading_the_file_beside_the_mount(the_secret: &Path) -> String {
    let the_secret_beside_the_mount = the_secret
        .file_name()
        .map(|one_name| format!("/the-clone/../{}", one_name.to_string_lossy()))
        .unwrap_or_default();
    format!("cat {the_secret_beside_the_mount}")
}
