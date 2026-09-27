//! The domain reaches for nothing that can touch the world.
//!
//! `aisdlc-domain` has no dependencies at all — not "few", not "only serde". Nothing. Its
//! manifest must not carry a `[dependencies]` or a `[dev-dependencies]` section with an
//! entry in it, and its source must not name an `extern crate`, an `async` block, an
//! `await` or an `unsafe`, and must not reach for a filesystem, a process, a clock, a
//! socket or a thread.
//!
//! ## Two spellings of one reach
//!
//! **The rule is a path segment, not an import line**, and there are two of them because
//! two spellings reach the same place:
//!
//! 1. a stretch of code may not carry `std::fs`, `std::process`, `std::time`, `std::net`
//!    or `std::thread` **as a path prefix, wherever it appears** — `use std::fs;`,
//!    `use std::fs as filesystem;` and `std::fs::read_to_string(path)` are one answer to
//!    one question, and the last of them is the one that has no `use` in it at all;
//! 2. a `use std::{…}` group may not name `fs`, `process`, `time`, `net` or `thread` in its
//!    braces, because `use std::{fs, io};` does not contain the text `std::fs` anywhere.
//!
//! Both escapes were found by a reader of this file, reported rather than answered, and
//! refused in the middle of the cycle that found them — widening a check mid-cycle is how a
//! check becomes a snapshot of the day it was written. They are answered here together, and
//! the two rules are one list of five modules read two ways rather than two lists somebody
//! has to keep in step.
//!
//! ## The file is taken apart, and not read a line at a time
//!
//! A source is read as stretches, by the scanner the gates share, and only the stretches
//! that are code are asked anything. **A comment is not a line that begins with two
//! slashes**: `/** … */` is a comment and a `//` inside a string is not one. The old
//! hand-rolled strip dropped the `//` lines and called the rest code, which is why a doc
//! comment that *mentions* a forbidden path is now a fixture rather than a hazard: a rule
//! that refuses the opposite of what a sentence says teaches its reader to ignore it.
//!
//! This is the one rule in the repository nothing else can break quietly, so the checker
//! is exercised on every input it must refuse and on every input it must leave alone.
//! The last test reads the real files rather than a fixture: a fixture proves the checker
//! works, and only the real files prove the domain obeys it.
#![deny(missing_docs)]

use aisdlc_gates::scanning::{HowTheFileIsWritten, WhatIsWrittenThere};
use std::path::{Path, PathBuf};

/// The manifest of the domain crate, reached from the root of the repository rather than
/// from wherever the process happens to be standing, because a test that reads a file by
/// a path relative to itself proves nothing on a machine holding two checkouts of it.
const THE_MANIFEST_OF_THE_DOMAIN: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/crates/domain/Cargo.toml");

/// The directory holding the domain's source, for the same reason as the manifest above.
const THE_SOURCE_DIRECTORY_OF_THE_DOMAIN: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/crates/domain/src");

/// The two sections a manifest may not carry an entry in. **A section with nothing in it
/// is accepted**, because the rule is about what the domain depends on and not about how
/// its manifest is arranged: an empty heading is a promise nobody has been asked to keep.
const THE_SECTIONS_A_MANIFEST_MAY_NOT_HAVE: [&str; 2] = ["[dependencies]", "[dev-dependencies]"];

/// The words a source may not say outside its own documentation, each one a promise the
/// domain does not make: it names no crate from outside itself, it is not concurrent, and
/// it is not memory-unsafe.
const THE_WORDS_THE_DOMAIN_MAY_NOT_SAY: [&str; 4] = ["extern crate", "async", "await", "unsafe"];

/// The modules that can touch the world, named as the last segment of a path rather than as
/// the whole of it, so that the path rule and the grouped rule are one list read two ways.
///
/// `std::mem` is deliberately absent: a process's own memory is not the world, and a domain
/// that can move a value is a domain whose moves a reader has to reason about.
const THE_MODULES_THAT_TOUCH_THE_WORLD: [&str; 5] = ["fs", "process", "time", "net", "thread"];

/// The crate the world is in, and the separator a path in it is joined with, which is the
/// front of every path a source is refused for and the last word of `use std::{`.
const THE_CRATE_THE_WORLD_IS_IN_AND_ITS_SEPARATOR: &str = "std::";

const A_MANIFEST_CARRYING_SERDE: &str =
    "[package]\nname = \"aisdlc-domain\"\n\n[dependencies]\nserde = \"1\"\n";

const A_MANIFEST_CARRYING_REGEX_AS_A_DEV_DEPENDENCY: &str =
    "[package]\nname = \"aisdlc-domain\"\n\n[dev-dependencies]\nregex = \"1\"\n";

const A_MANIFEST_CARRYING_A_TABLE_OF_DEPENDENCIES: &str =
    "[package]\nname = \"aisdlc-domain\"\n\n[dependencies.serde]\nversion = \"1\"\n";

const A_MANIFEST_CARRYING_NO_ENTRY_AT_ALL: &str =
    "[package]\nname = \"aisdlc-domain\"\n\n[dependencies]\n";

const A_SOURCE_IMPORTING_THE_FILESYSTEM: &str = "use std::fs;\n";

const A_SOURCE_IMPORTING_THE_FILESYSTEM_UNDER_ANOTHER_NAME: &str = "use std::fs as files;\n";

const A_SOURCE_IMPORTING_A_PROCESS: &str = "use std::process::Command;\n";

const A_SOURCE_IMPORTING_THE_CLOCK: &str = "use std::time::Instant;\n";

const A_SOURCE_IMPORTING_A_SOCKET: &str = "use std::net::TcpStream;\n";

const A_SOURCE_IMPORTING_A_THREAD: &str = "use std::thread;\n";

const A_SOURCE_DECLARING_AN_ASYNC_FUNCTION: &str = "pub async fn asked() {}\n";

const A_SOURCE_DECLARING_AN_UNSAFE_FUNCTION: &str = "pub unsafe fn transfer() {}\n";

const A_SOURCE_NAMING_EXTERN_CRATE: &str = "extern crate serde;\n";

const A_SOURCE_THAT_AWAITS: &str = "fn asked() {\n    let answer = look().await;\n}\n";

/// A source that reads a file with no `use` in it anywhere, so there is no `use` line for a
/// checker that reads lines to find. The gate reported a clean file on this one.
const A_SOURCE_REACHING_THE_FILESYSTEM_WITHOUT_IMPORTING_IT: &str =
    "let text = std::fs::read_to_string(path)?;\n";

/// A source that imports the filesystem among others in one group, so the text `std::fs` is
/// nowhere in the line. The gate reported a clean file on this one too.
const A_SOURCE_IMPORTING_THE_FILESYSTEM_IN_A_GROUP: &str = "use std::{fs, io};\n";

/// A source whose group opens with a module that touches nothing, which is what makes the
/// grouped import a rule rather than one lucky spelling of it.
const A_SOURCE_NAMING_A_SOCKET_IN_A_GROUP: &str = "use std::{io::Write, net::TcpStream};\n";

/// A group of modules that touch nothing, so the grouped rule is asked about a group it must
/// leave alone as well as one it must refuse.
const A_GROUP_OF_MODULES_THAT_TOUCH_NOTHING: &str = "use std::{fmt, mem};\n";

/// A source whose only reach is on the fifth line, under two lines of documentation, so a
/// refusal has to carry the line the file has rather than the line it had once the
/// documentation was dropped.
const A_SOURCE_REACHING_THE_FILESYSTEM_BELOW_ITS_DOCUMENTATION: &str = concat!(
    "//! This crate has no dependencies at all.\n",
    "\n",
    "/// The text of a file, which this crate does not read.\n",
    "pub fn the_text_of(path: &str) -> String {\n",
    "    std::fs::read_to_string(path).unwrap_or_default()\n",
    "}\n"
);

/// A module whose documentation names the filesystem in order to say it never reaches for
/// it, which is what documentation is for.
const A_MODULE_NAMING_THE_FILESYSTEM: &str =
    "//! This crate never names std::fs, or any other way of reading a file.\n";

/// The same sentence as a block of documentation, which is the form a line that begins with
/// two slashes cannot drop and a kind of stretch drops without being asked to.
const A_BLOCK_NAMING_THE_FILESYSTEM: &str =
    "/** This crate never names std::fs, or any other way of reading a file. */\n";

/// A source that is nothing but an ordinary function, which is what a domain looks like on
/// the day it is written and what no rule of this gate has anything to say about.
const A_SOURCE_CARRYING_A_FUNCTION_AND_NO_COMMENT: &str =
    "pub fn a_work_item_is_new() -> bool {\n    true\n}\n";

const A_SOURCE_IMPORTING_MEMORY: &str = "use std::mem;\n";

const A_SOURCE_IMPORTING_FORMATTING: &str = "use std::fmt;\n";

const A_MODULE_SAYING_IT_NEEDS_NOT_ONLY_SERDE: &str =
    "//! This crate has no dependencies at all, not \"only serde\". Nothing.\n";

const A_MODULE_SAYING_NO_FUNCTION_IS_ASYNC: &str =
    "//! No function here is async, and none of them may become async.\n";

/// Every way the domain could reach for something, each reported as text a reader can act
/// on rather than as a yes or a no, because a manifest and a source file are files a
/// person has to open and a line in them to find.
///
/// A fixture of this gate proves the checker works. Only the real files prove the domain
/// obeys it, and both are asserted: the refusal fixtures and the real assertion below.
fn what_is_wrong_with(manifest: &str, source: &str) -> Vec<String> {
    let mut refusals = what_the_manifest_carries(manifest);
    refusals.extend(what_the_source_reaches_for(source));
    refusals
}

/// The entries under every dependency section a manifest carries, each naming the line it
/// was found on. A manifest is read a line at a time because a section ends where the next
/// heading begins, and a search across the whole text would find a `[dependencies]` that
/// has been closed off and call it a dependency.
fn what_the_manifest_carries(manifest: &str) -> Vec<String> {
    let lines: Vec<&str> = manifest.lines().collect();
    let mut refusals = Vec::new();
    for (line_number, line) in lines.iter().enumerate() {
        if let Some(header) = the_dependency_section_a_line_opens(line) {
            refusals.extend(the_entries_under(&lines, header, line_number));
        }
    }
    refusals
}

/// The dependency section a line opens, whether the line writes the heading itself or a
/// table named for it, and `None` for every other line. A table has to count as an opening
/// because `[dependencies.serde]` declares a dependency as plainly as `serde = "1"` does,
/// and a walk that recognised only the first spelling would call that manifest empty.
fn the_dependency_section_a_line_opens(line: &str) -> Option<&'static str> {
    THE_SECTIONS_A_MANIFEST_MAY_NOT_HAVE
        .iter()
        .copied()
        .find(|header| opens_a_section_named(line, header))
}

/// Whether a line is the heading of one named section or a table belonging to it.
fn opens_a_section_named(line: &str, header: &str) -> bool {
    let text = line.trim();
    text == header || text.starts_with(&the_tables_under(header))
}

/// The prefix every table of a section shares, which is the heading without its closing
/// bracket. Both the walk and the check that a section has opened read it from here,
/// because the two spellings differ by that bracket alone and a reader who has to spot
/// that in two places is reading the same subtraction twice.
fn the_tables_under(header: &str) -> String {
    format!("{}.", header.trim_end_matches(']'))
}

/// The entries between a section's heading and the next heading. A heading *of* that
/// section is one of them rather than the end of it, because `[dependencies.serde]` is a
/// way of declaring a dependency and a walk that stopped at it would call that manifest
/// empty.
fn the_entries_under(lines: &[&str], header: &str, section_line: usize) -> Vec<String> {
    let mut refusals = Vec::new();
    for (line_number, line) in lines.iter().enumerate().skip(section_line + 1) {
        if is_a_heading_of_another_section(line, header) {
            break;
        }
        if is_an_entry_of_a_section(line) {
            refusals.push(what_the_entry_is(header, line, line_number));
        }
    }
    refusals
}

/// One refusal naming the section, what it carries and the line it was carried on. The
/// line is counted from one, because that is how a person reading a file counts them.
fn what_the_entry_is(header: &str, entry: &str, line_number: usize) -> String {
    format!(
        "{header} carries {} on line {}",
        entry.trim(),
        line_number + 1
    )
}

/// Whether a line is a heading that closes the section being walked. Anything that does
/// not open with a bracket is not a heading, and a `#` line is the case that matters: a
/// commented-out heading declares nothing, and reading one as a declaration would report a
/// manifest that depends on nothing.
fn is_a_heading_of_another_section(line: &str, header: &str) -> bool {
    let text = line.trim();
    if !text.starts_with('[') {
        return false;
    }
    let is_a_table_of_this_section = text.starts_with(&the_tables_under(header));
    !is_a_table_of_this_section
}

/// Whether a line says something rather than saying nothing, which is the question a
/// manifest walk asks once it has decided a heading has not closed the section. **A blank
/// line and a `#` line say nothing, and a heading would** — the walk breaks on a heading
/// before this is asked, so the two questions are kept apart rather than answered twice.
fn is_an_entry_of_a_section(line: &str) -> bool {
    let text = line.trim();
    !text.is_empty() && !text.starts_with('#')
}

/// Every reach in a source, one refusal per stretch of code that makes one.
///
/// The file is taken apart by the scanner the gates share rather than by dropping the lines
/// that begin with two slashes, because a comment is not a line that begins with two slashes
/// and a `//` inside a string is not a comment. A stretch that is a comment, a string, a
/// character or the space between two things says nothing to the compiler and so reaches
/// for nothing.
///
/// **The one hole the scanner leaves open is an unclosed string**, which makes the walk read
/// the rest of the file as one string and hides every comment after it. For this gate that
/// is the safe direction: an unclosed string is a compile error, and a file cargo compiled
/// cannot hold one. It is written down here rather than worked around, because working
/// around it would be a fourth way of deciding what a comment is.
fn what_the_source_reaches_for(source: &str) -> Vec<String> {
    let read = HowTheFileIsWritten::of(source);
    let mut refusals = Vec::new();
    for stretch in read.stretches() {
        if stretch.kind() == WhatIsWrittenThere::Code {
            refusals.extend(what_this_stretch_reaches_for(
                stretch.line_it_starts_on(),
                stretch.text(),
            ));
        }
    }
    refusals
}

/// What one stretch of code reaches for, one refusal per rule it breaks, so a line that
/// breaks two of them says so twice rather than hiding the second.
fn what_this_stretch_reaches_for(line_number: usize, code: &str) -> Vec<String> {
    let mut refusals = the_words_it_says(line_number, code);
    refusals.extend(the_paths_it_reaches_for(line_number, code));
    refusals.extend(the_modules_a_group_names(line_number, code));
    refusals
}

/// The promises the domain does not make, said out loud on a line: it names no crate from
/// outside itself, it is not concurrent, and it is not memory-unsafe.
fn the_words_it_says(line_number: usize, code: &str) -> Vec<String> {
    let mut refusals = Vec::new();
    for word in THE_WORDS_THE_DOMAIN_MAY_NOT_SAY {
        if is_a_whole_word_in(code, word) {
            refusals.push(format!("line {line_number} says {word}"));
        }
    }
    refusals
}

/// The modules of the world this stretch of code reaches for, each found as a **path prefix
/// wherever the prefix appears** rather than as an import on a `use` line.
///
/// **The segment is the rule and not the line**, and it is not a refinement of the line: a
/// qualified reach — `std::fs::read_to_string(path)` — carries no `use` at all, and a
/// grouped one — `use std::{fs, io};` — does not contain the text `std::fs`, so a checker
/// reading the line rather than the segment reported a clean file on both of them.
///
/// **This is the second time this checker has been asked to widen for the same reason.** The
/// first report was the qualified reach and the second was the group, and both were refused
/// rather than answered, because a check widened in the middle of a cycle is a check that
/// had been a snapshot. They are answered here together, by asking one question of one
/// list: is this path written here, and is this name one of the five in a group?
fn the_paths_it_reaches_for(line_number: usize, code: &str) -> Vec<String> {
    let mut refusals = Vec::new();
    for module in THE_MODULES_THAT_TOUCH_THE_WORLD {
        let path = the_path_of(module);
        if is_a_whole_word_in(code, &path) {
            refusals.push(format!("line {line_number} reaches for {path}"));
        }
    }
    refusals
}

/// The modules of the world this stretch of code names among others in the braces of a group,
/// which is the second spelling the first rule cannot see: `use std::{fs, io};` puts `fs`
/// beside `io` and never beside the crate it comes from.
fn the_modules_a_group_names(line_number: usize, code: &str) -> Vec<String> {
    let mut refusals = Vec::new();
    for at in where_each_group_opens(code) {
        refusals.extend(what_the_braces_refuse(line_number, &code[at + 1..]));
    }
    refusals
}

/// The path a module of the world is spelled with, built from the one crate the world is in
/// so that the path rule and the grouped rule cannot disagree about which crate is meant.
fn the_path_of(module: &str) -> String {
    format!("{THE_CRATE_THE_WORLD_IS_IN_AND_ITS_SEPARATOR}{module}")
}

/// Whether a word stands in a stretch whole rather than inside a longer name, so that a name
/// carrying one of them as part of itself is not counted as a use of it.
///
/// A path is a whole thing too, which is what the first rule leans on: `std::fs` in
/// `mystd::fs` is not the filesystem, and `std::fs_extra` is not it either. The words and the
/// paths are asked the same question, so there is one answer to it.
fn is_a_whole_word_in(text: &str, word: &str) -> bool {
    for (at, _) in text.match_indices(word) {
        if stands_on_its_own(text, at, word) {
            return true;
        }
    }
    false
}

/// Whether the word beginning at `at` is bounded by something that is not part of a name.
/// A dot does not bound it, because `.await` is the same reach written differently.
fn stands_on_its_own(text: &str, at: usize, word: &str) -> bool {
    let follows_a_name = text[..at]
        .chars()
        .next_back()
        .is_some_and(is_part_of_a_name);
    let end = at + word.len();
    let followed_by_a_name = text[end..].chars().next().is_some_and(is_part_of_a_name);
    !follows_a_name && !followed_by_a_name
}

/// What counts as part of a name. Underscore included, because a name is its letters and
/// its underscores, and a search that stopped at a letter would be evaded by one.
fn is_part_of_a_name(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// Where each group of the whole of the world opens, as the offset of its brace, because a
/// group is written in more than one way and `use std::{`, `use  std::{` and `pub use
/// std::{` are all one group.
fn where_each_group_opens(code: &str) -> Vec<usize> {
    code.match_indices('{')
        .map(|(at, _brace)| at)
        .filter(|at| opens_a_group_of_the_whole_of_the_world(&code[..*at]))
        .collect()
}

/// Whether the code before a brace opens a group of the whole of the world, which is spelled
/// with an import of the crate the world is in, then its separator, then the brace. The two
/// words are read backwards because that is the order they are read in by a reader, who
/// looks at the brace first and works out what opened it.
fn opens_a_group_of_the_whole_of_the_world(before_the_brace: &str) -> bool {
    let mut backwards = before_the_brace.split_whitespace().rev();
    let names_the_whole = backwards
        .next()
        .is_some_and(|word| word == THE_CRATE_THE_WORLD_IS_IN_AND_ITS_SEPARATOR);
    names_the_whole && backwards.next() == Some("use")
}

/// What the braces of one group refuse, one refusal per module of the world it names, and
/// nothing at all for a group naming only modules that touch none.
fn what_the_braces_refuse(line_number: usize, the_braces: &str) -> Vec<String> {
    let mut refusals = Vec::new();
    for name in the_names_in_the_braces(the_braces) {
        if is_a_module_that_touches_the_world(&name) {
            let path = the_path_of(&name);
            refusals.push(format!("line {line_number} imports {path} in a group"));
        }
    }
    refusals
}

/// The names a group's braces hold, one per entry, each the leading run of a name in that
/// entry: `fs` in `fs` and `fs` in `fs as filesystem` and `net` in `net::TcpStream`.
///
/// A nested group is not walked, and a name inside one is read as an entry of its own. That
/// over-refuses rather than under-refuses, and the only thing it over-refuses is a nested
/// group that is not valid Rust, which a crate cargo compiled cannot hold.
fn the_names_in_the_braces(after_the_brace: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut entry = String::new();
    for character in after_the_brace.chars() {
        if character == '}' {
            break;
        }
        if character == ',' {
            names.push(the_leading_name(&entry));
            entry.clear();
        } else {
            entry.push(character);
        }
    }
    names.push(the_leading_name(&entry));
    names
}

/// The name an entry of a group begins with, which is the module it brings in: the run of
/// name-parts at the front of the entry, and nothing after the first of them.
fn the_leading_name(entry: &str) -> String {
    entry
        .trim_start()
        .chars()
        .take_while(|character| is_part_of_a_name(*character))
        .collect()
}

/// Whether a name in a group's braces is one of the modules that touch the world, read from
/// the same five names the path rule is read from so the two rules cannot drift apart.
fn is_a_module_that_touches_the_world(name: &str) -> bool {
    THE_MODULES_THAT_TOUCH_THE_WORLD.contains(&name)
}

/// The text of a file, refusing to go on with a file it could not read, because a gate
/// that checks nothing is a gate that passed.
///
/// **This file reaches for the filesystem where the domain may not**, and says so plainly
/// rather than hiding behind an import: the gate is the thing that reads the disk, and a
/// reader who found `std::fs` here and no explanation would be right to wonder which of the
/// two rules it had broken.
fn the_text_of(path: &Path) -> String {
    let name = path.display();
    std::fs::read_to_string(path)
        .unwrap_or_else(|wrong| panic!("{name} could not be read: {wrong}"))
}

/// Every `.rs` file under the domain's source directory, which is what "its source" means
/// and which a test asserting on one file would quietly narrow.
fn the_rust_sources_of_the_domain() -> Vec<PathBuf> {
    let mut found = Vec::new();
    let directory = std::fs::read_dir(THE_SOURCE_DIRECTORY_OF_THE_DOMAIN);
    let listed = directory.expect("the domain's source directory could not be listed");
    for entry in listed {
        let path = entry
            .expect("a name in the domain's source could not be read")
            .path();
        let is_rust = path
            .extension()
            .is_some_and(|name| name.eq_ignore_ascii_case("rs"));
        if is_rust {
            found.push(path);
        }
    }
    found
}

/// Each source file of the domain on its own, so a refusal names the file it was found
/// in rather than naming a concatenation nobody can open.
fn assert_every_source_of_the_domain_reaches_for_nothing() {
    let sources = the_rust_sources_of_the_domain();
    assert!(
        !sources.is_empty(),
        "no source of the domain was read, so a green run would have proved nothing"
    );
    for source in sources {
        let refusals = what_is_wrong_with("", &the_text_of(&source));
        let named = format!("{:?} reaches for something: {refusals:?}", source.display());
        assert!(refusals.is_empty(), "{named}");
    }
}

#[test]
fn a_manifest_carrying_serde_in_its_dependencies_is_refused() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_SERDE, "");
    assert!(
        !refusals.is_empty(),
        "a manifest with a dependency was accepted"
    );
}

#[test]
fn a_manifest_carrying_regex_as_a_dev_dependency_is_refused() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_REGEX_AS_A_DEV_DEPENDENCY, "");
    assert!(
        !refusals.is_empty(),
        "a manifest with a dev-dependency was accepted"
    );
}

#[test]
fn a_manifest_carrying_a_table_of_dependencies_is_refused() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_A_TABLE_OF_DEPENDENCIES, "");
    assert!(
        !refusals.is_empty(),
        "a manifest declaring a dependency as a table was accepted"
    );
}

#[test]
fn a_manifest_carrying_no_entry_at_all_is_accepted() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_NO_ENTRY_AT_ALL, "");
    assert!(
        refusals.is_empty(),
        "an empty section is not a dependency: {refusals:?}"
    );
}

#[test]
fn a_source_importing_the_filesystem_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_FILESYSTEM);
    assert!(
        !refusals.is_empty(),
        "an import of the filesystem was accepted"
    );
}

#[test]
fn a_source_importing_the_filesystem_under_another_name_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_FILESYSTEM_UNDER_ANOTHER_NAME);
    assert!(
        !refusals.is_empty(),
        "a renamed import of the filesystem was accepted"
    );
}

#[test]
fn a_source_importing_a_process_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_A_PROCESS);
    assert!(!refusals.is_empty(), "an import of a process was accepted");
}

#[test]
fn a_source_importing_the_clock_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_CLOCK);
    assert!(!refusals.is_empty(), "an import of the clock was accepted");
}

#[test]
fn a_source_importing_a_socket_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_A_SOCKET);
    assert!(!refusals.is_empty(), "an import of a socket was accepted");
}

#[test]
fn a_source_importing_a_thread_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_A_THREAD);
    assert!(!refusals.is_empty(), "an import of a thread was accepted");
}

#[test]
fn a_source_reaching_the_filesystem_without_importing_it_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_REACHING_THE_FILESYSTEM_WITHOUT_IMPORTING_IT);
    assert!(
        !refusals.is_empty(),
        "a qualified reach for the filesystem, which imports nothing, was accepted: {refusals:?}"
    );
}

#[test]
fn a_source_importing_the_filesystem_in_a_group_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_FILESYSTEM_IN_A_GROUP);
    assert!(
        !refusals.is_empty(),
        "a grouped import of the filesystem was accepted: {refusals:?}"
    );
}

#[test]
fn a_source_naming_a_socket_in_a_group_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_NAMING_A_SOCKET_IN_A_GROUP);
    assert!(
        !refusals.is_empty(),
        "a socket named among other modules in a group was accepted: {refusals:?}"
    );
}

#[test]
fn a_group_of_modules_that_touch_nothing_is_accepted() {
    let refusals = what_is_wrong_with("", A_GROUP_OF_MODULES_THAT_TOUCH_NOTHING);
    assert!(
        refusals.is_empty(),
        "a group of modules that touch nothing is not a reach: {refusals:?}"
    );
}

#[test]
fn a_module_naming_the_filesystem_is_accepted() {
    let refusals = what_is_wrong_with("", A_MODULE_NAMING_THE_FILESYSTEM);
    assert!(
        refusals.is_empty(),
        "documentation saying the opposite is not a reach: {refusals:?}"
    );
}

#[test]
fn a_block_naming_the_filesystem_is_accepted() {
    let refusals = what_is_wrong_with("", A_BLOCK_NAMING_THE_FILESYSTEM);
    assert!(
        refusals.is_empty(),
        "a block of documentation is a comment however it begins: {refusals:?}"
    );
}

#[test]
fn a_source_carrying_a_function_and_no_comment_is_accepted() {
    let refusals = what_is_wrong_with("", A_SOURCE_CARRYING_A_FUNCTION_AND_NO_COMMENT);
    assert!(
        refusals.is_empty(),
        "an ordinary function is what the domain is made of: {refusals:?}"
    );
}

#[test]
fn a_source_declaring_an_async_function_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_DECLARING_AN_ASYNC_FUNCTION);
    assert!(!refusals.is_empty(), "an async function was accepted");
}

#[test]
fn a_source_declaring_an_unsafe_function_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_DECLARING_AN_UNSAFE_FUNCTION);
    assert!(!refusals.is_empty(), "an unsafe function was accepted");
}

#[test]
fn a_source_naming_extern_crate_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_NAMING_EXTERN_CRATE);
    assert!(!refusals.is_empty(), "an extern crate was accepted");
}

#[test]
fn a_source_that_awaits_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_THAT_AWAITS);
    assert!(!refusals.is_empty(), "an await was accepted");
}

#[test]
fn a_source_importing_memory_is_accepted() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_MEMORY);
    assert!(refusals.is_empty(), "memory is not the world: {refusals:?}");
}

#[test]
fn a_source_importing_formatting_is_accepted() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_FORMATTING);
    assert!(
        refusals.is_empty(),
        "formatting touches nothing: {refusals:?}"
    );
}

#[test]
fn a_module_saying_it_needs_not_only_serde_is_accepted() {
    let refusals = what_is_wrong_with("", A_MODULE_SAYING_IT_NEEDS_NOT_ONLY_SERDE);
    assert!(
        refusals.is_empty(),
        "prose about a crate is not a dependency: {refusals:?}"
    );
}

#[test]
fn a_module_saying_no_function_is_async_is_accepted() {
    let refusals = what_is_wrong_with("", A_MODULE_SAYING_NO_FUNCTION_IS_ASYNC);
    assert!(
        refusals.is_empty(),
        "documentation saying the opposite is not a reach: {refusals:?}"
    );
}

#[test]
fn a_refusal_in_a_source_names_the_line_it_was_found_on() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_FILESYSTEM);
    let refusal = refusals
        .first()
        .expect("the import of the filesystem was not refused at all");
    let unnamed = format!("a refusal that names no line is a refusal nobody can act on: {refusal}");
    assert!(refusal.contains("line 1"), "{unnamed}");
}

#[test]
fn a_refusal_below_a_doc_comment_names_the_line_it_is_written_on() {
    let refusals = what_is_wrong_with("", A_SOURCE_REACHING_THE_FILESYSTEM_BELOW_ITS_DOCUMENTATION);
    let refusal = refusals
        .first()
        .expect("the reach on the fifth line was not refused at all");
    let unnamed =
        format!("a refusal that names the wrong line names a line that is not there: {refusal}");
    assert!(refusal.contains("line 5"), "{unnamed}");
}

#[test]
fn a_refusal_in_a_manifest_names_the_line_the_entry_was_found_on() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_SERDE, "");
    let refusal = refusals
        .first()
        .expect("the dependency of the manifest was not refused at all");
    let unnamed = format!("a refusal that names no line is a refusal nobody can act on: {refusal}");
    assert!(refusal.contains("line 5"), "{unnamed}");
}

/// The rule itself, read off the real files rather than off a fixture, because a fixture
/// cannot go stale in a way anybody notices and a real manifest can.
#[test]
fn the_domain_depends_on_nothing() {
    let manifest = the_text_of(Path::new(THE_MANIFEST_OF_THE_DOMAIN));
    assert!(
        manifest.contains("aisdlc-domain"),
        "the manifest read was not the domain's own, so a green run would have proved nothing"
    );
    let refusals = what_is_wrong_with(&manifest, "");
    assert!(
        refusals.is_empty(),
        "the manifest of the domain carries: {refusals:?}"
    );
    assert_every_source_of_the_domain_reaches_for_nothing();
}
