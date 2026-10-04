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
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};

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

    /// Whether anything at all stands at the path, **and it is the disk answering — which is the
    /// only answer worth having, because this is the one project that is a real directory.**
    fn does_anything_stand_at(&self, a_path: &Path) -> bool {
        a_path.exists()
    }

    /// What one file says, read from where the path resolved to.
    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String> {
        std::fs::read_to_string(a_path.as_str()).ok()
    }

    /// One file is written, **and it is refused when one is already there.**
    ///
    /// **The check is `create_new` and not `write`,** because `create_new` is the only one of the two
    /// that fails when the file exists — **and a run writes an artifact once per version, so a second
    /// write at that version would replace the document an approval still names.**
    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        let opened = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(a_path.as_str());
        let the_file = opened.map_err(|_there_is_one| there_is_one(a_path))?;
        the_words_are_written_into(a_path, the_file, what_it_says)
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

/// There is already a file there, **and it is one refusal so the caller above reads as two steps
/// rather than as a match on the filesystem's error.**
fn there_is_one(a_path: &ThePathInsideTheProject) -> TheProjectRefusedToWrite {
    TheProjectRefusedToWrite::there_is_already_such_a_file(a_path)
}

/// The words are written into a file that was just created, **and a refusal here is not a refusal to
/// open** — the file exists and is empty, **and that is the state a reader would find if the write
/// is retried.**
fn the_words_are_written_into(
    a_path: &ThePathInsideTheProject,
    mut the_file: std::fs::File,
    what_it_says: &str,
) -> Result<(), TheProjectRefusedToWrite> {
    use std::io::Write;
    let written = the_file.write_all(what_it_says.as_bytes());
    written.map_err(|no_room| there_was_no_room(a_path, &no_room.to_string()))
}

/// The disk would not take the words, **and its own sentence travels whole** — "no space left on
/// device" and "permission denied" are different answers and a run can act on neither without
/// knowing which.
fn there_was_no_room(
    a_path: &ThePathInsideTheProject,
    what_it_said: &str,
) -> TheProjectRefusedToWrite {
    let the_filesystem_said = format!("the filesystem said: {what_it_said}");
    TheProjectRefusedToWrite::there_was_no_room_for_it(a_path, &the_filesystem_said)
}
