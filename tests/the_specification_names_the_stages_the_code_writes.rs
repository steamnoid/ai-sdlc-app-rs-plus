//! The glossary's table and the enum have not drifted apart.
//!
//! **The glossary promises this in prose — "a table rather than prose, because a test reads
//! it to check the code against it" — and until now nothing read it.** A promise in a
//! document is a wish, and a wish about a table is the one a table is most able to break
//! quietly: seven rows in the prose and six in the code is not a compile error anywhere.
//!
//! The stages are read out of the section that declares them and not out of the whole
//! document, because the glossary also tables the roles, the organisations and the
//! artifact kinds, and a reader that took every capital in the file would collect all of
//! them and compare the lot.
//!
//! **The comparison is made on the source's own text and not by calling the domain.** The
//! gates may not depend on the crates they judge, and a gate that imported `EVERY_STAGE`
//! would pull the domain into the package that is supposed to be standing outside it.

use aisdlc_gates::scanning::{self, HowTheFileIsWritten, WhatIsWrittenThere};

const THE_GLOSSARY: &str = "docs/domain-glossary.md";
const THE_DOMAIN_STAGE: &str = "crates/domain/src/stage.rs";
const THE_HEADING_THE_STAGES_ARE_UNDER: &str = "## The stages";

/// The capitals the glossary's stages table names, read out of the table.
fn the_stages_the_specification_names() -> Vec<String> {
    let the_section =
        the_section_under(&the_text_of(THE_GLOSSARY), THE_HEADING_THE_STAGES_ARE_UNDER);
    let mut named = Vec::new();
    for line in the_section.lines() {
        if let Some(one) = the_capital_in_the_first_cell_of(line) {
            named.push(one);
        }
    }
    named
}

/// The capitals the domain's source writes a stage as.
///
/// **Read out of the string literals and not out of the code around them.** The walk tells
/// a written name from the code that produces it: `Stage::Idle => ` is one stretch, `IDLE`
/// is the next. A name *is* the second, and the first version of this gate read the first
/// and found nothing — the walk had already handed over the literal separately and the
/// gate had reached past it to re-parse what it was given.
///
/// The other half comes for free: a capital mentioned in the file's own documentation is a
/// `Document` stretch and cannot be mistaken for a stage. The rule that a name has to be
/// written to count is enforced by *where* it is rather than by a check that might miss it.
fn the_stages_the_code_writes() -> Vec<String> {
    let the_code = the_text_of(THE_DOMAIN_STAGE);
    let mut written = Vec::new();
    for stretch in HowTheFileIsWritten::of(&the_code).stretches() {
        if stretch.kind() != WhatIsWrittenThere::AString {
            continue;
        }
        if let Some(one) = the_capital_a_written_name_holds(stretch.text()) {
            written.push(one);
        }
    }
    written
}

/// The capital one written name holds, if it holds one at all.
///
/// The quotes are the literal's delimiters and not part of the name, and a literal that is
/// empty or holds anything else is prose: the refusal's own message, a format string, a
/// name to be reported back to whoever wrote it.
fn the_capital_a_written_name_holds(a_written_name: &str) -> Option<String> {
    let without_the_quotes = a_written_name.trim_matches('"');
    let only_capitals = !without_the_quotes.is_empty()
        && without_the_quotes
            .chars()
            .all(|one| one.is_ascii_uppercase() || one == '_');
    if !only_capitals {
        return None;
    }
    Some(without_the_quotes.to_string())
}

/// The capital one row of the glossary's table puts in its first cell, if it is one.
fn the_capital_in_the_first_cell_of(line: &str) -> Option<String> {
    let after_the_bar = line.trim().strip_prefix("| `")?;
    let (the_name, _) = after_the_bar.split_once('`')?;
    let only_capitals = !the_name.is_empty()
        && the_name
            .chars()
            .all(|one| one.is_ascii_uppercase() || one == '_');
    if !only_capitals {
        return None;
    }
    Some(the_name.to_string())
}

/// Everything from the heading down to the next heading of the same weight.
fn the_section_under(the_document: &str, heading: &str) -> String {
    let mut collecting = false;
    let mut the_section = Vec::new();
    for line in the_document.lines() {
        if line.trim() == heading {
            collecting = true;
            continue;
        }
        if collecting && line.starts_with("## ") {
            break;
        }
        if collecting {
            the_section.push(line);
        }
    }
    the_section.join("\n")
}

fn the_text_of(wanted: &str) -> String {
    scanning::the_text_at(&scanning::the_root_of_the_checkout().join(wanted))
}

#[test]
fn the_specification_names_exactly_the_stages_the_code_writes() {
    let named = the_stages_the_specification_names();
    let written = the_stages_the_code_writes();

    assert_eq!(
        named, written,
        "the glossary's table and the code have drifted apart, and the table is the one that is right"
    );
}

#[test]
fn there_are_six_stages_on_both_sides() {
    let named = the_stages_the_specification_names();
    let written = the_stages_the_code_writes();

    assert_eq!(named.len(), 6, "the table is the specification: {named:?}");
    assert_eq!(written.len(), 6, "and the code writes as many: {written:?}");
}

#[test]
fn a_stage_the_glossary_names_is_a_stage_the_code_writes() {
    let named = the_stages_the_specification_names();

    for one in &named {
        assert!(
            the_stages_the_code_writes().contains(one),
            "{one} is a stage the specification names and the code does not write: {named:?}"
        );
    }
}

#[test]
fn the_table_was_actually_read() {
    let named = the_stages_the_specification_names();

    assert!(
        !named.is_empty(),
        "the gate read no stage at all, which is a pass it has not earned"
    );
    assert!(
        named.contains(&"IDLE".to_string()) && named.contains(&"DONE".to_string()),
        "and it read the two that are unmistakably a stage's endpoints, so it is reading this table: {named:?}"
    );
}

#[test]
fn the_source_was_actually_read() {
    let written = the_stages_the_code_writes();

    assert!(
        !written.is_empty(),
        "the gate read no capital from the code, so it has compared the table with nothing"
    );
    let mut deduplicated = written.clone();
    deduplicated.sort();
    deduplicated.dedup();
    assert_eq!(
        deduplicated.len(),
        written.len(),
        "a name written twice is one stage and not two: {written:?}"
    );
}
