//! The gate for RULE 2.5: a role is a type, and the set of roles is closed.
//!
//! The five are transcribed here rather than imported, on purpose. A test that reads its
//! own expectation from the code it checks proves nothing, and `EVERY_ROLE` is the thing
//! being checked — so importing it would be a gate that agrees with whatever the code
//! happens to say today.
//!
//! **The property that matters is the one the compiler holds.** A role is an enum, so a
//! sixth one does not compile until a `match` over it is completed, and a place that asks
//! which discipline is accountable gains a case or the build stops. The runtime assertions
//! below cannot hold that property; they hold the spelling, the count and the refusal,
//! which are the parts a reader would otherwise have to take on trust.

use aisdlc_domain::{EVERY_ROLE, Role};

/// The five roles, written as the specification writes them, in its order.
///
/// Spelled out here and not read from `Role`, for the reason in this file's first
/// paragraph. The order is the specification's, and a test that kept its own order could
/// not tell a reordering from a change of membership.
const THE_FIVE_ROLES_THE_SPECIFICATION_NAMES: [&str; 5] = ["PO", "DEV", "QA", "SEC", "ARCH"];

#[test]
fn a_role_is_written_with_its_own_name_in_capitals() {
    let written: Vec<String> = EVERY_ROLE.iter().map(ToString::to_string).collect();
    assert_eq!(
        written, THE_FIVE_ROLES_THE_SPECIFICATION_NAMES,
        "a role is an enum and it is written in capitals, and a reader reads the written form rather than the variant"
    );
}

#[test]
fn there_are_exactly_five_roles() {
    assert_eq!(
        EVERY_ROLE.len(),
        5,
        "the set is closed and this is its size, and a sixth one is a change to the specification rather than a change to this file"
    );
}

#[test]
fn every_role_the_specification_names_exists() {
    for name in THE_FIVE_ROLES_THE_SPECIFICATION_NAMES {
        assert!(
            Role::try_from(name).is_ok(),
            "{name} is one of the five, and a name the specification writes must be a name a role answers to"
        );
    }
}

#[test]
fn a_name_nobody_wrote_is_not_a_role() {
    for name in ["AI", "OWNER", "TEAM", "", "   ", "po", "PO ", "Product"] {
        let refused = Role::try_from(name);
        assert!(
            refused.is_err(),
            "{name:?} is not one of the five, and a name nobody wrote down never becomes one"
        );
    }
}

#[test]
fn a_name_nobody_wrote_says_which_name_it_was_looking_for() {
    let Err(refusal) = Role::try_from("AI") else {
        panic!("AI is not one of the five, so the refusal is the only answer");
    };
    assert_eq!(
        refusal.what_was_looked_for(),
        "AI",
        "a refusal that does not quote the name back spends a correction and teaches the caller nothing"
    );
    let said = refusal.to_string();
    assert!(
        said.contains("AI"),
        "the written refusal names the name too, and a reader debugging a run reads the written one: {said}"
    );
}
