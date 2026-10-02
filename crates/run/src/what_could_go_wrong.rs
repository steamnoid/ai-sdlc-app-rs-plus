//! What the security analyst concluded, and the two things its list may and may not contain.
//!
//! **An empty list is a result and not a fault.** A model asked *what could go wrong here* answers
//! with something, **and an analyst that has to produce a finding to be believed will produce
//! one** — so a rule demanding at least one would convert a calm repository into a made-up
//! vulnerability, **and that made-up finding is what a person then rules on at a gate.** This is
//! why `what_was_inspected` is a field: a report saying *nothing* has to say **what it looked at**
//! to mean anything.
//!
//! **And a finding naming no place is refused**, because a finding about nowhere is a finding
//! about nothing — the sister implementation's history records a real run citing a path that
//! resolved to nothing **while looking exactly like a link.**

use std::fmt;

/// How bad a risk is, **and each level names what it does to the run rather than a colour.**
///
/// **`AMinorRisk` and not `Low`.** A colour scale says how somebody felt, **and the only part of
/// a severity a gate can act on is what it does.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HowBadItIs {
    /// Worth writing down, **and it does not stop the work.**
    AMinorRisk,
    /// Worth fixing before this ships, **and it does not stop the work either.**
    AModerateRisk,
    /// Stops the shipment until somebody has dealt with it.
    ARiskThatStopsAShipment,
}

/// One thing that could go wrong, **and the file it lives in.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASecurityFinding {
    /// What is at risk, **and never a blank.**
    pub what_is_at_risk: String,
    /// Where in the repository, **and never a blank — a finding about nowhere is about nothing.**
    pub where_in_the_repository: String,
    /// What it does to the run, **and never free text.**
    pub how_bad_it_is: HowBadItIs,
}

impl ASecurityFinding {
    /// How bad it is, **so a caller sorting or reading one does not reach for the field.**
    pub fn how_bad_it_is(&self) -> HowBadItIs {
        self.how_bad_it_is
    }
}

/// What the security analyst concluded, **and what it says it looked at.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatIsAtRisk {
    /// The risks, **and none of them is a result in itself.**
    pub findings: Vec<ASecurityFinding>,
    /// What the run read, **and this is what makes an empty list mean anything.**
    pub what_was_inspected: Vec<String>,
}

/// Why these findings cannot be put in front of a person, **and each is a different fact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatCouldGoWrong {
    /// A finding naming no place, **quoted whole so a reader can see what it was about.**
    AFindingAboutNowhere {
        /// What it claimed, **because the missing half is the place and not the risk.**
        what_it_claimed: String,
    },
    /// A finding with no risk in it, **quoted whole because the severity is the only other half.**
    ANamedRisk {
        /// Where it claimed to be, **so the reader knows which file the blank belongs to.**
        where_it_claimed: String,
    },
}

impl WhatCouldGoWrong {
    /// The finding named no place, **and it quotes what the finding *was* about.**
    pub fn a_finding_about_nowhere(what_it_claimed: &str) -> Self {
        WhatCouldGoWrong::AFindingAboutNowhere {
            what_it_claimed: what_it_claimed.to_string(),
        }
    }

    /// The finding named no risk, **and it quotes where it claimed to be.**
    pub fn a_named_risk(where_it_claimed: &str) -> Self {
        WhatCouldGoWrong::ANamedRisk {
            where_it_claimed: where_it_claimed.to_string(),
        }
    }
}

impl fmt::Display for WhatCouldGoWrong {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            WhatCouldGoWrong::AFindingAboutNowhere { what_it_claimed } => format!(
                "{what_it_claimed} names no place in the repository, and a finding about nowhere \
                 is a finding about nothing"
            ),
            WhatCouldGoWrong::ANamedRisk { where_it_claimed } => format!(
                "{where_it_claimed} is named with no risk in it, and a severity with nothing \
                 attached to it is a colour on a chart — the one artefact a person cannot act on"
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for WhatCouldGoWrong {}

impl WhatIsAtRisk {
    /// The findings a gate would be shown, **worst first, or refused.**
    ///
    /// **Sorting is the point of this function.** A person reads a list top to bottom and stops
    /// reading, **so a list in the order a model produced it puts the risk that stops a shipment
    /// wherever the model happened to put it.**
    pub fn what_the_gate_may_be_shown(&self) -> Result<Vec<ASecurityFinding>, WhatCouldGoWrong> {
        self.each_finding_names_a_place_and_a_risk()?;
        let mut worst_first = self.findings.clone();
        worst_first.sort_by_key(|one_finding| std::cmp::Reverse(one_finding.how_bad_it_is));
        Ok(worst_first)
    }

    /// Whether every finding names both halves, **and an empty list passes here on purpose.**
    fn each_finding_names_a_place_and_a_risk(&self) -> Result<(), WhatCouldGoWrong> {
        for one_finding in &self.findings {
            if one_finding.where_in_the_repository.trim().is_empty() {
                return Err(WhatCouldGoWrong::a_finding_about_nowhere(
                    &one_finding.what_is_at_risk,
                ));
            }
            if one_finding.what_is_at_risk.trim().is_empty() {
                return Err(WhatCouldGoWrong::a_named_risk(
                    &one_finding.where_in_the_repository,
                ));
            }
        }
        Ok(())
    }
}
