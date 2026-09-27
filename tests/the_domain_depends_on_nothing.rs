//! The domain reaches for nothing that can touch the world.
//!
//! `aisdlc-domain` has no dependencies at all — not "few", not "only serde". Nothing. Its
//! manifest must not carry a `[dependencies]` or a `[dev-dependencies]` section with an
//! entry in it, and its source must not name an `extern crate`, an `async` block, an
//! `await` or an `unsafe`, and must not import a filesystem, a process, a clock, a socket
//! or a thread.
//!
//! This is the one rule in the repository nothing else can break quietly, so the checker
//! is exercised on every input it must refuse and on every input it must leave alone.
//! The last test reads the real files rather than a fixture: a fixture proves the checker
//! works, and only the real files prove the domain obeys it.
#![deny(missing_docs)]

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

/// The modules that can touch the world, written as the paths an import of them is spelled
/// with. `std::mem` is deliberately absent: a process's own memory is not the world, and a
/// domain that can move a value is a domain whose moves a reader has to reason about.
const THE_MODULES_THAT_TOUCH_THE_WORLD: [&str; 5] = [
    "std::fs",
    "std::process",
    "std::time",
    "std::net",
    "std::thread",
];

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

/// Every reach in a source, found outside the documentation, because documentation is
/// where a file says what it is not.
fn what_the_source_reaches_for(source: &str) -> Vec<String> {
    let mut refusals = Vec::new();
    for (line_number, line) in the_code_of(source) {
        refusals.extend(what_the_code_reaches_for(line_number, line));
    }
    refusals
}

/// The code of a source with its documentation gone, each line keeping the number it has in
/// the file it came from. The numbers are kept because a refusal whose line number is the
/// number it had after the documentation was dropped points at a line that is not there.
fn the_code_of(source: &str) -> Vec<(usize, &str)> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| !is_documentation(line))
        .map(|(line_number, line)| (line_number + 1, line))
        .collect()
}

/// Whether a line is documentation, which is the whole of what is stripped before the code
/// is searched. **Only a line whose first non-space characters are `//` counts**, and that
/// is sound rather than merely convenient because `the_code_carries_no_comments` forbids
/// `//` anywhere else in this repository: there is no trailing comment on a line of code
/// for a search to be fooled by, and that other gate is what makes this one true.
fn is_documentation(line: &str) -> bool {
    line.trim_start().starts_with("//")
}

/// What one line of code reaches for: one refusal per word it says and one per module it
/// imports, so a line that breaks two rules says so twice rather than hiding the second.
fn what_the_code_reaches_for(line_number: usize, line: &str) -> Vec<String> {
    let mut refusals = Vec::new();
    for word in THE_WORDS_THE_DOMAIN_MAY_NOT_SAY {
        if is_a_whole_word_in(line, word) {
            refusals.push(format!("line {line_number} says {word}"));
        }
    }
    for module in THE_MODULES_THAT_TOUCH_THE_WORLD {
        if the_line_imports(line, module) {
            refusals.push(format!("line {line_number} imports {module}"));
        }
    }
    refusals
}

/// Whether a word stands on a line whole rather than inside a longer name, so that a name
/// carrying one of these words as part of itself is not counted as a use of it.
fn is_a_whole_word_in(line: &str, word: &str) -> bool {
    for (at, _) in line.match_indices(word) {
        if stands_on_its_own(line, at, word) {
            return true;
        }
    }
    false
}

/// Whether the word beginning at `at` is bounded by something that is not part of a name.
/// A dot does not bound it, because `.await` is the same reach written differently.
fn stands_on_its_own(line: &str, at: usize, word: &str) -> bool {
    let follows_a_name = at > 0 && is_a_letter_or_digit(line.as_bytes()[at - 1]);
    let end = at + word.len();
    let followed_by_a_name = end < line.len() && is_a_letter_or_digit(line.as_bytes()[end]);
    !follows_a_name && !followed_by_a_name
}

/// What counts as part of a name. Underscore included, because a name is its letters and
/// its underscores, and a search that stopped at a letter would be evaded by one.
fn is_a_letter_or_digit(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Whether a line imports a module, which it does by writing `use` and then the module's
/// path. A `use` anywhere on the line counts rather than only at its start, because
/// `pub use std::fs;` imports the filesystem as plainly as `use std::fs;` does.
fn the_line_imports(line: &str, module: &str) -> bool {
    let words: Vec<&str> = line.split_whitespace().collect();
    for (at, word) in words.iter().enumerate() {
        let after_the_use = words.get(at + 1);
        if *word == "use" && after_the_use.is_some_and(|next| names_the_module(next, module)) {
            return true;
        }
    }
    false
}

/// Whether a word written after a `use` is that module's path and not the beginning of a
/// longer one, so that `std::fs` is the filesystem while `std::fs_extra` is not.
fn names_the_module(word: &str, module: &str) -> bool {
    if !word.starts_with(module) {
        return false;
    }
    let what_follows = &word[module.len()..];
    let goes_on_or_stops = what_follows.is_empty() || what_follows.starts_with("::");
    let ends_the_path = what_follows.starts_with(';') || what_follows.starts_with(" as ");
    goes_on_or_stops || ends_the_path
}

/// The text of a file, refusing to go on with a file it could not read, because a gate
/// that checks nothing is a gate that passed.
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
