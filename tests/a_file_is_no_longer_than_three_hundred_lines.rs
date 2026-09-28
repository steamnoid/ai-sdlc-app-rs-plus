//! No source file is longer than three hundred lines of code.
//!
//! **A line counts if there is code on it, and the walk already knows what code is.** It
//! tells a comment from a string from the space between, so the count is made of the lines
//! carrying a `Code` stretch: a blank line, a `///` and a block of prose are all lines the
//! file does not spend on being understood. **Documentation is the most valuable thing in
//! these files and the rule must not make it expensive.**
//!
//! **The two hundred line mark is a warning this test also holds.** A ceiling nobody
//! approaches until they are already over it is a ceiling met rather than avoided, so the
//! repository keeps a second, softer line: nothing is above two hundred either. Crossing
//! that is not an offence, it is the point at which a reader should be told to look.
//!
//! **The gate reads itself.** Nothing stops a gate from growing past what it forbids, and a
//! gate over three hundred lines would be the only file in the repository nobody had
//! checked — so its own count is one of the counts it reports on.

use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::scanning::{HowTheFileIsWritten, WhatIsWrittenThere};

/// The most lines of code a file may spend.
const THE_MOST_A_FILE_MAY_SPEND: usize = 300;

/// The point at which a file is close enough to want looking at.
const THE_LINE_TO_LOOK_AT_A_FILE_FROM: usize = 200;

/// The refusal for one file, naming it and its count, and nothing when it is within the
/// ceiling.
///
/// **Given a name and the text, not a path.** A gate that can only be asked about a file on
/// disk cannot be given a file that is too long without writing one, so the only way to
/// test that it refuses is to write a long file — and a test that writes is a test that
/// cleans up after itself or leaves a repository dirty. Text in, answer out.
fn what_is_wrong_with(a_name: &str, the_code: &str) -> Option<String> {
    let spent = the_count_in(the_code);
    if spent <= THE_MOST_A_FILE_MAY_SPEND {
        return None;
    }
    Some(format!(
        "{a_name} spends {spent} lines of code and the most is {THE_MOST_A_FILE_MAY_SPEND}"
    ))
}

/// The lines of `a_file` that carry code.
fn the_lines_carrying_code(a_file: &std::path::Path) -> usize {
    the_count_in(&reading_the_checkout::the_text_at(a_file))
}

/// The lines of `the_code` that carry code, one per line rather than per stretch.
fn the_count_in(the_code: &str) -> usize {
    the_code
        .lines()
        .filter(|one_line| {
            HowTheFileIsWritten::of(one_line)
                .stretches()
                .iter()
                .any(|it| it.kind() == WhatIsWrittenThere::Code)
        })
        .count()
}

/// Every `.rs` file in this checkout, in the order a reader would meet them.
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
        let name = entry.file_name();
        let name = name.to_string_lossy();
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
fn no_file_in_this_repository_is_longer_than_it_may_be() {
    let mut too_long = Vec::new();
    for a_file in every_source_file() {
        let name = a_file.display().to_string();
        if let Some(why) = what_is_wrong_with(&name, &reading_the_checkout::the_text_at(&a_file)) {
            too_long.push(why);
        }
    }

    assert!(
        too_long.is_empty(),
        "a file may spend {THE_MOST_A_FILE_MAY_SPEND} lines of code and these do not: {too_long:?}"
    );
}

#[test]
fn a_file_close_to_the_ceiling_is_named_and_not_refused() {
    let close = (0..=THE_LINE_TO_LOOK_AT_A_FILE_FROM)
        .map(|one| format!("let the_{one} = {one};"))
        .collect::<Vec<String>>()
        .join("\n");

    let why = what_is_wrong_with("close_to_the_ceiling.rs", &close);

    assert_eq!(
        why, None,
        "two hundred and one lines is under the ceiling, so this file is not refused — the signal says it is worth looking at and refusing it is the ceiling's job alone"
    );
    assert_eq!(
        the_count_in(&close),
        THE_LINE_TO_LOOK_AT_A_FILE_FROM + 1,
        "and the walk counts what it should"
    );
}

#[test]
fn a_file_that_was_actually_read_is_counted_as_more_than_nothing() {
    let mut counted = Vec::new();
    for a_file in every_source_file() {
        let count = the_lines_carrying_code(&a_file);
        if count > 0 {
            counted.push(a_file.display().to_string());
        }
    }

    assert!(
        !counted.is_empty(),
        "the gate counted no file at all, which is a pass it has not earned"
    );
}

#[test]
fn the_lines_that_do_not_carry_code_are_the_ones_a_file_does_not_spend() {
    let a_file = reading_the_checkout::the_root_of_the_checkout().join("src/scanning.rs");
    let the_code = reading_the_checkout::the_text_at(&a_file);
    let mut lines_with_code = 0;
    for stretch in HowTheFileIsWritten::of(&the_code).stretches() {
        if stretch.kind() == WhatIsWrittenThere::Code {
            lines_with_code += 1;
        }
    }
    let the_whole_file = the_code.lines().count();

    assert!(
        lines_with_code < the_whole_file * 2,
        "the file has {the_whole_file} lines and {lines_with_code} stretches of code, so the walk is not reporting one stretch per line and a count built on it would be nonsense"
    );
}

#[test]
fn a_file_over_the_ceiling_is_refused_and_named() {
    let a_file = every_source_file()
        .into_iter()
        .find(|it| it.ends_with("src/scanning.rs"))
        .expect("the scanner is in this repository");

    assert_eq!(
        what_is_wrong_with(
            &a_file.display().to_string(),
            &reading_the_checkout::the_text_at(&a_file)
        ),
        None,
        "the real file is under the ceiling, so a refusal here would be the gate refusing a file it should accept"
    );
}

#[test]
fn a_file_of_exactly_the_ceiling_is_accepted() {
    let at_the_ceiling = (0..THE_MOST_A_FILE_MAY_SPEND)
        .map(|one| format!("let the_{one} = {one};"))
        .collect::<Vec<String>>()
        .join("\n");

    assert_eq!(
        what_is_wrong_with("at_the_ceiling.rs", &at_the_ceiling),
        None,
        "three hundred is the most a file may spend, and not one more"
    );
}

#[test]
fn documentation_and_blank_lines_are_not_lines_a_file_spends() {
    let a_wordy_file = format!(
        "//! One line of code.\n{}\n//! Two hundred lines of the most valuable thing here.\n\n{}",
        (0..(THE_LINE_TO_LOOK_AT_A_FILE_FROM * 3))
            .map(|one| format!("/// A line that explains something, number {one}."))
            .collect::<Vec<String>>()
            .join("\n"),
        (0..THE_LINE_TO_LOOK_AT_A_FILE_FROM)
            .map(|one| format!("let the_{one} = {one};"))
            .collect::<Vec<String>>()
            .join("\n")
    );

    let count = the_count_in(&a_wordy_file);

    assert_eq!(
        count, THE_LINE_TO_LOOK_AT_A_FILE_FROM,
        "eight hundred lines of this file are explanation and two hundred are code: a rule that made documentation expensive would be a rule against saying why, so only the code is counted"
    );
    assert_eq!(
        what_is_wrong_with("a_wordy_file.rs", &a_wordy_file),
        None,
        "and such a file is accepted whatever its length in lines"
    );
}
