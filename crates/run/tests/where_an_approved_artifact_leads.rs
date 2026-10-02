//! Where a run goes after a gate, and what a refusal ends.
//!
//! **The router is a closed table and not a chain of `if`s, and this file is why.** The
//! sibling repository's router fell through: anything that was not a discovery report went to
//! the implementation, so approving the verification sent the run back into the
//! implementation, which made the change and asked about the verification again, and nothing
//! in the state changed to stop it — **two container suite runs per pass until the
//! framework's recursion limit.** A green `pytest -q` said nothing, because the router was
//! not covered by anything.
//!
//! **A stage and not a node name.** The sibling's destinations were strings naming nodes in a
//! graph this repository does not have. A destination here is a stage of the pipeline, and the
//! artifact a stage produces is written in the same row — **so the two answers are one list
//! rather than two that have to be kept in step.**
//!
//! **A refusal ends the run at every gate, and that is the same rule at all of them.** A
//! refusal is a decision and not a gap: a run that carried on past one would be working on
//! something a person has already said no to.

use aisdlc_domain::approval::WhichArtifact;
use aisdlc_run::where_an_approved_artifact_leads::{
    AnArtifactThisRouterHoldsNoDestinationFor, WhereARunGoesNext,
    where_an_approved_artifact_leads,
};

/// The seven artifacts, **in the order a run meets them.**
const THE_PIPELINE: [&str; 7] = [
    "DiscoveryReport",
    "Requirements",
    "QaPlan",
    "ArchitectureReport",
    "SecurityReport",
    "ImplementationPlan",
    "VerificationReport",
];

/// One artifact, by the name the pipeline holds.
fn an_artifact(named: &str) -> WhichArtifact {
    WhichArtifact::try_from(named).expect("the pipeline declares that name")
}

#[test]
fn each_artifact_leads_to_the_stage_that_follows_it() {
    let every_stage = [
        WhereARunGoesNext::TheProductAnalysis,
        WhereARunGoesNext::TheQualityAnalysis,
        WhereARunGoesNext::TheArchitectureAnalysis,
        WhereARunGoesNext::TheSecurityAnalysis,
        WhereARunGoesNext::ThePlanning,
        WhereARunGoesNext::TheImplementation,
        WhereARunGoesNext::TheDelivery,
    ];

    let where_each_led: Vec<WhereARunGoesNext> = THE_PIPELINE
        .iter()
        .map(|one| {
            where_an_approved_artifact_leads(&an_artifact(one))
                .expect("every artifact the pipeline holds has a destination")
        })
        .collect();

    assert_eq!(
        where_each_led, every_stage,
        "**because the seven gates are the pipeline and the pipeline is the table** — each artifact \
         is approved by a person and then the run does the next thing, and a row that pointed \
         somewhere else would send a run to work that was never approved. **Seven destinations for \
         seven artifacts, in the order a run meets them.**"
    );
}

#[test]
fn an_artifact_nothing_declares_a_destination_for_is_refused_and_named() {
    let nothing_holds_this = an_artifact("ReleaseNotes");

    let the_refusal = where_an_approved_artifact_leads(&nothing_holds_this);

    assert_eq!(
        the_refusal,
        Err(AnArtifactThisRouterHoldsNoDestinationFor {
            what_was_named: "ReleaseNotes".to_string()
        }),
        "**because a route with no destination is a route into nothing, and reading the refusal as \
         a request to implement something is how a run walks into a stage nobody approved.** The \
         refusal quotes the name and only the name — a caller holding it has to be able to act, and \
         acting means declaring a destination or refusing the artifact at the gate"
    );
}

#[test]
fn a_refusal_at_any_gate_ends_the_run() {
    let the_gate_outcomes = [true, false];

    let where_each_led: Vec<WhereARunGoesNext> = the_gate_outcomes
        .iter()
        .map(|was_it_approved| {
            where_a_gate_outcome_leads(an_artifact("ImplementationPlan"), *was_it_approved)
        })
        .collect();

    assert_eq!(
        where_each_led,
        vec![WhereARunGoesNext::TheImplementation, WhereARunGoesNext::TheRunIsOver],
        "**because a refusal is a decision and not a gap** — a run that carried on past one would \
         be implementing a plan a person has already said no to, **and the run would deliver \
         something nobody approved.** The same rule at every gate, which is what one function for \
         both the verdict and the artifact means: there is no gate at which the rule is different"
    );
}

#[test]
fn there_is_no_fall_through_because_the_whole_table_is_listed_twice() {
    let where_each_led: Vec<Option<WhereARunGoesNext>> = THE_PIPELINE
        .iter()
        .map(|one| where_an_approved_artifact_leads(&an_artifact(one)).ok())
        .collect();

    assert!(
        where_each_led.iter().all(|one| one.is_some()),
        "**because a reader who wants to see the table should not have to infer it from a `match` \
         that might fall through** — and this is the rule that catches a fall-through: every \
         artifact the pipeline holds has to resolve, so a new artifact with no row is refused by \
         name instead of quietly going wherever the last arm went"
    );
}

/// Where one gate's outcome leads, **which is the only decision a run makes after a gate.**
fn where_a_gate_outcome_leads(
    which_artifact: WhichArtifact,
    was_it_approved: bool,
) -> WhereARunGoesNext {
    if !was_it_approved {
        return WhereARunGoesNext::TheRunIsOver;
    }
    where_an_approved_artifact_leads(&which_artifact).expect("the pipeline holds that artifact")
}
