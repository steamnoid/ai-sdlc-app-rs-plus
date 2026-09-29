//! The run clones a repository, and says which of the four ways it could not when it could not.
//!
//! **Three of the four ways are about a value read out of a command that failed.** A directory
//! that already holds something is refused before git runs at all, because a run resumes and the
//! directory is derived — **a refusal that arrives after the damage is not a refusal.**
//!
//! **A complaint this system does not name becomes `GitIsUnhappy`, and not a guess.** The use case
//! reads git's words to tell *not found* from *nobody has committed to it yet*, and a reader that
//! could not match a word would otherwise have to answer a fourth question it cannot answer.

use aisdlc_domain::why_cloning_the_repository_failed::WhichWayItFailed;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_run_makes_a_checkout::the_run_makes_a_checkout;
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

/// The run meeting a directory and a program, and the program kept so a test can read it.
fn the_run_meeting(
    what_git_says: Vec<WhatTheProgramSaid>,
    something_stands_there: bool,
) -> (
    aisdlc_run::the_run_makes_a_checkout::HowCloningWent,
    AProgramThatAnswers,
) {
    let mut a_program = AProgramThatAnswers::saying_in_order(what_git_says);
    let a_directory = WhetherTheDirectoryIs {
        something_stands_there,
    };
    let how_it_went = the_run_makes_a_checkout(
        &mut a_program,
        &a_directory,
        &WhatWeTellAProgramToDo::clone_the_repository(a_clone_into("a-destination")),
    );
    (how_it_went, a_program)
}

struct WhetherTheDirectoryIs {
    something_stands_there: bool,
}

impl WhetherAnythingStandsThere for WhetherTheDirectoryIs {
    fn does_anything_stand_at(&self, _a_directory: &std::path::Path) -> bool {
        self.something_stands_there
    }
}

/// Git failing with those words and nothing else.
fn git_saying(what_git_said: &str) -> Vec<WhatTheProgramSaid> {
    vec![WhatTheProgramSaid::of(false, what_git_said)]
}
#[test]
fn a_destination_that_already_holds_something_is_refused_before_git_is_run() {
    let (how_it_went, what_it_was_asked) = the_run_meeting(Vec::new(), true);

    assert_eq!(
        how_it_went.which_way_it_failed().unwrap(),
        WhichWayItFailed::TheDestinationAlreadyHoldsSomething,
        "**because a run resumes and the directory is derived, so this is a decision made before git was run at all** — a refusal that arrives after the damage is not a refusal, and a run that has already written over a resumed run's clone has not refused anything"
    );
    assert!(
        what_it_was_asked.what_it_was_asked.borrow().is_empty(),
        "**and git was never run at all**, because a double with no answers left would have failed loudly and a real program would have copied a repository before anybody decided it should not"
    );
}

#[test]
fn gits_not_found_is_its_own_way_and_not_gits_discontent() {
    let (how_it_went, _) = the_run_meeting(git_saying("not found"), false);

    assert_eq!(
        how_it_went.which_way_it_failed().unwrap(),
        WhichWayItFailed::NoSuchRepository,
        "**because somebody named a repository that is not there, and that is a wrong address rather than a broken network** — the two fixes are not the same and a caller told only that it failed has learned nothing"
    );
}

#[test]
fn a_repository_nobody_committed_to_is_not_a_repository_that_is_not_there() {
    let (how_it_went, _) = the_run_meeting(git_saying("empty repository"), false);

    assert_eq!(
        how_it_went.which_way_it_failed().unwrap(),
        WhichWayItFailed::NothingCommittedToItYet,
        "**because somebody created the repository and has not committed to it, and that is a real repository with nothing in it** — the same address and a different fact, and a value read out of a command that failed is that command's complaint wearing a field's name"
    );
}

#[test]
fn a_complaint_this_system_does_not_name_becomes_gits_discontent() {
    let (how_it_went, _) = the_run_meeting(git_saying("could not resolve host"), false);

    assert_eq!(
        how_it_went.which_way_it_failed().unwrap(),
        WhichWayItFailed::GitIsUnhappy,
        "**and not a guess** — this system names two of git's complaints and refuses to invent a fifth way, so a word it does not recognise becomes the way that means *something, and not something we can tell you*"
    );
}

#[test]
fn what_git_said_is_carried_and_not_summarised() {
    let (how_it_went, _) = the_run_meeting(git_saying("could not resolve host"), false);

    assert_eq!(
        how_it_went.why_it_failed().unwrap().what_git_said(),
        "could not resolve host",
        "**because a value read out of a command that failed is that command's complaint wearing a field's name** — and summarising it here is the one thing that would leave the caller unable to tell git's trouble from ours"
    );
}

#[test]
fn the_commit_and_the_ref_are_asked_of_git_and_never_remembered() {
    let (how_it_went, what_it_was_asked) = the_run_meeting(
        vec![
            WhatTheProgramSaid::of(true, "Cloning into 'a-destination'..."),
            WhatTheProgramSaid::of(true, "e1c2d3"),
            WhatTheProgramSaid::of(true, "HEAD"),
        ],
        false,
    );

    let what_it_was_asked = what_it_was_asked.what_it_was_asked.borrow();
    assert!(
        how_it_went.the_clone().is_some(),
        "**because a run that asked the three questions and then refused has still refused** — and a test that only counted the questions would call that a pass"
    );
    assert_eq!(
        what_it_was_asked.len(),
        3,
        "**because a clone, a commit and a ref are three questions and not one** — and a run that filled the commit in from an earlier run would report its changes against a commit it stopped at days ago"
    );
    assert!(
        what_it_was_asked[1].is_the_commit_of_the_clone().is_some()
            && what_it_was_asked[2].is_the_ref_of_the_clone().is_some(),
        "**and the last two ask what the clone is standing on**, because a value read out of the clone is the only one that is about the commit the clone is actually on"
    );
}

#[test]
fn a_clone_that_succeeded_is_a_clone_and_the_commit_and_the_ref_it_stands_on() {
    let (how_it_went, _) = the_run_meeting(
        vec![
            WhatTheProgramSaid::of(true, "Cloning into 'a-destination'..."),
            WhatTheProgramSaid::of(true, "e1c2d3"),
            WhatTheProgramSaid::of(true, "HEAD"),
        ],
        false,
    );

    let the_clone = how_it_went
        .the_clone()
        .expect("a clone that worked is a clone");
    assert_eq!(
        the_clone.the_commit().as_str(),
        "e1c2d3",
        "**and the commit is what git said about the clone, not what the run expected**"
    );
    assert_eq!(
        the_clone.ref_name().as_str(),
        "HEAD",
        "**and the ref may be HEAD**, because a detached clone has no branch and naming the field `branch` would be a lie in exactly the case where the value matters"
    );
}
