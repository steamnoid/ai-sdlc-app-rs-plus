//! No name ends on a word that says nothing about the thing it names.
//!
//! **The list is read out of `docs/domain-glossary.md` and not restated here.** A list of
//! banned words copied into a gate is a second list to keep in step, and the copy would be
//! green on the day the glossary gained a word. The glossary is the source of truth and this
//! gate is the reader.
//!
//! **A name is judged on its last word, and a family as a prefix or a suffix.** The last
//! word of an identifier is what the name claims the thing *is* — `AWorkRepository` claims
//! a repository, and this system already has a word for the one it means. A word that merely
//! contains a banned one is not caught, because `StateMachineError` contains `State` and is
//! neither a state machine nor the stage.
//!
//! **Words the glossary blesses are exempt and the exemption is written down**, so adding a
//! word to the list needs no change in code and blessing one is a visible act.

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::reading_the_glossary::{
    it_as_letters, the_glossary_text, the_last_word_of, the_word_under,
    the_words_the_glossary_refuses,
};
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// Every refusal in one file, given its name and its text.
fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for one_name in the_names_the_code_introduces(the_code) {
        if let Some(word) = the_banned_word_one_name_ends_on(&one_name) {
            found.push(format!(
                "{a_file_name} names {one_name}, which ends on a banned word: {word}"
            ));
        }
    }
    found
}

/// The banned word this name ends on, unless the glossary blesses it here.
fn the_banned_word_one_name_ends_on(one_name: &str) -> Option<String> {
    let refused = the_words_the_glossary_refuses(&the_glossary_text());
    let word = refused
        .iter()
        .find(|one| a_name_ends_on(&it_as_letters(one), one_name))?;
    if the_glossary_blesses_the_same_word_as_a_whole_name(word, one_name) {
        return None;
    }
    if the_glossary_blesses_it_as_a_compounds_ending(word, one_name) {
        return None;
    }
    Some(word.clone())
}

/// Whether the name is exactly the word the glossary blesses on its own.
fn the_glossary_blesses_the_same_word_as_a_whole_name(a_word: &str, one_name: &str) -> bool {
    let the_same_word = it_as_letters(a_word) == it_as_letters(one_name);
    the_glossary_blesses(BLESSED_AS_A_WHOLE_NAME, &it_as_letters(a_word)) && the_same_word
}

/// Whether the name is a compound ending on a word the glossary blesses there.
fn the_glossary_blesses_it_as_a_compounds_ending(a_word: &str, one_name: &str) -> bool {
    let the_same_word = it_as_letters(a_word) == it_as_letters(one_name);
    the_glossary_blesses(BLESSED_AS_A_COMPOUNDS_ENDING, &it_as_letters(a_word)) && !the_same_word
}

/// Whether a name ends on a word, compared as letters and digits alone.
///
/// **On the last word and not on the last characters.** `the_report` ends on the word
/// `report` and not on `Port`, and a comparison that ignored the boundary would have called
/// a function a port in a repository whose ports are named after what they reach.
fn a_name_ends_on(a_word: &str, one_name: &str) -> bool {
    let the_last_word = the_last_word_of(one_name);
    it_as_letters(&the_last_word) == a_word
}

/// Whether the glossary writes a word on one of its two exemption lines.
///
/// **A single word, not a list.** Each line names exactly one, and a reader that returned a
/// list made `contains` a question about the list rather than about the word.
fn the_glossary_blesses(the_heading: &str, a_word: &str) -> bool {
    the_word_under(&the_glossary_text(), the_heading).as_deref() == Some(a_word)
}

/// The words the glossary blesses as a whole name and refuses as a compound's ending.
const BLESSED_AS_A_WHOLE_NAME: &str = "Blessed as a whole name";

/// The words the glossary refuses alone and blesses as a compound's ending.
const BLESSED_AS_A_COMPOUNDS_ENDING: &str = "Refused alone, blessed as a compound's ending";

/// The keywords a name can follow, and which is why a name is an introduction.
const THE_KEYWORDS_A_NAME_CAN_FOLLOW: [&str; 6] = ["struct", "enum", "trait", "type", "fn", "mod"];

/// Every name the code introduces, and nothing it merely uses.
///
/// **The difference is the whole of this gate.** `Result` is on the glossary's list and it is
/// also the type every fallible function in Rust returns; a rule that judged uses would
/// refuse the whole crate on its first line. A name a *use* does not introduce is somebody
/// else's name, and this repository does not get to have opinions about `std`.
fn the_names_the_code_introduces(the_code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for one_line in the_lines_of_code(the_code) {
        found.extend(the_names_after_each_keyword(&one_line));
    }
    found.retain(|one| !one.is_empty());
    found
}

/// The keyword that may be followed by a name, or nothing.
fn the_keyword_a_line_holds(one_line: &str) -> Option<&str> {
    for keyword in THE_KEYWORDS_A_NAME_CAN_FOLLOW {
        let is_there = one_line
            .split_whitespace()
            .any(|one| one.trim_end_matches('{') == keyword);
        if is_there {
            return Some(keyword);
        }
    }
    None
}

/// The names a line introduces, as the word after each keyword it holds.
fn the_names_after_each_keyword(one_line: &str) -> Vec<String> {
    let Some(_keyword) = the_keyword_a_line_holds(one_line) else {
        return Vec::new();
    };
    let mut found = Vec::new();
    let mut parts = one_line.split_whitespace().peekable();
    while let Some(one) = parts.next() {
        let is_a_keyword = THE_KEYWORDS_A_NAME_CAN_FOLLOW.contains(&one.trim_end_matches('{'));
        let the_name_after = parts.peek().map(|it| (*it).to_string());
        if is_a_keyword {
            found.extend(the_name_after);
        }
    }
    found
        .iter()
        .map(|one| one.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .collect()
}

/// Every line of the code that carries code on it.
fn the_lines_of_code(the_code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for stretch in HowTheFileIsWritten::of(the_code).stretches() {
        append_the_code_lines_of(stretch, &mut found);
    }
    found
}

/// Add one stretch's lines, skipping the ones with nothing on them.
fn append_the_code_lines_of(a_stretch: &AStretchOfTheFile, found: &mut Vec<String>) {
    let is_code = a_stretch.kind() == WhatIsWrittenThere::Code;
    if !is_code {
        return;
    }
    for one_line in a_stretch.text().split('\n') {
        if one_line.trim().is_empty() {
            continue;
        }
        found.push(one_line.to_string());
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
fn no_name_in_this_repository_ends_on_a_word_that_says_nothing() {
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
        "a name has to claim what the thing is: {found:?}"
    );
}

#[test]
fn a_name_ending_on_a_banned_word_is_refused_and_named() {
    let the_code = "struct WorkRepository {\n    the_thing: u8,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec!["a_file.rs names WorkRepository, which ends on a banned word: Repository".to_string()],
        "and the refusal names the word, because 'a name is wrong' is not something a reader can act on"
    );
}

#[test]
fn the_word_the_glossary_blesses_is_accepted_on_its_own() {
    let the_code = "struct Repository {\n    owner: String,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "this system's Repository is the public GitHub repository the glossary blesses, and it is not the same thing as a persistence abstraction at all"
    );
}

#[test]
fn a_name_merely_containing_a_banned_word_is_accepted() {
    let the_code =
        "struct StateMachineError {\n    why: String,\n}\nstruct BaseThing {\n    one: u8,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "`StateMachineError` contains `State` and ends on `Error`; a rule that matched anywhere would have refused the domain's own error type"
    );
}

#[test]
fn a_snake_case_name_is_judged_by_its_letters_and_not_its_capitals() {
    let the_code = "struct a_work_repository {\n    one: u8,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec![
            "a_file.rs names a_work_repository, which ends on a banned word: Repository"
                .to_string()
        ],
        "one name, two spellings, one judgement: the guard normalises a name to letters and digits, so the rule is not a rule about capitals"
    );
}
