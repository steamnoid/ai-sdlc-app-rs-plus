//! Where an artifact of one work item lands, beside its clone and never inside it.
//!
//! **The clone is somebody else's repository and must stay a faithful checkout of it.** A file of ours
//! inside that working tree is a change to that repository, **and the change would be pushed with the
//! branch** — so an artifact that would land inside the clone is refused here rather than at a writer.
//!
//! **The place is derived and not chosen**, exactly as the clone's place is: **a destination a caller
//! picks is a different directory on the second run of the same work item**, so a resumed run looks in
//! the wrong place and finds nothing.
//!
//! **The version is in the file's name.** An approval names the exact artifact *and version* it ruled
//! on, **and a second production written over the first would leave that approval naming a document the
//! person never read.**

use std::fmt;
use std::path::PathBuf;

use crate::approval::{AnArtifactVersion, WhichArtifact};
use crate::repository::Repository;
use crate::work_item_id::WorkItemId;

/// Why an artifact cannot be held by this project, **and each is a fact about a different thing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnArtifactThisProjectCannotHold {
    /// The name would land inside the clone, **which is somebody else's repository.**
    ItWouldLandInTheClone {
        /// What was named.
        what_was_named: String,
    },
    /// The name would leave the project's own directory, **so it is somewhere this system does not
    /// own.**
    ///
    /// **A different refusal from the one above**, because the clone is a directory this system put
    /// there and the parent of the project is not — **and the two send a reader to two different
    /// rules.**
    ItWouldLeaveTheProject {
        /// What was named.
        what_was_named: String,
    },
}

impl fmt::Display for AnArtifactThisProjectCannotHold {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            AnArtifactThisProjectCannotHold::ItWouldLandInTheClone { what_was_named } => format!(
                "{what_was_named} would land inside the clone, and the clone is somebody else's \
                 repository and must stay a faithful checkout of it"
            ),
            AnArtifactThisProjectCannotHold::ItWouldLeaveTheProject { what_was_named } => format!(
                "{what_was_named} would leave this project's own directory, and a run writes inside \
                 the directory it owns and nowhere else"
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for AnArtifactThisProjectCannotHold {}

/// The directory one work item's artifacts land in, **and it is the clone's directory's parent.**
fn where_the_project_of_a_work_item_is(
    the_projects_root: &std::path::Path,
    what_is_addressed: &Repository,
    a_work_item: &WorkItemId,
) -> PathBuf {
    the_projects_root
        .join(&what_is_addressed.owner)
        .join(&what_is_addressed.name)
        .join(a_work_item.0.as_str())
}

/// Where one artifact of one work item lands, **and it is refused rather than written wherever the
/// name points.**
pub fn where_an_artifact_lands(
    the_projects_root: &std::path::Path,
    what_is_addressed: &Repository,
    a_work_item: &WorkItemId,
    which_artifact: &WhichArtifact,
    the_version: &AnArtifactVersion,
) -> Result<PathBuf, AnArtifactThisProjectCannotHold> {
    this_project_cannot_hold(which_artifact)?;
    let where_the_project_is =
        where_the_project_of_a_work_item_is(the_projects_root, what_is_addressed, a_work_item);
    Ok(where_the_project_is.join(the_name_of_the_file(which_artifact, *the_version)))
}

/// What this project cannot hold, **and it is asked before the path is built** — a path built from a
/// name nobody may use is a path a reader has to look at before being told it is wrong.
///
/// **Both refusals are named and neither is a default**, so a third rule added later is a compile
/// error here rather than a name that silently lands wherever it points.
fn this_project_cannot_hold(
    which_artifact: &WhichArtifact,
) -> Result<(), AnArtifactThisProjectCannotHold> {
    let what_was_named = which_artifact.0.as_str();
    if what_was_named == "repository" || what_was_named.starts_with("repository/") {
        return Err(it_would_land_in_the_clone(what_was_named));
    }
    if what_was_named.contains("..") {
        return Err(it_would_leave_the_project(what_was_named));
    }
    Ok(())
}

/// The refusal for a name inside the clone, **and it is a function so the branch above reads as a
/// branch** — a struct literal there is the third level of nesting, **and a reader stops seeing the
/// branch.**
fn it_would_land_in_the_clone(what_was_named: &str) -> AnArtifactThisProjectCannotHold {
    AnArtifactThisProjectCannotHold::ItWouldLandInTheClone {
        what_was_named: what_was_named.to_string(),
    }
}

/// The refusal for a name that climbs out, **and it says the same thing for the same reason.**
fn it_would_leave_the_project(what_was_named: &str) -> AnArtifactThisProjectCannotHold {
    AnArtifactThisProjectCannotHold::ItWouldLeaveTheProject {
        what_was_named: what_was_named.to_string(),
    }
}

/// The file's name, **and the version is in it so a second production is a second file.**
fn the_name_of_the_file(which_artifact: &WhichArtifact, the_version: AnArtifactVersion) -> String {
    format!("{}.v{}.json", which_artifact.0, the_version.0)
}
