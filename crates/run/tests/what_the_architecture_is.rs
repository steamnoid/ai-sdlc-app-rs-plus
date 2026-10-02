//! What the architecture analyst concluded, and the one claim a manifest has to back.
//!
//! **A claimed dependency the manifests do not declare is the worst thing in this artifact.** A
//! module that says it depends on something the project never asked for is a run planning a
//! change against an architecture that does not exist — **and it is the failure the architecture
//! tier already measured**, against a real model on a real repository, and measured *passing*.
//!
//! **So the check is a comparison and not a judgement**, because a model asked whether a
//! dependency is real will answer yes, **and a manifest either names it or it does not.**

use aisdlc_run::what_the_architecture_is::{
    AModule, NoArchitectureWasDescribed, WhatTheArchitectureIs,
};

/// One module, **with one dependency the project's manifest really does declare.**
fn a_module() -> AModule {
    AModule {
        what_it_is_called: String::from("a_record_of_a_change"),
        what_it_is_for: String::from("the port a change is made through"),
        what_it_depends_on: vec![String::from("serde")],
    }
}

/// What the analyst concluded, **one module and a sentence about the whole.**
fn what_the_architecture_is() -> WhatTheArchitectureIs {
    WhatTheArchitectureIs {
        modules: vec![a_module()],
        how_the_whole_is_put_together: String::from(
            "every agent reads the clone through the same two ports",
        ),
        what_a_manifest_declares: vec![String::from("serde")],
    }
}

#[test]
fn an_architecture_naming_a_dependency_no_manifest_declares_is_refused() {
    let mut one_module = a_module();
    one_module.what_it_depends_on = vec![String::from("tokio")];
    let the_architecture = WhatTheArchitectureIs {
        modules: vec![one_module],
        ..what_the_architecture_is()
    };

    let the_refusal = the_architecture.the_dependencies_the_manifests_declare();

    assert!(
        matches!(
            &the_refusal,
            Err(NoArchitectureWasDescribed::ADependencyNoManifestDeclares { .. })
        ),
        "**because a module that depends on something the project never asked for is a run \
         planning a change against an architecture that does not exist** — and this is the failure \
         the architecture tier already measured against a real model and measured passing. What \
         was refused: {the_refusal:?}"
    );
}

#[test]
fn an_architecture_naming_no_module_is_refused() {
    let the_refusal = WhatTheArchitectureIs {
        modules: vec![],
        ..what_the_architecture_is()
    }
    .the_dependencies_the_manifests_declare();

    assert!(
        matches!(
            &the_refusal,
            Err(NoArchitectureWasDescribed::NoModuleWasNamed)
        ),
        "**because a sentence about how the whole is put together is not a list anybody can plan \
         against** — and it reads as an architecture, because it is a paragraph about one. What \
         was refused: {the_refusal:?}"
    );
}

#[test]
fn a_module_named_without_saying_what_it_is_for_is_refused() {
    let the_refusal = WhatTheArchitectureIs {
        modules: vec![AModule {
            what_it_is_called: String::from("lib"),
            what_it_is_for: String::from(" "),
            what_it_depends_on: vec![String::from("serde")],
        }],
        ..what_the_architecture_is()
    }
    .the_dependencies_the_manifests_declare();

    assert!(
        matches!(
            &the_refusal,
            Err(NoArchitectureWasDescribed::AModuleNamedOnly { .. })
        ),
        "**because a module named without a purpose is a filename** — and this system's naming \
         rule says the same thing about code: a name that does not say what the thing is is a \
         name nobody can hold to. What was refused: {the_refusal:?}"
    );
}

#[test]
fn a_whole_described_only_in_prose_is_refused_when_no_module_is_named() {
    let the_refusal = WhatTheArchitectureIs {
        modules: vec![],
        how_the_whole_is_put_together: String::from(
            "the agents and the ports and the outside, in that order",
        ),
        what_a_manifest_declares: vec![String::from("serde")],
    }
    .the_dependencies_the_manifests_declare();

    assert!(
        matches!(
            &the_refusal,
            Err(NoArchitectureWasDescribed::NoModuleWasNamed)
        ),
        "**because prose about an architecture is the shape a model answers in when nothing holds \
         it to a list** — and the fix for that is the refusal, not a longer prompt. What was \
         refused: {the_refusal:?}"
    );
}

#[test]
fn an_architecture_whose_dependencies_the_manifests_all_declare_is_accepted() {
    let the_refusal = what_the_architecture_is().the_dependencies_the_manifests_declare();

    assert!(
        the_refusal.is_ok(),
        "**because this is the ordinary case and a gate must be able to see it pass** — a check \
         that refused everything would deliver nothing, and this repository's own history is six \
         real runs spent on a run that refused everything it was asked to do. What was refused: \
         {the_refusal:?}"
    );
}
