//! The project on disk, and nothing else the world can be asked about.
//!
//! **Three questions and no others: what is in it, what does one file say, and may one file be
//! written.** The implementation is the local filesystem, in `aisdlc-outside`.
//!
//! **A file is asked for by a path that already landed inside the project**, so the shape rule
//! and the confinement rule are applied before this trait is ever called and a caller cannot
//! ask to read outside the project by handing over a string. **The same is true of a write**, and
//! that is the whole reason the writer takes a path rather than a directory and a name.

use crate::the_path_inside_the_project::ThePathInsideTheProject;

/// A project a run reads from **and writes to.**
pub trait TheProjectOnDisk {
    /// Where this project lives, and every project has a place.
    ///
    /// **A path and not nothing, because a project that cannot say where it is cannot be told
    /// it is the right one.** A run that made a checkout knows where it landed, and comparing
    /// the two is the only thing standing between a run and a listing of a directory it never
    /// cloned — **and every name in that listing would be a real name of some other project.**
    fn where_it_lives(&self) -> &std::path::Path;
    /// The names of everything in it.
    fn what_is_in_it(&mut self) -> Vec<String>;

    /// What one file says, or nothing when it is not there.
    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String>;

    /// One file is written, **and it is refused rather than replacing a file that is already
    /// there.**
    ///
    /// **A refusal and not an overwrite, and that is the whole of this method.** A run writes an
    /// artifact once per version, **and an approval names the exact artifact and version it ruled
    /// on** — so a second write at that version would replace the document under an approval that
    /// still names it, **and the file a person read and the file on disk would be different
    /// documents with one name.**
    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite>;
}

/// Why a project would not write a file, **and the two are different facts about the world.**
///
/// **A file that is already there and a file there is no room for are told apart**, because a run
/// answers them differently: **the first is refused forever and the second is worth trying again.**
/// One refusal naming both would send a reader after a permission problem when the cause was a full
/// disk, **and a reader who goes to the wrong file does not come back.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheProjectRefusedToWrite {
    /// There is already a file at that place, **and it is named so a reader can go and look.**
    ThereIsAlreadySuchAFile {
        /// Which file is already there.
        what_was_asked_for: String,
    },
    /// The file could not be opened or filled, **and the world said why.**
    ThereWasNoRoomForIt {
        /// Which file could not be written.
        what_was_asked_for: String,
        /// What the world said, in its own words.
        what_it_said: String,
    },
}

impl TheProjectRefusedToWrite {
    /// There is already a file at that place.
    pub fn there_is_already_such_a_file(a_path: &ThePathInsideTheProject) -> Self {
        TheProjectRefusedToWrite::ThereIsAlreadySuchAFile {
            what_was_asked_for: a_path.as_str().to_string(),
        }
    }

    /// The file could not be opened or filled, **and the reason travels whole.**
    pub fn there_was_no_room_for_it(a_path: &ThePathInsideTheProject, what_it_said: &str) -> Self {
        TheProjectRefusedToWrite::ThereWasNoRoomForIt {
            what_was_asked_for: a_path.as_str().to_string(),
            what_it_said: what_it_said.to_string(),
        }
    }

    /// Which file was refused, **and a reader who cannot open it cannot check the claim.**
    pub fn what_was_asked_for(&self) -> &str {
        match self {
            TheProjectRefusedToWrite::ThereIsAlreadySuchAFile { what_was_asked_for }
            | TheProjectRefusedToWrite::ThereWasNoRoomForIt {
                what_was_asked_for, ..
            } => what_was_asked_for,
        }
    }
}

impl std::fmt::Display for TheProjectRefusedToWrite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let why_not = match self {
            TheProjectRefusedToWrite::ThereIsAlreadySuchAFile { what_was_asked_for } => format!(
                "{what_was_asked_for} is already there, and a run writes an artifact once per \
                 version — so writing it again would replace the document an approval still names"
            ),
            TheProjectRefusedToWrite::ThereWasNoRoomForIt {
                what_was_asked_for,
                what_it_said,
            } => format!("{what_was_asked_for} could not be written: {what_it_said}"),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for TheProjectRefusedToWrite {}

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
