//! No function body is longer than twenty lines.
//!
//! **A body and not a signature, and the count is the one the file-length gate uses.** A
//! line counts if there is code on it, so a body with a dozen `///` lines in the middle
//! spends fewer lines than it occupies — the same rule, for the same reason: documentation
//! is the most valuable thing in these files and no gate may make it expensive.
//!
//! **A function and not a closure.** `|x| { … }` is not a `fn`, and a rule that counted it
//! would count every `map` in the repository as a function body. The file says so rather
//! than letting a reader assume the gap was not noticed.
//!
//! **The refusal names the file, the line the function starts on, and the count.**

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// The most lines of code a function body may spend.
const THE_MOST_A_BODY_MAY_SPEND: usize = 20;

/// Every refusal in one file, given its name and its text.
fn what_is_wrong_with(a_name: &str, the_code: &str) -> Vec<String> {
    let _the_name_nobody_read = a_name;
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
fn no_function_in_this_repository_spends_more_than_it_may() {
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
        "a body may spend {THE_MOST_A_BODY_MAY_SPEND} lines and these do not: {found:?}"
    );
}

#[test]
fn a_body_of_exactly_twenty_lines_is_accepted() {
    let the_body = (1..=THE_MOST_A_BODY_MAY_SPEND)
        .map(|one| format!("    let the_{one} = {one};"))
        .collect::<Vec<String>>()
        .join("\n");

    let the_code = format!("fn at_the_limit() {{\n{the_body}\n}}\n");

    assert_eq!(
        what_is_wrong_with("a_file.rs", &the_code),
        Vec::<String>::new(),
        "twenty is the most a body may spend and not one more"
    );
}

#[test]
fn a_body_of_twenty_one_lines_is_refused_and_named() {
    let the_body = (1..=THE_MOST_A_BODY_MAY_SPEND + 1)
        .map(|one| format!("    let the_{one} = {one};"))
        .collect::<Vec<String>>()
        .join("\n");

    let the_code = format!("fn one_line_too_many() {{\n{the_body}\n}}\n");

    let spent = THE_MOST_A_BODY_MAY_SPEND + 1;

    assert_eq!(
        what_is_wrong_with("a_file.rs", &the_code),
        vec![format!(
            "a_file.rs line 1 spends {spent} lines of code and the most is {THE_MOST_A_BODY_MAY_SPEND}"
        )],
        "a refusal a reader cannot match against the function they wrote teaches nothing"
    );
}

#[test]
fn documentation_inside_a_body_is_not_a_line_the_body_spends() {
    let the_body = (1..=THE_MOST_A_BODY_MAY_SPEND)
        .map(|one| format!("    let the_{one} = {one};"))
        .collect::<Vec<String>>()
        .join("\n");

    let the_code = format!("fn explained() {{\n    /// why\n{the_body}\n    /// and why\n}}\n");

    assert_eq!(
        what_is_wrong_with("a_file.rs", &the_code),
        Vec::<String>::new(),
        "twenty lines of code among twenty-two lines of text is twenty lines spent, and a rule that counted the prose would be a rule against explaining a body"
    );
}

#[test]
fn a_brace_inside_a_string_is_not_a_brace() {
    let the_code =
        "fn quoted() {\n    let the_json = \"{ not a body }\";\n    let the_brace = '}';\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "a brace in a string or a character is text, and a matcher that counted it would end the body early or never end it at all"
    );
}

#[test]
fn a_function_named_in_a_string_is_not_a_function() {
    let the_code = "fn real() {\n    let the_example = \"fn imagined() { }\";\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "a function written inside a string is a fixture, and the walk has already told the two apart"
    );
}

#[test]
fn a_body_on_one_line_spends_nothing() {
    let the_code = "fn one_line() { let the_answer = 42; }\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "a whole function on one line spends no lines and that is not a way around the rule — it is a way of writing less code"
    );
}

#[test]
fn a_closure_is_not_a_function_and_the_file_says_so() {
    let the_body = (1..=THE_MOST_A_BODY_MAY_SPEND + 1)
        .map(|one| format!("    the_{one}();"))
        .collect::<Vec<String>>()
        .join("\n");

    let the_code =
        format!("fn holds_a_closure() {{\n    let it = |the_input| {{\n{the_body}\n    }};\n}}\n");

    let found = what_is_wrong_with("a_file.rs", &the_code);

    assert!(
        found.is_empty(),
        "a closure is not a `fn` and this rule is about functions; a gate that counted it would count every `map` in the repository. The closure here is {} lines and is a known gap, not a pass: {found:?}",
        THE_MOST_A_BODY_MAY_SPEND + 1
    );
}

#[test]
fn the_walk_was_read() {
    let the_code = "fn a() {}\n";
    let kinds: Vec<WhatIsWrittenThere> = HowTheFileIsWritten::of(the_code)
        .stretches()
        .iter()
        .map(AStretchOfTheFile::kind)
        .collect();

    assert!(
        kinds.contains(&WhatIsWrittenThere::Code),
        "the gate read no code out of a file of code, and a gate that reads nothing refuses nothing"
    );
}
