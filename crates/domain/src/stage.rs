//! The stages a work item is at, and the closed set of them.
//!
//! A stage is never a bare string, and it is written with its own name in capitals. The
//! variant is the reading of that name and the writing of it is the capitals, so the two
//! are one concept with two spellings and the capitals exist in exactly one place.
//!
//! Nothing here may reach outside this crate. There is no parsing of a file, no reading of
//! a clock and no name of a model, and the crate holding this file has no dependencies at
//! all — which is the rule every other boundary in the repository rests on.

use std::convert::TryFrom;
use std::fmt;

/// Every stage, in the order the specification lists them.
///
/// The specification's order rather than any other, so a caller crossing one thing with
/// every stage does not write a second list to keep in step. The tests transcribe the six
/// rather than import this, because a test that reads its own expectation from the code it
/// checks proves nothing.
pub const EVERY_STAGE: [Stage; 6] = [
    Stage::Idle,
    Stage::AwaitingAgentPickup,
    Stage::InProgressByAgent,
    Stage::AwaitingHumanApproval,
    Stage::Ready,
    Stage::Done,
];

/// A name that is not one of the six stages.
///
/// **A refusal and not an `Option`, and it carries the name that was looked for.** A
/// caller asking "is this a stage" has the same question whichever way the answer came
/// out, and an `Option` would make it choose which of two shapes to hold before it knows
/// anything. The name matters for the same reason a model is corrected better when it is
/// handed its own spelling back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStageThatNobodyNamed {
    what_was_looked_for: String,
}

impl AStageThatNobodyNamed {
    /// The refusal for a name that is not one of the six.
    pub fn for_a_name(name: &str) -> AStageThatNobodyNamed {
        AStageThatNobodyNamed {
            what_was_looked_for: name.to_string(),
        }
    }

    /// The name that was looked for and is not one of the six.
    pub fn what_was_looked_for(&self) -> &str {
        &self.what_was_looked_for
    }
}

impl fmt::Display for AStageThatNobodyNamed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} is not one of the six stages",
            self.what_was_looked_for
        )
    }
}

impl std::error::Error for AStageThatNobodyNamed {}

/// Where a work item is in its lifecycle.
///
/// **The variant is the reading of a stage's name and the capitals are the writing.** A
/// reader looking for a stage finds it in the specification's spelling, and a reader of
/// this file finds it in Rust's, and the two are joined by one `Display` and one `TryFrom`
/// rather than by a reader's memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    /// Nothing is happening. No agent may hold the work item here.
    Idle,
    /// An agent may claim this work item, and one must.
    AwaitingAgentPickup,
    /// An agent is acting on this work item, and it is the agent that claimed it.
    InProgressByAgent,
    /// A human must rule before anything continues, and nobody holds the work item.
    AwaitingHumanApproval,
    /// The entry state. A person queued this and nobody holds it yet.
    Ready,
    /// Finished, and terminal: there is no stage to move on to from here.
    Done,
}

impl TryFrom<&str> for Stage {
    type Error = AStageThatNobodyNamed;

    /// The stage this name writes, or the refusal quoting the name back.
    ///
    /// **The name is compared exactly as written, and a name with a space in it is a
    /// different name.** A reading that trimmed would answer about a stage nobody wrote
    /// down, and the caller would never learn that its own spelling was wrong — which is
    /// the one thing a model writing a stage name most needs to be told.
    fn try_from(name: &str) -> Result<Stage, AStageThatNobodyNamed> {
        let stage = match name {
            "IDLE" => Stage::Idle,
            "AWAITING_AGENT_PICKUP" => Stage::AwaitingAgentPickup,
            "IN_PROGRESS_BY_AGENT" => Stage::InProgressByAgent,
            "AWAITING_HUMAN_APPROVAL" => Stage::AwaitingHumanApproval,
            "READY" => Stage::Ready,
            "DONE" => Stage::Done,
            _ => return Err(AStageThatNobodyNamed::for_a_name(name)),
        };
        Ok(stage)
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let written = match self {
            Stage::Idle => "IDLE",
            Stage::AwaitingAgentPickup => "AWAITING_AGENT_PICKUP",
            Stage::InProgressByAgent => "IN_PROGRESS_BY_AGENT",
            Stage::AwaitingHumanApproval => "AWAITING_HUMAN_APPROVAL",
            Stage::Ready => "READY",
            Stage::Done => "DONE",
        };
        write!(formatter, "{written}")
    }
}
