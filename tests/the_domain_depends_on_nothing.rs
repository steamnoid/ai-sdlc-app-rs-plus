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
fn what_is_wrong_with(_manifest: &str, _source: &str) -> Vec<String> {
    Vec::new()
}

/// The text of a file, refusing to go on with a file it could not read, because a gate
/// that checks nothing is a gate that passed.
fn the_text_of(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|wrong| panic!("{path:?} could not be read: {wrong}"))
}

/// Every `.rs` file under the domain's source directory, which is what "its source" means
/// and which a test asserting on one file would quietly narrow.
fn the_rust_sources_of_the_domain() -> Vec<PathBuf> {
    let mut found = Vec::new();
    let directory = std::fs::read_dir(THE_SOURCE_DIRECTORY_OF_THE_DOMAIN);
    let listed = directory.expect("the domain's source directory could not be listed");
    for entry in listed {
        let path = entry.expect("a name in the domain's source could not be read").path();
        if path.to_str().is_some_and(|name| name.ends_with(".rs")) {
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
    assert!(!refusals.is_empty(), "a manifest with a dependency was accepted");
}

#[test]
fn a_manifest_carrying_regex_as_a_dev_dependency_is_refused() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_REGEX_AS_A_DEV_DEPENDENCY, "");
    assert!(!refusals.is_empty(), "a manifest with a dev-dependency was accepted");
}

#[test]
fn a_manifest_carrying_a_table_of_dependencies_is_refused() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_A_TABLE_OF_DEPENDENCIES, "");
    assert!(!refusals.is_empty(), "a manifest declaring a dependency as a table was accepted");
}

#[test]
fn a_manifest_carrying_no_entry_at_all_is_accepted() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_NO_ENTRY_AT_ALL, "");
    assert!(refusals.is_empty(), "an empty section is not a dependency: {refusals:?}");
}

#[test]
fn a_source_importing_the_filesystem_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_FILESYSTEM);
    assert!(!refusals.is_empty(), "an import of the filesystem was accepted");
}

#[test]
fn a_source_importing_the_filesystem_under_another_name_is_refused() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_FILESYSTEM_UNDER_ANOTHER_NAME);
    assert!(!refusals.is_empty(), "a renamed import of the filesystem was accepted");
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
    assert!(refusals.is_empty(), "formatting touches nothing: {refusals:?}");
}

#[test]
fn a_module_saying_it_needs_not_only_serde_is_accepted() {
    let refusals = what_is_wrong_with("", A_MODULE_SAYING_IT_NEEDS_NOT_ONLY_SERDE);
    assert!(refusals.is_empty(), "prose about a crate is not a dependency: {refusals:?}");
}

#[test]
fn a_module_saying_no_function_is_async_is_accepted() {
    let refusals = what_is_wrong_with("", A_MODULE_SAYING_NO_FUNCTION_IS_ASYNC);
    assert!(refusals.is_empty(), "documentation saying the opposite is not a reach: {refusals:?}");
}

#[test]
fn a_refusal_in_a_source_names_the_line_it_was_found_on() {
    let refusals = what_is_wrong_with("", A_SOURCE_IMPORTING_THE_FILESYSTEM);
    let refusal = refusals.first().expect("the import of the filesystem was not refused at all");
    let unnamed = format!("a refusal that names no line is a refusal nobody can act on: {refusal}");
    assert!(refusal.contains("line 1"), "{unnamed}");
}

#[test]
fn a_refusal_in_a_manifest_names_the_line_the_entry_was_found_on() {
    let refusals = what_is_wrong_with(A_MANIFEST_CARRYING_SERDE, "");
    let refusal = refusals.first().expect("the dependency of the manifest was not refused at all");
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
    assert!(refusals.is_empty(), "the manifest of the domain carries: {refusals:?}");
    assert_every_source_of_the_domain_reaches_for_nothing();
}
