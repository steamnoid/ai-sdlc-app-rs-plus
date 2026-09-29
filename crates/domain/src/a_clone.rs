//! A checkout on disk, and the commit and the ref it is standing on.
//!
//! **Both are read from the clone and never remembered.** A value kept from an earlier run is a
//! value about the wrong commit — and a run that resumed believing it knows which commit it has
//! would report its changes against one it stopped at days ago.
//!
//! **`ref_name` and not `branch`, because a checkout is not always on a branch.** Cloning a
//! detached ref leaves none behind, and the honest answer for a checkout on none is `HEAD`.

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

/// The commit a clone is standing on, as `git` wrote it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TheCommit(String);

impl TheCommit {
    /// The commit, as it was written.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A commit that names nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACommitThatNamesNothing(String);

impl ACommitThatNamesNothing {
    /// What was written, quoted back, and the same door `ARefThatNamesNothing` has.
    ///
    /// **A method and not a public field, and the same as the ref's.** One of the two refusals
    /// gave its words away through the tuple and the other through a door, so a caller reporting
    /// either of them had to know which was which — **an asymmetry between two refusals that say
    /// the same kind of thing is a thing a reader has to notice rather than a thing a type states.**
    pub fn what_was_written(&self) -> &str {
        &self.0
    }
}

impl Error for ACommitThatNamesNothing {}

impl fmt::Display for ACommitThatNamesNothing {
    /// Write the refusal, quoting the input back.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} is not a commit, and a blank is the wrongest one there is",
            self.0
        )
    }
}

impl TryFrom<&str> for TheCommit {
    type Error = ACommitThatNamesNothing;

    /// Read a commit, refusing a blank.
    fn try_from(written: &str) -> Result<Self, Self::Error> {
        if written.trim().is_empty() {
            return Err(ACommitThatNamesNothing(written.to_string()));
        }
        Ok(TheCommit(written.to_string()))
    }
}

/// The ref a clone is standing on, which may be a branch and may be `HEAD`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TheRef(String);

impl TheRef {
    /// The ref, as it was written.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A ref that names nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARefThatNamesNothing(String);

impl ARefThatNamesNothing {
    /// The name that was written, exactly as it was written.
    pub fn what_was_written(&self) -> &str {
        &self.0
    }
}

impl Error for ARefThatNamesNothing {}

impl fmt::Display for ARefThatNamesNothing {
    /// Write the refusal, quoting the input back.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} is not a ref", self.0)
    }
}

impl TryFrom<&str> for TheRef {
    type Error = ARefThatNamesNothing;

    /// Read a ref, refusing a blank and **accepting `HEAD`**.
    ///
    /// **`HEAD` is a ref and not a branch**, and refusing it would be telling a caller to invent
    /// a branch name for a checkout that is on none.
    fn try_from(written: &str) -> Result<Self, Self::Error> {
        if written.trim().is_empty() {
            return Err(ARefThatNamesNothing(written.to_string()));
        }
        Ok(TheRef(written.to_string()))
    }
}

/// A checkout on disk, and what it is standing on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACloneOnDisk {
    where_it_lands: PathBuf,
    the_commit: TheCommit,
    ref_name: TheRef,
}

impl ACloneOnDisk {
    /// The clone, with the commit and the ref `git` reported for it.
    ///
    /// **Not a `Result`.** A clone that was read out of a clone always has both, and a
    /// `Result` here would be a door nobody has to go through — a failure a caller must handle
    /// for a case that cannot happen is a failure a caller will eventually stop reading.
    pub fn the_clone_at(where_it_lands: PathBuf, the_commit: TheCommit, ref_name: TheRef) -> Self {
        Self {
            where_it_lands,
            the_commit,
            ref_name,
        }
    }

    /// Where the clone is.
    pub fn where_it_lands(&self) -> &Path {
        &self.where_it_lands
    }

    /// The commit it is standing on, read from the clone.
    pub fn the_commit(&self) -> &TheCommit {
        &self.the_commit
    }

    /// The ref it is standing on, read from the clone, and `HEAD` when it is on no branch.
    pub fn ref_name(&self) -> &TheRef {
        &self.ref_name
    }
}
