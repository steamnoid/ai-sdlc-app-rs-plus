//! Whether anything stands at a directory, answered by the real filesystem.
//!
//! **A directory that exists and holds nothing holds nothing, and that is the whole answer.** A
//! run that made its own destination and then found it empty has to be allowed to clone into it —
//! a check on the directory's own existence refuses a first run forever, and a run that can never
//! start is a run that never finds out what it is wrong about.

use std::path::Path;

use aisdlc_run::whether_anything_stands_there::WhetherAnythingStandsThere;

/// The real disk, answering whether anything stands somewhere on it.
pub struct WhetherAnythingStandsOnDisk;

impl WhetherAnythingStandsThere for WhetherAnythingStandsOnDisk {
    /// Whether anything at all is there — a file, a directory, or nothing.
    ///
    /// **The directory is read and not checked for existing**, because existing is a different
    /// question: a place can be there and hold nothing at all, and a run that made its own
    /// destination is in exactly that state.
    ///
    /// **A directory that cannot be read answers `false`, and not a panic.** A run asking about a
    /// place it may not look is a run with a problem this question is not the place to report,
    /// and a question that panics takes the run down over something it could have refused.
    fn does_anything_stand_at(&self, a_directory: &Path) -> bool {
        std::fs::read_dir(a_directory).is_ok_and(|mut what_is_there| what_is_there.next().is_some())
    }
}
