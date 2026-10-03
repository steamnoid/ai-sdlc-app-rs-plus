//! Whether the path that results lands inside the project.
//!
//! **The confinement rule, and it is not the shape rule.** A string that cannot be a path is
//! refused by `a_path_in_the_project` before anything here is asked, and this one is asked
//! about the other half: where the path that *can* be a path actually points.
//!
//! **The decision is by the resolved location and not by the spelling.** `..` is a way out
//! however it is written and a symlink is a way out however innocent it looks, and resolution
//! returns a different spelling from the one it was given.

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::a_path_in_the_project::APathInTheProject;

/// A path whose resolved location is inside the project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePathInsideTheProject(PathBuf);

impl ThePathInsideTheProject {
    /// The **resolved** spelling, which is not the spelling that was written.
    pub fn as_str(&self) -> &str {
        self.0.to_str().unwrap_or_default()
    }
}

/// A path that does not resolve inside the project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APathOutsideTheProject {
    written: String,
    why_not: String,
}

impl APathOutsideTheProject {
    /// The spelling that was written, and the one it resolved to.
    pub fn what_was_written(&self) -> &str {
        &self.written
    }

    /// Why this path does not land inside the project, naming where it resolved to.
    pub fn why_not(&self) -> &str {
        &self.why_not
    }

    /// This refusal, for a path that does not land inside.
    pub fn because(written: &str, why_not: &str) -> Self {
        Self {
            written: written.to_string(),
            why_not: why_not.to_string(),
        }
    }
}

impl Error for APathOutsideTheProject {}

impl fmt::Display for APathOutsideTheProject {
    /// Write the refusal, naming both spellings.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} does not land inside the project: {}",
            self.written, self.why_not
        )
    }
}

/// Where the path lands, asked by resolving it rather than by reading it.
pub fn the_path_inside_the_project(
    the_project: &Path,
    a_path: &APathInTheProject,
) -> Result<ThePathInsideTheProject, APathOutsideTheProject> {
    let resolved_project = the_resolved_root(the_project);
    let resolved = the_resolved_path(&resolved_project, a_path);
    let Some(resolved) = resolved else {
        return Err(APathOutsideTheProject::because(
            a_path.as_str(),
            "nothing to resolve, so the path names nowhere and is inside nothing",
        ));
    };
    if !resolved.starts_with(&resolved_project) {
        return Err(APathOutsideTheProject::because(
            a_path.as_str(),
            &format!(
                "it resolves to {}, which is outside the project",
                resolved.display()
            ),
        ));
    }
    Ok(ThePathInsideTheProject(resolved))
}

/// The project's own resolved location, so both sides of the comparison are resolved.
fn the_resolved_root(the_project: &Path) -> PathBuf {
    the_project
        .canonicalize()
        .unwrap_or_else(|_| the_project.to_path_buf())
}

/// Where a path lands once the disk has had its say about it.
fn the_resolved_path(the_project: &Path, a_path: &APathInTheProject) -> Option<PathBuf> {
    the_project.join(a_path.as_str()).canonicalize().ok()
}

/// Where a file that is **not written yet** will land, **and it is a second door and not the first
/// one loosened.**
///
/// **This is the writer's finding about the reader's rule.** `the_path_inside_the_project` resolves
/// the whole path, **so it refuses anything that is not already there** — which is right for a read
/// and impossible for a write, **because a write is a file that does not exist yet.** The first
/// version of the writer tried to fix that by resolving the longest existing prefix, **and it broke
/// three of this file's own tests**: one saying a path that does not exist is refused, one saying
/// `..` is a way out however it is written, and one saying the refusal quotes what it resolved to.
/// **All three were right and the fix was wrong.**
///
/// **So the parent is resolved instead of the path.** The parent must exist, **which it does** — the
/// work item's own directory is created when its clone is made — and every segment below it is
/// checked for shape by the same rule the reader uses, **so a `..` and an empty segment are refused
/// here exactly as they are there.**
pub fn the_path_a_file_will_be_written_at(
    the_project: &Path,
    a_path: &APathInTheProject,
) -> Result<ThePathInsideTheProject, APathOutsideTheProject> {
    let resolved_project = the_resolved_root(the_project);
    let the_directory = the_directory_it_would_be_written_into(the_project, a_path);
    let the_resolved_directory = the_directory.canonicalize();
    let Ok(the_directory) = the_resolved_directory else {
        return Err(APathOutsideTheProject::because(
            a_path.as_str(),
            "the directory this file would be written into is not there, so the file has nowhere to land",
        ));
    };
    if !the_directory.starts_with(&resolved_project) {
        return Err(the_directory_is_outside(a_path, &the_directory));
    }
    let the_name = the_name_of_the_file(a_path)?;
    Ok(ThePathInsideTheProject(the_directory.join(the_name)))
}

/// The directory a file would be written into, **and it is resolved against the project and not
/// against nothing** — a rule that joined a relative path onto the current directory would be a rule
/// about wherever the process happened to be started.
fn the_directory_it_would_be_written_into(
    the_project: &Path,
    a_path: &APathInTheProject,
) -> PathBuf {
    let as_written = Path::new(a_path.as_str())
        .parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    the_project.join(as_written)
}

/// The directory is outside the project, **and it is named so a reader can go and look at where.**
fn the_directory_is_outside(
    a_path: &APathInTheProject,
    where_it_is: &Path,
) -> APathOutsideTheProject {
    APathOutsideTheProject::because(
        a_path.as_str(),
        &format!(
            "it would be written into {}, which is outside the project",
            where_it_is.display()
        ),
    )
}

/// The name of the file, **and a last segment that is not a name is refused** — `..`, `.` and nothing
/// are all ways of naming no file, **and a write with no file name is a write with no destination.**
fn the_name_of_the_file(a_path: &APathInTheProject) -> Result<String, APathOutsideTheProject> {
    let as_written = Path::new(a_path.as_str())
        .file_name()
        .map_or_else(String::new, |one_name| {
            one_name.to_string_lossy().into_owned()
        });
    let is_a_name = !as_written.is_empty() && as_written != "." && as_written != "..";
    if !is_a_name {
        return Err(APathOutsideTheProject::because(
            a_path.as_str(),
            "the last segment is not a file's name, so there is no file here to write",
        ));
    }
    Ok(as_written)
}
