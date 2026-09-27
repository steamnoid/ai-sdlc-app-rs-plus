//! The dependencies point inward, and the table that says so is a rule and not a snapshot.
//!
//! `aisdlc-domain` names nothing. `aisdlc-run` names the domain. `aisdlc-outside` names the
//! run and the domain. A fourth ring, `aisdlc-desk`, is added in a later phase and names
//! the outside — **and only the three crates that exist are judged.** A crate the table does
//! not name may name anything, so the day `aisdlc-desk` arrives this gate is already ready
//! for it. A table copied out of today's manifests would break that day instead, and a gate
//! that has to be repaired before it will accept a change stops anybody from making one.
//!
//! Nothing here is trusted. Every `Cargo.toml` under `crates/` is read from the disk, and
//! the parse is asked what it found before a single edge is judged, because a guard that
//! read nothing and agreed with everything would be green and would be a lie.
//!
//! ## A crate from outside is a different gate's business
//!
//! **A dependency is a violation when the crate it names is a crate of this workspace.**
//! `aisdlc-run` naming `aisdlc-outside` is a violation. `aisdlc-outside` naming `serde` is
//! not, and neither is `reqwest` or `rusqlite` or `tokio`. Which crates from outside are
//! welcome is the business of `every_third_party_crate_is_on_its_crate_allowlist`, which
//! does not exist yet and is the next gate to be written. A rule about the direction of the
//! layers that also refused `serde` would fail on the day `aisdlc-outside` is given a
//! database and a GitHub to reach, **for a reason that is not a rule**.
//!
//! ## Every table a manifest may declare a dependency in is read
//!
//! Six of them, in both of the ways a manifest spells one — a `name` and a value on a line
//! of its own, and a table header that writes the name into the header:
//! `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`,
//! `[target.'cfg(unix)'.dependencies]`, `[target.'cfg(windows)'.dev-dependencies]` and
//! `[target.'cfg(target_os = "macos")'.build-dependencies]`.
//!
//! **Only the first two of those six were read before this was repaired**, in both of their
//! spellings, and the four that were not are a gate that agreed with everything. A manifest
//! could reach any crate on any platform and this gate reported nothing, which is worse than
//! having no gate at all: a missing gate is not in the table and a silent one is, and a
//! reader who trusts a gate that is not looking stops looking. One manifest carrying all six
//! is read by one test, so a table the reading skipped would be a name missing from an
//! assertion rather than a defect nobody is told about.
//!
//! ## What this gate does not read
//!
//! `[workspace.dependencies]`, because the table is written in the root manifest, which this
//! gate does not open, and a crate cannot declare a dependency for the whole workspace. A
//! member that inherits one from it is written `aisdlc-domain = { workspace = true }` with
//! no `path` in it, and is caught anyway: a crate of this workspace is one whose directory
//! sits under `crates/`, and that is asked about as well as the path.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// What a set of crates declares: every crate beside every dependency that crate names,
/// keyed by the crate's own name as its manifest spells it.
type WhatEachCrateDeclares = BTreeMap<String, BTreeSet<ADependency>>;

/// What each crate is allowed to name, keyed the same way. One entry per crate whose
/// direction this repository has decided, and a crate with no entry here is not judged.
type WhatEachCrateMayName = BTreeMap<String, BTreeSet<String>>;

/// The crates that are part of this workspace, which is a different question from the crates
/// a manifest happens to name.
///
/// **A crate of this workspace is one reached by a `path`, or one whose own manifest sits in
/// a directory under `crates/`.** Which of the two makes a name one is asked about in
/// `a_dependency_is_a_crate_of_this_workspace`, which is where the rule and both of its
/// reasons live.
type TheCratesOfThisWorkspace = BTreeSet<String>;

/// The dependencies of one manifest, keyed by the name it wrote for each.
///
/// A map and not a set because a name declared in two tables is **one dependency rather
/// than two**, and a key is the only place that fact can be kept: the reading has to
/// remember the table it was first named under and still hear about a path written under
/// the other one.
type TheDependenciesOfOneManifest = BTreeMap<String, ADependency>;

/// One dependency a manifest declares: the name it wrote, whether it reached that name by a
/// `path`, and the table it wrote it in.
///
/// All three are carried because all three are facts about a declaration rather than
/// opinions about it. The name is what the direction rule judges, the path is one of the
/// two things that make a crate a crate of this workspace, and the table is what a failure
/// quotes so that a reader is sent to the one place the dependency is written down.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ADependency {
    name: String,
    was_reached_by_a_path: bool,
    the_table_it_was_declared_in: String,
}

impl ADependency {
    /// A dependency a manifest declares, reached or not reached by a path, in a table.
    fn of(name: &str, was_reached_by_a_path: bool, the_table_it_was_declared_in: &str) -> Self {
        Self {
            name: name.to_string(),
            was_reached_by_a_path,
            the_table_it_was_declared_in: the_table_it_was_declared_in.to_string(),
        }
    }

    /// The name a manifest wrote, which is the name the direction rule is about.
    fn name(&self) -> &str {
        &self.name
    }

    /// Whether the manifest reached this crate by a `path`, which is the first of the two
    /// answers to whether it is a crate of this workspace.
    fn was_reached_by_a_path(&self) -> bool {
        self.was_reached_by_a_path
    }

    /// The table this dependency is written in, which is what a failure quotes.
    fn the_table_it_was_declared_in(&self) -> &str {
        &self.the_table_it_was_declared_in
    }
}

/// The direction, for the three crates that exist.
///
/// A `const` and not a function body, so the table is one thing to read and one place to
/// change. `aisdlc-desk` is absent on purpose: it does not exist yet, and a rule that
/// refuses a crate nobody has written is a rule about this week's snapshot rather than
/// about the direction the layers are meant to have.
const THE_DIRECTION_OF_EVERY_CRATE_THAT_EXISTS: &[(&str, &[&str])] = &[
    ("aisdlc-domain", &[]),
    ("aisdlc-run", &["aisdlc-domain"]),
    ("aisdlc-outside", &["aisdlc-run", "aisdlc-domain"]),
];

/// The tables a manifest may declare a dependency in, and the whole of them.
///
/// Three names, and they are asked about the **last** word of a section rather than the
/// first, so that a condition written in front of one of them by a target table reaches it.
/// **A table not named here is not read, and nothing warns anybody that it was skipped**,
/// which is why this is one list of three a reader can hold against cargo's own rather than
/// a rule with a hole in it: a table cargo adds later is a line somebody has to add here, and
/// a reading that had not been told would go on agreeing with the manifests it cannot see.
const THE_TABLES_A_DEPENDENCY_MAY_BE_DECLARED_IN: [&str; 3] =
    ["dependencies", "dev-dependencies", "build-dependencies"];

/// What a line before the first heading of a manifest is written under, which is nowhere.
const NO_SECTION_YET: &str = "";

/// A manifest that declares a dependency in every table a manifest may declare one in, each
/// time under a different name.
///
/// Six names for six tables on purpose: a table the reading skips is then a name missing
/// from the answer rather than a second copy of a name another table already gave, and a
/// reader of a failed assertion can see which table it was.
const A_MANIFEST_CARRYING_EVERY_SHAPE_OF_A_DEPENDENCY: &str = "\
[package]
name = \"aisdlc-run\"

[dependencies]
aisdlc-outside = { path = \"../outside\" }

[dev-dependencies]
aisdlc-desk = { path = \"../desk\" }

[build-dependencies]
aisdlc-paperwork = { path = \"../paperwork\" }

[target.'cfg(unix)'.dependencies]
aisdlc-ledger = { path = \"../ledger\" }

[target.'cfg(windows)'.dev-dependencies]
aisdlc-signing = { path = \"../signing\" }

[target.'cfg(target_os = \"macos\")'.build-dependencies]
aisdlc-invoices = { path = \"../invoices\" }
";

/// A manifest that writes a dependency into a table's own header, which is the form that
/// leaves no line of its own for a reading of lines to find.
const A_MANIFEST_WRITING_A_DEPENDENCY_INTO_A_TABLE_HEADER: &str = "\
[package]
name = \"aisdlc-run\"

[dependencies.aisdlc-desk]
path = \"../desk\"
";

/// The same spelling underneath a target table, which is a condition, a table and a name all
/// in one line.
const A_MANIFEST_WRITING_A_DEPENDENCY_INTO_A_TARGET_HEADER: &str = "\
[package]
name = \"aisdlc-run\"

[target.'cfg(unix)'.dev-dependencies.aisdlc-outside]
path = \"../outside\"
";

/// The defect this gate was repaired for: the run reaching out from under a target table,
/// on a platform where that table applies. A reading that stops at `[dependencies]` sees a
/// manifest with one legal edge in it and reports nothing at all.
const A_MANIFEST_REACHING_OUTWARD_UNDER_A_TARGET_TABLE: &str = "\
[package]
name = \"aisdlc-run\"

[target.'cfg(unix)'.dependencies]
aisdlc-desk = { path = \"../desk\" }
";

/// A manifest reaching a crate of this workspace from under a target table's
/// dev-dependencies, which is the other of the two target tables and which the reading that
/// stopped at the first one missed just as quietly.
const A_MANIFEST_REACHING_A_CRATE_UNDER_A_TARGET_DEV_DEPENDENCIES_TABLE: &str = "\
[package]
name = \"aisdlc-run\"

[target.'cfg(windows)'.dev-dependencies]
serde = { path = \"../serde\" }
";

/// A manifest whose tables are not tables of dependencies, and whose keys would be read as
/// dependency names by a reading that asked every table the same question.
///
/// `workspace = true` under `[lints]` is not a dependency called `workspace`, and a
/// `default = []` under `[features]` is not a dependency called `default`. The target table
/// in here is the one that is not a table of dependencies either: `[target.'cfg(unix)']` is
/// a table of targets, and the thing written under it is a target's name.
const A_MANIFEST_CARRYING_TABLES_THAT_DECLARE_NO_DEPENDENCY: &str = "\
[package]
name = \"aisdlc-run\"

[lints]
workspace = true

[features]
default = []

[target.'cfg(unix)']
aisdlc-desk = { path = \"../desk\" }
";

/// A manifest with a dependency commented out, which is a sentence in a file rather than a
/// declaration in one, and a reading that counted it would report a violation naming a word
/// out of a comment.
const A_MANIFEST_WITH_A_DEPENDENCY_COMMENTED_OUT: &str = "\
[package]
name = \"aisdlc-run\"

[dependencies]
# aisdlc-outside = { path = \"../outside\" }
aisdlc-domain = { path = \"../domain\" }
";

/// A manifest naming one crate in two tables, the second of them the one that reaches it by
/// a path, so that a name reached in one place and declared in another is read as the one
/// thing it is.
const A_MANIFEST_NAMING_ONE_CRATE_IN_TWO_TABLES: &str = "\
[package]
name = \"aisdlc-run\"

[dependencies]
aisdlc-desk = \"0.1\"

[target.'cfg(unix)'.dependencies]
aisdlc-desk = { path = \"../desk\" }
";

/// Every edge in the workspace that points outward, one message per edge.
///
/// Both arguments are read and neither is assumed: `declared` is what the manifests on the
/// disk say and `allowed` is the rule. The walk is over the rule rather than over the
/// manifests, and that is the whole design in one line — a crate the rule does not name is
/// never reached, and neither is a crate the rule names that is not on the disk yet.
///
/// **Only a crate of this workspace is a violation here.** `aisdlc-run` naming
/// `aisdlc-outside` is one. `aisdlc-outside` naming `serde` is not, and neither is `reqwest`
/// or `rusqlite` or `tokio`: which crates from outside are welcome is the business of
/// `every_third_party_crate_is_on_its_crate_allowlist`, which does not exist yet. A rule
/// about the direction of the layers that also refused `serde` would fail on the day
/// `aisdlc-outside` is given a database and a GitHub to reach, for a reason that is not a
/// rule. The next person to widen this should read that before they widen it.
fn what_points_the_wrong_way(
    declared: &WhatEachCrateDeclares,
    allowed: &WhatEachCrateMayName,
    crates_of_this_workspace: &TheCratesOfThisWorkspace,
) -> Vec<String> {
    let mut pointing_outward = vec![];
    for (crate_name, its_allowed) in allowed {
        let Some(its_own) = declared.get(crate_name) else {
            continue;
        };
        pointing_outward.extend(the_edges_that_leave(
            crate_name,
            its_own,
            its_allowed,
            crates_of_this_workspace,
        ));
    }
    pointing_outward
}

/// The edges one crate's own dependencies include that it is not allowed to name.
///
/// A crate nobody has is not a crate that depends on anything, so a crate the rule names
/// and the disk does not have contributes no edges and no verdict; the walk that calls this
/// skips it before it gets here. A dependency that is not a crate of this workspace is left
/// alone for the same reason a missing crate is: this gate is about which way an edge
/// points inside the layers, and whether a crate from a registry is welcome is a different
/// question with a different answer for every crate that asks it.
fn the_edges_that_leave(
    crate_name: &str,
    its_own: &BTreeSet<ADependency>,
    its_allowed: &BTreeSet<String>,
    crates_of_this_workspace: &TheCratesOfThisWorkspace,
) -> Vec<String> {
    its_own
        .iter()
        .filter(|dependency| !its_allowed.contains(dependency.name()))
        .filter(|dependency| {
            a_dependency_is_a_crate_of_this_workspace(dependency, crates_of_this_workspace)
        })
        .map(|dependency| {
            format!(
                "{crate_name} names {}, declared in [{}]",
                dependency.name(),
                dependency.the_table_it_was_declared_in()
            )
        })
        .collect()
}

/// Whether a dependency is a crate of this workspace, and so a thing this gate judges rather
/// than a crate whose welcome is another gate's business.
///
/// **A crate of this workspace is one reached by a `path`, or one whose own manifest sits in
/// a directory under `crates/`, and the path is asked first.** A name is not a membership:
/// a crate on a registry that happens to be spelled `aisdlc-desk` is named by neither and is
/// left alone, and a rule that went by the name would refuse it on the day somebody
/// published a crate with our name on it. `crates/domain` is named by both at once, which
/// is the case the two agree on and neither of them has anything to add.
fn a_dependency_is_a_crate_of_this_workspace(
    dependency: &ADependency,
    crates_of_this_workspace: &TheCratesOfThisWorkspace,
) -> bool {
    dependency.was_reached_by_a_path() || crates_of_this_workspace.contains(dependency.name())
}

/// What every crate in the workspace declares, keyed by the name in its own manifest.
///
/// A manifest that cannot be read panics rather than contributing nothing, because a crate
/// this gate failed to open would be a crate this gate agreed with. **A directory under
/// `crates/` with no manifest in it is a mistake and not an absence**: a scratch directory
/// left behind by a run, a tree copied half way, a rename caught in the act — each of them
/// is a real thing that has gone wrong, and every one of them would be invisible to a gate
/// that skipped what it could not find. A directory without a manifest is not a state this
/// repository is in, so the answer is a panic naming it rather than a green gate.
fn the_dependencies_every_crate_declares() -> WhatEachCrateDeclares {
    let mut declared = WhatEachCrateDeclares::new();
    for directory in the_crate_directories() {
        let manifest = directory.join("Cargo.toml");
        let text = std::fs::read_to_string(&manifest).expect("a crate has a readable manifest");
        declared.insert(the_name_of_the_crate(&text), the_dependencies_in(&text));
    }
    declared
}

/// The crates of this workspace, from the manifests this gate has read.
///
/// Every crate this gate reads is in a directory under `crates/`, so every one of them is a
/// crate of this workspace by its directory alone, and the map it is read from is already
/// the list of them. The path is the other of the two answers and it is asked per
/// dependency, where the manifest that wrote it is to hand. The workspace also has a
/// package at its root, `aisdlc-gates`, which sits outside `crates/` and is named by no
/// manifest; it depends on nothing, so there is no edge in it for this gate to judge.
fn the_crates_of_this_workspace(declared: &WhatEachCrateDeclares) -> TheCratesOfThisWorkspace {
    declared.keys().cloned().collect()
}

/// Every crate directory in `crates/`, read from the disk rather than from a list written
/// beside this gate, so a crate added tomorrow is judged by this gate and not by whichever
/// gate next remembers it.
fn the_crate_directories() -> Vec<PathBuf> {
    let crates = the_root_of_the_checkout().join("crates");
    let mut found = vec![];
    for entry in std::fs::read_dir(&crates).expect("the workspace has a readable crates/") {
        let entry = entry.expect("every entry in crates/ is readable");
        if entry.path().is_dir() {
            found.push(entry.path());
        }
    }
    found
}

/// The root of the checkout, found by walking up from where cargo compiled this file.
///
/// `CARGO_MANIFEST_DIR` is fixed by cargo when this file is compiled and names the checkout
/// itself, so it is a thing a reader can follow and not a thing a test has to be configured
/// with. `crates/` is the thing worth walking up to, because it is the workspace and the
/// rule is about the workspace.
fn the_root_of_the_checkout() -> PathBuf {
    let mut here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !here.join("crates").is_dir() {
        assert!(
            here.pop(),
            "no crates/ directory above where this file was compiled"
        );
    }
    here
}

/// The dependencies one manifest declares, read out of every table it may declare one in.
///
/// Both spellings of a dependency count, because both are ways to depend on something: a
/// `name` and a value on a line of its own, and a table header that writes the dependency in
/// the header. Reading only the first would pass a crate that depends on anything as a
/// crate that depends on nothing, and a gate that cannot be seen to fail is a gate nobody
/// has exercised. **The reading walks lines and carries two things** — the section it is
/// inside, and the dependency a header wrote into it — because a key under a header names
/// the header's dependency rather than a dependency of its own.
fn the_dependencies_in(manifest: &str) -> BTreeSet<ADependency> {
    let mut declared = TheDependenciesOfOneManifest::new();
    let mut the_section = NO_SECTION_YET;
    let mut the_name_in_the_header: Option<&str> = None;
    for line in manifest.lines() {
        if let Some(header) = the_name_of_the_section(line) {
            the_section = header;
            the_name_in_the_header = the_dependency_named_in_a_header(header);
            remember_the_name_a_header_wrote(the_section, the_name_in_the_header, &mut declared);
        } else {
            remember_what_a_line_declares(line, the_section, the_name_in_the_header, &mut declared);
        }
    }
    the_dependencies_that_were_read(declared)
}

/// The name a section header declares, or `None` for a line that is not a header.
///
/// `None` is not an absence to shrug at: it is how the walk knows a header has not just
/// closed the section it was reading, which is the only state this parser carries.
fn the_name_of_the_section(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if !trimmed.starts_with('[') {
        return None;
    }
    Some(trimmed.trim_start_matches('[').trim_end_matches(']'))
}

/// Whether a section reaches a table of dependencies, whether it stops at that table or
/// carries a dependency's name in the same line as the table's own name.
///
/// The **last** word of the name is asked first, because a target table writes a condition
/// in front of the name and the condition is the part that varies: `dependencies`,
/// `target.'cfg(unix)'.dependencies` and `target.'cfg(windows)'.dev-dependencies` are one
/// answer to one question. A section whose last word is not one of the three is asked about
/// the table inside it, which is what makes the lines under `[dependencies.aisdlc-domain]`
/// the lines of a dependency rather than a section of their own. A section with no table in
/// it is not a table of dependencies whatever else it is, which is what makes `[lints]` with
/// `workspace = true` in it a lints table and not a dependency called `workspace`.
fn is_a_table_of_dependencies(section: &str) -> bool {
    let the_table = the_table_a_section_opens(section);
    THE_TABLES_A_DEPENDENCY_MAY_BE_DECLARED_IN.contains(&the_last_word_of(section))
        || THE_TABLES_A_DEPENDENCY_MAY_BE_DECLARED_IN.contains(&the_table)
}

/// The dependency a section header writes into its own text, and `None` for a header that
/// writes only the name of its table.
///
/// `[dependencies.aisdlc-domain]` declares a dependency as plainly as `aisdlc-domain =
/// "../domain"` does, and a reading that recognised only the second would call that manifest
/// empty. The name sits in the word in front of the table's own name, so a header with no
/// table word there names no dependency — which is what a target table such as
/// `[target.'cfg(unix)']` is: a table of targets, and not a dependency called `cfg(unix)`.
fn the_dependency_named_in_a_header(section: &str) -> Option<&str> {
    let word_before_the_table = the_word_before_the_last_of(section);
    if !THE_TABLES_A_DEPENDENCY_MAY_BE_DECLARED_IN.contains(&word_before_the_table) {
        return None;
    }
    Some(the_last_word_of(section))
}

/// The table a section reaches, which for a dependency under a target table includes the
/// condition and which never includes the dependency's name a header wrote in the same line.
///
/// The table is the run of words up to and including the last one that is the name of a
/// table, so `[target.'cfg(unix)'.dependencies]` is that whole table while
/// `[target.'cfg(unix)'.dev-dependencies.aisdlc-outside]` is the first of it. A failure can
/// then quote a table a reader can go and look at rather than a manifest to search.
fn the_table_a_section_opens(section: &str) -> &str {
    let mut how_far_the_table_reaches = 0;
    let mut how_far_we_have_read = 0;
    for word in section.split('.') {
        let the_end_of_this_word = how_far_we_have_read + word.len();
        if THE_TABLES_A_DEPENDENCY_MAY_BE_DECLARED_IN.contains(&word) {
            how_far_the_table_reaches = the_end_of_this_word;
        }
        how_far_we_have_read = the_end_of_this_word + 1;
    }
    &section[..how_far_the_table_reaches]
}

/// The last word of a section name, which is the part that says what kind of table it is.
fn the_last_word_of(section: &str) -> &str {
    section.rsplit('.').next().unwrap_or(section)
}

/// The word in front of the last one, which is where the name of a table sits when a header
/// has also written a dependency's name in the line: `[dependencies.aisdlc-domain]`.
fn the_word_before_the_last_of(section: &str) -> &str {
    let the_rest_of_it = match section.rsplit_once('.') {
        Some((the_rest, _)) => the_rest,
        None => NO_SECTION_YET,
    };
    the_last_word_of(the_rest_of_it)
}

/// Note the dependency a header wrote into its own text, which the lines under it will not
/// name again.
fn remember_the_name_a_header_wrote(
    the_section: &str,
    the_name_in_the_header: Option<&str>,
    declared: &mut TheDependenciesOfOneManifest,
) {
    let Some(named) = the_name_in_the_header else {
        return;
    };
    let found = ADependency::of(named, false, the_table_a_section_opens(the_section));
    remember_a_dependency(declared, found);
}

/// Note what one line under a table of dependencies adds: a dependency of its own, or a
/// path the dependency a header wrote was reached by.
///
/// A line under a header is a **key of that dependency** and not a dependency of its own,
/// so `version = "1"` and `path = "../domain"` under `[dependencies.aisdlc-domain]` are the
/// two facts about one dependency rather than two more dependencies. The header form is the
/// only one where a path can arrive that way, and a reading that skipped those lines would
/// answer the question of membership from the directory alone every time.
fn remember_what_a_line_declares(
    line: &str,
    the_section: &str,
    the_name_in_the_header: Option<&str>,
    declared: &mut TheDependenciesOfOneManifest,
) {
    if !is_a_table_of_dependencies(the_section) {
        return;
    }
    let Some(named) = the_name_in_the_header else {
        remember_the_dependency_on_a_line(line, the_section, declared);
        return;
    };
    if the_line_is_a_path_to_a_crate(line) {
        mark_it_as_reached_by_a_path(named, declared);
    }
}

/// Note the dependency a line of its own declares, read as `name = value`.
fn remember_the_dependency_on_a_line(
    line: &str,
    the_section: &str,
    declared: &mut TheDependenciesOfOneManifest,
) {
    let Some(named) = the_dependency_on_a_line(line) else {
        return;
    };
    let reached_by_a_path = the_line_reaches_by_a_path(line);
    let found = ADependency::of(
        named,
        reached_by_a_path,
        the_table_a_section_opens(the_section),
    );
    remember_a_dependency(declared, found);
}

/// Note a dependency, and a name declared in two tables is **one dependency rather than
/// two**: it is reported under the table that named it first, because that is where the edge
/// is written down, and a path in either of the tables is a path, because reaching a crate
/// is one thing done in two places rather than two things done once each.
fn remember_a_dependency(declared: &mut TheDependenciesOfOneManifest, found: ADependency) {
    if !declared.contains_key(found.name()) {
        declared.insert(found.name().to_string(), found);
        return;
    }
    if found.was_reached_by_a_path() {
        mark_it_as_reached_by_a_path(found.name(), declared);
    }
}

/// Record that a dependency already found was reached by a path, whether or not it was found
/// with one: a name may be declared before the path that reaches it is written down.
fn mark_it_as_reached_by_a_path(named: &str, declared: &mut TheDependenciesOfOneManifest) {
    if let Some(reached) = declared.get(named).map(reached_by_a_path) {
        declared.insert(named.to_string(), reached);
    }
}

/// The same dependency, one this manifest reached by a path.
fn reached_by_a_path(dependency: &ADependency) -> ADependency {
    ADependency::of(
        dependency.name(),
        true,
        dependency.the_table_it_was_declared_in(),
    )
}

/// The dependencies the reading found, as a set, which is the shape the direction rule
/// asks its question in.
fn the_dependencies_that_were_read(
    declared: TheDependenciesOfOneManifest,
) -> BTreeSet<ADependency> {
    declared.into_values().collect()
}

/// The name a manifest gives its own crate, because a directory's name is a guess and the
/// manifest is the answer: `crates/run` is `aisdlc-run`, and the rule is written in the
/// second spelling.
fn the_name_of_the_crate(manifest: &str) -> String {
    for line in manifest.lines() {
        if line.starts_with("name") && line.contains('=') {
            let named = the_text_after_the_first_equals(line);
            return named
                .expect("the caller checked for an equals sign")
                .to_string();
        }
    }
    panic!("every crate manifest names its crate");
}

/// The name before the first `=`, which is where a manifest writes a dependency's name or a
/// key of a dependency's.
///
/// A comment and an empty line name nothing, and both are ordinary in a manifest. A parse
/// that counted a comment as an edge would report a violation naming a word out of a
/// sentence, which is a gate that cries wolf and is therefore a gate nobody believes.
fn the_dependency_on_a_line(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.starts_with('#') {
        return None;
    }
    let (name, _) = trimmed.split_once('=')?;
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    Some(name)
}

/// Whether a line declares a dependency this manifest reached by a `path`.
///
/// A path is written inside the value and nowhere else — `aisdlc-domain = { path =
/// "../domain" }` — so the question is asked of the value rather than of the whole line, and
/// the white space is taken out first because a manifest is free to write as much of it as
/// it likes. The `path` written under a table header is a different question with a
/// different answer, and `the_line_is_a_path_to_a_crate` is what asks it.
fn the_line_reaches_by_a_path(line: &str) -> bool {
    let Some(value) = the_text_after_the_first_equals(line) else {
        return false;
    };
    let without_the_white_space: String = value.chars().filter(|it| !it.is_whitespace()).collect();
    without_the_white_space.contains("path=")
}

/// Whether a line under a table header is the `path` key of the dependency that header
/// named, which is how the header form of a dependency says it reached a crate beside us.
fn the_line_is_a_path_to_a_crate(line: &str) -> bool {
    the_dependency_on_a_line(line).is_some_and(|name| name == "path")
}

/// The text after the first `=`, without the quotes a manifest puts around it, and `None`
/// for a line that has no `=` in it.
fn the_text_after_the_first_equals(line: &str) -> Option<&str> {
    let (_, value) = line.split_once('=')?;
    Some(value.trim().trim_matches('"'))
}

/// A workspace of crates written the way this gate reads one, so that a fixture says what
/// it is rather than how to spell a map.
///
/// Every name is reached by a path, because that is how a manifest beside us is written and
/// a path is what makes a name a crate of this workspace.
fn a_crate_naming_each(crates: &[(&str, &[&str])]) -> WhatEachCrateDeclares {
    a_crate_naming_what_it_reaches(crates, "dependencies", true)
}

/// The same, with every name written the way a manifest names a crate on a registry: a
/// version and no `path`, which is the only thing that says the name is not one of ours.
fn a_crate_naming_each_from_a_registry(crates: &[(&str, &[&str])]) -> WhatEachCrateDeclares {
    a_crate_naming_what_it_reaches(crates, "dependencies", false)
}

/// The same, with every name written under a target table, because which table an edge is
/// written in is not what decides it and a fixture has to be able to say so.
fn a_crate_naming_each_under_a_target_table(crates: &[(&str, &[&str])]) -> WhatEachCrateDeclares {
    a_crate_naming_what_it_reaches(crates, "target.'cfg(unix)'.dependencies", true)
}

/// The declared map a fixture describes, with every name written in one table and reached in
/// one way, so that a fixture says which crates are its own once rather than in every test.
fn a_crate_naming_what_it_reaches(
    crates: &[(&str, &[&str])],
    the_table: &str,
    reached_by_a_path: bool,
) -> WhatEachCrateDeclares {
    let mut declared = WhatEachCrateDeclares::new();
    for (crate_name, its_own) in crates {
        let its_own: BTreeSet<ADependency> = its_own
            .iter()
            .map(|name| ADependency::of(name, reached_by_a_path, the_table))
            .collect();
        declared.insert(crate_name.to_string(), its_own);
    }
    declared
}

/// The workspace a list of manifests describes, read by the reading the disk is read by, so
/// that a test can ask the whole gate about a manifest rather than about a map it built for
/// itself.
///
/// The name of each crate is read out of its own manifest, exactly as it is on the disk,
/// because a fixture that wrote the name beside the manifest would be a fixture that could
/// disagree with the manifest it is a fixture of.
fn the_workspace_these_manifests_describe(manifests: &[&str]) -> WhatEachCrateDeclares {
    let mut declared = WhatEachCrateDeclares::new();
    for manifest in manifests {
        declared.insert(
            the_name_of_the_crate(manifest),
            the_dependencies_in(manifest),
        );
    }
    declared
}

/// Whether a manifest's reading names a dependency, so that a test can ask about one name
/// without knowing how a dependency is put together.
fn the_reading_named(read: &BTreeSet<ADependency>, name: &str) -> bool {
    read.iter().any(|dependency| dependency.name() == name)
}

/// The crates of this workspace for a fixture in which **every dependency is reached by a
/// path**, which is the first of the two things that make a crate one of ours.
///
/// A `fn` and not a `const` so that it is a value a test passes rather than a name a test
/// has to look up, and an empty set rather than a list of the three crates on the disk
/// because a fixture has no directory to ask about and the path already answered.
fn by_a_path_alone() -> TheCratesOfThisWorkspace {
    TheCratesOfThisWorkspace::new()
}

/// The crates of this workspace a fixture is working with, for the fixtures that are about
/// the second of the two answers: a name is one of ours because a directory under `crates/`
/// holds its manifest, which is a fact a fixture has to be handed because it has no
/// directory to ask about.
fn a_workspace_holding(names: &[&str]) -> TheCratesOfThisWorkspace {
    names.iter().copied().map(String::from).collect()
}

/// The rule, read from the table above rather than written out a second time here, so there
/// is one copy of the direction and one place to change it.
fn what_each_may_name() -> WhatEachCrateMayName {
    let mut allowed = WhatEachCrateMayName::new();
    for (crate_name, its_allowed) in THE_DIRECTION_OF_EVERY_CRATE_THAT_EXISTS {
        allowed.insert(
            crate_name.to_string(),
            its_allowed.iter().copied().map(String::from).collect(),
        );
    }
    allowed
}

/// The rule as a whole, against the workspace that is actually on the disk.
#[test]
fn the_dependencies_point_inward() {
    let declared = the_dependencies_every_crate_declares();
    let crates_of_this_workspace = the_crates_of_this_workspace(&declared);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &crates_of_this_workspace);
    assert!(
        pointing_outward.is_empty(),
        "these edges point outward: {pointing_outward:?}"
    );
}

/// A guard that read nothing and agreed with everything would be green and would be a lie,
/// so the parse is asked what it found before any edge is judged: the three crates that
/// exist, and the one edge the glossary says `aisdlc-run` has. If this test is ever deleted
/// the gate above is worth nothing, because it would be agreeing with an empty map.
#[test]
fn the_manifests_on_disk_were_read_and_not_assumed() {
    let declared = the_dependencies_every_crate_declares();
    for crate_name in ["aisdlc-domain", "aisdlc-run", "aisdlc-outside"] {
        assert!(
            declared.contains_key(crate_name),
            "{crate_name} is not there"
        );
    }
    assert!(
        the_reading_named(&declared["aisdlc-run"], "aisdlc-domain"),
        "aisdlc-run was read as naming nothing"
    );
}

/// The manifest carrying every table is read out of all of them, and a table the reading
/// skips is a name missing from this answer rather than a silence.
#[test]
fn a_manifest_carrying_every_shape_of_a_dependency_is_read() {
    let read = the_dependencies_in(A_MANIFEST_CARRYING_EVERY_SHAPE_OF_A_DEPENDENCY);
    for named in [
        "aisdlc-outside",
        "aisdlc-desk",
        "aisdlc-paperwork",
        "aisdlc-ledger",
        "aisdlc-signing",
        "aisdlc-invoices",
    ] {
        assert!(
            the_reading_named(&read, named),
            "{named} was not read out of {read:?}"
        );
    }
}

/// A dependency a table header writes is read, which is the form that leaves no line of its
/// own for a reading of lines to find.
#[test]
fn a_dependency_written_into_a_table_header_is_read() {
    let read = the_dependencies_in(A_MANIFEST_WRITING_A_DEPENDENCY_INTO_A_TABLE_HEADER);
    assert!(
        the_reading_named(&read, "aisdlc-desk"),
        "a header wrote the dependency and nothing was read out of {read:?}"
    );
}

/// A dependency a target table's own header writes is read, and a condition, a table and a
/// name all in one line are three things to tell apart rather than one thing to guess at.
#[test]
fn a_dependency_written_into_a_target_header_is_read() {
    let read = the_dependencies_in(A_MANIFEST_WRITING_A_DEPENDENCY_INTO_A_TARGET_HEADER);
    assert!(
        the_reading_named(&read, "aisdlc-outside"),
        "a header wrote the dependency and nothing was read out of {read:?}"
    );
}

/// The tables that declare no dependency are read as tables that declare none, so a key of
/// `[lints]` and a target's name under `[target.'cfg(unix)']` are not two dependencies this
/// gate then has to agree with.
#[test]
fn a_table_that_declares_no_dependency_is_read_as_one_that_declares_none() {
    let read = the_dependencies_in(A_MANIFEST_CARRYING_TABLES_THAT_DECLARE_NO_DEPENDENCY);
    assert!(
        read.is_empty(),
        "a table that is not a table of dependencies was read as one: {read:?}"
    );
}

/// A dependency written out in a comment is a sentence about a dependency rather than a
/// declaration of one, and a gate that reported it would be reporting a word out of a
/// comment.
#[test]
fn a_commented_out_dependency_is_not_a_dependency() {
    let read = the_dependencies_in(A_MANIFEST_WITH_A_DEPENDENCY_COMMENTED_OUT);
    assert!(
        !the_reading_named(&read, "aisdlc-outside"),
        "a comment was read as a declaration: {read:?}"
    );
    assert!(
        the_reading_named(&read, "aisdlc-domain"),
        "the real dependency beside the comment was not read: {read:?}"
    );
}

/// The fixture is one run depending on the outside, which is the edge the whole rule
/// exists to refuse, and the message says which table wrote it down.
#[test]
fn a_run_naming_the_outside_is_refused() {
    let declared = a_crate_naming_each(&[("aisdlc-run", &["aisdlc-outside"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-run names aisdlc-outside, declared in [dependencies]"]
    );
}

/// The fixture is the domain depending on the run. The direction of that one edge is what
/// every other boundary in this repository rests on, so it is the edge with the fewest
/// excuses.
#[test]
fn a_domain_naming_the_run_is_refused() {
    let declared = a_crate_naming_each(&[("aisdlc-domain", &["aisdlc-run"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-domain names aisdlc-run, declared in [dependencies]"]
    );
}

/// The fixture is the outside naming a crate that does not exist yet, in a ring inside it.
/// A crate nobody has written is still not a dependency the outside is allowed, and the
/// path in the manifest is what makes it one of ours whatever the day it is written on.
#[test]
fn the_outside_naming_the_desk_is_refused() {
    let declared = a_crate_naming_each(&[("aisdlc-outside", &["aisdlc-desk"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-outside names aisdlc-desk, declared in [dependencies]"]
    );
}

/// The edge the reading used to walk past, on a platform where the table it is written under
/// applies: refused, and the failure says which of the six tables it was found in.
#[test]
fn a_dependency_under_a_target_table_is_named_in_the_failure() {
    let declared =
        the_workspace_these_manifests_describe(&[A_MANIFEST_REACHING_OUTWARD_UNDER_A_TARGET_TABLE]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-run names aisdlc-desk, declared in [target.'cfg(unix)'.dependencies]"]
    );
}

/// The other of the two target tables, which is the one a reading that stopped at
/// `[target.'cfg(unix)'.dependencies]` would go on missing just as quietly.
#[test]
fn a_dependency_under_a_target_dev_dependencies_table_is_named_in_the_failure() {
    let declared = the_workspace_these_manifests_describe(&[
        A_MANIFEST_REACHING_A_CRATE_UNDER_A_TARGET_DEV_DEPENDENCIES_TABLE,
    ]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-run names serde, declared in [target.'cfg(windows)'.dev-dependencies]"]
    );
}

/// A dependency a table header wrote is refused for the same reason as one written on a
/// line of its own, and **the `path` written under that header is the only reason it is
/// refused at all** — there is no crate directory here for it to be named by.
#[test]
fn a_dependency_named_in_a_table_header_is_named_in_the_failure() {
    let declared = the_workspace_these_manifests_describe(&[
        A_MANIFEST_WRITING_A_DEPENDENCY_INTO_A_TABLE_HEADER,
    ]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-run names aisdlc-desk, declared in [dependencies]"]
    );
}

/// The other of the two answers to whether a crate is one of ours, on the very edge
/// `a_run_naming_the_outside_is_refused` refuses for the first of them: a manifest that
/// names a crate of this workspace with a version and no `path` has still named one, because
/// that crate's own manifest sits in a directory under `crates/`. The two tests differ in
/// one fact, and neither would pass if the reading had only one of the two answers.
#[test]
fn a_crate_named_without_a_path_is_refused_all_the_same() {
    let declared = a_crate_naming_each_from_a_registry(&[("aisdlc-run", &["aisdlc-outside"])]);
    let the_workspace_holding_the_outside = a_workspace_holding(&["aisdlc-outside"]);
    let pointing_outward = what_points_the_wrong_way(
        &declared,
        &what_each_may_name(),
        &the_workspace_holding_the_outside,
    );
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-run names aisdlc-outside, declared in [dependencies]"]
    );
}

/// A crate named in two tables is one crate, and it is one crate that was reached by a path
/// because the second of the two tables says so and the first does not — **and it is
/// reported under the table that named it first**, which is the only one a reader looking at
/// the first table would find it in. The workspace is empty of members here, so a path is the
/// only reason this edge is a violation at all.
#[test]
fn a_crate_named_in_two_tables_is_one_crate_reached_by_a_path() {
    let declared =
        the_workspace_these_manifests_describe(&[A_MANIFEST_NAMING_ONE_CRATE_IN_TWO_TABLES]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert_eq!(
        pointing_outward,
        vec!["aisdlc-run names aisdlc-desk, declared in [dependencies]"]
    );
}

/// The fixture is the run naming the domain, which is the one edge that is always legal and
/// so is the edge a broken gate would most likely refuse.
#[test]
fn a_run_naming_the_domain_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-run", &["aisdlc-domain"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is the outside naming both of the rings inside it, which is the whole of what
/// the outside is allowed.
#[test]
fn the_outside_naming_the_run_and_the_domain_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-outside", &["aisdlc-run", "aisdlc-domain"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// **A target table does not make a legal edge illegal, and does not make an illegal one
/// legal.** The outside may name the domain, and where it writes that down is not what
/// decides it: this is the table the reading used to walk past, and the edge it hid was
/// legal, so reading it is a question of what is read rather than of what is refused.
#[test]
fn the_outside_naming_the_domain_under_a_target_table_is_accepted() {
    let declared =
        a_crate_naming_each_under_a_target_table(&[("aisdlc-outside", &["aisdlc-domain"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The three crates a later phase gives the outside, and **not this gate's business**:
/// whether a crate from outside is welcome is `every_third_party_crate_is_on_its_crate_allowlist`.
#[test]
fn the_outside_naming_serde_and_reqwest_and_rusqlite_is_accepted() {
    let declared = a_crate_naming_each_from_a_registry(&[(
        "aisdlc-outside",
        &["serde", "reqwest", "rusqlite"],
    )]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// A runtime, which is the crate this repository has the most reason to want a rule about
/// and the crate this gate is the wrong place to say anything about: the allowlist is, and
/// a rule here would refuse it in `aisdlc-run` and in `aisdlc-outside` alike.
#[test]
fn the_outside_naming_tokio_is_accepted() {
    let declared = a_crate_naming_each_from_a_registry(&[("aisdlc-outside", &["tokio"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// A crate on a registry that happens to be spelled like one of ours is not one of ours, and
/// this is the fixture that says so where `the_outside_naming_the_desk_is_refused` says the
/// opposite. The two differ in one fact — whether the manifest reached it by a path — and a
/// rule that went by the name would have to pick one of them.
#[test]
fn a_name_that_only_looks_like_a_crate_of_this_workspace_is_accepted() {
    let declared = a_crate_naming_each_from_a_registry(&[("aisdlc-outside", &["aisdlc-desk"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is the desk naming the outside, and it is accepted **because the rule does not
/// name the desk yet.** This is the difference between a rule and a snapshot, and it is the
/// one assertion here that would fail for a gate written as a list of today's edges.
#[test]
fn the_desk_naming_the_outside_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-desk", &["aisdlc-outside"])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is a crate that exists and depends on nothing, which must not be confused
/// with a crate that was never read. An absent entry and an empty one are different facts.
#[test]
fn a_crate_naming_nothing_at_all_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-run", &[])]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is the other half of "a rule and not a snapshot": every crate the rule names
/// is absent from the disk, and an absent crate is not a failure. The gate has to be able to
/// run on the morning before the first ring is written.
#[test]
fn the_desk_does_not_exist_yet_and_does_not_fail_the_gate() {
    let declared = a_crate_naming_each(&[]);
    let pointing_outward =
        what_points_the_wrong_way(&declared, &what_each_may_name(), &by_a_path_alone());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}
