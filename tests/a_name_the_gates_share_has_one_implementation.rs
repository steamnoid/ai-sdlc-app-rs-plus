//! The gate for RULE 2.1 in the test tree: a name the gates share has one implementation.
//!
//! Every rule in this repository is a test, and the tests are code, so RULE 2.1 applies
//! to them exactly as it applies to the domain. `WorkItem` has one name in production and
//! in the tests; so must `the_text_at`.
//!
//! **Six helpers now have two, three or four copies, and the copies are already
//! different.** `every_rust_file_in_the_workspace` walks three roots in one gate and two
//! in another. `the_text_at` panics in one copy and returns a `Result` in another. A gate
//! and its reader having quietly forked is the failure a name gate exists to prevent, and
//! this gate is what stops the forking rather than a reviewer's memory of it.
//!
//! The shared code is `pub` in the library rather than duplicated, because a private
//! helper in a test binary is reported dead the moment the test that does not use it is
//! compiled, and silencing that is `#[allow(dead_code)]` — which `no_allow_silences_a_lint`
//! refuses. So the choice was never duplication or silence; it was duplication or one
//! public place, and this gate is the argument for the second.

use std::collections::BTreeMap;
use std::path::Path;

use aisdlc_gates::scanning::{self, HowTheFileIsWritten, WhatIsWrittenThere, the_text_at};

/// The helpers more than one gate has needed, and the place each one has to live.
///
/// A gate may add a helper here. A gate may not define a helper of one of these names
/// anywhere else, and the table is here so that the rule is a list a reader can check
/// rather than a judgement about what counts as a helper.
const THE_HELPERS_THE_GATES_SHARE: [&str; 6] = [
    "the_root_of_the_checkout",
    "the_text_at",
    "the_glossary_text",
    "every_rust_file_under",
    "every_rust_file_in_the_workspace",
    "every_rust_file_of_the_system",
];

/// Where a shared helper is allowed to be defined: the library, and nowhere else.
const THE_LIBRARY: &str = "src/";

/// The lines of code a test file holds, so a definition inside a `///` is not counted.
///
/// A helper written in a fixture is a fixture. This gate reads the same way every other
/// gate here reads, and the difference is the whole of why it can be trusted about a file
/// that discusses the names it is judging.
fn the_code_lines_of(path: &Path) -> Vec<String> {
    HowTheFileIsWritten::of(&the_text_at(path))
        .stretches()
        .iter()
        .filter(|stretch| stretch.kind() == WhatIsWrittenThere::Code)
        .map(|stretch| stretch.text().to_string())
        .collect()
}

/// Every line of every test file, in the order they were read, with the file each is in.
///
/// The map is what lets a refusal say "this name is defined here and here" rather than
/// "somewhere", which is the difference between a gate nobody can act on and one somebody
/// can fix in a minute.
fn every_line_of_every_test() -> BTreeMap<String, Vec<String>> {
    let mut lines = BTreeMap::new();
    for path in scanning::every_rust_file_under(&["tests"]) {
        let name = path
            .file_name()
            .map_or_default(|n| n.to_string_lossy().to_string());
        lines.insert(name, the_code_lines_of(&path));
    }
    lines
}

fn the_lines_defining(line: &str, name: &str) -> bool {
    let starts_a_function = line.trim_start().starts_with("fn ");
    let names_the_function = line
        .split_whitespace()
        .any(|word| word.split('(').next().is_some_and(|head| head == name));
    starts_a_function && names_the_function
}

fn where_a_shared_helper_is_defined(name: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (file, lines) in every_line_of_every_test() {
        if lines.iter().any(|line| the_lines_defining(line, name)) {
            found.push(format!("tests/{file}"));
        }
    }
    found
}

#[test]
fn a_shared_helper_is_not_defined_in_the_test_tree() {
    let mut refused = Vec::new();
    for name in THE_HELPERS_THE_GATES_SHARE {
        let defined_in = where_a_shared_helper_is_defined(name);
        if defined_in.len() > 1 {
            refused.push(format!("{name} is defined in {}", defined_in.join(" and ")));
        }
    }
    assert!(
        refused.is_empty(),
        "one concept, one name, everywhere — and a helper with two copies is two names already disagreeing. \
         They belong in {THE_LIBRARY}scanning.rs: {refused:?}"
    );
}

#[test]
fn the_table_of_shared_helpers_is_not_empty() {
    assert!(
        THE_HELPERS_THE_GATES_SHARE.len() > 3,
        "the table of shared helpers holds {} entries, which is too few to be the list",
        THE_HELPERS_THE_GATES_SHARE.len()
    );
    let found = scanning::every_rust_file_under(&["tests"]);
    assert!(
        found.len() > 5,
        "the walk read {} test files, which is too few to be this repository's gates",
        found.len()
    );
}

#[test]
fn a_definition_written_in_a_fixture_is_not_a_definition() {
    let fixture = "    let the_text_at = something_worth_saying_about_it;";
    assert!(
        !the_lines_defining(fixture, "the_text_at"),
        "a name bound in a fixture is a fixture, and only a function defines a helper — a gate that counted bindings would refuse this repository's own tests for talking about its helpers"
    );
    assert!(
        the_lines_defining("fn the_text_at(path: &Path) -> String {", "the_text_at"),
        "a real definition is a definition, and a gate that could not see one would agree with everything"
    );
}
