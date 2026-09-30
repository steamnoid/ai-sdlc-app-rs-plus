//! What a discovery agent concluded, in the fields the specification lists.
//!
//! **Eight fields, and they are the specification's.** The glossary of the mother project names
//! them in one line — languages, frameworks, build system, CI system, architecture summary, key
//! components, evidence files, suggested changes — **and transcribing them is not designing them**,
//! which is the only reason this can be written at all.
//!
//! **A report that says nothing at all is a fault and not an artifact.** Silence that reached a
//! human at a gate would carry the authority of a finished analysis, **and a person cannot tell an
//! empty report from a real one by looking at it.**
//!
//! **An evidence address is derived and not asked for.** A model may leave an address out, and an
//! address a model may invent is worse than none — **so the model contributes the path and the
//! address follows from the repository the run is about.**

use aisdlc_domain::repository::Repository;
use aisdlc_run::what_discovery_concluded::{
    ADiscoveryReportThatSaysNothing, AnEvidenceOutsideTheRepository, WhatDiscoveryConcluded,
};

/// This repository, and the branch its links are built from.
fn this_repository() -> Repository {
    Repository::try_from("steamnoid/one").expect("two halves")
}

/// A report with something in every field that can have something.
fn a_report_that_says_something() -> WhatDiscoveryConcluded {
    WhatDiscoveryConcluded {
        languages: vec!["TypeScript".to_string()],
        frameworks: vec!["Next.js".to_string()],
        build_system: Some("npm".to_string()),
        ci_system: None,
        architecture_summary: "a web app and a clone".to_string(),
        key_components: vec!["web-app".to_string()],
        evidence_files: Vec::new(),
        suggested_changes: vec!["nothing yet".to_string()],
    }
}

#[test]
fn a_report_that_says_nothing_is_a_fault_and_not_an_artifact() {
    let nothing_at_all = WhatDiscoveryConcluded::everything_empty();

    let the_way_out = WhatDiscoveryConcluded::what_the_gate_may_be_shown(&nothing_at_all);

    assert!(
        the_way_out.is_err(),
        "**because silence that reached a human at a gate would carry the authority of a finished \\
         analysis** — and a person cannot tell an empty report from a real one by looking at it: \\
         {the_way_out:?}"
    );
}

#[test]
fn at_least_one_field_is_enough_because_a_repository_has_no_ci() {
    let a_repository_with_no_ci = WhatDiscoveryConcluded {
        ci_system: None,
        ..a_report_that_says_something()
    };

    let the_way_out = WhatDiscoveryConcluded::what_the_gate_may_be_shown(&a_repository_with_no_ci);

    assert!(
        the_way_out.is_ok(),
        "**and not all of them** — because a repository with no CI has nothing to say about CI and \
         must be able to say so. **Absence is representable**, and a rule demanding every field would \\
         only ever be satisfied by a report that guesses: {the_way_out:?}"
    );
}

#[test]
fn an_evidence_address_is_built_from_the_repository_and_the_path_not_asked_for() {
    let the_evidence =
        a_report_that_says_something().what_it_concluded_from(&["crates/domain/src/lib.rs"]);

    assert_eq!(
        the_evidence.first().map(|one| one.link.as_str()),
        Some("https://github.com/steamnoid/one/blob/main/crates/domain/src/lib.rs"),
        "**because a model contributes the path and the address follows from the repository the run \
         is about** — an address a model may invent is worse than none, and one it may leave out is \
         an address nobody can rely on clicking: {the_evidence:?}"
    );
}

#[test]
fn a_path_outside_the_repository_is_not_evidence_of_anything() {
    let the_way_out = WhatDiscoveryConcluded::what_it_concluded_from(&["../../etc/passwd"]);

    assert!(
        the_way_out.is_err(),
        "**because a report's links must stay on the repository the human was asked to approve** — \
         that is where they will be clicked: {the_way_out:?}"
    );
}
