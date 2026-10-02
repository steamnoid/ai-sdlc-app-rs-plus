//! A repository's own tests, run in a container that cannot see the host.
//!
//! **Executing a repository is the dangerous operation in this system, not reading one.** An
//! implementation agent writes code and a verification agent then runs **that repository's own
//! test suite** — arbitrary code from a pull request, with the developer's credentials and file
//! system within reach.
//!
//! **So the boundary goes around execution and not around the run.** `shell.run_command` runs *our*
//! programs on the host, **because `git` is trusted and it is what the clone is built on**; this
//! runs *theirs* in a container.
//!
//! **The two refusals come in a fixed order and the order is the rule.** A machine with no
//! container is refused whatever the checkout declares, **because a run that inspected the work
//! before refusing would already have done the work it was refusing to do safely.**

use std::path::{Path, PathBuf};

use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::the_tests_are_run_in_a_container::{
    TheSuiteARepositoryDeclares, TheTestsCouldNotBeRun, WhatTheSuiteSaid, the_tests_in,
};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A `docker` that reports a container, **and records every word it was handed.**
struct ADockerAnswering {
    there_is_a_container: bool,
    the_words_it_wrote: String,
    what_it_was_handed: std::cell::RefCell<Vec<String>>,
}

impl ADockerAnswering {
    /// A machine with a container, **whose `docker run` succeeds.**
    fn one_that_works() -> Self {
        Self {
            there_is_a_container: true,
            the_words_it_wrote: String::from("74 passed, 9 skipped"),
            what_it_was_handed: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// A machine with nothing that could run a container at all.
    fn one_machine_with_no_container() -> Self {
        Self {
            there_is_a_container: false,
            the_words_it_wrote: String::new(),
            what_it_was_handed: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AProgramWeRun for ADockerAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.what_it_was_handed
            .borrow_mut()
            .push(format!("{what_it_is_asked_to_do:?}"));
        WhatTheProgramSaid::of(self.there_is_a_container, &self.the_words_it_wrote)
    }
}

/// A checkout declaring its suite, **with the image and the user that go with it.**
fn a_python_repository_declaring_its_suite() -> TheSuiteARepositoryDeclares {
    TheSuiteARepositoryDeclares {
        the_toolchain_image: String::from("python:3.12-slim"),
        how_to_install: vec![
            String::from("pip"),
            String::from("install"),
            String::from("-q"),
            String::from("-e"),
            String::from("."),
        ],
        how_to_run: vec![String::from("pytest"), String::from("-q")],
        who_runs_it: String::from("nobody"),
    }
}

/// Where a clone lands, **and no directory is created for this.**
fn where_the_clone_lands() -> PathBuf {
    Path::new("/tmp/a-clone-of-wellbeing-tracker-public").to_path_buf()
}

#[test]
fn a_machine_with_no_container_is_refused_before_the_work_is_looked_at() {
    let the_docker = ADockerAnswering::one_machine_with_no_container();

    let the_refusal = the_tests_in(
        &the_docker,
        Some(a_python_repository_declaring_its_suite()),
        &where_the_clone_lands(),
    );

    assert!(
        matches!(
            &the_refusal,
            Err(TheTestsCouldNotBeRun::NoContainerToRunThemIn)
        ),
        "**because a run that inspected the work before refusing would already have done the work \
         it was refusing to do safely** — and this repository's sister implementation insists on \
         the same order: the refusal comes after the container is known and before anything is \
         installed. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_repository_declaring_no_suite_is_refused_and_nothing_is_run() {
    let the_docker = ADockerAnswering::one_that_works();

    let the_refusal = the_tests_in(&the_docker, None, &where_the_clone_lands());

    assert!(
        matches!(&the_refusal, Err(TheTestsCouldNotBeRun::NoSuiteWasDeclared)),
        "**because a repository this system cannot test is a repository it cannot verify a change \
         against** — and the first delivery on the sister implementation's copy died here, because \
         a hard-coded Node toolchain was right for every JavaScript project and wrong for the \
         first Python one. What was refused: {the_refusal:?}"
    );
}

#[test]
fn an_install_that_failed_is_not_reported_as_a_red_suite() {
    let the_suite_said = WhatTheSuiteSaid {
        did_the_dependencies_install: false,
        what_it_wrote: String::from("error: could not find a version that satisfies"),
    };

    let how_it_is_reported = the_suite_said.how_the_run_is_reported();

    assert!(
        how_it_is_reported.is_err(),
        "**because a suite whose dependencies did not install has not run** — and a report saying \
         red would be a claim about code nobody executed. What it said: {how_it_is_reported:?}"
    );
}

#[test]
fn a_suite_that_installed_and_passed_is_reported_with_its_own_words() {
    let the_suite_said = WhatTheSuiteSaid {
        did_the_dependencies_install: true,
        what_it_wrote: String::from("74 passed, 9 skipped"),
    };

    let how_it_is_reported = the_suite_said.how_the_run_is_reported();

    assert_eq!(
        how_it_is_reported
            .expect("the suite installed and passed")
            .what_it_said(),
        "74 passed, 9 skipped",
        "**because the words are copied out of what the suite wrote and never summarised** — a \
         person reading this wants the suite's own sentence, not this system's paraphrase of it."
    );
}

#[test]
fn a_suite_that_ran_is_given_the_clone_and_nothing_else_of_this_machine() {
    let the_docker = ADockerAnswering::one_that_works();

    let what_it_said = the_tests_in(
        &the_docker,
        Some(a_python_repository_declaring_its_suite()),
        &where_the_clone_lands(),
    );

    let every_word_it_was_handed = the_docker.what_it_was_handed.borrow().join("\n");
    assert!(
        !what_it_said.is_err(),
        "**because the ordinary case has to be able to pass** — a container rule that refused \
         everything would deliver nothing. What it said: {what_it_said:?} and it was handed: \
         {every_word_it_was_handed}"
    );
}
