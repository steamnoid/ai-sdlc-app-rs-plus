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

/// What a row of the gate table promises, as `(the gate it names, the phrase in its last cell)`.
///
/// **Only the table whose header names a gate.** `AGENTS.md` holds four tables and the other
/// three have three columns, so a reader that took every row of four cells found the layers
/// table, the layout table and two rows of prose, and reported twenty-one gates of which
/// eleven are sentences.
fn what_the_table_promises(the_document: &str) -> Vec<(String, String)> {
    the_rows_of_the_gate_table(the_document)
        .into_iter()
        .filter_map(|(name, what_it_promises)| {
            let it_names_a_gate = name.starts_with(|c: char| c.is_ascii_lowercase());
            if it_names_a_gate {
                Some((name, what_it_promises))
            } else {
                None
            }
        })
        .collect()
}

/// The rows of the gate table, as `(the name in its second cell, its last cell)`.
fn the_rows_of_the_gate_table(the_document: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut inside = false;
    for one_line in the_document.lines() {
        if one_line.starts_with("| Rule |") {
            inside = true;
            continue;
        }
        if !inside || !one_line.starts_with('|') {
            inside = inside && one_line.starts_with('|');
            continue;
        }
        found.push(the_two_cells_of(one_line));
    }
    found
}

/// The name in a row's second cell and the last cell that says something.
fn the_two_cells_of(one_line: &str) -> (String, String) {
    let cells: Vec<&str> = one_line.split('|').collect();
    let name = cells.get(2).map_or("", |one| one.trim().trim_matches('`'));
    (name.to_string(), the_last_cell_with_something_in_it(&cells))
}

/// The last cell that says something, which is the column of when or of built.
///
/// **The last one and not the fourth**, because the gate table has three columns and a reader
/// that reached for the fourth found the empty cell the trailing bar leaves behind. It then
/// read every three-column row as saying nothing about whether the gate exists.
fn the_last_cell_with_something_in_it(cells: &[&str]) -> String {
    cells
        .iter()
        .rev()
        .find(|one| !one.trim().is_empty())
        .map(|one| one.trim().to_string())
        .unwrap_or_default()
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
    let it_exists = the_gate_exists(the_gate);
    let the_table_denies_it = the_phrase.contains("not built");
    if it_exists && the_table_denies_it {
        return Some(format!("{the_gate} is built and the table says not built"));
    }
    if !it_exists && !the_table_denies_it {
        return Some(format!(
            "{the_gate} is named, is not built, and does not say so"
        ));
    }
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
