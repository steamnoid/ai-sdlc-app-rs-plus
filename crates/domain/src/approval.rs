//! The names an approval is about, and the name of whoever ruled.
//!
//! **`WhichArtifact` is a name and not a closed set of eight**, because the agents that
//! produce the eight have not been written yet and naming their identifiers is their
//! business. A closed set written before them would bake a list into the enum and into
//! every gate, and every agent arriving later would have to match a name it did not choose.
//! The type refuses a blank and nothing else, and the first artifact whose name is wrong is
//! found by the router refusing to send a gate there.

use std::fmt;

/// The prefix an agent is named with, and the only one that may not rule.
///
/// **`autopilot:` is a different namespace on purpose.** A policy sharing this one would be
/// refused along with the agents it is defined in contrast to, and an autopilot that may
/// approve every gate could then not rule at all.
const THE_PREFIX_AN_AGENT_IS_NAMED_WITH: &str = "aisdlc-";

/// Which artifact a decision was about, and never a bare string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WhichArtifact(pub String);

/// A version below one, which is not a version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnArtifactVersionBelowOne(pub u32);

/// The party that ruled, recorded verbatim and never replaced.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WhoDecided(pub String);

/// An approval that names nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnApprovalThatNamesNothing(pub String);

/// A decision that names nobody.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADecisionThatNamesNobody(pub String);

impl TryFrom<&str> for WhichArtifact {
    type Error = AnApprovalThatNamesNothing;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.trim().is_empty() {
            return Err(AnApprovalThatNamesNothing(name.to_string()));
        }
        Ok(WhichArtifact(name.to_string()))
    }
}

impl fmt::Display for WhichArtifact {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Display for AnApprovalThatNamesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} names no artifact", self.0)
    }
}

impl std::error::Error for AnApprovalThatNamesNothing {}

/// The version of an artifact somebody ruled on. Counts from one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnArtifactVersion(pub u32);

impl TryFrom<u32> for AnArtifactVersion {
    type Error = AnArtifactVersionBelowOne;

    fn try_from(number: u32) -> Result<Self, Self::Error> {
        if number < 1 {
            return Err(AnArtifactVersionBelowOne(number));
        }
        Ok(AnArtifactVersion(number))
    }
}

impl fmt::Display for AnArtifactVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl fmt::Display for AnArtifactVersionBelowOne {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} is below one, and an artifact version counts from one",
            self.0
        )
    }
}

impl std::error::Error for AnArtifactVersionBelowOne {}

impl TryFrom<&str> for WhoDecided {
    type Error = ADecisionThatNamesNobody;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.trim().is_empty() {
            return Err(ADecisionThatNamesNobody(name.to_string()));
        }
        Ok(WhoDecided(name.to_string()))
    }
}

impl fmt::Display for WhoDecided {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Display for ADecisionThatNamesNobody {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} names nobody who decided", self.0)
    }
}

impl std::error::Error for ADecisionThatNamesNobody {}

/// The ruling itself, and the `WhoDecided` it carries verbatim.
///
/// **An answer and not a record, and the difference is the whole of the type.** An
/// `Approval` is what an affirmative decision leaves behind; a refusal is a decision too
/// and it leaves nothing. A caller holding one of these must not assume a record exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalDecision {
    /// Which work item this is about.
    pub work_item_id: crate::work_item_id::WorkItemId,
    /// Which artifact this is about.
    pub which_artifact: WhichArtifact,
    /// Which version of it.
    pub the_version: AnArtifactVersion,
    /// Whether the answer was yes.
    pub approved: bool,
    /// Who ruled, verbatim.
    pub decided_by: WhoDecided,
    /// Why, in the words of whoever ruled, **and never a blank and never a paraphrase.**
    ///
    /// **A ruling with no reason is a decision somebody else could have made.** Refusing costs
    /// nothing to express and leaves no record at all, **so approving is the half where an
    /// unreasoned answer would be hidden** — and this is the field that says what was looked at.
    ///
    /// **Required of a refusal too**, because the trace after a refused run is exactly where
    /// somebody needs to know why — **and an optional reason is two standards of evidence in one
    /// table, where a person might skip it and a policy would never.**
    pub reason: String,
}

/// A ruling that carries no ground for anybody to read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ADecisionNobodyGaveAReasonFor {
    /// The reason is blank, **and it is refused rather than filled in** because a reason this
    /// system composed would be a reason nobody gave.
    TheReasonWasBlank,
}

impl fmt::Display for ADecisionNobodyGaveAReasonFor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "the reason is blank, and a record with no reason is a decision somebody else could \
             have made — refusing costs nothing to express and leaves no record at all, so this is \
             the half where an unreasoned answer would be hidden"
        )
    }
}

impl std::error::Error for ADecisionNobodyGaveAReasonFor {}

impl ApprovalDecision {
    /// Whether this ruling may be recorded, **and a blank reason is the only thing that stops it.**
    ///
    /// **A separate question and not a check inside `what_the_ruling_left_behind`,** because a
    /// caller that asks *why was nothing recorded* and a caller that asks *may this be recorded*
    /// are two readers: the first is reading a trace after the fact, **and the second is standing
    /// at a gate and needs to be told now.**
    pub fn why_it_carries_no_ground(&self) -> Result<(), ADecisionNobodyGaveAReasonFor> {
        if self.reason.trim().is_empty() {
            return Err(ADecisionNobodyGaveAReasonFor::TheReasonWasBlank);
        }
        Ok(())
    }
}

/// An agent tried to rule.
///
/// The prohibition is not a convention here; it is an assertion, and this exists to be
/// raised. An agent may ask for work and may not rule on the result, and those are two
/// halves enforced in two places.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDecidedError(pub String);

/// A record that somebody approved an exact artifact at an exact version of one work item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approval {
    /// Which work item this is about.
    pub work_item_id: crate::work_item_id::WorkItemId,
    /// Which artifact, exactly.
    pub which_artifact: WhichArtifact,
    /// Which version, exactly.
    pub the_version: AnArtifactVersion,
    /// Who ruled, verbatim.
    pub decided_by: WhoDecided,
    /// Why, **in the words of whoever ruled, copied and never summarised.**
    pub reason: String,
}

impl std::error::Error for AgentDecidedError {}

impl fmt::Display for AgentDecidedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} is an agent, and an agent may ask for work and may not rule on the result",
            self.0
        )
    }
}

impl ApprovalDecision {
    /// The ruling a person or a policy gave about one version of one artifact.
    pub fn the_ruling_about(
        work_item_id: crate::work_item_id::WorkItemId,
        which_artifact: WhichArtifact,
        the_version: AnArtifactVersion,
        approved: bool,
        decided_by: WhoDecided,
        reason: &str,
    ) -> Result<Self, AgentDecidedError> {
        if decided_by.0.starts_with(THE_PREFIX_AN_AGENT_IS_NAMED_WITH) {
            return Err(AgentDecidedError(decided_by.0));
        }
        Ok(ApprovalDecision {
            work_item_id,
            which_artifact,
            the_version,
            approved,
            decided_by,
            reason: String::from(reason),
        })
    }
}

impl Approval {
    /// What an affirmative ruling left behind, and nothing at all after a refusal.
    /// The record a ruling leaves behind, **and nothing at all for a refusal.**
    ///
    /// **A refusal leaves no record of work done**, and it still has to carry its reason on the
    /// ruling itself — **because the trace after a refused run is exactly where somebody needs to
    /// know why**, and a record is the wrong place because there is no record.
    ///
    /// **A blank reason leaves nothing either, and that is checked before the ruling is read for
    /// anything else** — **a record carrying no reason is a decision somebody else could have
    /// made**, and it would be handed on looking exactly like a considered one.
    pub fn what_the_ruling_left_behind(a_ruling: &ApprovalDecision) -> Option<Self> {
        if a_ruling.why_it_carries_no_ground().is_err() {
            return None;
        }
        if !a_ruling.approved {
            return None;
        }
        Some(Approval {
            work_item_id: a_ruling.work_item_id.clone(),
            which_artifact: a_ruling.which_artifact.clone(),
            the_version: a_ruling.the_version,
            decided_by: a_ruling.decided_by.clone(),
            reason: a_ruling.reason.clone(),
        })
    }
}
