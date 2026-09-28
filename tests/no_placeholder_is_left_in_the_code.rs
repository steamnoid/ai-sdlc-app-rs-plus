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
use aisdlc_gates::reading_the_glossary::the_block_containing;
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// The macros that say the work is not done, in the code.
const THE_MACROS_THAT_SAY_IT: [&str; 3] = ["todo!", "unimplemented!", "unreachable!"];

/// Every refusal in one file, given its name and its text.
fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
    let the_glossary = the_glossary_text();
    let messages = the_panic_messages_in(the_code);
    let mut found = Vec::new();
    for (the_line, one_line) in the_lines_of_code(the_code) {
        found.extend(the_refusal_of(
            &one_line,
            a_file_name,
            the_line,
            &the_glossary,
            &messages,
        ));
    }
    found
}

/// The refusal for one line, if it leaves something unfinished behind.
fn the_refusal_of(
    one_line: &str,
    a_file_name: &str,
    the_line: usize,
    the_glossary: &str,
    messages: &[(usize, String)],
) -> Vec<String> {
    let mut found = the_macros_this_line_says(one_line, a_file_name, the_line);
    found.extend(the_refused_word_in_a_message(
        a_file_name,
        the_line,
        the_glossary,
        messages,
    ));
    found
}

/// Every `panic!` message in the code, as `(the line it is on, the message)`.
///
/// **Out of the string stretches and not out of the code**, because a walk that tells a string
/// from the code around it hands the two over separately — and the message *is* the string.
/// The first version read the code line and found `panic!(` and nothing else, because
/// `"FIXME this is wrong"` was a stretch of its own that the line never contained.
fn the_panic_messages_in(the_code: &str) -> Vec<(usize, String)> {
    let mut found: Vec<(usize, String)> = Vec::new();
    let mut a_panic_is_open_on: Option<usize> = None;
    for stretch in HowTheFileIsWritten::of(the_code).stretches() {
        let the_line = stretch.line_it_starts_on();
        let is_code = stretch.kind() == WhatIsWrittenThere::Code;
        if is_code && stretch.text().contains("panic!") {
            a_panic_is_open_on = Some(the_line);
            continue;
        }
        if a_panic_is_open_on.is_some() && stretch.kind() == WhatIsWrittenThere::AString {
            found.push((the_line, stretch.text().to_string()));
        }
    }
    found
}

/// The macros on this line that say the work is not done.
fn the_macros_this_line_says(one_line: &str, a_file_name: &str, the_line: usize) -> Vec<String> {
    THE_MACROS_THAT_SAY_IT
        .iter()
        .filter(|one_macro| one_line.contains(*one_macro))
        .map(|one_macro| format!("{a_file_name} line {the_line} is a {one_macro}()"))
        .collect()
}

/// The refused word this line's panic message carries, if it carries one.
///
/// **Only a message and not any string**, because a string that is not a message is a value
/// and a value holding the word is an example, a fixture, or text shown to a person.
fn the_refused_word_in_a_message(
    a_file_name: &str,
    the_line: usize,
    the_glossary: &str,
    messages: &[(usize, String)],
) -> Vec<String> {
    let refused = the_words_the_glossary_refuses(the_glossary);
    let the_message = messages
        .iter()
        .filter(|(on, _)| *on == the_line)
        .map(|(_, one)| one.to_lowercase())
        .collect::<Vec<String>>()
        .join(" ");
    refused
        .iter()
        .filter(|one| the_message.contains(&one.to_lowercase()))
        .map(|one| {
            format!(
                "{a_file_name} line {the_line} is a panic message carrying a refused word: {one}"
            )
        })
        .collect()
}

/// The glossary's own text, read once by every function that needs it.
fn the_glossary_text() -> String {
    reading_the_checkout::the_text_at(
        &reading_the_checkout::the_root_of_the_checkout().join("docs/domain-glossary.md"),
    )
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
fn no_placeholder_is_left_in_the_code() {
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
        the_words_the_glossary_refuses(&the_glossary).contains(&"TODO".to_string()),
        "the block the guard reads is the one the document writes, and a copy of it in the gate would be green the day the document changed"
    );
}

/// The words the glossary refuses, read out of the block it writes them in.
///
/// **Out of the block and not out of a list here**, because a copy in the gate would be green
/// the day the document gained or lost a word — and this is the second time a rule in this
/// repository has been unwritten and the third time a gate has found it.
fn the_words_the_glossary_refuses(the_glossary: &str) -> Vec<String> {
    let mut found: Vec<String> =
        the_block_containing(the_glossary, "rather than the unfinished work")
            .split_whitespace()
            .map(str::to_string)
            .collect();
    found.sort();
    found.dedup();
    found
}

/// Every line of the code that carries code on it, as `(the line, the line)`.
///
/// **Joined, because a line can arrive in pieces.** A string in the middle of it ends the
/// code stretch and starts another on the same line — `panic!("FIXME")` is two code stretches
/// and one string — and a reader that treated each stretch as a line reported one panic
/// message twice. **This is the fifth reader in this repository to make that mistake**, and
/// the joining is now written out in three files because three gates needed it and none of
/// them knew the others had.
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
