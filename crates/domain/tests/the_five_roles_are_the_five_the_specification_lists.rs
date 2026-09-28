//! The five roles, and the two spellings of each.
//!
//! **Nothing here reads a file**, for the same reason the stage tests do not: this is a
//! test of the domain, and the domain may not reach for a filesystem — not even a test of
//! it, and least of all to fetch the list of disciplines it would otherwise have to trust
//! itself about. The specification's table is checked by a gate, in a package allowed to
//! read, and it reads the source rather than calling into a crate it may not depend on.
//!
//! What is proved here is the pair of spellings, the refusal, and the absence of a
//! discipline that is not one. **"Every role is legitimate at every stage" is deliberately
//! not proved here**, because nothing in this file pairs a role with a stage: the rule is
//! that no function constrains the two against each other, and a test asserting it would
//! have to call something to assert. It belongs to the first function that could violate
//! it, which is `validate_state`.

use aisdlc_domain::role::{ARoleThatNobodyNamed, EVERY_ROLE, Role};

#[test]
fn there_are_exactly_five_roles() {
    assert_eq!(
        EVERY_ROLE.len(),
        5,
        "the specification's table has five rows, and a sixth is a discipline nobody has"
    );
}

#[test]
fn every_role_is_named_once() {
    let mut seen: Vec<String> = EVERY_ROLE.iter().map(ToString::to_string).collect();
    seen.sort();
    let before = seen.len();
    seen.dedup();

    assert_eq!(seen.len(), before, "a discipline is listed twice: {seen:?}");
}

#[test]
fn writing_a_role_and_reading_it_back_gives_the_same_role() {
    for role in EVERY_ROLE {
        let written = role.to_string();
        let read = Role::try_from(written.as_str());

        assert_eq!(
            read.ok(),
            Some(role),
            "{role} writes as {written} and does not read back as itself"
        );
    }
}

#[test]
fn a_name_nobody_wrote_is_refused() {
    let result = Role::try_from("INVENTED");

    assert_eq!(
        result,
        Err(ARoleThatNobodyNamed("INVENTED".to_string())),
        "a discipline nobody wrote down is not a discipline, and a work item recorded against one is work nobody answers for"
    );
}

#[test]
fn a_role_is_refused_in_the_wrong_case() {
    let result = Role::try_from("po");

    assert_eq!(
        result,
        Err(ARoleThatNobodyNamed("po".to_string())),
        "the capitals are the writing and accepting both spellings is accepting two"
    );
}

#[test]
fn the_refusal_says_which_name_was_refused_and_how_many_there_are() {
    let said = ARoleThatNobodyNamed("INVENTED".to_string()).to_string();

    assert!(
        said.contains("INVENTED"),
        "a refusal a reader cannot match teaches nothing: {said}"
    );
    assert!(
        said.contains("five"),
        "and it says the size of the set: {said}"
    );
}

/// The five pairs, as the specification's table gives them.
const EVERY_ROLE_AND_ITS_CAPITALS: [(&str, Role); 5] = [
    ("PO", Role::Product),
    ("DEV", Role::Development),
    ("QA", Role::Quality),
    ("SEC", Role::Security),
    ("ARCH", Role::Architecture),
];

#[test]
fn a_role_is_written_in_capitals_and_is_the_variant_a_reader_expects() {
    for (written, expected) in EVERY_ROLE_AND_ITS_CAPITALS {
        assert_eq!(
            Role::try_from(written).ok(),
            Some(expected),
            "{written} is {expected} in the specification's table and reads as something else"
        );
        assert_eq!(
            expected.to_string(),
            written,
            "the capitals are the writing and the variant is the reading, and this pair disagrees"
        );
    }
}

#[test]
fn there_is_no_role_that_is_not_a_discipline() {
    let the_names: Vec<String> = EVERY_ROLE.iter().map(ToString::to_string).collect();

    assert!(
        !the_names.contains(&"AI".to_string()),
        "a handoff is not a discipline, and a set whose own definition excludes one of its members is not a set of disciplines"
    );
}
