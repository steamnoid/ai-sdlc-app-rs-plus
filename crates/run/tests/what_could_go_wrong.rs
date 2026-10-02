//! What the security analyst concluded, and the two things its list may and may not contain.
//!
//! **A report with no findings is a real result and not a fault.** A model asked "what could go
//! wrong here" answers with something, **and an analyst that has to produce a finding to be
//! believed will produce one** — so a rule demanding at least one would convert a calm repository
//! into a made-up vulnerability, and that made-up finding is what a person rules on at a gate.
//!
//! **A finding naming no place in the repository is refused instead.** The sister implementation's
//! history records a real run citing a path that resolved to nothing **while looking exactly like
//! a link** — and a finding about nowhere is a finding about nothing.

use aisdlc_run::what_could_go_wrong::{
    ASecurityFinding, HowBadItIs, WhatCouldGoWrong, WhatIsAtRisk,
};

/// One risk, **in the file it lives in.**
fn a_finding() -> ASecurityFinding {
    ASecurityFinding {
        what_is_at_risk: String::from(
            "the credential is read out of the environment on every call",
        ),
        where_in_the_repository: String::from("crates/outside/src/the_environment.rs"),
        how_bad_it_is: HowBadItIs::AModerateRisk,
    }
}

/// One risk at a given severity, **and the risk is named so the order can be read back.**
fn a_finding_of(how_bad_it_is: HowBadItIs, what_is_at_risk: &str) -> ASecurityFinding {
    ASecurityFinding {
        what_is_at_risk: String::from(what_is_at_risk),
        how_bad_it_is,
        ..a_finding()
    }
}

/// The severity the gate would be shown first, **and it is the assertion and not a helper.**
fn the_worst_of(every_finding: &[ASecurityFinding]) -> HowBadItIs {
    let first = every_finding.first();
    first.map_or(HowBadItIs::AMinorRisk, ASecurityFinding::how_bad_it_is)
}

/// Every severity in the order the gate would be shown, **for the failure message.**
fn every_severity_in_the_order_shown(every_finding: &[ASecurityFinding]) -> Vec<HowBadItIs> {
    every_finding
        .iter()
        .map(ASecurityFinding::how_bad_it_is)
        .collect()
}

#[test]
fn a_report_naming_no_finding_is_a_result_and_not_a_fault() {
    let the_report = WhatIsAtRisk {
        findings: vec![],
        what_was_inspected: vec![String::from("every file the run read")],
    };

    let may_it_be_shown = the_report.what_the_gate_may_be_shown();

    assert!(
        may_it_be_shown.is_ok(),
        "**because a repository with no risk is a repository with no risk, and an analyst that has \
         to produce a finding to be believed will produce one** — so a rule demanding at least one \
         would convert a calm repository into a made-up vulnerability, and that made-up finding is \
         what a person then rules on. What was refused: {may_it_be_shown:?}"
    );
}

#[test]
fn a_finding_naming_no_place_in_the_repository_is_refused() {
    let the_report = WhatIsAtRisk {
        findings: vec![ASecurityFinding {
            where_in_the_repository: String::from(" "),
            ..a_finding()
        }],
        what_was_inspected: vec![String::from("every file the run read")],
    };

    let the_refusal = the_report.what_the_gate_may_be_shown();

    assert!(
        matches!(
            &the_refusal,
            Err(WhatCouldGoWrong::AFindingAboutNowhere { .. })
        ),
        "**because a finding about nowhere is a finding about nothing** — and the sister \
         implementation's history records a real run citing a path that resolved to nothing while \
         looking exactly like a link. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_finding_naming_no_risk_is_refused() {
    let the_report = WhatIsAtRisk {
        findings: vec![ASecurityFinding {
            what_is_at_risk: String::from("  "),
            ..a_finding()
        }],
        what_was_inspected: vec![String::from("every file the run read")],
    };

    let the_refusal = the_report.what_the_gate_may_be_shown();

    assert!(
        matches!(&the_refusal, Err(WhatCouldGoWrong::ANamedRisk { .. })),
        "**because a severity with no risk attached to it is a colour on a chart** — and a chart \
         is the one artefact a person cannot act on. What was refused: {the_refusal:?}"
    );
}

/// Three findings in the worst-first order, **arrived at in the least useful one.**
fn three_findings_in_the_wrong_order() -> WhatIsAtRisk {
    WhatIsAtRisk {
        findings: vec![
            a_finding(),
            a_finding_of(
                HowBadItIs::ARiskThatStopsAShipment,
                "a pull request from anybody runs",
            ),
            a_finding_of(
                HowBadItIs::AMinorRisk,
                "a dependency is two releases behind",
            ),
        ],
        what_was_inspected: vec![String::from("every file the run read")],
    }
}

#[test]
fn the_risk_that_stops_a_shipment_is_listed_first_whatever_order_they_arrived_in() {
    let the_report = three_findings_in_the_wrong_order();

    let what_the_gate_would_be_shown = the_report
        .what_the_gate_may_be_shown()
        .expect("every finding named a place and a risk");

    assert_eq!(
        (
            what_the_gate_would_be_shown.len(),
            the_worst_of(&what_the_gate_would_be_shown)
        ),
        (3, HowBadItIs::ARiskThatStopsAShipment),
        "**because a person reads a list top to bottom and stops reading** — and a list in the \
         order a model produced it puts the risk that stops a shipment wherever the model put it. \
         The order the gate is shown: {:?}",
        every_severity_in_the_order_shown(&what_the_gate_would_be_shown)
    );
}
