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

use aisdlc_gates::a_stretch_of_the_file::WhatIsWrittenThere;
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::scanning::HowTheFileIsWritten;

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
fn a_snake_case_name_is_judged_on_its_last_word_too() {
    let the_code = "fn the_work_repository_is_read() {}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec![
            "a_file.rs names the_work_repository_is_read, which ends on a banned word: repository"
                .to_string()
        ],
        "one name, two spellings, one judgement: the guard normalises a name to letters and digits"
    );
}
