//! What an agent has produced, and the closed set of names this system knows.
//!
//! **`WhichArtifact` is an open `String` and it accepts anything.** A gate that approves an
//! artifact nobody here has ever heard of is a gate that approved a stranger's paperwork — and the
//! record names it as approved, which is the one thing a record must never do.
//!
//! **The seven names are the specification's, written out in the mother project's router**, and a
//! run may name one of them and nothing else. **An artifact with no rule written for it is a gap in
//! the pipeline and not a fault in the run** — so the refusal says that, rather than blaming a run
//! for a name this repository has not declared.

use aisdlc_run::what_an_agent_has_produced::WhatAnAgentHasProduced;

#[test]
fn a_name_the_specification_writes_out_is_a_name_this_system_knows() {
    let every_one_the_specification_lists = [
        "DiscoveryReport",
        "Requirements",
        "QaPlan",
        "ArchitectureReport",
        "SecurityReport",
        "ImplementationPlan",
        "VerificationReport",
    ];

    let the_ones_refused: Vec<String> = every_one_the_specification_lists
        .iter()
        .filter_map(|one| WhatAnAgentHasProduced::new(one, 1).err())
        .map(|why_not| why_not.to_string())
        .collect();

    assert!(
        the_ones_refused.is_empty(),
        "**because these seven names are written out in the specification's own router** — and a run \\
         naming one of them and being refused would mean this repository is narrower than the \\
         pipeline it is supposed to run: {the_ones_refused:?}"
    );
}

#[test]
fn a_name_nobody_declared_is_refused_before_it_reaches_a_gate() {
    let what_was_produced = WhatAnAgentHasProduced::new("AProposalOfMine", 1);

    assert!(
        what_was_produced.is_err(),
        "**because a gate that approves an artifact nobody here has heard of is a gate that \\
         approved a stranger's paperwork** — and the record names it as approved, which is the one \\
         thing a record must never do: {what_was_produced:?}"
    );
}

#[test]
fn the_refusal_says_it_is_a_gap_here_and_not_a_fault_of_the_run() {
    let the_way_out = WhatAnAgentHasProduced::new("AProposalOfMine", 1);

    let the_words = the_way_out
        .expect_err("nothing declares that one")
        .to_string();
    assert!(
        the_words.contains("gap") || the_words.contains("not declared"),
        "**because an artifact with no rule written for it is a gap in the pipeline and not a fault \\
         in the run** — blaming a run for a name this repository has not declared sends somebody to \\
         fix the run: {the_words}"
    );
}

#[test]
fn a_version_below_one_is_never_a_version() {
    let what_was_produced = WhatAnAgentHasProduced::new("DiscoveryReport", 0);

    assert!(
        what_was_produced.is_err(),
        "**because version zero is the state before there was a version** — and a gate presented \\
         with version zero is being shown nothing and asked to rule on it: {what_was_produced:?}"
    );
}
