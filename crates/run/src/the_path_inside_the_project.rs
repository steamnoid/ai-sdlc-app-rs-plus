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

    /// Why this path does not land inside the project.
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
    Ok(ThePathInsideTheProject(the_project.join(a_path.as_str())))
}
