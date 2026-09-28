//! No name is built out of a word the glossary refused.
//!
//! **The blessing is asked before the words.** `StateMachineError` carries `State`, which the
//! stage table's `Not` column refuses, and the name is refused by no gate here because the
//! glossary blesses it in full. A gate that asked the words first would have refused the
//! domain's own error type, and a gate that cannot be run against its own author is a gate
//! nobody checked.
//!
//! **Six of the refused words are refused only as a whole name.** `Step`, `check`, `handle`,
//! `Owner`, `Assignee` and `Team` are ordinary English, and a gate that refused them inside
//! a name would refuse this repository for writing prose — `step_back_over_the_white_space`
//! is a real function about whitespace and is refused by no rule here.
//!
//! **Everything is read out of `docs/domain-glossary.md`.** A copy of a list of refused words
//! inside a gate is a second list, and the copy would be green on the day the glossary
//! changed.

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::reading_the_glossary::{
    it_as_letters, the_blessed_names, the_glossary_spelling_of, the_glossary_text,
    the_refused_as_letters, the_words_of, the_words_refused_anywhere,
    the_words_refused_only_as_whole_names,
};
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// Every refusal in one file, given its name and its text.
fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
    let the_glossary = the_glossary_text();
    let mut found = Vec::new();
    for one_name in the_names_the_code_introduces(the_code) {
        found.extend(the_refusal_of(&one_name, &the_glossary, a_file_name));
    }
    found
}

/// The refusal for one name, if the glossary refuses it.
fn the_refusal_of(one_name: &str, the_glossary: &str, a_file_name: &str) -> Vec<String> {
    if the_blessed_names(the_glossary).contains(&it_as_letters(one_name)) {
        return Vec::new();
    }
    if the_words_refused_only_as_whole_names(the_glossary).contains(&it_as_letters(one_name)) {
        return vec![format!(
            "{a_file_name} names {one_name}, which is a refused word on its own"
        )];
    }
    let carried = the_word_it_carries_from(the_glossary, one_name);
    carried
        .into_iter()
        .map(|one| format!("{a_file_name} names {one_name}, which carries a refused word: {one}"))
        .collect()
}

/// The refused word this name carries, if it carries one.
fn the_word_it_carries_from(the_glossary: &str, one_name: &str) -> Option<String> {
    let refused = the_refused_as_letters(the_glossary);
    let whole_names_only = the_words_refused_only_as_whole_names(the_glossary);
    let carried = the_words_of(one_name);
    let found = carried
        .iter()
        .find(|one| refused.contains(one) && !whole_names_only.contains(one))?;
    the_glossary_spelling_of(the_glossary, found)
}

/// The keywords a name can follow, and which is why a name is an introduction.
const THE_KEYWORDS_A_NAME_CAN_FOLLOW: [&str; 6] = ["struct", "enum", "trait", "type", "fn", "mod"];

/// Every name the code introduces, and nothing it merely uses.
fn the_names_the_code_introduces(the_code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for one_line in the_lines_of_code(the_code) {
        found.extend(the_names_immediately_after(&one_line));
    }
    found.retain(|one| !one.is_empty());
    found
}

/// The word after every keyword on a line.
fn the_names_immediately_after(one_line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut parts = one_line.split_whitespace().peekable();
    while let Some(one) = parts.next() {
        let the_name_after = parts.peek().map(|it| (*it).to_string());
        if THE_KEYWORDS_A_NAME_CAN_FOLLOW.contains(&one.trim_end_matches('{')) {
            found.extend(the_name_after);
        }
    }
    found
        .iter()
        .map(|one| one.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .collect()
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
fn no_name_in_this_repository_is_built_out_of_a_refused_word() {
    let mut found = Vec::new();
    for a_file in every_source_file() {
        let name = a_file.display().to_string();
        found.extend(what_is_wrong_with(
            &name,
            &reading_the_checkout::the_text_at(&a_file),
        ));
    }

    assert!(found.is_empty(), "one concept, one name: {found:?}");
}

#[test]
fn a_name_built_out_of_a_refused_word_is_refused_and_named() {
    let the_code = "struct CurrentStatus {\n    one: u8,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec!["a_file.rs names CurrentStatus, which carries a refused word: Status".to_string()],
        "a compound carrying Status is carrying the wrong word for this system's Stage, and the refusal names which one"
    );
}

#[test]
fn a_blessed_name_is_accepted_even_though_it_carries_a_refused_word() {
    let the_code = "struct StateMachineError {\n    why: String,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "the glossary blesses the name in full and a gate asks the blessing before it asks the words — a gate that did otherwise would have refused the domain's own error type on its first line"
    );
}

#[test]
fn a_word_refused_only_alone_is_refused_alone() {
    let the_code = "fn check() {}\nfn step() {}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec![
            "a_file.rs names check, which is a refused word on its own".to_string(),
            "a_file.rs names step, which is a refused word on its own".to_string(),
        ],
        "these six are ordinary English and are refused as whole names only"
    );
}

#[test]
fn a_word_refused_only_alone_is_accepted_inside_a_name() {
    let the_code = "fn step_back_over_the_white_space() {}\nfn the_check_it_made() {}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "one of these is a real function in this repository and is about whitespace, and a gate that refused the word inside a name would have refused this repository for writing prose"
    );
}

#[test]
fn the_glossary_was_read_and_not_assumed() {
    let the_glossary = the_glossary_text();
    let the_refused_anywhere = the_words_refused_anywhere(&the_glossary);
    let the_whole_names_only = the_words_refused_only_as_whole_names(&the_glossary);

    assert!(
        the_refused_anywhere.contains(&"Status".to_string()),
        "the stage table's Not column refuses Status and the guard found it as the glossary spells it, because a refusal quotes the document: {the_refused_anywhere:?}"
    );
    assert!(
        the_whole_names_only.contains(&"step".to_string()),
        "and the block of six names is read from the glossary: {the_whole_names_only:?}"
    );
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
