//! The project on disk, and nothing else the world can be asked about.
//!
//! **Two questions and no others: what is in it, and what does one file say.** The
//! implementation is the local filesystem, in `aisdlc-outside`.
//!
//! **A file is asked for by a path that already landed inside the project**, so the shape rule
//! and the confinement rule are applied before this trait is ever called and a caller cannot
//! ask to read outside the project by handing over a string.

use crate::the_path_inside_the_project::ThePathInsideTheProject;

/// A project a run reads from.
pub trait TheProjectOnDisk {
    /// The names of everything in it.
    fn what_is_in_it(&mut self) -> Vec<String>;

    /// What one file says, or nothing when it is not there.
    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String>;
}

/// A file the project does not have, named for what was asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFileTheProjectDoesNotHave {
    what_was_asked_for: String,
}

impl AFileTheProjectDoesNotHave {
    /// Which file was asked for.
    pub fn what_was_asked_for(&self) -> &str {
        &self.what_was_asked_for
    }

    /// This refusal, for a file that is not there.
    pub fn because(a_path: &ThePathInsideTheProject) -> Self {
        Self {
            what_was_asked_for: a_path.as_str().to_string(),
        }
    }
}
