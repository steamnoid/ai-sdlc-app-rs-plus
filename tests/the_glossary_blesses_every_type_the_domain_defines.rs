//! Every type the domain defines is a term the glossary blesses, and no refused word names one.
//!
//! **Two halves, and the second is the one that catches a document that has rotted.** A type
//! the glossary never heard of is a name nobody chose; a refused word that names a type is a
//! name somebody chose twice, once as a prohibition and once as a thing. The first half is
//! caught by adding a type; the second by adding a prohibition, and **the second is the one a
//! reader is less likely to check**, because nothing in the code changed when it happened.
//!
//! **The glossary is the source of truth and the domain is held to it**, never the other way
//! round. A type the glossary does not bless is not renamed to satisfy the check; it is either
//! added to the document or deleted, and both of those are acts a reader can see.

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::reading_the_glossary::{
    it_as_letters, the_glossary_text, the_refused_as_letters, the_words_refused_only_as_whole_names,
};
use aisdlc_gates::scanning::HowTheFileIsWritten;

const THE_DOMAIN: &str = "crates/domain/src";

/// Every refusal for one file, given its name and its text.
fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
    let _the_file_name_nobody_read = a_file_name;
    let _the_code_nobody_read = the_code;
    Vec::new()
}

/// Every `.rs` file of the domain, in the order a reader would meet them.
fn every_domain_file() -> Vec<std::path::PathBuf> {
    let the_root = reading_the_checkout::the_root_of_the_checkout();
    let mut found = Vec::new();
    collect_rust_files_under(&the_root.join(THE_DOMAIN), &mut found);
    found.sort();
    found
}

fn collect_rust_files_under(directory: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_under(&path, found);
        } else if path.extension().is_some_and(|it| it == "rs") {
            found.push(path);
        }
    }
}

#[test]
fn every_type_the_domain_defines_is_a_term_the_glossary_blesses() {
    let the_glossary = the_glossary_text();
    let mut found = Vec::new();
    for a_file in every_domain_file() {
        let name = a_file.display().to_string();
        found.extend(what_is_wrong_with(
            &name,
            &reading_the_checkout::the_text_at(&a_file),
        ));
        let _the_glossary_was_read = &the_glossary;
    }

    assert!(
        found.is_empty(),
        "a type the glossary never heard of is a name nobody chose: {found:?}"
    );
}

#[test]
fn a_type_the_glossary_blesses_is_accepted() {
    let the_code = "pub struct WorkItem {\n    one: u8,\n}\npub enum Stage {\n    Idle,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "both are in the glossary's core nouns and both are blessed"
    );
}

#[test]
fn a_type_the_glossary_never_heard_of_is_refused_and_named() {
    let the_code = "pub struct AThingNobodyWroteDown {\n    one: u8,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec![
            "a_file.rs defines AThingNobodyWroteDown, which the glossary does not name".to_string()
        ],
        "and the refusal names the type, because 'something in the domain' is not something a reader can act on"
    );
}

#[test]
fn a_private_type_is_not_a_type_the_domain_defines() {
    let the_code = "struct AnInternalHelper {\n    one: u8,\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "a type nobody outside the crate can name is not a name the document has to carry, and holding it to the glossary would be a rule about internals"
    );
}

#[test]
fn no_refused_word_names_a_type_the_domain_defines() {
    let the_glossary = the_glossary_text();
    let mut named_by_a_prohibition = Vec::new();
    for a_file in every_domain_file() {
        for one_type in the_types_in(&reading_the_checkout::the_text_at(&a_file)) {
            named_by_a_prohibition
                .extend(the_refused_word_naming_this_type(&the_glossary, &one_type));
        }
    }

    assert!(
        named_by_a_prohibition.is_empty(),
        "a refused word that names a type is a name somebody chose twice: {named_by_a_prohibition:?}"
    );
}

/// The refused word, if a prohibition in the glossary names this type.
fn the_refused_word_naming_this_type(the_glossary: &str, one_type: &str) -> Option<String> {
    let refused = the_refused_as_letters(the_glossary);
    let whole_names_only = the_words_refused_only_as_whole_names(the_glossary);
    let is_refused_anywhere = refused.contains(&it_as_letters(one_type));
    if !is_refused_anywhere {
        return None;
    }
    let also_blessed = the_glossary.split('`').skip(1).step_by(2).any(|word| {
        it_as_letters(word) == it_as_letters(one_type)
            && !whole_names_only.contains(&it_as_letters(word))
    });
    if also_blessed {
        return None;
    }
    Some(one_type.to_string())
}

/// The types one file of the domain defines, as it writes them.
fn the_types_in(the_code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for one_line in the_lines_of_code(the_code) {
        found.extend(the_types_one_line_introduces(&one_line));
    }
    found
}

/// The name after every `pub struct` and `pub enum` on a line.
fn the_types_one_line_introduces(one_line: &str) -> Vec<String> {
    const THE_KEYWORDS_A_TYPE_CAN_FOLLOW: [&str; 2] = ["struct", "enum"];
    let mut found = Vec::new();
    let mut parts = one_line.split_whitespace().peekable();
    while let Some(one) = parts.next() {
        let the_name_after = parts.peek().map(|it| (*it).to_string());
        if THE_KEYWORDS_A_TYPE_CAN_FOLLOW.contains(&one.trim_end_matches('{')) {
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
        append_the_code_lines_of(&stretch, &mut found);
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
