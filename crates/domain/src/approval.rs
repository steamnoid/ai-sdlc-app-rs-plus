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
