//! What a stage kept is written beside the clone, and exactly once.
//!
//! **A stage keeps what the agent said and there was nowhere to put it.** `WhatTheStageLeft` holds
//! the words and `where_an_artifact_lands` holds the path — **and nothing joined them, so the words
//! stopped at the edge of the process.**
//!
//! **The path is derived by the domain and not composed here.** A writer that built the path itself
//! would be a second derivation nobody has to keep in step with the one in the domain.

use std::fmt;

use aisdlc_domain::where_an_artifact_lands::{
    AnArtifactThisProjectCannotHold, where_an_artifact_lands,
};

use crate::a_path_in_the_project::{
    APathInTheProject, AStringThatIsNotAPath, a_path_in_the_project,
};
use crate::the_path_inside_the_project::{
    APathOutsideTheProject, ThePathInsideTheProject, the_path_a_file_will_be_written_at,
};
use crate::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use crate::where_a_run_stands::WhereARunStands;

/// Why what a stage kept could not be written, **and each is a fact about a different thing.**
///
/// **The four that came from elsewhere keep the type that refused them**, so a reader told "the disk
/// already holds that file" knows it was the disk and not the path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatCouldNotBeWritten {
    /// The run is on no artifact, **or has produced none**, **and both are the same fact here: there
    /// is no name and no version to write.**
    TheRunHasProducedNothing,
    /// The derived path would not resolve, **and the shape rule's own refusal travels whole.**
    ThePathIsNotAPath {
        /// What the shape rule said.
        why_not: Box<AStringThatIsNotAPath>,
    },
    /// The derived path would not land inside the project, **and the confinement rule's own refusal
    /// travels whole** — from the door meant for a file that is not written yet, **which resolves the
    /// directory rather than the file.**
    ThePathWouldNotLandInside {
        /// What the confinement rule said.
        why_not: Box<APathOutsideTheProject>,
    },
    /// The artifact's name is one this project cannot hold, **and the domain's own refusal travels
    /// whole** — an artifact that would land inside the clone is refused by the derivation and not
    /// here.
    TheProjectCannotHoldIt {
        /// What the domain said.
        why_not: Box<AnArtifactThisProjectCannotHold>,
    },
    /// There is already a file at that place, **and the project's own refusal travels whole.**
    ///
    /// **The variant names the world fact and not the layer**, so a caller can tell which of the
    /// two it was without opening the refusal inside — **and the refusal travels anyway, because the
    /// project's own words are the evidence.**
    ThereIsAlreadySuchAFile {
        /// What the project said.
        why_not: Box<TheProjectRefusedToWrite>,
    },
    /// The file could not be opened or filled, **and the world's own words travel with the name.**
    ThereWasNoRoomForIt {
        /// What the project said.
        why_not: Box<TheProjectRefusedToWrite>,
    },
}

impl fmt::Display for WhatCouldNotBeWritten {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.in_words())
    }
}

impl WhatCouldNotBeWritten {
    /// A refusal the run did not raise, **in the words of whatever raised it.**
    ///
    /// **Four of the six are somebody else's**, and prefixing them with a sentence of the run's own
    /// would make the router's fault and the disk's look like one — **and those two send a reader to
    /// two different files.**
    fn in_words(&self) -> String {
        match self {
            WhatCouldNotBeWritten::TheRunHasProducedNothing => String::from(
                "this run has produced nothing yet, so there is no name and no version to write",
            ),
            WhatCouldNotBeWritten::ThePathIsNotAPath { why_not } => {
                format!("the path is not a path: {}", why_not.what_was_written())
            }
            WhatCouldNotBeWritten::ThePathWouldNotLandInside { why_not } => {
                format!("the path is not inside the project: {why_not}")
            }
            WhatCouldNotBeWritten::TheProjectCannotHoldIt { why_not } => {
                format!("this project cannot hold that artifact: {why_not}")
            }
            WhatCouldNotBeWritten::ThereIsAlreadySuchAFile { why_not } => {
                format!("the file is already there: {why_not}")
            }
            WhatCouldNotBeWritten::ThereWasNoRoomForIt { why_not } => {
                format!("there was no room for the file: {why_not}")
            }
        }
    }
}

impl std::error::Error for WhatCouldNotBeWritten {}

/// Write what a stage kept, **at the path the run derives, and hand back where it landed.**
///
/// **The artifact and the version come from the run and not from the caller.** A caller naming them
/// could write a file under a name no approval will ever name, **and the file would sit beside the
/// clone looking exactly like an artifact.**
///
/// **`the_project.where_it_lives()` is the root the run's paths are derived from** — and that is
/// what the confinement check is made against, **so a path that climbed out of it is refused by the
/// rule rather than by this function.**
pub fn what_a_stage_kept_is_written(
    the_project: &mut impl TheProjectOnDisk,
    the_run: &WhereARunStands,
    what_the_agent_said: &str,
) -> Result<ThePathInsideTheProject, WhatCouldNotBeWritten> {
    let where_it_goes = where_the_run_says_this_artifact_lands(the_run)?;
    let inside_the_project = the_path_a_file_will_be_written_at(
        the_project.where_it_lives(),
        &the_path_written(&where_it_goes)?,
    );
    let inside_the_project =
        inside_the_project.map_err(|why_not| WhatCouldNotBeWritten::ThePathWouldNotLandInside {
            why_not: Box::new(why_not),
        })?;
    the_project
        .a_file_is_written(&inside_the_project, what_the_agent_said)
        .map_err(the_project_refuses_in_the_run_s_own_words)?;
    Ok(inside_the_project)
}

/// The project's refusal in the run's own two words, **and which word is chosen by asking the
/// refusal rather than by matching on the type** — so a third refusal to a project is a compile error
/// here rather than a silent fallthrough.
fn the_project_refuses_in_the_run_s_own_words(
    why_not: TheProjectRefusedToWrite,
) -> WhatCouldNotBeWritten {
    match &why_not {
        TheProjectRefusedToWrite::ThereIsAlreadySuchAFile { .. } => the_file_is_there(why_not),
        TheProjectRefusedToWrite::ThereWasNoRoomForIt { .. } => there_was_no_room_for_it(why_not),
    }
}

/// The file is already there, **and it is built by a function so the match above reads as a match.**
fn the_file_is_there(why_not: TheProjectRefusedToWrite) -> WhatCouldNotBeWritten {
    WhatCouldNotBeWritten::ThereIsAlreadySuchAFile {
        why_not: Box::new(why_not),
    }
}

/// There was no room for the file, **and the world's own words travel with the name.**
fn there_was_no_room_for_it(why_not: TheProjectRefusedToWrite) -> WhatCouldNotBeWritten {
    WhatCouldNotBeWritten::ThereWasNoRoomForIt {
        why_not: Box::new(why_not),
    }
}

/// The root the writer derives from, **and it is empty on purpose.**
///
/// **The shape and confinement rules both take a path *inside* the project**, and the project's own
/// root is added by the project when it resolves one. **So the writer asks the domain for the name
/// inside the project and hands that over** — and asking with a real root would hand the rules an
/// absolute path, which they refuse, **because a rule that confined a run's artifacts against a path
/// the run had already rooted would be a rule about a path nothing else in the system holds.**
/// What the run has produced, **and both halves are asked at once** because a run on no artifact and
/// a run that has produced nothing are the same fact here: **there is no name and no version to
/// write.**
fn what_the_run_has_produced(
    which_artifact: Option<&aisdlc_domain::approval::WhichArtifact>,
    the_version: Option<aisdlc_domain::approval::AnArtifactVersion>,
) -> Result<
    (
        &aisdlc_domain::approval::WhichArtifact,
        aisdlc_domain::approval::AnArtifactVersion,
    ),
    WhatCouldNotBeWritten,
> {
    match (which_artifact, the_version) {
        (Some(a_name), Some(a_version)) => Ok((a_name, a_version)),
        _ => Err(WhatCouldNotBeWritten::TheRunHasProducedNothing),
    }
}

/// The empty root the writer derives from, **and it is empty on purpose.**
///
/// **The shape and confinement rules both take a path *inside* the project**, and the project's own
/// root is added by the project when it resolves one. **So the writer asks the domain for the name
/// inside the project and hands that over** — and asking with a real root would hand the rules an
/// absolute path, which they refuse, **because a rule that confined a run's artifacts against a path
/// the run had already rooted would be a rule about a path nothing else in the system holds.**
///
/// **A function and not a constant**, because `Path::new` is not a `const fn` in this toolchain, and
/// a workaround for that would be a worse trade than the function.
fn inside_the_project() -> &'static std::path::Path {
    std::path::Path::new("")
}

/// Where the run says this artifact lands, **and it is the domain's derivation and not one written
/// here.**
fn where_the_run_says_this_artifact_lands(
    the_run: &WhereARunStands,
) -> Result<std::path::PathBuf, WhatCouldNotBeWritten> {
    let which_artifact = the_run.which_artifact_it_is_on();
    let the_version = the_run.which_version_of_its_artifact_exists();
    let (which_artifact, the_version) = what_the_run_has_produced(which_artifact, the_version)?;
    let the_work_item = the_run.the_work_item_it_is();
    where_an_artifact_lands(
        inside_the_project(),
        &the_work_item.the_repository(),
        &the_work_item.the_work_item_id(),
        which_artifact,
        &the_version,
    )
    .map_err(|why_not| WhatCouldNotBeWritten::TheProjectCannotHoldIt {
        why_not: Box::new(why_not),
    })
}

/// The derived path as the shape rule's own type, **and it is refused here rather than handed on as
/// a string** — a path that is not a path is a fact about the name, not about the disk.
fn the_path_written(
    where_it_goes: &std::path::Path,
) -> Result<APathInTheProject, WhatCouldNotBeWritten> {
    let as_written = where_it_goes.to_str().unwrap_or_default();
    a_path_in_the_project(as_written).map_err(|why_not| WhatCouldNotBeWritten::ThePathIsNotAPath {
        why_not: Box::new(why_not),
    })
}
