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
pub struct AStringThatIsNotAPath {
    written: String,
    why: String,
}

impl AStringThatIsNotAPath {
    /// The string that was written, exactly as it was written.
    pub fn what_was_written(&self) -> &str {
        &self.written
    }

    /// Why this string is not a path.
    pub fn why_not(&self) -> String {
        self.why.clone()
    }

    /// This refusal, for a string that is not a path and the reason why.
    pub fn because(a_string: &str, why_not: &str) -> Self {
        Self {
            written: a_string.to_string(),
            why: why_not.to_string(),
        }
    }
}

impl Error for AStringThatIsNotAPath {}

impl fmt::Display for AStringThatIsNotAPath {
    /// Write the refusal, quoting the input back.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} is not a path: {}", self.written, self.why)
    }
}

/// The shape of a string, asked before anything touches the disk.
pub fn a_path_in_the_project(a_string: &str) -> Result<APathInTheProject, AStringThatIsNotAPath> {
    if let Some(why_not) = why_this_is_not_a_path(a_string) {
        return Err(AStringThatIsNotAPath::because(a_string, &why_not));
    }
    Ok(APathInTheProject(PathBuf::from(a_string)))
}

/// Why a string is not a path, or nothing when it is one.
fn why_this_is_not_a_path(a_string: &str) -> Option<String> {
    if let Some(why_not) = why_the_whole_string_is_not_a_path(a_string) {
        return Some(why_not);
    }
    why_a_component_is_not_a_path(a_string)
}

/// Why the string as a whole is not a path, whatever its shape.
fn why_the_whole_string_is_not_a_path(a_string: &str) -> Option<String> {
    if a_string.trim().is_empty() {
        return Some(
            "a blank is not a short path, and one that trims to nothing names nothing".to_string(),
        );
    }
    if a_string.contains('\0') {
        return Some(
            "no filesystem holds a zero byte, and a path carrying one names nothing".to_string(),
        );
    }
    None
}

/// Why one component is not a component, or nothing when they all are.
fn why_a_component_is_not_a_path(a_string: &str) -> Option<String> {
    for one_component in a_string.split('/') {
        if one_component.is_empty() {
            return Some(
                "an empty segment is not a directory, and the disk reads it as one".to_string(),
            );
        }
        if one_component.len() > THE_LONGEST_A_COMPONENT_MAY_BE {
            return Some("one component is longer than any component may be".to_string());
        }
    }
    None
}
