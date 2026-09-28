//! No source file carries an `#[allow(…)]` or an `#[expect(…)]`.
//!
//! **`cargo clippy -- -D warnings` already denies every warning, so an `#[allow]` here can
//! only be silencing something the gate command would already have refused.** It is either a
//! bug in the code or a loophole in the rule, and either way it is a place where the two have
//! stopped agreeing without anybody deciding anything.
//!
//! **A `Cargo.toml` `[lints]` table is not one.** It configures clippy for a whole crate
//! rather than standing in a source file, and the root one carries five `allow`s on purpose —
//! `module_name_repetitions` among them — because `WorkItem` has a `work_item_id` and the
//! domain is right about that and clippy is not. Reading it as a suppression would be the
//! same error as reading a `.gitignore` line as a comment.
//!
//! **Only the code is read.** An `#[allow]` written in a doc comment is a sentence about
//! suppressions, and one written inside a string is a value; the walk tells both from an
//! attribute that stands, and this gate asks the walk.

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// The two attributes that silence something, as they are written.
const THE_ATTRIBUTES_THAT_SILENCE: [&str; 2] = ["#[allow(", "#[expect("];

/// Every refusal in one file, given its name and its text.
fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (the_line, one_line) in the_lines_of_code(the_code) {
        found.extend(the_suppressions_in(&one_line, a_file_name, the_line));
    }
    found
}

/// The suppressions on one line, quoted as they are written.
fn the_suppressions_in(one_line: &str, a_file_name: &str, the_line: usize) -> Vec<String> {
    THE_ATTRIBUTES_THAT_SILENCE
        .iter()
        .filter(|one| one_line.contains(*one))
        .map(|one| {
            let as_written = the_attribute_as_written(one_line, one);
            format!("{a_file_name} line {the_line} silences something: {as_written}")
        })
        .collect()
}

/// The whole attribute on a line, closing bracket included.
///
/// **In the code and not out of the text**, because a line holds several attributes and a
/// reader told "something on line 4" cannot tell which one. An unclosed bracket is taken to
/// the end of the line, because a file that does not compile is not a file this gate decides.
fn the_attribute_as_written(one_line: &str, the_attribute: &str) -> String {
    let from_the_attribute = &one_line[one_line.find(the_attribute).unwrap_or(0)..];
    let the_closing = from_the_attribute.find(']').map(|it| it + 1);
    match the_closing {
        Some(where_it_closes) => from_the_attribute[..where_it_closes].to_string(),
        None => from_the_attribute.trim_end().to_string(),
    }
}

/// Every `.rs` file in this checkout, skipping what a build leaves behind.
fn every_source_file() -> Vec<std::path::PathBuf> {
    let mut found = Vec::new();
    collect_from(
        &reading_the_checkout::the_root_of_the_checkout(),
        &mut found,
    );
    found.sort();
    found
}

fn collect_from(directory: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "target" || name == ".git" {
            continue;
        }
        if path.is_dir() {
            collect_from(&path, found);
        } else if path.extension().is_some_and(|it| it == "rs") {
            found.push(path);
        }
    }
}

#[test]
fn no_source_file_in_this_repository_silences_anything() {
    let mut found = Vec::new();
    for a_file in every_source_file() {
        let name = a_file.display().to_string();
        found.extend(what_is_wrong_with(
            &name,
            &reading_the_checkout::the_text_at(&a_file),
        ));
    }

    assert!(
        found.is_empty(),
        "a gate and a loop here have stopped agreeing: {found:?}"
    );
}

#[test]
fn an_allow_is_refused_with_the_line_and_the_attribute_as_written() {
    let the_code =
        "fn a() {\n    #[allow(clippy::too_many_arguments)]\n    let the_answer = 42;\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec![
            "a_file.rs line 2 silences something: #[allow(clippy::too_many_arguments)]".to_string()
        ],
        "and the refusal quotes the attribute exactly as it is written, because a reader who has to go and look is a reader who has to do the gate's work"
    );
}

#[test]
fn an_expect_is_refused_too() {
    let the_code = "fn a() {\n    #[expect(clippy::something)]\n    let the_answer = 42;\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec!["a_file.rs line 2 silences something: #[expect(clippy::something)]".to_string()],
        "an `expect` is a claim that a warning is coming, and a claim the gate cannot check is not a smaller version of the same thing"
    );
}

#[test]
fn an_allow_written_in_documentation_is_accepted() {
    let the_code =
        "//! This repository refuses `#[allow(clippy::foo)]` in a source file.\n\nfn a() {}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "an attribute named in a doc comment is a sentence about suppressions, and the walk already told the two apart"
    );
}

#[test]
fn an_allow_written_inside_a_string_is_accepted() {
    let the_code =
        "fn a() {\n    let the_example = \"#[allow(clippy::foo)]\";\n    the_example\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "inside a string it is a value, and a value that is a fixture is the first line of half the tests in this repository"
    );
}

#[test]
fn the_two_attributes_are_the_ones_written() {
    let the_code = "#[allow(a)]\n#[expect(b)]\n#[derive(Clone)]\n#[test]\nstruct One;\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec![
            "a_file.rs line 1 silences something: #[allow(a)]".to_string(),
            "a_file.rs line 2 silences something: #[expect(b)]".to_string(),
        ],
        "an attribute that does not silence is not a suppression, whatever its brackets look like"
    );
}

/// Every line of the code that carries code on it, as `(the line, the line)`.
///
/// **Joined, because a line can arrive in pieces** when a string sits in the middle of it.
fn the_lines_of_code(the_code: &str) -> Vec<(usize, String)> {
    let mut found: Vec<(usize, String)> = Vec::new();
    for stretch in HowTheFileIsWritten::of(the_code).stretches() {
        append_the_lines_of(stretch, &mut found);
    }
    found.retain(|(_, one_line)| !one_line.trim().is_empty());
    found
}

/// Add one stretch's lines, numbered as the file numbers them.
fn append_the_lines_of(a_stretch: &AStretchOfTheFile, found: &mut Vec<(usize, String)>) {
    let is_code = a_stretch.kind() == WhatIsWrittenThere::Code;
    if !is_code {
        return;
    }
    for (offset, one_line) in a_stretch.text().split('\n').enumerate() {
        let the_line = a_stretch.line_it_starts_on() + offset;
        let Some(last) = found.last_mut() else {
            found.push((the_line, one_line.to_string()));
            continue;
        };
        if last.0 == the_line {
            last.1.push_str(one_line);
            continue;
        }
        found.push((the_line, one_line.to_string()));
    }
}
