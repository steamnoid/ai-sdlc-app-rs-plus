//! The project on disk, answering the questions `TheProjectOnDisk` asks.
//!
//! **The only crate that names a concrete thing, and a change here changes what the world
//! does.** Everything else names this trait.
//!
//! **A listing is sorted and project-relative.** Two runs of the same code over the same
//! directory must return the same list in the same order, or a model's answer to the same
//! question varies and nothing downstream can tell a changed repository from a reshuffled one.

use std::path::{Path, PathBuf};

use aisdlc_run::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::TheProjectOnDisk;

/// Directories that are a checkout's own business and not the repository's.
const WHAT_IS_NOT_THE_REPOSITORY: [&str; 2] = [".git", "target"];

/// A project, read from one directory.
pub struct TheProjectAsItIsOnDisk {
    the_root: PathBuf,
}

impl TheProjectAsItIsOnDisk {
    /// The project at one directory on a real disk.
    pub fn at(the_root: &Path) -> Self {
        Self {
            the_root: the_root.to_path_buf(),
        }
    }
}

impl TheProjectOnDisk for TheProjectAsItIsOnDisk {
    /// The directory it was opened at, which is the one thing a project on a disk always has.
    fn where_it_lives(&self) -> &Path {
        &self.the_root
    }

    /// The names of everything in it, relative to the project and sorted.
    fn what_is_in_it(&mut self) -> Vec<String> {
        let mut found = Vec::new();
        collect_below(&self.the_root, &self.the_root, &mut found);
        found.sort();
        found
    }

    /// What one file says, read from where the path resolved to.
    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String> {
        std::fs::read_to_string(a_path.as_str()).ok()
    }
}

/// The names of everything below one directory, as paths relative to the project.
fn collect_below(the_root: &Path, the_directory: &Path, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(the_directory) else {
        return;
    };
    for one_entry in entries.flatten() {
        if WHAT_IS_NOT_THE_REPOSITORY.contains(&one_entry.file_name().to_string_lossy().as_ref()) {
            continue;
        }
        let the_path = one_entry.path();
        if the_path.is_dir() {
            collect_below(the_root, &the_path, found);
            continue;
        }
        if let Ok(the_name) = the_path.strip_prefix(the_root) {
            found.push(the_name.to_string_lossy().to_string());
        }
    }
}
