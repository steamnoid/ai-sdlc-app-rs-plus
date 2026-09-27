//! The gate for RULE 0.3: an attribute that silences a lint does not appear in this
//! repository.
//!
//! `#[allow(…)]`, `#[expect(…)]` and the inner forms of both do not appear in this
//! repository's Rust files. Not `dead_code`, not `clippy::pedantic`, not `unsafe_code`,
//! not anything.
//!
//! **The reason is not purity. It is that the gate command has already decided all of
//! it.** `cargo clippy --workspace --all-targets -- -D warnings` denies every warning
//! this repository's own toolchain can produce, so an `#[allow]` here can only be
//! silencing something that command would have refused. An `#[allow]` is therefore either
//! a bug in the code or a hole in this rule, and those are one finding: a place where the
//! code and the rules have stopped agreeing and nobody decided anything.
//!
//! `#[expect(…)]` is refused for the same reason and not out of politeness. It is the same
//! escape under a name that reads like a promise, and a gate that caught one of the two
//! and missed the other would be a gate a reader trusts wrongly.
//!
//! ## Where this came from
//!
//! The reading helpers in `src/scanning.rs` are `pub` rather than copied into each gate,
//! and they are `pub` for this reason and not for tidiness: a private helper in a test
//! binary is reported dead the moment the test that does not use it is compiled, and
//! silencing that is `#[allow(dead_code)]`. The choice was never duplication or silence.
//! It was duplication, or one public place, and the one public place is what let this
//! rule stay absolute.
//!
//! ## Why this is not a search for the characters
//!
//! This file holds `#[allow(…)]` inside string literals, because a fixture has to carry
//! the very thing it is a fixture of. A search for the characters would refuse this file,
//! and a gate that is wrong about the file that defines it is a gate whose next finding is
//! not read. So the source is taken apart by `HowTheFileIsWritten` and only the stretches
//! of code are judged: a string is a value holding text, a comment is a sentence beside
//! the code, and an attribute is code.
//!
//! ## What a refusal says
//!
//! The file, the line, and the whole attribute exactly as the file wrote it — including
//! the lines it was written over, because an attribute spread over two lines is one
//! attribute, and quoting half of it hides the half that matters.

use std::fmt::Write as _;
use std::path::Path;

use aisdlc_gates::scanning::{self, HowTheFileIsWritten, WhatIsWrittenThere};

/// One attribute that silences a lint: the line it opens on, and the attribute as the file
/// wrote it.
///
/// The two are kept together because a refusal needs both and neither is enough. The line
/// is what a reader opens the file at, and the text is what they are looking for when they
/// get there; a refusal carrying the line alone makes the reader search for it.
#[derive(Debug)]
struct ASuppression {
    /// The line the attribute opens on, counted from 1.
    the_line_it_starts_on: usize,
    /// The attribute exactly as the file has it, over however many lines it takes.
    the_attribute_as_written: String,
}

/// Every suppression in `source`, in the order the file wrote them.
///
/// It is empty on purpose: a reader that has not been told what a suppression is cannot
/// be asked to find one, and a reader that finds them all without being told has learned
/// the rule from the reader rather than from the specification. Every refusal above is
/// this list, and this list is what has to change.
fn the_suppressions_in(_source: &str) -> Vec<ASuppression> {
    Vec::new()
}

/// How many attributes `source` writes in its own code, counted rather than judged.
///
/// **This is the number that says the reading reached an attribute at all, and it is
/// load-bearing because there is not one suppression in this repository — that is the
/// rule.** A walk that had stopped reading attributes would refuse nothing, would be
/// green, and would agree with every fixture above for the wrong reason. A gate that can
/// only be seen to work when it fails is a gate that has never been seen to work, so the
/// count is asked of the same stretches the refusals are read out of.
fn how_many_attributes_are_in(source: &str) -> usize {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .filter(|stretch| {
            stretch.kind() == WhatIsWrittenThere::Code && stretch.text().starts_with("#[")
        })
        .count()
}

/// How many attributes this repository's own code writes, over every file the walk found.
fn how_many_attributes_the_workspace_holds() -> usize {
    scanning::every_rust_file_in_the_workspace()
        .iter()
        .map(|path| how_many_attributes_are_in(&scanning::the_text_at(path)))
        .sum()
}

/// What one file's refusals say: the file, the line, and the attribute, one attribute to a
/// line.
///
/// Nothing at all when there is nothing to refuse, and that is the whole report rather
/// than a line saying everything is fine — the count of what a gate refused is what a
/// reader needs, and a gate that says nothing on a breach and nothing on a pass cannot be
/// told apart from a gate that read nothing at all.
fn what_a_refused_suppression_says(the_file: &Path, refused: &[ASuppression]) -> String {
    if refused.is_empty() {
        return String::new();
    }
    let mut said = String::new();
    for one_refused in refused {
        writeln!(
            said,
            "{}:{}: {}",
            the_file.display(),
            one_refused.the_line_it_starts_on,
            one_refused.the_attribute_as_written
        )
        .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

/// Everything this repository has to answer for: one line per suppression, over every file
/// the walk found.
///
/// **The walk reads `.rs` files and does not widen, and the reason is worth reading before
/// somebody widens it.** A `Cargo.toml` carries a `[lints]` table, and the root one here
/// sets `module_name_repetitions = "allow"` and four more beside it. Those are not this
/// gate's business: a lint configured in a manifest is a setting for a whole crate, read
/// by clippy at build time, and it silences nothing in a source file. This gate is about
/// what a file says to the compiler, and a manifest is not a file the compiler reads in
/// that way. Widening the walk to read manifests would make this gate refuse the
/// repository's own lint configuration — the configuration that exists because
/// `-D warnings` is real and a handful of pedantic lints have to be turned off somewhere.
fn what_the_workspace_would_have_to_answer_for() -> String {
    let mut said = String::new();
    for path in scanning::every_rust_file_in_the_workspace() {
        let refused = the_suppressions_in(&scanning::the_text_at(&path));
        said.push_str(&what_a_refused_suppression_says(&path, &refused));
    }
    said
}

/// The one suppression this gate refuses in `source`, and a sentence naming the rule when
/// there is not exactly one.
///
/// A test that indexed into a list the reading had left empty would fail with a number
/// rather than with the rule it is here to protect, and a test that passed over two
/// refusals would have proved the one thing this gate is about and not the other.
fn the_one_suppression_refused_in(source: &str, because: &str) -> ASuppression {
    let refused = the_suppressions_in(source);

    assert_eq!(
        refused.len(),
        1,
        "{because}, and this is what it refused: {refused:?}"
    );
    refused.into_iter().next().expect("the count above is one")
}

/// Assert that the reading of `source` saw the one attribute it was given and refused no
/// suppression, and say why when it did not.
///
/// The two questions are asked together because either alone is satisfied by a broken
/// reading: a walk that saw no attribute at all refuses no suppression, and a walk that
/// called every attribute a suppression refuses this repository's own gates. `because`
/// names which of the two a failure was.
fn the_reading_saw_one_attribute_and_refused_nothing(source: &str, because: &str) {
    let attributes_saw = how_many_attributes_are_in(source);
    let refused = the_suppressions_in(source);

    assert_eq!(
        attributes_saw, 1,
        "{because}, and the reading saw {attributes_saw} attributes in it"
    );
    assert!(
        refused.is_empty(),
        "{because}, and it refused: {refused:?}"
    );
}

#[test]
fn an_allow_of_dead_code_on_a_function_is_refused() {
    let source = "#[allow(dead_code)]\nfn a_function_nobody_calls() {}\n";
    let refused = the_one_suppression_refused_in(source, "this is the plain case");

    assert_eq!(
        refused.the_line_it_starts_on,
        1,
        "the attribute is the first line of the file"
    );
    assert_eq!(
        refused.the_attribute_as_written,
        "#[allow(dead_code)]",
        "and it is quoted whole, exactly as the file wrote it"
    );
}

#[test]
fn an_allow_of_a_clippy_lint_on_a_type_is_refused() {
    let source = "#[allow(clippy::too_many_arguments)]\nstruct ATypeWithSevenFields { one: u8 }\n";
    let refused =
        the_one_suppression_refused_in(source, "a lint of the pedantic group is the same escape");

    assert_eq!(
        refused.the_attribute_as_written,
        "#[allow(clippy::too_many_arguments)]",
        "the lint's own path is inside the brackets and is quoted along with them"
    );
}

#[test]
fn an_inner_allow_at_the_top_of_a_file_is_refused() {
    let source = "#![allow(unused_imports)]\n\nuse a_module::a_name;\n";
    let refused = the_one_suppression_refused_in(
        source,
        "the inner form covers every line of the file, and it is the stronger and rarer of the two",
    );

    assert_eq!(
        refused.the_line_it_starts_on,
        1,
        "an inner attribute stands at the top of a file and nowhere else in it"
    );
    assert_eq!(
        refused.the_attribute_as_written,
        "#![allow(unused_imports)]",
        "the exclamation mark is part of the attribute rather than a decoration on it"
    );
}

#[test]
fn an_allow_of_unsafe_code_is_refused() {
    let source = "#[allow(unsafe_code)]\nfn reaching_beyond_what_rust_allows() {}\n";
    let refused = the_one_suppression_refused_in(
        source,
        "the lint this repository sets to `forbid` is the one a person is most tempted to argue with",
    );

    assert_eq!(
        refused.the_attribute_as_written,
        "#[allow(unsafe_code)]",
        "and the rule has no exception, so neither does the reading"
    );
}

#[test]
fn an_expect_of_dead_code_is_refused() {
    let source = "#[expect(dead_code)]\nfn a_function_nobody_calls() {}\n";
    let refused = the_one_suppression_refused_in(
        source,
        "the same escape under a name that reads like a promise",
    );

    assert_eq!(
        refused.the_attribute_as_written,
        "#[expect(dead_code)]",
        "a gate that caught `allow` and missed this would be a gate a reader trusts wrongly"
    );
}

#[test]
fn an_allow_written_over_two_lines_is_refused() {
    let source = "#[allow(clippy::pedantic,\n    )]\nfn a_function_with_many_arguments() {}\n";
    let refused = the_one_suppression_refused_in(
        source,
        "an attribute written over two lines is one attribute and not two halves",
    );

    assert_eq!(
        refused.the_line_it_starts_on,
        1,
        "and it is the line it opens on that is reported, because that is where it opens"
    );
    assert_eq!(
        refused.the_attribute_as_written,
        "#[allow(clippy::pedantic,\n    )]",
        "and it is quoted whole, with the line it was written over and that line's indentation"
    );
}

#[test]
fn a_refusal_names_the_file_the_line_and_the_attribute() {
    let the_file = Path::new("crates/aisdlc-domain/src/aisdlc_domain/a_stage.rs");
    let source = "#[allow(clippy::pedantic,\n    )]\n";
    let refused = the_one_suppression_refused_in(
        source,
        "the same two lines, refused for the same reason, read by the report instead of by the walk",
    );

    let said = what_a_refused_suppression_says(the_file, &[refused]);

    assert!(
        said.starts_with("crates/aisdlc-domain/src/aisdlc_domain/a_stage.rs:1: #[allow("),
        "a refusal a reader cannot open is a refusal nobody can act on, and this carries the file and the line: {said}"
    );
    assert!(
        said.contains("clippy::pedantic,\n    )]"),
        "and the attribute is quoted whole rather than the first line of it: {said}"
    );
}

#[test]
fn an_allow_written_inside_a_string_is_accepted() {
    let source = "let written = \"#[allow(dead_code)]\";\n#[derive(Debug)]\nstruct AThing;\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a string is a value holding text, and the attribute on the next line is what says the reading looked past it",
    );
}

#[test]
fn a_source_with_no_attribute_at_all_is_accepted() {
    let source = "fn nothing_to_say() {}\n";

    assert_eq!(
        how_many_attributes_are_in(source),
        0,
        "the fixture is a whole file with no attribute in it, so a reading that saw one would be reading something the file does not have"
    );
    assert!(
        the_suppressions_in(source).is_empty(),
        "and a file that attributes nothing is not a file this gate objects to"
    );
}

#[test]
fn a_derive_is_accepted() {
    let source = "#[derive(Debug, Clone, PartialEq, Eq)]\nstruct AWorkItem;\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a derive is how this repository writes its own types, and it is what the count above is counting",
    );
}

#[test]
fn a_cfg_is_accepted() {
    let source = "#[cfg(test)]\nfn a_doubled_assertion() {}\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a `cfg` decides whether a thing is compiled and silences nothing",
    );
}

#[test]
fn a_test_is_accepted() {
    let source = "#[test]\nfn a_rule_is_kept() {}\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a test is how every rule in this repository is written, and it silences nothing",
    );
}

#[test]
fn a_path_is_accepted() {
    let source = "#[path = \"a_reading.rs\"]\nmod a_reading;\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a path names a file, and a name is not a silence",
    );
}

#[test]
fn a_must_use_is_accepted() {
    let source = "#[must_use]\nfn a_reading() -> usize {}\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a `must_use` asks a reader to use what a function returns, and it silences nothing",
    );
}

#[test]
fn a_serde_is_accepted() {
    let source = "#[serde(rename = \"a_work_item\")]\nstruct AWorkItem;\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a `serde` names what a field is called on the wire, and it silences nothing",
    );
}

#[test]
fn a_comment_naming_the_attribute_is_accepted() {
    let source = "//! why #[allow(dead_code)] is not written here\n#[derive(Debug)]\nstruct AThing;\n";

    the_reading_saw_one_attribute_and_refused_nothing(
        source,
        "a comment is a sentence beside the code, and the attribute under it is what says the reading reached the code",
    );
}

/// The real gate, and the only assertion here that reads the repository rather than a
/// fixture. Every one above is a demonstration; this one is the rule, and it is the one
/// worth keeping if the rest were deleted.
#[test]
fn no_allow_silences_a_lint() {
    let said = what_the_workspace_would_have_to_answer_for();

    assert!(
        said.is_empty(),
        "an attribute that silences a lint is either a bug in the code or a hole in this rule, by file, by line and by attribute:\n{said}"
    );
}

/// **A gate that read no files and agreed with all of them would be green and would be a
/// lie**, and there is no way to tell that apart from a clean repository by looking at what
/// it refused. So the walk is asked what it found before anything is judged: a file under
/// each of the three roots, and an attribute in the code of the files it read.
#[test]
fn the_walk_read_a_file_under_every_root_and_found_an_attribute() {
    let found = scanning::every_rust_file_in_the_workspace();
    for root in ["src", "crates", "tests"] {
        let under = scanning::the_root_of_the_checkout().join(root);
        assert!(
            found.iter().any(|path| path.starts_with(&under)),
            "the walk read no .rs file under {root}/, so it agreed with a directory it never opened"
        );
    }
    assert!(
        how_many_attributes_the_workspace_holds() > 0,
        "the walk reached no attribute in any of them, and this repository is written in attributes, so a reading that had stopped looking would agree with all of it"
    );
}
