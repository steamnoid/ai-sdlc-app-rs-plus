//! The run clones a repository, and says which of the four ways it could not when it could not.
//!
//! **Three of the four ways are about a value read out of a command that failed.** A directory
//! that already holds something is refused before git runs at all, because a run resumes and the
//! directory is derived — **a refusal that arrives after the damage is not a refusal.**
//!
//! **A complaint this system does not name becomes `GitIsUnhappy`, and not a guess.** The use case
//! reads git's words to tell *not found* from *nobody has committed to it yet*, and a reader that
//! could not match a word would otherwise have to answer a fourth question it cannot answer.

use aisdlc_domain::a_clone::ACloneOnDisk;
use aisdlc_domain::why_cloning_the_repository_failed::WhichWayItFailed;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_run_clones_a_repository::the_run_clones_a_repository;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use aisdlc_run::whether_anything_stands_there::WhetherAnythingStandsThere;

fn a_clone_into(a_directory: &str) -> ARepositoryToClone {
    ARepositoryToClone::of(
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("target")
            .join(a_directory),
    )
}

/// A program that says what it is given, in order, and remembers what it was asked.
struct AProgramThatAnswers {
    what_it_says: std::cell::RefCell<std::collections::VecDeque<WhatTheProgramSaid>>,
    what_it_was_asked: std::cell::RefCell<Vec<WhatWeTellAProgramToDo>>,
}

impl AProgramThatAnswers {
    fn saying_in_order(what_it_says: Vec<WhatTheProgramSaid>) -> Self {
        Self {
            what_it_says: std::cell::RefCell::new(what_it_says.into()),
            what_it_was_asked: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AProgramWeRun for AProgramThatAnswers {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.what_it_was_asked
            .borrow_mut()
            .push(what_it_is_asked_to_do.clone());
        self.what_it_says
            .borrow_mut()
            .pop_front()
            .expect("a double with no answers left, because a use case asked for more than the test wrote down")
    }
}

struct SomethingStandsThere;
struct NothingStandsThere;

impl WhetherAnythingStandsThere for SomethingStandsThere {
    fn does_anything_stand_at(&self, _a_directory: &std::path::Path) -> bool {
        true
    }
}

impl WhetherAnythingStandsThere for NothingStandsThere {
    fn does_anything_stand_at(&self, _a_directory: &std::path::Path) -> bool {
        false
    }
}

#[test]
fn a_destination_that_already_holds_something_is_refused_before_git_is_run() {
    let the_clone = a_clone_into("a-destination-already-taken");
    let mut a_program_with_nothing_to_say = AProgramThatAnswers::saying_in_order(Vec::new());

    let how_it_went = the_run_clones_a_repository(
        &mut a_program_with_nothing_to_say,
        &mut SomethingStandsThere,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone),
    );

    assert_eq!(
        how_it_went.which_way_it_failed().unwrap(),
        WhichWayItFailed::TheDestinationAlreadyHoldsSomething,
        "**because a run resumes and the directory is derived, so this is a decision made before git was run at all** — a refusal that arrives after the damage is not a refusal, and a run that has already written over a resumed run's clone has not refused anything"
    );
}

#[test]
fn gits_not_found_is_its_own_way_and_not_gits_discontent() {
    let the_clone = a_clone_into("a-destination-nobody-has-taken");
    let mut a_program = AProgramThatAnswers::saying_in_order(vec![WhatTheProgramSaid::of(
        false,
        "fatal: repository 'https://github.com/steamnoid/nope.git' not found",
    )]);

    let how_it_went = the_run_clones_a_repository(
        &mut a_program,
        &mut NothingStandsThere,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone),
    );

    assert_eq!(
        how_it_went.which_way_it_failed().unwrap(),
        WhichWayItFailed::NoSuchRepository,
        "**because somebody named a repository that is not there, and that is a wrong address rather than a broken network** — the two fixes are not the same and a caller told only that it failed has learned nothing"
    );
}

#[test]
fn a_complaint_this_system_does_not_name_becomes_gits_discontent() {
    let the_clone = a_clone_into("a-destination-gave-gits-trouble");
    let mut a_program = AProgramThatAnswers::saying_in_order(vec![WhatTheProgramSaid::of(
        false,
        "fatal: could not resolve host: github.com",
    )]);

    let how_it_went = the_run_clones_a_repository(
        &mut a_program,
        &mut NothingStandsThere,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone),
    );

    assert_eq!(
        how_it_went.which_way_it_failed().unwrap(),
        WhichWayItFailed::GitIsUnhappy,
        "**and not a guess** — this system names two of git's complaints and refuses to invent a fifth way, so a word it does not recognise becomes the way that means *something, and not something we can tell you*"
    );
}

#[test]
fn what_git_said_is_carried_and_not_summarised() {
    let the_clone = a_clone_into("a-destination-carrying-gits-words");
    let git_words = "fatal: could not resolve host: github.com";
    let mut a_program =
        AProgramThatAnswers::saying_in_order(vec![WhatTheProgramSaid::of(false, git_words)]);

    let how_it_went = the_run_clones_a_repository(
        &mut a_program,
        &mut NothingStandsThere,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone),
    );

    assert_eq!(
        how_it_went.why_it_failed().unwrap().what_git_said(),
        git_words,
        "**because a value read out of a command that failed is that command's complaint wearing a field's name** — and summarising it here is the one thing that would leave the caller unable to tell git's trouble from ours"
    );
}

#[test]
fn the_commit_and_the_ref_are_asked_of_git_and_never_remembered() {
    let the_clone = a_clone_into("a-destination-nobody-has-read-yet");
    let mut a_program = AProgramThatAnswers::saying_in_order(vec![
        WhatTheProgramSaid::of(true, "Cloning into 'a-destination'..."),
        WhatTheProgramSaid::of(true, "e1c2d3"),
        WhatTheProgramSaid::of(true, "HEAD"),
    ]);

    let how_it_went = the_run_clones_a_repository(
        &mut a_program,
        &mut NothingStandsThere,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone),
    );

    let what_it_asked = a_program.what_it_was_asked.borrow();
    assert_eq!(
        what_it_asked.len(),
        3,
        "**because a clone, a commit and a ref are three questions and not one** — and a run that filled the commit in from an earlier run would report its changes against a commit it stopped at days ago"
    );
    assert!(
        what_it_asked[1].is_the_commit_of_the_clone().is_some()
            && what_it_asked[2].is_the_ref_of_the_clone().is_some(),
        "**and the last two ask what the clone is standing on**, because a value read out of the clone is the only one that is about the commit the clone is actually on"
    );
}

#[test]
fn a_clone_that_succeeded_is_a_clone_and_the_commit_and_the_ref_it_stands_on() {
    let the_clone = a_clone_into("a-destination-nobody-has-cloned-yet");
    let mut a_program = AProgramThatAnswers::saying_in_order(vec![
        WhatTheProgramSaid::of(true, "Cloning into 'a-destination'..."),
        WhatTheProgramSaid::of(true, "e1c2d3"),
        WhatTheProgramSaid::of(true, "HEAD"),
    ]);

    let how_it_went = the_run_clones_a_repository(
        &mut a_program,
        &mut NothingStandsThere,
        &WhatWeTellAProgramToDo::clone_the_repository(the_clone),
    );

    let the_clone_that_is = how_it_went
        .the_clone()
        .expect("a clone that worked is a clone");
    let _: &ACloneOnDisk = the_clone_that_is;
    assert_eq!(
        the_clone_that_is.the_commit().as_str(),
        "e1c2d3",
        "**and the commit is what git said about the clone, not what the run expected**"
    );
    assert_eq!(
        the_clone_that_is.ref_name().as_str(),
        "HEAD",
        "**and the ref may be HEAD**, because a detached clone has no branch and naming the field `branch` would be a lie in exactly the case where the value matters"
    );
}
