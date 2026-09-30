//! Whether a deployment asked for an agent that offers work of its own.
//!
//! **Off unless a deployment says otherwise, and the word it must say is one word.** A word this
//! system does not know is a word it will not act on — **and guessing that `true` means yes would
//! put an agent that files issues into somebody's repository whose operator never asked for one.**

/// Whether an agent may offer work, as a value and not as a lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherAnAgentOffers {
    /// A deployment asked, and an agent may file work of its own.
    Yes,
    /// Nobody asked, and a human's filing is the only work this system has.
    No,
}
