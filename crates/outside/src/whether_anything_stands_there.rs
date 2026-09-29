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

impl WhetherAnythingStandsOnDisk {
    /// The disk, which is always there and is not a thing that gets constructed.
    pub const as_it_is: Self = Self;
}

impl WhetherAnythingStandsThere for WhetherAnythingStandsOnDisk {
    fn does_anything_stand_at(&self, a_directory: &Path) -> bool {
        a_directory.exists()
    }
}
