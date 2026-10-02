//! A project on a real disk, changed, **and the one boundary that needs no credential and no
//! container to hold.**
//!
//! **A read and a write are two ports and not one.** `TheProjectOnDisk` says *"Two questions and
//! no others: what is in it, and what does one file say"* — **so a run that could write through
//! it would be a run whose reading boundary nobody could state**, and a repository this system
//! writes into is somebody else's working tree.
//!
//! **Nothing of ours goes inside the clone except the change itself**, and a path that steps out
//! of the clone is a file in somebody else's project — **so the refusal happens before any byte
//! is written**, where a person can be told.

use std::path::{Path, PathBuf};

use aisdlc_run::a_record_of_a_change::{AChangeIsRecorded, AFileThatWasNotWritten};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;

/// A project on a real disk, **and the only place this system writes a file.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheProjectAsItIsChanged {
    where_the_clone_lands: PathBuf,
}

impl TheProjectAsItIsChanged {
    /// The project a clone was made at, **and nothing else — the clone's place is the boundary.**
    ///
    /// **The clone by reference and not by value**, because a caller that still wants its clone
    /// afterwards has to keep it, **and handing this over by value would be a way of taking the
    /// only record of where a run's working tree is away from the thing that still needs it.**
    pub fn the_project_over_the_clone_at(of_which_clone: &ARepositoryToClone) -> Self {
        Self {
            where_the_clone_lands: of_which_clone.where_it_lands().to_path_buf(),
        }
    }

    /// Where the project lives, **and every project has a place.**
    pub fn where_it_lives(&self) -> &Path {
        &self.where_the_clone_lands
    }
}

impl AChangeIsRecorded for TheProjectAsItIsChanged {
    /// What one file says, **and nothing when the file is not there.**
    fn what_it_holds_of(&self, which_file: &str) -> Option<String> {
        std::fs::read_to_string(self.inside_the_clone(which_file)?).ok()
    }

    /// Write one file, **and the directory is made because a plan may name a test in a directory
    /// that does not exist yet.**
    ///
    /// **A refusal and not a truncated path**, because a path that steps out of the clone is a
    /// file in somebody else's project **and a writer that quietly shortened it would be writing
    /// somewhere nobody named.**
    fn one_file_is_written(
        &mut self,
        which_file: &str,
        what_it_says: &str,
    ) -> Result<(), AFileThatWasNotWritten> {
        let Some(where_it_lands) = self.inside_the_clone(which_file) else {
            return Err(AFileThatWasNotWritten(String::from(which_file)));
        };
        if let Some(the_directory) = where_it_lands.parent() {
            let _ = std::fs::create_dir_all(the_directory);
        }
        std::fs::write(where_it_lands, what_it_says)
            .map_err(|_| AFileThatWasNotWritten(String::from(which_file)))
    }
}

impl TheProjectAsItIsChanged {
    /// Where a named path lands, **and nothing at all when it would land outside the clone.**
    ///
    /// **`..` is refused by name and not by comparison, and the first version of this compared.**
    /// `Path::join` keeps the `..` in the result and `starts_with` compares components, **so
    /// `<clone>/../a-file` starts with `<clone>` and every such write passed the check** — the
    /// boundary was a lexical comparison doing the job of a rule about where a file may land.
    ///
    /// **The rule and not a check of whether the place exists** — the same reasoning
    /// `WhetherAnythingStandsOnDisk` gives for refusing to start a run inside a place that holds
    /// something: **a place that is there and a place that is safe to write to are two
    /// different questions, and only one of them is this one.**
    fn inside_the_clone(&self, which_file: &str) -> Option<PathBuf> {
        if which_file.split('/').any(|one| one == "..") {
            return None;
        }
        let the_candidate = self.where_the_clone_lands.join(which_file);
        if the_candidate.starts_with(&self.where_the_clone_lands) {
            return Some(the_candidate);
        }
        None
    }
}
