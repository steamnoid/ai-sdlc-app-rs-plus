//! The run clones a repository, and says which of the four ways it could not when it could not.
//!
//! **The directory is asked about before git is run, and that order is the rule.** A run resumes
//! and the directory is derived, so a destination that already holds something is a decision made
//! before the damage rather than a finding made after it — **a refusal that arrives after the
//! damage is not a refusal.**
//!
//! **The commit and the ref are read out of the clone and never remembered.** A value kept from
//! an earlier run is a value about the wrong commit, and a run that resumed believing it knows
//! which commit it has would report its changes against one it stopped at days ago.

use aisdlc_domain::a_clone::{ACloneOnDisk, TheCommit, TheRef};
use aisdlc_domain::why_cloning_the_repository_failed::{
    WhichWayItFailed, WhyCloningTheRepositoryFailed, why_cloning_the_repository_failed,
};

use crate::a_program_we_run::AProgramWeRun;
use crate::the_repository_to_clone_into::ARepositoryToClone;
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use crate::whether_anything_stands_there::WhetherAnythingStandsThere;

/// How cloning went, and why it failed when it did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowCloningWent {
    the_clone: Option<ACloneOnDisk>,
    why_it_failed: Option<WhyCloningTheRepositoryFailed>,
}

impl HowCloningWent {
    /// The clone, and nothing when it failed.
    pub fn the_clone(&self) -> Option<&ACloneOnDisk> {
        self.the_clone.as_ref()
    }

    /// Why it failed, and nothing when it worked.
    pub fn why_it_failed(&self) -> Option<&WhyCloningTheRepositoryFailed> {
        self.why_it_failed.as_ref()
    }

    /// Which way it failed, and nothing when it worked.
    pub fn which_way_it_failed(&self) -> Option<WhichWayItFailed> {
        self.why_it_failed
            .as_ref()
            .map(WhyCloningTheRepositoryFailed::which_way)
    }

    /// A clone that was made, and nothing failed.
    fn made(the_clone: ACloneOnDisk) -> Self {
        Self {
            the_clone: Some(the_clone),
            why_it_failed: None,
        }
    }

    /// A refusal, for a caller that has a direction and git's words.
    fn refused(why_it_failed: WhyCloningTheRepositoryFailed) -> Self {
        Self {
            the_clone: None,
            why_it_failed: Some(why_it_failed),
        }
    }
}

/// A refusal, named by which way it was and by whatever git said.
fn a_refusal_because(which_way: WhichWayItFailed, what_git_said: &str) -> HowCloningWent {
    HowCloningWent::refused(why_cloning_the_repository_failed(which_way, what_git_said))
}

/// What git wrote, read as one of the four ways cloning failed.
///
/// **Two words this system names and no more.** A complaint it does not recognise is
/// `GitIsUnhappy`, which means *something happened and it is not something we can tell you* — a
/// reader that matched every word would be maintaining a list of git's messages, and git writes
/// new ones.
fn the_way_git_failed_in(what_it_said: &str) -> WhichWayItFailed {
    if what_it_said.contains("not found") {
        return WhichWayItFailed::NoSuchRepository;
    }
    if what_it_said.contains("empty repository") {
        return WhichWayItFailed::NothingCommittedToItYet;
    }
    WhichWayItFailed::GitIsUnhappy
}

/// The commit git names, or a refusal naming the words it wrote instead.
fn a_commit_named_in(what_git_said: &str) -> Result<TheCommit, HowCloningWent> {
    TheCommit::try_from(what_git_said).map_err(|a_why| {
        a_refusal_because(WhichWayItFailed::GitIsUnhappy, a_why.what_was_written())
    })
}

/// The ref git names, or a refusal naming the words it wrote instead.
fn a_ref_named_in(what_git_said: &str) -> Result<TheRef, HowCloningWent> {
    TheRef::try_from(what_git_said).map_err(|a_why| {
        a_refusal_because(WhichWayItFailed::GitIsUnhappy, a_why.what_was_written())
    })
}

/// What git said about one question, or the refusal those words mean.
fn what_git_said_about(
    the_program: &mut impl AProgramWeRun,
    a_question: &WhatWeTellAProgramToDo,
) -> Result<String, HowCloningWent> {
    let what_it_said = the_program.run(a_question);
    if !what_it_said.did_it_work() {
        return Err(a_refusal_because(
            the_way_git_failed_in(what_it_said.the_words()),
            what_it_said.the_words(),
        ));
    }
    Ok(what_it_said.the_words().trim().to_string())
}

/// The commit and the ref, asked of git rather than remembered from a run that has ended.
fn the_commit_and_the_ref(
    the_program: &mut impl AProgramWeRun,
    the_clone_of: &ARepositoryToClone,
) -> Result<(TheCommit, TheRef), HowCloningWent> {
    let the_commit = what_git_said_about(
        the_program,
        &WhatWeTellAProgramToDo::the_commit_it_stands_on(the_clone_of.clone()),
    )?;
    let the_ref = what_git_said_about(
        the_program,
        &WhatWeTellAProgramToDo::the_ref_it_stands_on(the_clone_of.clone()),
    )?;
    Ok((a_commit_named_in(&the_commit)?, a_ref_named_in(&the_ref)?))
}

/// Git copied the repository, and then said what the copy is standing on.
fn what_the_clone_stands_on(
    the_program: &mut impl AProgramWeRun,
    the_clone_of: &ARepositoryToClone,
    the_request: &WhatWeTellAProgramToDo,
) -> Result<ACloneOnDisk, HowCloningWent> {
    what_git_said_about(the_program, the_request)?;
    let (commit, reference) = the_commit_and_the_ref(the_program, the_clone_of)?;
    Ok(ACloneOnDisk::the_clone_at(
        the_clone_of.where_it_lands().to_path_buf(),
        commit,
        reference,
    ))
}

/// What the destination and the request say before git is run, and nothing when they are content.
fn what_the_destination_says(
    the_directory: &impl WhetherAnythingStandsThere,
    what_it_is_asked_to_do: &WhatWeTellAProgramToDo,
    the_clone_of: &ARepositoryToClone,
) -> Option<HowCloningWent> {
    if the_directory.does_anything_stand_at(the_clone_of.where_it_lands()) {
        return Some(a_refusal_because(
            WhichWayItFailed::TheDestinationAlreadyHoldsSomething,
            "",
        ));
    }
    if !matches!(
        what_it_is_asked_to_do,
        WhatWeTellAProgramToDo::CloneTheRepository { .. }
    ) {
        return Some(a_refusal_because(
            WhichWayItFailed::GitIsUnhappy,
            "a question about a clone that has not been made",
        ));
    }
    None
}

/// The run clones a repository, asking the directory before it asks git.
///
/// **Three questions in order, and each one can stop the next.** Whether anything stands at the
/// destination, then whether git copied the repository, then what the copy is standing on — and a
/// run that skipped the first would overwrite a resumed run's clone to learn something git would
/// have told it anyway, ten seconds and one repository later.
pub fn the_run_makes_a_checkout(
    the_program: &mut impl AProgramWeRun,
    the_directory: &impl WhetherAnythingStandsThere,
    what_it_is_asked_to_do: &WhatWeTellAProgramToDo,
) -> HowCloningWent {
    let Some(the_clone_of) = what_it_is_asked_to_do.the_clone_it_is_about() else {
        return a_refusal_because(
            WhichWayItFailed::GitIsUnhappy,
            "a request that is not about a clone",
        );
    };
    if let Some(a_refusal) =
        what_the_destination_says(the_directory, what_it_is_asked_to_do, the_clone_of)
    {
        return a_refusal;
    }
    match what_the_clone_stands_on(the_program, the_clone_of, what_it_is_asked_to_do) {
        Ok(the_clone) => HowCloningWent::made(the_clone),
        Err(a_refusal) => a_refusal,
    }
}
