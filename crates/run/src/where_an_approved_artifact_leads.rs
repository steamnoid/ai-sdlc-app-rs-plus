//! Where a run goes after a gate, and what a refusal ends.
//!
//! **A table and not a chain of `if`s, because the chain is the defect that cost the sibling a
//! run.** Its router fell through: anything that was not a discovery report went to the
//! implementation, so approving the verification sent the run back into the implementation,
//! which made the change and asked about the verification again, **and nothing in the state
//! changed to stop it** — two container suite runs per pass until the framework's recursion
//! limit, reported by a green suite because the router was covered by nothing.
//!
//! **A stage and not a node name, because this repository has no graph.** A destination is a
//! stage of the pipeline, **and the artifact a stage produces is written in the same row** — so
//! the two answers are one list rather than two a reader has to keep in step.
//!
//! **A refusal is not a row, and the caller carries the verdict.** A work whose ruling was a
//! refusal is not delivered, **so the one function that knows that is here and not spread over
//! seven call sites** which would each have to remember it.

use aisdlc_domain::approval::WhichArtifact;

/// Every place a run can be after a gate, **and one of them is not being anywhere.**
///
/// **Eight and not seven**, because a refusal ends the run and that is a destination like any
/// other. **A run standing nowhere is not an error state** — it is a work whose ruling was a
/// refusal, which the domain's own table already treats as a real outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereARunGoesNext {
    /// The product analyst is asked what this work is.
    TheProductAnalysis,
    /// The quality analyst is asked what would prove it.
    TheQualityAnalysis,
    /// The architecture analyst is asked how the thing is put together.
    TheArchitectureAnalysis,
    /// The security analyst is asked what could go wrong.
    TheSecurityAnalysis,
    /// The plan is written.
    ThePlanning,
    /// The plan is carried out and checked.
    TheImplementation,
    /// The change is put on a branch and offered as a pull request.
    TheDelivery,
    /// Nothing further, because a person refused.
    TheRunIsOver,
}

/// An artifact the pipeline holds and the router has no destination for, **quoted whole.**
///
/// **A different fact from `AnArtifactThisSystemDoesNotKnow`, and the difference is which file
/// to open.** That one refuses a name the *pipeline* does not hold; this one refuses a name the
/// pipeline holds and the *table* has no row for. **A reader told only "it is not known here"
/// would go looking in the wrong file.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnArtifactThisRouterHoldsNoDestinationFor {
    /// What was named at the gate.
    pub what_was_named: String,
}

impl std::fmt::Display for AnArtifactThisRouterHoldsNoDestinationFor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} is approved at a gate and this router holds no destination for it, and a route with \
             no destination is a route into nothing — this is a gap in the table and not a request \
             to implement something",
            self.what_was_named
        )
    }
}

impl std::error::Error for AnArtifactThisRouterHoldsNoDestinationFor {}

/// Where an approved artifact leads, **and the refusal when the table has no row for it.**
///
/// **The seven rows and nothing else**, so an artifact added to the pipeline without a row here
/// is refused by name. **A `match` and not a lookup, because the compiler checks that every
/// artifact variant is handled** — a table of strings would compile with a row missing and
/// that row is the whole defect.
pub fn where_an_approved_artifact_leads(
    which_artifact: &WhichArtifact,
) -> Result<WhereARunGoesNext, AnArtifactThisRouterHoldsNoDestinationFor> {
    let the_destination = match which_artifact.0.as_str() {
        "DiscoveryReport" => Some(WhereARunGoesNext::TheProductAnalysis),
        "Requirements" => Some(WhereARunGoesNext::TheQualityAnalysis),
        "QaPlan" => Some(WhereARunGoesNext::TheArchitectureAnalysis),
        "ArchitectureReport" => Some(WhereARunGoesNext::TheSecurityAnalysis),
        "SecurityReport" => Some(WhereARunGoesNext::ThePlanning),
        "ImplementationPlan" => Some(WhereARunGoesNext::TheImplementation),
        "VerificationReport" => Some(WhereARunGoesNext::TheDelivery),
        _ => None,
    };
    the_destination.ok_or_else(|| AnArtifactThisRouterHoldsNoDestinationFor {
        what_was_named: which_artifact.to_string(),
    })
}

/// Where one gate's outcome leads, **and a refusal is the run being over at every gate.**
///
/// **Both halves in one function and not one of them here.** The table answers "and what
/// does the run do next", which says nothing about a person saying no; **a caller that had to
/// remember which gates were terminal would be a caller that eventually would not**, and the
/// sibling's loop is what not remembering costs.
pub fn where_a_gate_outcome_leads(
    which_artifact: &WhichArtifact,
    was_it_approved: bool,
) -> Result<WhereARunGoesNext, AnArtifactThisRouterHoldsNoDestinationFor> {
    if !was_it_approved {
        return Ok(WhereARunGoesNext::TheRunIsOver);
    }
    where_an_approved_artifact_leads(which_artifact)
}
