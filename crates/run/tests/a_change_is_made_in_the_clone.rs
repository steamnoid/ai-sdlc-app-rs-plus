//! A change the plan named, written into the clone and recorded in its history.
//!
//! **Nothing in this repository writes a file.** `TheProjectOnDisk` asks two questions and the
//! implementation of one is the local filesystem; **there is no `std::fs::write` in any
//! `src/` file in this workspace** — the only writes are three test files building fixtures. A
//! run that reads a repository and changes nothing is a run that delivers nothing, **and the
//! Definition of Done turns on "one end-to-end run produces a traceable pull request".**
//!
//! **A read and a write are two ports and not one, and that is the point.** The read port's own
//! docstring says "Two questions and no others: what is in it, and what does one file say" —
//! **so a run that could write through it would be a run whose reading boundary nobody could
//! state.** A repository this system writes into is somebody else's working tree.
//!
//! **Only where the plan named it, and that is the whole rule.** A plan a person approved is a
//! list of files; **an implementation that writes anything else is a change nobody approved**,
//! and a file outside the clone is a file in somebody else's project.
//!
//! **A write identical to what is already there is refused.** This repository's own history
//! records a run that wrote a file byte-identical to the one already there, committed nothing,
//! and was caught only by a `Nothing to push` from GitHub forty-five minutes later — **a change
//! nobody can see is not a change, and a commit of nothing is not a record.**

use aisdlc_run::a_change_is_made_in_the_clone::{
    AChangeCouldNotBeMade, AFileToWrite, WhatWasWritten, a_change_is_made_in_the_clone,
};
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::a_record_of_a_change::AChangeIsRecorded;
use aisdlc_run::what_a_planner_planned::WhatAPlannerPlanned;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// The plan a person approved, **naming one file to change and one test to bring.**
fn the_plan_a_person_approved() -> WhatAPlannerPlanned {
    WhatAPlannerPlanned {
        files_to_change: vec![String::from("AGENTS.md")],
        files_to_add: vec![],
        tests_to_add: vec![String::from("tests/a_gate_and_its_breaches.rs")],
        dependencies: vec![],
        migrations: vec![],
    }
}

/// One file, **with the words a person would read if they opened it.**
fn a_file_with(text: &str) -> AFileToWrite {
    AFileToWrite::the_file_at("AGENTS.md", text).expect("a path inside the project")
}

/// A project that remembers what it was asked to write, **and what it already held.**
struct AProjectRemembering {
    what_it_held_before: Vec<(String, String)>,
    what_it_was_asked_to_write: std::cell::RefCell<Vec<String>>,
}

impl AProjectRemembering {
    /// A project holding nothing, **and a writer that records what it was asked for.**
    fn holding_nothing() -> Self {
        Self {
            what_it_held_before: Vec::new(),
            what_it_was_asked_to_write: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// A project already holding this file, **which is what makes a write a change.**
    fn holding_one_file(named: &str, saying: &str) -> Self {
        Self {
            what_it_held_before: vec![(String::from(named), String::from(saying))],
            what_it_was_asked_to_write: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AChangeIsRecorded for AProjectRemembering {
    /// What it holds of one file, **and nothing when it holds nothing.**
    fn what_it_holds_of(&self, which_file: &str) -> Option<String> {
        self.what_it_held_before
            .iter()
            .find(|(named, _)| named == which_file)
            .map(|(_, saying)| saying.clone())
    }

    /// Write one file, **and record that it was asked to.**
    fn one_file_is_written(&mut self, which_file: &str, what_it_says: &str) {
        self.what_it_was_asked_to_write
            .borrow_mut()
            .push(format!("{which_file}: {what_it_says}"));
    }
}

/// A program that answers Git's questions, **and remembers the ones it was asked.**
struct AProgramGitAnswering {
    the_words_git_wrote: String,
    did_it_work: bool,
    what_it_was_asked: std::cell::RefCell<Vec<WhatWeTellAProgramToDo>>,
}

impl AProgramGitAnswering {
    /// A `git` that commits, **and answers with the commit it made.**
    fn a_git_that_commits() -> Self {
        Self {
            the_words_git_wrote: String::from(
                "[phase/97-a_change_is_made_in_the_clone 9f1c2a4] the gate a document sent the wrong way first\n 1 file changed",
            ),
            did_it_work: true,
            what_it_was_asked: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// A `git` that would not commit, **saying why in its own words.**
    fn a_git_that_would_not_commit() -> Self {
        let mut refusing = Self::a_git_that_commits();
        refusing.did_it_work = false;
        refusing.the_words_git_wrote = String::from("nothing to commit, working tree clean");
        refusing
    }
}

impl AProgramWeRun for AProgramGitAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.what_it_was_asked
            .borrow_mut()
            .push(what_it_is_asked_to_do.clone());
        WhatTheProgramSaid::of(self.did_it_work, &self.the_words_git_wrote)
    }
}

#[test]
fn every_file_the_plan_named_is_written_and_nothing_else_is() {
    let mut the_project = AProjectRemembering::holding_nothing();
    let the_git = AProgramGitAnswering::a_git_that_commits();

    let what_was_written = a_change_is_made_in_the_clone(
        &the_git,
        &mut the_project,
        &the_plan_a_person_approved(),
        &[a_file_with("the gate a document sent the wrong way first")],
    )
    .expect("the plan named that file and the write was a change");

    let what_the_plan_named_besides = what_was_written.files_written_ref();
    assert_eq!(
        (what_was_written.the_commit().as_str(), what_the_plan_named_besides.len()),
        ("9f1c2a4", 1),
        "**because a change is the files the plan named and the commit that records them** — and \
         both answers come from what `git` wrote, **because a commit this system composed would be \
         a record of something that never happened.** What was written: {:?}",
        what_was_written.files_written_ref()
    );
}

#[test]
fn a_file_the_plan_did_not_name_is_refused_and_nothing_is_written() {
    let mut the_project = AProjectRemembering::holding_nothing();
    let the_git = AProgramGitAnswering::a_git_that_commits();

    let the_refusal = a_change_is_made_in_the_clone(
        &the_git,
        &mut the_project,
        &the_plan_a_person_approved(),
        &[
            a_file_with("the gate a document sent the wrong way first"),
            AFileToWrite::the_file_at("src/lib.rs", "a change nobody approved")
                .expect("a path inside the project"),
        ],
    );

    let nothing_was_written = the_project.what_it_was_asked_to_write.borrow().is_empty();
    assert!(
        matches!(the_refusal, Err(AChangeCouldNotBeMade::AFileThePlanDidNotName { .. }))
            && nothing_was_written,
        "**because a plan a person approved is a list of files, and an implementation writing \
         anything else is a change nobody approved** — and a run that wrote the unapproved file and \
         then refused is a run that changed somebody's repository for nothing. What was written: \
         {:?}",
        the_project.what_it_was_asked_to_write.borrow()
    );
}

#[test]
fn a_write_identical_to_what_is_already_there_is_refused() {
    let mut the_project = AProjectRemembering::holding_one_file(
        "AGENTS.md",
        "the gate a document sent the wrong way first",
    );
    let the_git = AProgramGitAnswering::a_git_that_commits();

    let the_refusal = a_change_is_made_in_the_clone(
        &the_git,
        &mut the_project,
        &the_plan_a_person_approved(),
        &[a_file_with("the gate a document sent the wrong way first")],
    );

    assert!(
        matches!(the_refusal, Err(AChangeCouldNotBeMade::AWriteThatChangesNothing { .. })),
        "**because a change nobody can see is not a change** — this repository's history records a \
         run that wrote a file byte-identical to the one already there, committed nothing, and was \
         caught only by a `nothing to commit` from GitHub forty-five minutes later. Refused here, \
         where a person can be told: {the_refusal:?}"
    );
}

#[test]
fn a_commit_git_would_not_make_is_refused_with_gits_own_words() {
    let mut the_project = AProjectRemembering::holding_nothing();
    let the_git = AProgramGitAnswering::a_git_that_would_not_commit();

    let the_refusal = a_change_is_made_in_the_clone(
        &the_git,
        &mut the_project,
        &the_plan_a_person_approved(),
        &[a_file_with("the gate a document sent the wrong way first")],
    );

    assert!(
        matches!(the_refusal, Err(AChangeCouldNotBeMade::TheChangeWasNotCommitted { .. })),
        "**because a delivery that pushed a branch with no commit on it is refused by the far end, \
         a hundred steps later, for a cause that is a hundred steps back** — and `nothing to commit, \
         working tree clean` is the only thing that says so. Quoted, and not summarised: \
         {the_refusal:?}"
    );
}
