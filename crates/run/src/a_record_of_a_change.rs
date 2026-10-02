//! The port a change is made through, and the two answers it has to give.
//!
//! **A read and a write are two ports and not one.** `TheProjectOnDisk` says *"Two questions and
//! no others: what is in it, and what does one file say"* — **so a run that could write through
//! it would be a run whose reading boundary nobody could state**, and a repository this system
//! writes into is somebody else's working tree.

/// A file that was not written, **quoted whole, because the path is the whole of the refusal.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFileThatWasNotWritten(pub String);

impl std::fmt::Display for AFileThatWasNotWritten {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} was not written: a repository this system writes into is somebody else's working \
             tree, and a path that steps out of the clone is a file in somebody else's project",
            self.0
        )
    }
}

impl std::error::Error for AFileThatWasNotWritten {}

/// A change this system makes, **and the one thing it has to be able to answer before making it.**
pub trait AChangeIsRecorded {
    /// What one file holds, **and nothing when it holds nothing.**
    fn what_it_holds_of(&self, which_file: &str) -> Option<String>;

    /// Write one file, **and refuse the one path that must never be written.**
    ///
    /// **A `Result` and not nothing, because a write that quietly did not happen is a lie a run
    /// carries for the rest of its life** — it would report a change, commit nothing, and be
    /// caught by the far end. The caller has already refused anything wrong about the *contents*;
    /// **this is the refusal about the *place*, and the filesystem is the only one that can make
    /// it.**
    fn one_file_is_written(
        &mut self,
        which_file: &str,
        what_it_says: &str,
    ) -> Result<(), AFileThatWasNotWritten>;
}
