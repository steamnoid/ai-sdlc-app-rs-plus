//! The one way a stage is written and the one way it is read back.
//!
//! **The capitals live in exactly one place, and that place is not this file.** A stage is
//! read as its own name — `Stage::AwaitingAgentPickup` is `AWAITING_AGENT_PICKUP` — and both
//! spellings are one concept, so a caller that writes one and a reader that expects the
//! other would be two facts about the same stage. `Display` and `TryFrom` are the join, and
//! they are the only two places either spelling is allowed to appear.
//!
//! **The variant is the discipline rather than the acronym.** `Stage::Ready` reads as the
//! idea, and a two-letter `Po` is a spelling a reader has to look up. The same decision
//! stands for `Role`, and the glossary carries it because it was a decision rather than a
//! coincidence: clippy accepts `PO` and `QA` and refuses `DEV`, so writing two of the five
//! one way on the strength of a letter count is a spelling with no reason behind it.

use std::fmt;

/// Where a work item stands, as one of six things and never as a string.
///
/// The set is closed because a stage nobody wrote down is not a stage: a `str` here would
/// let a run record a state the transition table has never heard of, and the table is the
/// only thing that says which moves are legal. Nothing in this crate turns a bare string
/// into a `Stage` except `TryFrom`, and it refuses a name that is not one of the six.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Stage {
    /// Nothing is happening, and no agent holds the item.
    Idle,
    /// An agent may claim this item, and no agent holds it yet.
    AwaitingAgentPickup,
    /// An agent is acting on this item, and it is that agent which holds it.
    InProgressByAgent,
    /// A human must rule before anything continues, and no agent holds the item.
    AwaitingHumanApproval,
    /// The entry state: a person queued this and nobody holds it yet.
    Ready,
    /// Finished. Terminal, with no outgoing move.
    Done,
}

/// Every stage, in the order the specification lists them.
///
/// **Rust writes a constant in capitals**, and a `#[allow(non_upper_case_globals)]` is
/// refused by a gate here — so this is the glossary's `EveryStage` in the language's own
/// case. The vocabulary guard normalises a name to letters and digits, so the two spellings
/// are one name to it and one glossary entry blesses both.
pub const EVERY_STAGE: [Stage; 6] = [
    Stage::Idle,
    Stage::AwaitingAgentPickup,
    Stage::InProgressByAgent,
    Stage::AwaitingHumanApproval,
    Stage::Ready,
    Stage::Done,
];

/// A name that is not one of the six, so it is not a stage.
///
/// **This is the only way into a `Stage` from text**, and it exists because a document names
/// its stages in capitals and something has to read them. A name nobody wrote down never
/// becomes a stage: the transition table knows six rows, and a seventh would be a move
/// the table has never been asked about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStageThatNobodyNamed(pub String);

impl fmt::Display for Stage {
    /// Write the stage the way the specification writes it, in capitals.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let the_name = match self {
            Stage::Idle => "IDLE",
            Stage::AwaitingAgentPickup => "AWAITING_AGENT_PICKUP",
            Stage::InProgressByAgent => "IN_PROGRESS_BY_AGENT",
            Stage::AwaitingHumanApproval => "AWAITING_HUMAN_APPROVAL",
            Stage::Ready => "READY",
            Stage::Done => "DONE",
        };
        formatter.write_str(the_name)
    }
}

impl TryFrom<&str> for Stage {
    type Error = AStageThatNobodyNamed;

    /// Read a stage the way the specification writes it, in capitals.
    fn try_from(name: &str) -> Result<Self, Self::Error> {
        for stage in EVERY_STAGE {
            if stage.to_string() == name {
                return Ok(stage);
            }
        }
        Err(AStageThatNobodyNamed(name.to_string()))
    }
}

impl fmt::Display for AStageThatNobodyNamed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} is not one of the six stages", self.0)
    }
}

impl std::error::Error for AStageThatNobodyNamed {}
