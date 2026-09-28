//! Why cloning a repository failed, in four ways, and the caller can tell them apart.
//!
//! **The last two are this system's own findings** and both are about a value read out of a
//! command that failed: a directory that already holds something, and a repository somebody
//! created and has not committed to.
//!
//! **"No such repository" quotes git's own words inside ours**, because the caller has to tell
//! *no such repository* from *git is unhappy* from *git is not installed*. Three things that
//! feel alike and are not, and a refusal that collapses them makes the caller guess.

use std::error::Error;
use std::fmt;

/// Which way cloning failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichWayItFailed {
    /// The place the clone would land already holds something.
    TheDestinationAlreadyHoldsSomething,
    /// There is no such repository.
    NoSuchRepository,
    /// Git refused, for a reason this system does not name.
    GitIsUnhappy,
    /// Somebody created the repository and has not committed to it.
    NothingCommittedToItYet,
}

impl WhichWayItFailed {
    /// Every way, in the order the glossary gives them.
    pub fn every_way() -> Vec<WhichWayItFailed> {
        vec![
            WhichWayItFailed::TheDestinationAlreadyHoldsSomething,
            WhichWayItFailed::NoSuchRepository,
            WhichWayItFailed::GitIsUnhappy,
            WhichWayItFailed::NothingCommittedToItYet,
        ]
    }
}

/// Why cloning failed, in one of four ways, carrying whatever git said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhyCloningTheRepositoryFailed {
    which_way: WhichWayItFailed,
    what_git_said: String,
}

impl WhyCloningTheRepositoryFailed {
    /// Which way it failed.
    pub fn which_way(&self) -> WhichWayItFailed {
        self.which_way
    }

    /// What git said, exactly, and nothing when git was not the one who refused.
    pub fn what_git_said(&self) -> &str {
        &self.what_git_said
    }
}

impl Error for WhyCloningTheRepositoryFailed {}

impl fmt::Display for WhyCloningTheRepositoryFailed {
    /// Write the refusal, naming which way it was and what git said.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?}, and git said: {}",
            self.which_way, self.what_git_said
        )
    }
}

/// The refusal, in one of four ways.
pub fn why_cloning_the_repository_failed(
    which_way: WhichWayItFailed,
    what_git_said: &str,
) -> WhyCloningTheRepositoryFailed {
    WhyCloningTheRepositoryFailed {
        which_way,
        what_git_said: what_git_said.to_string(),
    }
}
