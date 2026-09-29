//! Whether anything stands at a directory, asked before anything is written there.
//!
//! **A question asked and not a thing returned**, because the answer is a yes or a no and a
//! `bool` is the whole of it. The directory is a real one and the question is about the
//! filesystem, so the answer comes from an implementation in `aisdlc-outside` and never from
//! here.

use std::path::Path;

/// Asking a real directory whether anything stands there.
pub trait WhetherAnythingStandsThere {
    /// Whether anything at all is there — a file, a directory, or nothing.
    fn does_anything_stand_at(&self, a_directory: &Path) -> bool;
}
