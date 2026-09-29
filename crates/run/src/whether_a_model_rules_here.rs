//! Whether a model may rule at the gate, as a value and not as a setting.
//!
//! **A value here and not a setting read in this crate.** A use case that reads its own
//! configuration has moved a decision out of a type and into a line of code, and the line is
//! somewhere a reader has to find it.
//!
//! **Two and not a bool, so a caller has to say which.** `Yes` and `No` read at the call site, and
//! a bare `true` says nothing about which thing is being set.

/// Whether a model is allowed to rule at the gate on this deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherAModelRulesHere {
    /// The model may be asked, and its answer is a ruling.
    Yes,
    /// The model is not asked, and the gate waits for a person.
    No,
}

impl WhetherAModelRulesHere {
    /// Both, in the order a reader meets them.
    pub fn every_answer() -> Vec<WhetherAModelRulesHere> {
        vec![WhetherAModelRulesHere::Yes, WhetherAModelRulesHere::No]
    }

    /// Whether a model may be asked at all.
    pub fn may_be_asked(&self) -> bool {
        matches!(self, WhetherAModelRulesHere::Yes)
    }
}
