//! What an agent has produced, and the closed set of names this system knows.
//!
//! **`WhichArtifact` is an open `String` and it accepts anything**, so this is the door that is
//! closed: a gate that approves an artifact nobody here has ever heard of is a gate that approved
//! a stranger's paperwork, **and the record names it as approved, which is the one thing a record
//! must never do.**
//!
//! **The seven names are the specification's, transcribed from the mother project's own router**,
//! and not composed here — a name a system chose for itself is a name nothing could be held to.

use aisdlc_domain::approval::{AnArtifactVersion, AnArtifactVersionBelowOne, WhichArtifact};

/// Every artifact the pipeline knows, in the order a run meets them.
const WHAT_THE_PIPELINE_KNOWS: [&str; 7] = [
    "DiscoveryReport",
    "Requirements",
    "QaPlan",
    "ArchitectureReport",
    "SecurityReport",
    "ImplementationPlan",
    "VerificationReport",
];

/// Nobody here has declared that name.
fn nobody_declared(what_was_named: &str) -> WhatWasNotAThingThisSystemHolds {
    WhatWasNotAThingThisSystemHolds::NoSuchArtifact(AnArtifactThisSystemDoesNotKnow {
        what_was_named: what_was_named.to_string(),
    })
}

/// A name nobody declared here, and what was given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnArtifactThisSystemDoesNotKnow {
    what_was_named: String,
}

impl std::fmt::Display for AnArtifactThisSystemDoesNotKnow {
    /// That the name is not declared, and that this is a gap and not a fault.
    ///
    /// **A gap here and not a fault of the run**, because an artifact with no rule written for it is
    /// a gap in the pipeline — **and blaming a run for a name this repository has not declared sends
    /// somebody to fix the run.**
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} is not declared here, so it is a gap in this pipeline and not a fault of the run. \
             What is declared is: {}",
            self.what_was_named,
            WHAT_THE_PIPELINE_KNOWS.join(", ")
        )
    }
}

/// The name and the version of one thing an agent produced, both checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAnAgentHasProduced {
    which_artifact: WhichArtifact,
    the_version: AnArtifactVersion,
}

impl WhatAnAgentHasProduced {
    /// What an agent says it produced, refused when the name is not declared or the version is
    /// not a version.
    pub fn new(
        what_was_named: &str,
        the_version: u32,
    ) -> Result<Self, WhatWasNotAThingThisSystemHolds> {
        if !WHAT_THE_PIPELINE_KNOWS.contains(&what_was_named) {
            return Err(nobody_declared(what_was_named));
        }
        let the_version = AnArtifactVersion::try_from(the_version)
            .map_err(WhatWasNotAThingThisSystemHolds::NotAVersion)?;
        Ok(WhatAnAgentHasProduced {
            which_artifact: WhichArtifact::try_from(what_was_named)
                .expect("the name was just found among the declared ones"),
            the_version,
        })
    }

    /// Which artifact it is, as a gate can read it.
    pub fn which_artifact(&self) -> &WhichArtifact {
        &self.which_artifact
    }

    /// Which version of it, because a version is what makes an approval go stale.
    pub fn the_version(&self) -> &AnArtifactVersion {
        &self.the_version
    }
}

/// Why what an agent said it produced is not a thing this system holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatWasNotAThingThisSystemHolds {
    /// Nobody here has declared that name.
    NoSuchArtifact(AnArtifactThisSystemDoesNotKnow),
    /// The version was not a version.
    NotAVersion(AnArtifactVersionBelowOne),
}

impl std::fmt::Display for WhatWasNotAThingThisSystemHolds {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhatWasNotAThingThisSystemHolds::NoSuchArtifact(why_not) => {
                write!(formatter, "{why_not}")
            }
            WhatWasNotAThingThisSystemHolds::NotAVersion(why_not) => {
                write!(formatter, "{why_not}")
            }
        }
    }
}
