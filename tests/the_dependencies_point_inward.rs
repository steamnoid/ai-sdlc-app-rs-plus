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
//! ## What this gate does not see
//!
//! A dependency written under a target table, `[target.'cfg(unix)'.dependencies]`, is not
//! read. That is a limit and not an oversight, and it is written here rather than in a
//! commit message because a reader of a gate has to be able to say what it will not catch.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// What a set of crates declares: every crate beside every dependency that crate names,
/// keyed by the crate's own name as its manifest spells it.
type WhatEachCrateDeclares = BTreeMap<String, BTreeSet<String>>;

/// What each crate is allowed to name, keyed the same way. One entry per crate whose
/// direction this repository has decided, and a crate with no entry here is not judged.
type WhatEachCrateMayName = BTreeMap<String, BTreeSet<String>>;

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

/// Every edge in the workspace that points outward, one message per edge.
///
/// Both arguments are read and neither is assumed: `declared` is what the manifests on the
/// disk say and `allowed` is the rule. The walk is over the rule rather than over the
/// manifests, and that is the whole design in one line — a crate the rule does not name is
/// never reached, and neither is a crate the rule names that is not on the disk yet.
fn what_points_the_wrong_way(
    declared: &WhatEachCrateDeclares,
    allowed: &WhatEachCrateMayName,
) -> Vec<String> {
    let mut pointing_outward = vec![];
    for (crate_name, its_allowed) in allowed {
        let Some(its_own) = declared.get(crate_name) else {
            continue;
        };
        pointing_outward.extend(the_edges_that_leave(crate_name, its_own, its_allowed));
    }
    pointing_outward
}

/// The edges one crate's own dependencies include that it is not allowed to name.
///
/// A crate nobody has is not a crate that depends on anything, so a crate the rule names
/// and the disk does not have contributes no edges and no verdict; the walk that calls this
/// skips it before it gets here. A third-party crate is judged exactly as a workspace crate
/// is, because an allowlist of welcome third-party crates is a separate gate rather than a
/// hole in this one.
fn the_edges_that_leave(
    crate_name: &str,
    its_own: &BTreeSet<String>,
    its_allowed: &BTreeSet<String>,
) -> Vec<String> {
    its_own
        .iter()
        .filter(|dependency| !its_allowed.contains(*dependency))
        .map(|dependency| format!("{crate_name} names {dependency}"))
        .collect()
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

/// What every crate in the workspace declares, keyed by the name in its own manifest.
///
/// A manifest that cannot be read panics rather than contributing nothing, because a crate
/// this gate failed to open would be a crate this gate agreed with, and a guard that agrees
/// with whatever it could not see is worse than no guard at all.
fn the_dependencies_every_crate_declares() -> WhatEachCrateDeclares {
    let mut declared = WhatEachCrateDeclares::new();
    for directory in the_crate_directories() {
        let manifest = directory.join("Cargo.toml");
        let text = std::fs::read_to_string(&manifest).expect("a crate has a readable manifest");
        declared.insert(the_name_of_the_crate(&text), the_dependencies_in(&text));
    }
    declared
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

/// The dependencies one manifest declares, read from `[dependencies]` and from
/// `[dev-dependencies]`.
///
/// Both spellings of a dependency count, because both are ways to depend on something: a
/// `name` and a value on a line of its own, and a table header that writes the dependency in
/// the header. Reading only the first would pass a crate that depends on anything as a
/// crate that depends on nothing, and a gate that cannot be seen to fail is a gate nobody
/// has exercised.
fn the_dependencies_in(manifest: &str) -> BTreeSet<String> {
    let mut declared = BTreeSet::new();
    let mut inside = false;
    for line in manifest.lines() {
        if let Some(section) = the_name_of_the_section(line) {
            inside = is_a_section_we_read(section);
            declared.extend(the_dependency_in_a_table_header(section));
        } else if inside {
            declared.extend(the_dependency_on_a_line(line));
        }
    }
    declared
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

/// Whether a section is one this gate reads, and no other.
///
/// The two are named here and nowhere else, so there is one answer to what was parsed. A
/// dependency under a target table is not one of them, and that limit is the reason this is
/// a two-line function: a reader can see the whole of what is read.
fn is_a_section_we_read(section: &str) -> bool {
    section == "dependencies" || section == "dev-dependencies"
}

/// The dependency a table header names in its own text, which is the one form of
/// `[dependencies]` that writes no line of its own for a reader to find.
fn the_dependency_in_a_table_header(section: &str) -> Option<String> {
    for prefix in ["dependencies.", "dev-dependencies."] {
        if let Some(named) = section.strip_prefix(prefix) {
            return Some(named.to_string());
        }
    }
    None
}

/// The name before the first `=`, which is where a manifest writes a dependency's name.
///
/// A comment and an empty line name nothing, and both are ordinary in a manifest. A parse
/// that counted a comment as an edge would report a violation naming a word out of a
/// sentence, which is a gate that cries wolf and is therefore a gate nobody believes.
fn the_dependency_on_a_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with('#') {
        return None;
    }
    let (name, _) = trimmed.split_once('=')?;
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    Some(name.to_string())
}

/// The name a manifest gives its own crate, because a directory's name is a guess and the
/// manifest is the answer: `crates/run` is `aisdlc-run`, and the rule is written in the
/// second spelling.
fn the_name_of_the_crate(manifest: &str) -> String {
    for line in manifest.lines() {
        if line.starts_with("name") && line.contains('=') {
            return the_value_after_the_first_equals(line);
        }
    }
    panic!("every crate manifest names its crate");
}

/// The text after the first `=`, without the quotes a manifest puts around it.
fn the_value_after_the_first_equals(line: &str) -> String {
    let (_, value) = line
        .split_once('=')
        .expect("the caller checked for an equals sign");
    value.trim().trim_matches('"').to_string()
}

/// A workspace of crates written the way this gate reads one, so that a fixture says what
/// it is rather than how to spell a map.
fn a_crate_naming_each(crates: &[(&str, &[&str])]) -> WhatEachCrateDeclares {
    let mut declared = WhatEachCrateDeclares::new();
    for (crate_name, its_own) in crates {
        declared.insert(
            crate_name.to_string(),
            its_own.iter().copied().map(String::from).collect(),
        );
    }
    declared
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
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
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
        declared["aisdlc-run"].contains("aisdlc-domain"),
        "aisdlc-run was read as naming nothing"
    );
}

/// The fixture is one run depending on the outside, which is the edge the whole rule
/// exists to refuse.
#[test]
fn a_run_naming_the_outside_is_refused() {
    let declared = a_crate_naming_each(&[("aisdlc-run", &["aisdlc-outside"])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert_eq!(pointing_outward, vec!["aisdlc-run names aisdlc-outside"]);
}

/// The fixture is the domain depending on the run. The direction of that one edge is what
/// every other boundary in this repository rests on, so it is the edge with the fewest
/// excuses.
#[test]
fn a_domain_naming_the_run_is_refused() {
    let declared = a_crate_naming_each(&[("aisdlc-domain", &["aisdlc-run"])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert_eq!(pointing_outward, vec!["aisdlc-domain names aisdlc-run"]);
}

/// The fixture is the outside naming a crate that is not one of ours, and it is refused by
/// this gate and not by the allowlist gate: an allowlist says which third-party crates are
/// welcome, and this one says a crate may name nothing at all outside its own direction.
#[test]
fn the_outside_naming_a_third_party_crate_is_refused() {
    let declared = a_crate_naming_each(&[("aisdlc-outside", &["tokio"])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert_eq!(pointing_outward, vec!["aisdlc-outside names tokio"]);
}

/// The fixture is the outside naming a crate that does not exist yet, in a ring inside it.
/// A crate nobody has written is still not a dependency the outside is allowed.
#[test]
fn the_outside_naming_the_desk_is_refused() {
    let declared = a_crate_naming_each(&[("aisdlc-outside", &["aisdlc-desk"])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert_eq!(pointing_outward, vec!["aisdlc-outside names aisdlc-desk"]);
}

/// The fixture is the run naming the domain, which is the one edge that is always legal and
/// so is the edge a broken gate would most likely refuse.
#[test]
fn a_run_naming_the_domain_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-run", &["aisdlc-domain"])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is the outside naming both of the rings inside it, which is the whole of what
/// the outside is allowed.
#[test]
fn the_outside_naming_the_run_and_the_domain_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-outside", &["aisdlc-run", "aisdlc-domain"])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is the desk naming the outside, and it is accepted **because the rule does not
/// name the desk yet.** This is the difference between a rule and a snapshot, and it is the
/// one assertion here that would fail for a gate written as a list of today's edges.
#[test]
fn the_desk_naming_the_outside_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-desk", &["aisdlc-outside"])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is a crate that exists and depends on nothing, which must not be confused
/// with a crate that was never read. An absent entry and an empty one are different facts.
#[test]
fn a_crate_naming_nothing_at_all_is_accepted() {
    let declared = a_crate_naming_each(&[("aisdlc-run", &[])]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}

/// The fixture is the other half of "a rule and not a snapshot": every crate the rule names
/// is absent from the disk, and an absent crate is not a failure. The gate has to be able to
/// run on the morning before the first ring is written.
#[test]
fn the_desk_does_not_exist_yet_and_does_not_fail_the_gate() {
    let declared = a_crate_naming_each(&[]);
    let pointing_outward = what_points_the_wrong_way(&declared, &what_each_may_name());
    assert!(pointing_outward.is_empty(), "{pointing_outward:?}");
}
