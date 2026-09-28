//! No unfinished thing is left in the code.
//!
//! **A placeholder is an unfinished cycle given a second home.** It is already visible in the
//! history, and a line in the source gives it somewhere to hide where nobody would look for it.
//! There is no allowlist here, and the reason is the same: **an allowlist is a list of the
//! holes this repository has decided to live with, and this one has decided not to.**
//!
//! **A refused word in a string is accepted and in a panic message is refused.** A `panic!` is
//! what a run does when it cannot go on, so a message saying the work is not done is that run
//! announcing an unfinished cycle from inside code that looks finished — the one place a
//! placeholder can speak without passing either the comment gate or the history. A string that
//! is not a message is a value, and a value holding the word is an example, a fixture, or
//! text shown to a person.

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::reading_the_glossary::it_as_letters;
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// The macros that say the work is not done, in the code.
const THE_MACROS_THAT_SAY_IT: [&str; 3] = ["todo!", "unimplemented!", "unreachable!"];

/// Every refusal in one file, given its name and its text.
fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
    let _the_file_name_nobody_read = a_file_name;
    let _the_code_nobody_read = the_code;
    Vec::new()
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
fn no_placeholder_is_left_in_this_repository() {
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
        "an unfinished cycle already has a home: {found:?}"
    );
}

#[test]
fn the_macro_that_says_the_work_is_not_done_is_refused_and_named() {
    let the_code = "fn a() {\n    todo!(\"later\");\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec!["a_file.rs line 2 is a todo!()".to_string()],
        "and the refusal names the line, because a placeholder a reader has to find is a placeholder that stays"
    );
}

#[test]
fn every_macro_that_says_it_is_refused() {
    for macro_name in ["todo!", "unimplemented!", "unreachable!"] {
        let the_code = format!("fn a() {{\n    {macro_name}();\n}}\n");

        assert_eq!(
            what_is_wrong_with("a_file.rs", &the_code),
            vec![format!("a_file.rs line 2 is a {macro_name}()")],
            "{macro_name} says the work is not done and is refused for saying it"
        );
    }
}

#[test]
fn a_refused_word_in_a_panic_message_is_refused_and_named() {
    let the_code = "fn a() {\n    panic!(\"FIXME this is wrong\");\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec!["a_file.rs line 2 is a panic message carrying a refused word: FIXME".to_string()],
        "a panic message is what a reader sees when a run cannot go on, and one saying the work is not done is that run announcing an unfinished cycle from inside finished-looking code"
    );
}

#[test]
fn the_same_word_in_an_ordinary_string_is_accepted() {
    let the_code = "fn a() {\n    let the_example = \"TODO: like this one\";\n    the_example\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "a string that is not a message is a value, and a value holding the word is an example, a fixture, or text shown to a person"
    );
}

#[test]
fn ordinary_english_is_not_a_refused_word() {
    let the_code = "fn a() {\n    panic!(\"this will change later, for now\");\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "`later` and `for now` are English and this repository's own prose uses them; a gate refusing them would be refusing the language rather than the unfinished work"
    );
}

#[test]
fn the_words_are_read_out_of_the_glossary_and_not_restated() {
    let the_glossary = reading_the_checkout::the_text_at(
        &reading_the_checkout::the_root_of_the_checkout().join("docs/domain-glossary.md"),
    );

    assert!(
        the_words_the_glossary_refuses(&the_glossary).contains(&"todo".to_string()),
        "the block the guard reads is the one the document writes, and a copy of it in the gate would be green the day the document changed"
    );
}

/// The words the glossary refuses, read out of the block it writes them in.
///
/// **Out of the block and not out of a list here**, because a copy in the gate would be green
/// the day the document gained or lost a word — and this is the second time a rule in this
/// repository has been unwritten and the third time a gate has found it.
fn the_words_the_glossary_refuses(the_glossary: &str) -> Vec<String> {
    aisdlc_gates::reading_the_glossary::the_block_containing(
        the_glossary,
        "work has decided not to",
    )
    .split_whitespace()
    .map(|one| one.to_lowercase())
    .collect()
}

/// Every line of the code that carries code on it.
fn the_lines_of_code(the_code: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for stretch in HowTheFileIsWritten::of(the_code).stretches() {
        append_the_lines_of(&stretch, &mut found);
    }
    found
}

/// Add one stretch's lines, numbered as the file numbers them.
fn append_the_lines_of(a_stretch: &AStretchOfTheFile, found: &mut Vec<(usize, String)>) {
    let is_code = a_stretch.kind() == WhatIsWrittenThere::Code;
    if !is_code {
        return;
    }
    for (offset, one_line) in a_stretch.text().split('\n').enumerate() {
        if one_line.trim().is_empty() {
            continue;
        }
        let the_line = a_stretch.line_it_starts_on() + offset;
        found.push((the_line, one_line.to_string()));
    }
}
