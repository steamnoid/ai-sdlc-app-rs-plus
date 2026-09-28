//! The names an approval is about, and the name of whoever ruled.
//!
//! **`WhichArtifact` is a name and not a closed set of eight**, because the agents that
//! produce the eight have not been written yet and naming their identifiers is their
//! business. A closed set written before them would bake a list into the enum and into
//! every gate, and every agent arriving later would have to match a name it did not choose.
//! The type refuses a blank and nothing else, and the first artifact whose name is wrong is
//! found by the router refusing to send a gate there.

use std::fmt;

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
    ) -> Result<Self, AgentDecidedError> {
        let _the_decider_was_not_asked_about = &decided_by;
        Ok(ApprovalDecision {
            work_item_id,
            which_artifact,
            the_version,
            approved,
            decided_by,
        })
    }
}

impl Approval {
    /// What an affirmative ruling left behind, and nothing at all after a refusal.
    pub fn what_the_ruling_left_behind(a_ruling: &ApprovalDecision) -> Option<Self> {
        let _the_ruling_was_not_read = a_ruling;
        Some(Approval {
            work_item_id: a_ruling.work_item_id.clone(),
            which_artifact: a_ruling.which_artifact.clone(),
            the_version: a_ruling.the_version,
            decided_by: a_ruling.decided_by.clone(),
        })
    }
}
