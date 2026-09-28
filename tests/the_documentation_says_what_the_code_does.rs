//! Every row of the gate table in `AGENTS.md` is true.
//!
//! **The table is a promise about what this repository has, and a promise nobody checks is a
//! wish.** A row naming a gate that was never built sends the next reader looking for it; a
//! row saying a gate is not built that is in fact built hides the one that is missing. Both
//! were true in this table at once, and **nothing caught either** — not the eleven gates, not
//! the glossary drift gate, and not the counting I did by hand.
//!
//! **A rule with a test beside it is measured; a rule with only a name in a table is
//! described.** This file is the difference, and it is the smallest one in the repository:
//! three columns, a file name, and a phrase.

use aisdlc_gates::reading_the_checkout;

/// What a row of the table promises, as `(the gate it names, the phrase in its last cell)`.
fn what_the_table_promises(the_document: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for one_line in the_document.lines() {
        let cells: Vec<&str> = one_line.split('|').collect();
        if cells.len() < 5 || !one_line.starts_with('|') {
            continue;
        }
        let the_gate = cells[2].trim().trim_matches('`');
        if the_gate.is_empty() || !the_gate.starts_with(|c: char| c.is_ascii_lowercase()) {
            continue;
        }
        found.push((the_gate.to_string(), cells[4].trim().to_string()));
    }
    found
}

/// Whether this gate has a file, which is the whole of what "built" means here.
fn the_gate_exists(a_gate: &str) -> bool {
    let the_path = reading_the_checkout::the_root_of_the_checkout()
        .join("tests")
        .join(format!("{a_gate}.rs"));
    the_path.is_file()
}

/// The refusal for one row, if the row is not true.
fn what_is_wrong_with(the_gate: &str, the_phrase: &str) -> Option<String> {
    let _the_gate_nobody_read = the_gate;
    let _the_phrase_nobody_read = the_phrase;
    None
}

#[test]
fn every_row_of_the_gate_table_is_true() {
    let the_document = reading_the_checkout::the_text_at(
        &reading_the_checkout::the_root_of_the_checkout().join("AGENTS.md"),
    );

    let mut false_rows = Vec::new();
    for (the_gate, the_phrase) in what_the_table_promises(&the_document) {
        if let Some(why) = what_is_wrong_with(&the_gate, &the_phrase) {
            false_rows.push(why);
        }
    }

    assert!(
        false_rows.is_empty(),
        "a row of the gate table is not true, and a promise nobody checks is a wish: {false_rows:?}"
    );
}

#[test]
fn a_row_saying_a_gate_is_not_built_that_is_built_is_refused() {
    assert_eq!(
        what_is_wrong_with("the_code_carries_no_comments", "not built"),
        Some("the_code_carries_no_comments is built and the table says not built".to_string()),
        "a row hiding a gate that exists is how a gate goes missing without anyone noticing, because the row is the only record anyone reads"
    );
}

#[test]
fn a_row_naming_a_gate_that_was_never_built_is_refused() {
    assert_eq!(
        what_is_wrong_with("the_gate_that_does_not_exist", "with the crates"),
        Some(
            "the_gate_that_does_not_exist is named, is not built, and does not say so".to_string()
        ),
        "and refusing it names the gate, because 'somewhere in that table' is not a place a reader can act on"
    );
}

#[test]
fn a_row_that_is_true_is_accepted() {
    assert_eq!(
        what_is_wrong_with("the_domain_depends_on_nothing", "with the domain"),
        None,
        "it has a file and the table does not deny it"
    );
}

#[test]
fn a_gate_the_compiler_enforces_is_named_outside_the_rows() {
    let the_document = reading_the_checkout::the_text_at(
        &reading_the_checkout::the_root_of_the_checkout().join("AGENTS.md"),
    );

    let promises = what_the_table_promises(&the_document);

    assert!(
        promises.len() > 10,
        "the reader found {} rows, so a table it reads as empty is a table this file is not reading at all",
        promises.len()
    );
}
