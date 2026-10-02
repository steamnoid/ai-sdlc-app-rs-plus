//! A branch this run made, on a remote, or the two named reasons it is not there.
//!
//! **A delivery is a git operation and so it goes through `AProgramWeRun` and not the
//! GitHub API.** The clone is here, the commit is in it, and pushing it is what `git` is
//! for — **and a pull request opened against a branch that was never pushed is a refusal
//! from the far end, two steps later, for a cause that is a hundred steps back.**
//!
//! **The remote is read before it is written, and that order is the whole rule.** Anything
//! else makes the refusal happen after the push, **and a push that has already landed is
//! not something to undo.**

use aisdlc_domain::a_clone::TheCommit;
use aisdlc_domain::the_addresses::the_push_address_of;
use aisdlc_domain::work_item_id::WorkItemId;

use crate::a_program_we_run::AProgramWeRun;
use crate::the_repository_to_clone_into::ARepositoryToClone;
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// The branch a run wants and the commit it points at.
///
/// **The commit is read from the clone and not asked for.** A caller that could name the
/// commit could push a commit it has not read, **and every later claim about what was
/// delivered would be about something else.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABranchToPush {
    which_branch: String,
    the_commit: TheCommit,
}

impl ABranchToPush {
    /// The branch a run wants, **refused when the name is one no branch can take.**
    ///
    /// **Refused here and not by the program, because a caller reading a program failure
    /// cannot tell its own naming mistake from a remote that is not there.**
    pub fn a_branch_named(which_branch: &str) -> Result<Self, ANameABranchCannotTake> {
        if !a_git_branch_may_be_named(which_branch) {
            return Err(ANameABranchCannotTake(which_branch.to_string()));
        }
        Ok(Self {
            which_branch: which_branch.to_string(),
            the_commit: TheCommit::try_from("HEAD").expect("a ref is a name"),
        })
    }

    /// The branch this run wants for one work item, **at the commit its clone is standing on.**
    ///
    /// **Both halves come from the run and neither is typed by a caller choosing them:** the name
    /// is derived from the work item and the commit is the one `git` reported. A caller that could
    /// name the commit could push a commit it has not read, **and every later claim about what was
    /// delivered would be about something else.**
    pub fn the_branch_this_run_wants(
        of_which_work_item: &WorkItemId,
        the_commit_the_clone_stands_on: &TheCommit,
    ) -> Result<Self, ANameABranchCannotTake> {
        let mut which_branch = Self::a_branch_named(&format!("aisdlc/{of_which_work_item}"))?;
        which_branch.the_commit = the_commit_the_clone_stands_on.clone();
        Ok(which_branch)
    }

    /// Which branch, word for word as a caller named it.
    pub fn which_branch(&self) -> &str {
        &self.which_branch
    }

    /// The commit the clone stands on, **which is the one this pushes and no other.**
    pub fn the_commit_it_points_at(&self) -> &TheCommit {
        &self.the_commit
    }
}

/// A branch that is on the remote, **with the address it went to.**
///
/// **The address and not a confirmation, so a caller can go and look at it rather than take
/// the tool's word for it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APushedBranch {
    /// Which branch is there.
    pub which_branch: String,
    /// Where it went, word for word as the address was composed.
    pub where_it_went: String,
}

/// Why a branch a run wanted is not on the remote.
///
/// **One type and two answers, because one rule produces both cases and a name covering
/// one of them is trusted for the other.** The two need different answers from a person: a
/// branch that already existed is this run's own doing, and a remote that is not there is
/// not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyABranchDidNotGo {
    /// The remote already carries this name, **quoted whole so a person can answer whose
    /// branch it is.**
    TheBranchIsAlreadyOnTheRemote {
        /// What `ls-remote` wrote, which is the only record of whose branch it is.
        what_the_remote_holds: String,
    },
    /// `git` refused, **and its own words are the only thing that says which refusal.**
    TheRemoteRefusedThePush {
        /// What `git` wrote when it would not push.
        what_git_said: String,
    },
}

impl std::fmt::Display for WhyABranchDidNotGo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhyABranchDidNotGo::TheBranchIsAlreadyOnTheRemote {
                what_the_remote_holds,
            } => write!(
                formatter,
                "this run does not move a branch it did not create, and the remote already holds \
                 one under this name: {what_the_remote_holds}"
            ),
            WhyABranchDidNotGo::TheRemoteRefusedThePush { what_git_said } => write!(
                formatter,
                "the remote would not take the branch, and said: {what_git_said}"
            ),
        }
    }
}

impl std::error::Error for WhyABranchDidNotGo {}

/// A name no branch can carry, **quoted whole.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ANameABranchCannotTake(pub String);

impl std::fmt::Display for ANameABranchCannotTake {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} is a name no branch can take, and a branch name is quoted into a command whose \
             shape is fixed — so it is refused here, where a person can be told, and not as a \
             program that said no",
            self.0
        )
    }
}

impl std::error::Error for ANameABranchCannotTake {}

/// The characters `git` refuses anywhere in a reference name, **and a name that would be read
/// as an option rather than as a name.**
///
/// **A leading dash and not only the forbidden characters, because that is the half the rule is
/// about:** a name reaching the program as `-something` is an option and not a branch, **and a
/// command line whose shape is fixed is the only thing standing between a work item's name and
/// this system's own options.**
fn a_git_branch_may_be_named(which_branch: &str) -> bool {
    if which_branch.trim().is_empty() || which_branch.starts_with('-') {
        return false;
    }
    if which_branch.contains(char::is_whitespace) || which_branch.contains("..") {
        return false;
    }
    !which_branch.contains(['~', '^', ':', '?', '*', '[', '\\'])
}

/// The branch is read about, then pushed, **or the two named reasons it is not there.**
///
/// **The refspec names `HEAD` and not the commit, so the commit on `ABranchToPush` is a record
/// rather than an instruction.** Pushing a commit a caller wrote out would let a run deliver
/// something it never read, **and `HEAD:refs/heads/<name>` is also what stops a delivery moving
/// the branch the checkout is standing on.**
pub fn a_branch_is_pushed(
    the_program: &impl AProgramWeRun,
    of_which_clone: &ARepositoryToClone,
    which_branch: &ABranchToPush,
) -> Result<APushedBranch, WhyABranchDidNotGo> {
    the_branch_is_not_there_yet(the_program, of_which_clone, which_branch)?;
    the_branch_was_taken(the_program, of_which_clone, which_branch)?;
    Ok(APushedBranch {
        which_branch: which_branch.which_branch().to_string(),
        where_it_went: the_push_address_of(of_which_clone.which_repository()),
    })
}

/// The reading, and the refusal when the remote already carries this name.
///
/// **Anything at all on the remote is a refusal, and the words are quoted whole** — a branch
/// somebody else is using is the case with teeth, **and a person can only answer "whose branch is
/// that" if they are told it.**
fn the_branch_is_not_there_yet(
    the_program: &impl AProgramWeRun,
    of_which_clone: &ARepositoryToClone,
    which_branch: &ABranchToPush,
) -> Result<(), WhyABranchDidNotGo> {
    let what_the_remote_holds = the_program.run(&WhatWeTellAProgramToDo::read_the_remote(
        of_which_clone.clone(),
        which_branch.which_branch(),
    ));
    let what_it_holds = what_the_remote_holds.the_words().trim();
    if what_it_holds.is_empty() {
        return Ok(());
    }
    Err(WhyABranchDidNotGo::TheBranchIsAlreadyOnTheRemote {
        what_the_remote_holds: what_it_holds.to_string(),
    })
}

/// The push, and the refusal when `git` would not take it.
///
/// **`did_it_work` and not what it looked happy about, and the words are quoted** — a program
/// that wrote a warning and returned zero did work, **and what `git` wrote is the only thing
/// that says which of its many refusals this was.**
fn the_branch_was_taken(
    the_program: &impl AProgramWeRun,
    of_which_clone: &ARepositoryToClone,
    which_branch: &ABranchToPush,
) -> Result<(), WhyABranchDidNotGo> {
    let what_git_said = the_program.run(&WhatWeTellAProgramToDo::push_the_branch(
        of_which_clone.clone(),
        which_branch.which_branch(),
    ));
    if what_git_said.did_it_work() {
        return Ok(());
    }
    Err(WhyABranchDidNotGo::TheRemoteRefusedThePush {
        what_git_said: what_git_said.the_words().trim().to_string(),
    })
}
