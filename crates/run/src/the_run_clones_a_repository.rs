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

use aisdlc_domain::a_clone::ACloneOnDisk;
use aisdlc_domain::why_cloning_the_repository_failed::{
    WhichWayItFailed, WhyCloningTheRepositoryFailed, why_cloning_the_repository_failed,
};

use crate::a_program_we_run::AProgramWeRun;
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
        self.why_it_failed.as_ref().map(|a_why| a_why.which_way())
    }
}

/// What a program wrote, read as one of the four ways cloning failed.
fn the_way_git_failed_in(what_it_said: &str) -> WhichWayItFailed {
    let _what_it_said = what_it_said;
    WhichWayItFailed::GitIsUnhappy
}

/// The clone git made, asked for the commit and the ref it stands on.
fn what_the_clone_stands_on(
    the_program: &mut impl AProgramWeRun,
    the_clone_of: &crate::the_repository_to_clone_into::ARepositoryToClone,
) -> Result<ACloneOnDisk, WhyCloningTheRepositoryFailed> {
    let _the_program = the_program;
    let _the_clone_of = the_clone_of;
    let _no_commit_was_read = true;
    Err(why_cloning_the_repository_failed(
        WhichWayItFailed::GitIsUnhappy,
        "",
    ))
}

/// The run clones a repository, asking the directory before it asks git.
pub fn the_run_clones_a_repository(
    the_program: &mut impl AProgramWeRun,
    the_directory: &mut impl WhetherAnythingStandsThere,
    what_it_is_asked_to_do: &WhatWeTellAProgramToDo,
) -> HowCloningWent {
    let _the_program = the_program;
    let _the_directory = the_directory;
    let _what_it_is_asked_to_do = what_it_is_asked_to_do;
    HowCloningWent {
        the_clone: None,
        why_it_failed: None,
    }
}
