//! The gate for the rule everything else in this repository rests on: `aisdlc-domain`
//! depends on nothing at all.
//!
//! Not "few". Not "only serde". The manifest has no `[dependencies]` section, and that is the
//! one boundary the compiler cannot be tricked out of — a crate with no dependencies cannot
//! reach a model, a filesystem, a clock or a process, whatever its source says.
//!
//! **It is checked rather than trusted because a manifest is text and text is edited.** The
//! source is read through the shared walk rather than searched, because a domain may name
//! `std::fs` in its documentation — the walk that would refuse that is the walk this gate
//! exists to make impossible.

use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::scanning::{HowTheFileIsWritten, WhatIsWrittenThere};

/// The manifest of the domain, and the directory its source lives in.
const THE_DOMAIN: &str = "crates/domain";
const ITS_MANIFEST: &str = "crates/domain/Cargo.toml";
const ITS_SOURCE: &str = "crates/domain/src";

/// The sections a manifest may not carry an entry in.
///
/// **A section with nothing in it is accepted**, because the rule is about what the domain
/// depends on and not about how its manifest is arranged: an empty heading is a promise
/// nobody has been asked to keep.
///
/// **The names carry no brackets**, because the brackets are the manifest's delimiters and
/// not part of the name — a reader that compared `[dependencies]` against the section it
/// just read would never match anything, and a gate that never matches is green forever.
const THE_SECTIONS_THE_DOMAIN_MAY_NOT_FILL: [&str; 2] = ["dependencies", "dev-dependencies"];

/// The five modules of the standard library that can touch the world.
///
/// **Drawn as five names and not as a list of everything `std` offers**, because a list of
/// every reach is a list to keep in step with a standard library, and a rule drawn at the
/// reach is what a rule about a boundary looks like. `temp_dir` is here and it carries no
/// credential at all — a gate that is asked to refuse a path lookup has to refuse where it
/// is going to look as well, or a domain can be handed the answer on a plate.
const THE_MODULES_THAT_REACH_THE_WORLD: [&str; 5] = ["fs", "process", "time", "net", "thread"];

/// The path the domain's own manifest is at, with the `std::` in front of it.
///
/// A domain that owns a module called `fs` is not reaching for the world, and the glossary
/// fixes this as a rule about the path rather than about the last segment.
const THE_PATH_EACH_REACH_STANDS_AT: &str = "std::";

/// What a manifest and its sources break, named so a reader can act on the finding.
fn what_is_wrong_with(manifest: &str, the_source: &str) -> Vec<String> {
    let mut what_broke = Vec::new();
    what_broke.extend(the_sections_that_are_filled(manifest));
    what_broke.extend(the_reaches_in_the_code(the_source));
    what_broke
}

/// The sections a manifest carries an entry in, with the entry quoted back.
///
/// **A table header is an entry too.** `[dependencies.serde]` writes the dependency in the
/// header, and a reader that looks only for `name = value` would pass a crate that depends on
/// anything as one that depends on nothing. That was the second silent pass in an earlier
/// version of this gate, found only because the first one was looked for on purpose.
fn the_sections_that_are_filled(manifest: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut the_section_standing = String::new();
    for line in manifest.lines() {
        if let Some(name) = the_section_a_line_opens(line) {
            the_section_standing = name;
            continue;
        }
        found.extend(the_entry_standing_in(&the_section_standing, line));
    }
    found
}

/// The refusal for one line, if the line is an entry in a section the domain may not fill.
///
/// **Any** section header closes the one before it, whether or not the gate cares about it —
/// a reader that only noticed the two forbidden names would carry `[dependencies]`'s
/// emptiness into `[package]` and report the manifest's own name as a dependency.
fn the_entry_standing_in(the_section: &str, line: &str) -> Vec<String> {
    if !THE_SECTIONS_THE_DOMAIN_MAY_NOT_FILL.contains(&the_section) {
        return Vec::new();
    }
    if line.trim_start().starts_with('#') || line.trim().is_empty() {
        return Vec::new();
    }
    vec![format!("{the_section} carries an entry: {}", line.trim())]
}

/// The name of the section a manifest line opens, or nothing if it opens none.
///
/// **Every** section is named, and not only the two the gate refuses: a section this reader
/// did not recognise is a section it cannot close, and the next entry would be read as
/// belonging to the last section it did recognise.
///
/// The table name is taken off, so `[dependencies]` and `[dependencies.serde]` are
/// recognised as the same section and a reader does not have to know the two spellings.
fn the_section_a_line_opens(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return None;
    }
    let the_name = trimmed.trim_start_matches('[').trim_end_matches(']');
    let without_the_table = the_name.split('.').next().unwrap_or(the_name);
    Some(without_the_table.to_string())
}

/// The reaches the domain's code makes, read out of the code and not out of its prose.
///
/// A fully qualified `std::fs::read_to_string(path)` with no import at all is a reach, and a
/// checker that looks for a `use` line misses it — that was the first escape from an earlier
/// version of this gate, and the two are the same hole seen from two sides.
fn the_reaches_in_the_code(the_source: &str) -> Vec<String> {
    let mut found = Vec::new();
    for stretch in HowTheFileIsWritten::of(the_source).stretches() {
        if stretch.kind() != WhatIsWrittenThere::Code {
            continue;
        }
        found.extend(the_reaches_in(stretch.text(), stretch.line_it_starts_on()));
    }
    found
}

/// The reach one stretch of code makes, if any.
fn the_reaches_in(the_line_of_code: &str, the_line_number: usize) -> Vec<String> {
    let mut found = Vec::new();
    for module in THE_MODULES_THAT_REACH_THE_WORLD {
        let the_path = format!("{THE_PATH_EACH_REACH_STANDS_AT}{module}");
        if the_line_of_code.contains(&the_path) {
            found.push(format!("line {the_line_number} reaches for {the_path}"));
        }
        if the_line_of_code.contains(&format!("{THE_PATH_EACH_REACH_STANDS_AT}{{")) {
            found.extend(the_module_in_a_group(
                module,
                the_line_of_code,
                the_line_number,
            ));
        }
    }
    found
}

/// A module named among others inside a `use std::{…}` group.
///
/// The group's own spellings vary — `use  std::{`, `pub use std::{` — and a checker that
/// knows one spelling is the same defect again, so the group is found by the two words
/// before the brace.
fn the_module_in_a_group(
    module: &str,
    the_line_of_code: &str,
    the_line_number: usize,
) -> Vec<String> {
    if !the_line_of_code.contains(module) {
        return Vec::new();
    }
    if !the_line_of_code.contains("use std::{") && !the_line_of_code.contains("use  std::{") {
        return Vec::new();
    }
    vec![format!(
        "line {the_line_number} names {module} in a group of modules"
    )]
}

#[test]
fn a_manifest_with_a_dependency_is_refused() {
    let manifest = "[package]\nname = \"x\"\n\n[dependencies]\nserde = \"1\"\n";

    let what_broke = what_is_wrong_with(manifest, "");

    assert_eq!(
        what_broke.len(),
        1,
        "a dependency is a dependency: {what_broke:?}"
    );
    assert!(
        what_broke[0].contains("serde"),
        "and the refusal quotes the entry, because a refusal a reader cannot match against what they wrote teaches nothing: {what_broke:?}"
    );
}

#[test]
fn a_manifest_with_a_dev_dependency_is_refused() {
    let manifest = "[dev-dependencies]\nregex = \"1\"\n";

    let what_broke = what_is_wrong_with(manifest, "");

    assert_eq!(
        what_broke.len(),
        1,
        "a dev-dependency is how a test gets a crate the domain cannot have, so the rule does not have two halves: {what_broke:?}"
    );
}

#[test]
fn a_manifest_with_a_table_header_dependency_is_refused() {
    let manifest = "[dependencies.serde]\nversion = \"1\"\n";

    let what_broke = what_is_wrong_with(manifest, "");

    assert_eq!(
        what_broke.len(),
        1,
        "a table header writes the dependency in the header, and a reader that looks only for `name = value` passes a crate that depends on anything as one that depends on nothing: {what_broke:?}"
    );
}

#[test]
fn a_manifest_with_an_empty_section_is_accepted() {
    let manifest = "[dependencies]\n\n[dev-dependencies]\n\n[package]\nname = \"x\"\n";

    assert!(
        what_is_wrong_with(manifest, "").is_empty(),
        "the rule is about what the domain depends on and not about how its manifest is arranged"
    );
}

#[test]
fn the_real_manifest_of_the_domain_is_accepted() {
    let manifest = the_text_of_a_manifest();

    assert!(
        what_is_wrong_with(&manifest, "").is_empty(),
        "the domain's own manifest carries a section it may not, and the rule is not a sentence"
    );
    assert!(
        the_sections_that_are_filled(&manifest).is_empty(),
        "and the reader agrees, on the real manifest, having skipped the comment that names the section in order to say it is absent"
    );
}

#[test]
fn a_qualified_reach_with_no_import_is_refused() {
    let source = "pub fn the_text_of(path: &str) -> String {\n    std::fs::read_to_string(path).unwrap_or_default()\n}";

    let what_broke = what_is_wrong_with("", source);

    assert_eq!(
        what_broke.len(),
        1,
        "a fully qualified reach with no import is the reach the rule is about: {what_broke:?}"
    );
    assert!(
        what_broke[0].contains("line 2"),
        "and the refusal names the line: {what_broke:?}"
    );
}

#[test]
fn a_grouped_reach_is_refused() {
    let source = "use std::{fs, io};\n\npub fn a() {}";

    let what_broke = what_is_wrong_with("", source);

    assert_eq!(
        what_broke.len(),
        1,
        "a group names the module just as plainly as a path does, and the two are the same hole: {what_broke:?}"
    );
}

#[test]
fn a_reach_named_in_documentation_is_accepted() {
    let source =
        "//! The domain does not use std::fs, and this is where it says so.\n\npub fn a() {}";

    assert!(
        what_is_wrong_with("", source).is_empty(),
        "a domain may name what it does not reach for in its own documentation, and a gate that refused that would be refusing this repository's rule to be explained"
    );
}

#[test]
fn a_module_of_the_domains_own_is_accepted() {
    let source = "use crate::fs::read_the_shape_of_it;\n\npub fn a() {}";

    assert!(
        what_is_wrong_with("", source).is_empty(),
        "a domain that owns a module called `fs` is not reaching for the world, and the rule is about the path rather than about the last segment"
    );
}

#[test]
fn the_reaches_of_the_real_domain_are_read() {
    let the_source = the_source_of_the_domain();

    let what_broke = what_is_wrong_with(&the_text_of_a_manifest(), &the_source);

    assert!(
        what_broke.is_empty(),
        "the domain reaches for something, and the rule every boundary here rests on is not a sentence: {what_broke:?}"
    );
    assert!(
        !the_source.is_empty(),
        "the gate read no source at all, which is a pass it has not earned"
    );
    let read = HowTheFileIsWritten::of(&the_source);
    assert!(
        !read.stretches().is_empty(),
        "and the walk made stretches of it rather than answering about an empty string"
    );
}

/// The text of the domain's manifest, or a panic naming it.
fn the_text_of_a_manifest() -> String {
    let the_path = the_path_under_the_checkout(ITS_MANIFEST);
    std::fs::read_to_string(&the_path)
        .unwrap_or_else(|why| panic!("{} must be readable: {why}", the_path.display()))
}

/// Every line of the domain's source, joined, because the checker is asked about a file and
/// a file is read as one reading.
fn the_source_of_the_domain() -> String {
    let mut the_source = String::new();
    for path in every_rust_file_under_the_domain() {
        the_source.push_str(&reading_the_checkout::the_text_at(&path));
        the_source.push('\n');
    }
    the_source
}

/// Every `.rs` file of the domain, in the order a reader would meet them.
fn every_rust_file_under_the_domain() -> Vec<std::path::PathBuf> {
    let the_root = the_path_under_the_checkout(ITS_SOURCE);
    let mut found = Vec::new();
    collect_rust_files_under(&the_root, &mut found);
    found.sort();
    found
}

fn collect_rust_files_under(directory: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_under(&path, found);
        } else if path.extension().is_some_and(|one| one == "rs") {
            found.push(path);
        }
    }
}

/// A path inside this checkout, from where cargo compiled this file.
fn the_path_under_the_checkout(wanted: &str) -> std::path::PathBuf {
    let mut here = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !here.join(THE_DOMAIN).is_dir() {
        assert!(
            here.pop(),
            "no {THE_DOMAIN} directory above where this file was compiled"
        );
    }
    here.join(wanted)
}
