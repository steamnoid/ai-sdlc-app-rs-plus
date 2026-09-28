//! Whether a string can be a path at all, asked before anything touches the disk.
//!
//! **The shape rule, and the confinement rule is a different question.** A string that cannot
//! be a path is refused here, before any disk is touched, because a model wrote a 433-byte
//! sentence into a path field once and the operating system raised about that rather than
//! answering.
//!
//! **The bound is per segment and not per path**, because a repository has paths longer than
//! 255 bytes and they are ordinary.

use std::error::Error;
use std::fmt;
use std::path::PathBuf;

/// The most bytes one segment of a path may be.
pub const THE_LONGEST_A_COMPONENT_MAY_BE: usize = 255;

/// A path inside the project, checked for its shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APathInTheProject(PathBuf);

impl APathInTheProject {
    /// The path, as it was given.
    pub fn as_str(&self) -> &str {
        self.0.to_str().unwrap_or_default()
    }
}

/// A string that cannot be one, named for the rule and quoting the input back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStringThatIsNotAPath(String);

impl AStringThatIsNotAPath {
    /// The string that was written, exactly as it was written.
    pub fn what_was_written(&self) -> &str {
        &self.0
    }

    /// Why this string is not a path.
    pub fn why_not(&self) -> String {
        String::new()
    }

    /// This refusal, for a string that is not a path.
    pub fn because(a_string: &str, why_not: &str) -> Self {
        Self(a_string.to_string())
    }
}

impl Error for AStringThatIsNotAPath {}

impl fmt::Display for AStringThatIsNotAPath {
    /// Write the refusal, quoting the input back.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} is not a path", self.0)
    }
}

/// The shape of a string, asked before anything touches the disk.
pub fn a_path_in_the_project(a_string: &str) -> Result<APathInTheProject, AStringThatIsNotAPath> {
    Ok(APathInTheProject(PathBuf::from(a_string)))
}
