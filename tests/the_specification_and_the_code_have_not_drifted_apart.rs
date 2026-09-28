//! The glossary's tables and the enums have not drifted apart.
//!
//! **The glossary promises this in prose** — a stage table is "a table rather than prose,
//! because a test reads it to check the code against it" — and it had said so since before
//! the wipe with nothing reading it. Seven rows in the prose and six in the code is not a
//! compile error anywhere, and both documents would be right about their own contents.
//!
//! **Each set is read out of the section that declares it**, not out of the whole document,
//! because the glossary also tables the organisations and the artifact kinds, and a reader
//! that took every capital in the file would collect all of them and compare the lot.
//!
//! **One reader, two tables.** The first version of this gate read the stages and had no
//! version that read anything else — so when `Role` arrived it reported a clean result about
//! a table it had never opened, which is the exact failure a gate exists to prevent. A
//! second file with the same reader in it would be that same defect once more.

use aisdlc_gates::scanning::{self, HowTheFileIsWritten, WhatIsWrittenThere};

const THE_GLOSSARY: &str = "docs/domain-glossary.md";
const THE_HEADING_THE_STAGES_ARE_UNDER: &str = "## The stages";
const THE_HEADING_THE_ROLES_ARE_UNDER: &str = "## The roles";
const THE_DOMAIN_STAGE: &str = "crates/domain/src/stage.rs";
const THE_DOMAIN_ROLE: &str = "crates/domain/src/role.rs";

/// How many rows each of the specification's tables declares.
const HOW_MANY_STAGES: usize = 6;
const HOW_MANY_ROLES: usize = 5;

/// A table the specification declares, and the domain file that answers for it.
///
/// **Named as a pair and not as two arguments**, because the whole point of this file is
/// that the two sides have to be read together: a reader that took the table here and the
/// file there would be free to be given one of each and would then compare a stage table
/// with a role enum, and report a drift that is a mistake in the gate.
struct ATableAndTheFileAnsweringForIt {
    heading: &'static str,
    the_domain_file: &'static str,
    how_many: usize,
    what_it_declares: &'static str,
    the_first_and_the_last: [&'static str; 2],
}

const THE_STAGES: ATableAndTheFileAnsweringForIt = ATableAndTheFileAnsweringForIt {
    heading: THE_HEADING_THE_STAGES_ARE_UNDER,
    the_domain_file: THE_DOMAIN_STAGE,
    how_many: HOW_MANY_STAGES,
    what_it_declares: "stages",
    the_first_and_the_last: ["IDLE", "DONE"],
};

const THE_ROLES: ATableAndTheFileAnsweringForIt = ATableAndTheFileAnsweringForIt {
    heading: THE_HEADING_THE_ROLES_ARE_UNDER,
    the_domain_file: THE_DOMAIN_ROLE,
    how_many: HOW_MANY_ROLES,
    what_it_declares: "roles",
    the_first_and_the_last: ["PO", "ARCH"],
};

const EVERY_TABLE: [ATableAndTheFileAnsweringForIt; 2] = [THE_STAGES, THE_ROLES];

/// The capitals the specification's table names, read out of that table.
fn the_names_under(table: &ATableAndTheFileAnsweringForIt) -> Vec<String> {
    let the_section = the_section_under(&the_text_of(THE_GLOSSARY), table.heading);
    let mut named = Vec::new();
    for line in the_section.lines() {
        if let Some(one) = the_capital_in_the_first_cell_of(line) {
            named.push(one);
        }
    }
    named
}

/// The capitals the domain's file writes a name as.
///
/// **Read out of the string literals and not out of the code around them.** The walk tells a
/// written name from the code that produces it: `Stage::Idle => ` is one stretch, `IDLE` is
/// the next. A name *is* the second, and the first version of this gate read the first and
/// found nothing — it had reached past the walk's boundary to re-parse a literal the walk
/// had already handed over separately.
///
/// The other half comes for free: a capital mentioned in the file's own documentation is a
/// comment, and the walk will not hand it over as a name. The rule that a name has to be
/// written to count is enforced by *where* the text sits rather than by a check that might
/// one day fail to notice.
fn the_capitals_written_in(table: &ATableAndTheFileAnsweringForIt) -> Vec<String> {
    let the_code = the_text_of(table.the_domain_file);
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

/// The capital one row of the glossary's table puts in its first cell, if it is one.
fn the_capital_in_the_first_cell_of(line: &str) -> Option<String> {
    let after_the_bar = line.trim().strip_prefix("| `")?;
    let (the_name, _) = after_the_bar.split_once('`')?;
    if the_name.is_empty() {
        return None;
    }
    if !the_name
        .chars()
        .all(|one| one.is_ascii_uppercase() || one == '_')
    {
        return None;
    }
    Some(the_name.to_string())
}

/// The capital one written name holds, if it holds one at all.
///
/// The quotes are the literal's delimiters and not part of the name, and a literal that is
/// empty or holds anything else is prose: the refusal's own message, a format string, a name
/// to be reported back to whoever wrote it.
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
fn the_specification_names_exactly_what_the_code_writes() {
    for table in &EVERY_TABLE {
        let named = the_names_under(table);
        let written = the_capitals_written_in(table);

        assert_eq!(
            named, written,
            "the glossary's {} table and the code have drifted apart, and the table is the one that is right",
            table.what_it_declares
        );
    }
}

#[test]
fn there_are_as_many_on_both_sides_as_the_specification_declares() {
    for table in &EVERY_TABLE {
        let named = the_names_under(table);
        let written = the_capitals_written_in(table);

        assert_eq!(
            named.len(),
            table.how_many,
            "the {} table is the specification and it has this many rows: {named:?}",
            table.what_it_declares
        );
        assert_eq!(
            written.len(),
            table.how_many,
            "and the code writes as many {}: {written:?}",
            table.what_it_declares
        );
    }
}

#[test]
fn a_name_the_specification_gives_is_one_the_code_writes() {
    for table in &EVERY_TABLE {
        let named = the_names_under(table);
        let written = the_capitals_written_in(table);

        for one in &named {
            assert!(
                written.contains(one),
                "{one} is one of the {} the specification names and the code does not write it: {named:?}",
                table.what_it_declares
            );
        }
    }
}

#[test]
fn both_sides_were_actually_read() {
    for table in &EVERY_TABLE {
        let named = the_names_under(table);
        let written = the_capitals_written_in(table);

        assert!(
            !named.is_empty(),
            "the gate read no {} at all: {named:?}",
            table.what_it_declares
        );
        assert!(
            !written.is_empty(),
            "and read no capital out of {}: {written:?}",
            table.the_domain_file
        );
        for endpoint in table.the_first_and_the_last {
            assert!(
                named.contains(&endpoint.to_string()) && written.contains(&endpoint.to_string()),
                "and {endpoint} is the unmistakable end of this table, so it is reading this one: {named:?} / {written:?}"
            );
        }
    }
}

#[test]
fn no_name_is_written_twice_on_either_side() {
    for table in &EVERY_TABLE {
        assert_no_repeats(&the_names_under(table), table.what_it_declares);
        assert_no_repeats(&the_capitals_written_in(table), table.what_it_declares);
    }
}

/// The refusal when a name is listed twice on one side, and nothing when it is not.
fn assert_no_repeats(the_names: &[String], what_it_declares: &str) {
    let mut sorted = the_names.to_vec();
    sorted.sort();
    let before = sorted.len();
    sorted.dedup();

    assert_eq!(
        sorted.len(),
        before,
        "a {what_it_declares} is written twice on one side of this comparison: {sorted:?}"
    );
}
