//! A branch this run made, on a remote, or the two reasons it is not there.
//!
//! **Nothing in this repository pushes anything.** `git` is asked for a clone, a commit and
//! a ref and for `remote add upstream`, `the_push_address_of` has zero callers, and
//! `ABranchToPush`, `APushedBranch`, `TheBranchIsAlreadyOnTheRemote` and
//! `TheRemoteRefusedThePush` are named in the glossary and unbuilt. **A run that cannot put
//! a branch on a remote cannot offer a pull request, and a pull request is the whole of
//! what a delivered run leaves behind.**
//!
//! **`ls-remote` is asked before anything is pushed, and the order is the rule.** It reads
//! the remote without changing it, which is what makes the refusal possible *before* rather
//! than after — **and a push that has already landed is not something to undo.**
//!
//! **`HEAD:refs/heads/<name>` and not the local branch name.** Pushing the branch the
//! checkout is standing on would move *that* branch on the remote, and the branch the
//! checkout is standing on is somebody else's `main`.
//!
//! **The commit is taken from the clone and not asked for.** A caller that could name the
//! commit could push a commit it has not read, **and every later claim about what was
//! delivered would be about something else.**

use aisdlc_domain::a_clone::{ACloneOnDisk, TheCommit, TheRef};
use aisdlc_domain::repository::Repository;
use aisdlc_run::a_branch_goes_to_a_remote::{
    ABranchToPush, APushedBranch, WhyABranchDidNotGo, a_branch_is_pushed,
};
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// The repository a delivery is pushed to, **and its push address.**
fn the_repository_it_is_pushed_to() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

/// A clone standing on a commit, which is where the commit comes from.
fn a_clone_standing_on(its_commit: &str) -> ACloneOnDisk {
    ACloneOnDisk::the_clone_at(
        std::path::PathBuf::from("/tmp/a-clone-this-run-made"),
        TheCommit::try_from(its_commit).expect("a commit is a name"),
        TheRef::try_from("main").expect("main is a ref"),
    )
}

/// A branch a run wants, **named for the work item that made it.**
fn a_branch_this_run_wants() -> ABranchToPush {
    ABranchToPush::a_branch_named("aisdlc/WELLBEINGT-42").expect("a name a git branch may take")
}

/// A program that answers each request in turn, **and remembers what it was asked.**
struct AProgramAnsweringInTurn {
    what_the_remote_already_holds: String,
    the_push_words: String,
    what_it_was_asked: std::cell::RefCell<Vec<WhatWeTellAProgramToDo>>,
}

impl AProgramAnsweringInTurn {
    /// A program whose `ls-remote` answers this, and whose push answers the other thing.
    fn the_remote_holding(what_the_remote_already_holds: &str) -> Self {
        Self {
            what_the_remote_already_holds: what_the_remote_already_holds.to_string(),
            the_push_words: String::from("To github.com:steamnoid/wellbeing-tracker-public.git"),
            what_it_was_asked: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AProgramWeRun for AProgramAnsweringInTurn {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.what_it_was_asked
            .borrow_mut()
            .push(what_it_is_asked_to_do.clone());
        if what_it_is_asked_to_do.does_this_change_the_remote() {
            return WhatTheProgramSaid::of(false, &self.the_push_words);
        }
        WhatTheProgramSaid::of(true, &self.what_the_remote_already_holds)
    }
}

/// Every question this program was asked, **in the order it was asked them.**
fn every_question_asked_of(a_program: &AProgramAnsweringInTurn) -> Vec<String> {
    a_program
        .what_it_was_asked
        .borrow()
        .iter()
        .map(|one| format!("{one:?}"))
        .collect()
}

#[test]
fn the_branch_is_asked_about_before_it_is_pushed() {
    let the_program = AProgramAnsweringInTurn::the_remote_holding("");

    let _ = a_branch_is_pushed(
        &the_program,
        &a_clone_standing_on("9f1c2a4"),
        &a_branch_this_run_wants(),
    );

    let every_question = every_question_asked_of(&the_program);
    let the_first_is_a_reading = every_question
        .first()
        .is_some_and(|one| one.contains("ls-remote"));
    assert!(
        the_first_is_a_reading && every_question.len() == 2,
        "**because `ls-remote` reads the remote without changing it, which is what makes the \
         refusal possible before rather than after — and a push that has already landed is not \
         something to undo.** The run asked, in order: {every_question:?}"
    );
}

#[test]
fn a_branch_somebody_else_is_using_is_refused_and_quotes_what_the_remote_holds() {
    let what_the_remote_holds = "9f1c2a4b\trefs/heads/aisdlc/WELLBEINGT-42";
    let the_program = AProgramAnsweringInTurn::the_remote_holding(what_the_remote_holds);

    let the_refusal = a_branch_is_pushed(
        &the_program,
        &a_clone_standing_on("9f1c2a4"),
        &a_branch_this_run_wants(),
    );

    assert_eq!(
        the_refusal,
        Err(WhyABranchDidNotGo::TheBranchIsAlreadyOnTheRemote {
            what_the_remote_holds: what_the_remote_holds.to_string()
        }),
        "**because this run does not move a branch it did not create, and that is the case with \
         teeth** — 'nothing to push' describes only a branch that already holds this run's own \
         commit, and says nothing about a branch somebody else is using. The refusal quotes what \
         the remote holds, **because a person can only answer 'whose branch is that' if they are \
         told it.** A second run for one filing is ordinary, and the earlier branch is that run's"
    );
}

#[test]
fn a_remote_that_refuses_the_push_is_refused_with_gits_own_words() {
    let the_program = AProgramAnsweringInTurn::the_remote_holding("");
    let mut the_program_that_fails = the_program;
    the_program_that_fails.the_push_words = String::from("Permission denied (publickey).");

    let the_refusal = a_branch_is_pushed(
        &the_program_that_fails,
        &a_clone_standing_on("9f1c2a4"),
        &a_branch_this_run_wants(),
    );

    assert_eq!(
        the_refusal,
        Err(WhyABranchDidNotGo::TheRemoteRefusedThePush {
            what_git_said: "Permission denied (publickey).".to_string()
        }),
        "**because the two need different answers from a person: a branch that already existed is \
         this run's own doing, and a remote that is not there is not.** One refusal would make a \
         person read a name of a different thing, **and the words git wrote are the only thing that \
         says which.** Quoted, and not summarised"
    );
}

#[test]
fn a_branch_that_went_comes_back_with_the_address_it_went_to() {
    let the_program = AProgramAnsweringInTurn::the_remote_holding("");

    let what_was_pushed = a_branch_is_pushed(
        &the_program,
        &a_clone_standing_on("9f1c2a4"),
        &a_branch_this_run_wants(),
    );

    assert_eq!(
        what_was_pushed,
        Ok(APushedBranch {
            which_branch: "aisdlc/WELLBEINGT-42".to_string(),
            where_it_went: "git@github.com:steamnoid/wellbeing-tracker-public.git".to_string(),
        }),
        "**because a caller can go and look at it rather than take the tool's word for it** — a \
         branch that was pushed and not recorded is a delivery nobody can find, and the address is \
         the push address of the repository **rather than one composed from what the run believed.**"
    );
}

#[test]
fn the_branch_goes_as_head_to_refs_and_never_as_the_ref_the_checkout_stands_on() {
    let the_program = AProgramAnsweringInTurn::the_remote_holding("");

    let _ = a_branch_is_pushed(
        &the_program,
        &a_clone_standing_on("9f1c2a4"),
        &a_branch_this_run_wants(),
    );

    let what_the_push_was = every_question_asked_of(&the_program)
        .last()
        .cloned()
        .expect("a push was asked for");
    assert!(
        what_the_push_was.contains("HEAD:refs/heads/aisdlc/WELLBEINGT-42")
            && !what_the_push_was.contains("\"main\""),
        "**because pushing the branch the checkout is standing on would move *that* branch on the \
         remote** — and the branch the checkout is standing on is somebody else's `main`. The push \
         was: {what_the_push_was}"
    );
}

#[test]
fn a_branch_name_that_git_would_refuse_is_refused_before_any_program_runs() {
    let the_program = AProgramAnsweringInTurn::the_remote_holding("");

    let the_refusal = a_branch_this_run_wants_named("aisdlc/--not-a-branch");

    assert_eq!(
        the_refusal,
        Err(ANameABranchCannotTake(String::from(
            "aisdlc/--not-a-branch"
        ))),
        "**because a branch name is quoted into a command whose shape is fixed: nothing a caller \
         supplies reaches the program as an option** — and a name git would refuse has to be \
         refused here, where a person can be told, rather than as a program that said no. The \
         program was asked nothing: {}",
        every_question_asked_of(&the_program).len()
    );
}

/// A branch a run wants, **or the refusal when git would not take the name at all.**
fn a_branch_this_run_wants_named(which_branch: &str) -> Result<ABranchToPush, ANameABranchCannotTake> {
    ABranchToPush::a_branch_named(which_branch)
}

/// A name a branch cannot take, **quoted whole.**
#[derive(Debug, Clone, PartialEq, Eq)]
struct ANameABranchCannotTake(String);
