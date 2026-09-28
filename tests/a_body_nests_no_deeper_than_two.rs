//! No statement inside a function body is nested deeper than two levels.
//!
//! **Levels are counted with braces and not with indentation**, because indentation is a
//! promise a person makes and braces are what the compiler reads. A body whose third level
//! is written with the same four spaces as the first is still three levels deep.
//!
//! **Two and not one.** A function is one block and a statement inside it is at level one;
//! an `if` inside that is two; a third is refused. A body that reads as a staircase is a
//! body a reader has to hold in their head backwards.
//!
//! **Only inside a function.** An `impl` block and a `struct` are not a body, and a rule
//! that counted their nesting would refuse every type in the domain for being a field of a
//! field.

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// The deepest a statement may sit inside a body.
const THE_DEEPEST_A_STATEMENT_MAY_SIT: usize = 2;

/// Every refusal in one file, given its name and its text.
fn what_is_wrong_with(a_name: &str, the_code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (the_line, sits) in the_statements_and_how_deep_they_sit(the_code) {
        if sits <= THE_DEEPEST_A_STATEMENT_MAY_SIT {
            continue;
        }
        found.push(format!(
            "{a_name} line {the_line} sits {sits} deep in a body and the deepest is {THE_DEEPEST_A_STATEMENT_MAY_SIT}"
        ));
    }
    found
}

/// Every statement inside a function body and the number of levels it sits at.
fn the_statements_and_how_deep_they_sit(the_code: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut a_reading = AReading::new();
    for (the_line, one_line) in the_lines_of_code(the_code) {
        found.extend(a_reading.of_one_line(the_line, &one_line));
    }
    found
}

/// How deep inside a body the walk is, and where a function began.
struct AReading {
    inside_a_function: bool,
    how_deep: usize,
}

impl AReading {
    /// A reading that is not inside any function.
    fn new() -> Self {
        AReading {
            inside_a_function: false,
            how_deep: 0,
        }
    }

    /// The statements on this line, and how deep each one sits.
    /// **The line that opens a body is not itself a statement in it**, so its own brace does
    /// not count and it is not reported: `if three {` sits at level two and is a level two
    /// statement, and the first one that is too deep is the one under it.
    fn of_one_line(&mut self, the_line: usize, one_line: &str) -> Vec<(usize, usize)> {
        if one_line.contains("fn ") {
            self.inside_a_function = true;
            self.how_deep = 0;
            return Vec::new();
        }
        let mut found = Vec::new();
        if self.inside_a_function && !closes_a_body(one_line) {
            found.push((the_line, self.how_deep));
        }
        self.how_deep = self.how_deep.saturating_add(one_line.matches('{').count());
        self.how_deep = self.how_deep.saturating_sub(one_line.matches('}').count());
        found
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

/// Every line of the code that carries code on it, as `(the line it is on, the line)`.
fn the_lines_of_code(the_code: &str) -> Vec<(usize, String)> {
    let mut found: Vec<(usize, String)> = Vec::new();
    for stretch in HowTheFileIsWritten::of(the_code).stretches() {
        if stretch.kind() != WhatIsWrittenThere::Code {
            continue;
        }
        append_the_lines_of(stretch, &mut found);
    }
    found.retain(|(_, one_line)| !one_line.trim().is_empty());
    found
}

/// Add one stretch's lines, joining a line the walk cut in two.
///
/// **A line can arrive in pieces**, because a string in the middle of it ends the code
/// stretch and starts another on the same line — `format!("{} …", x)` is two pieces, and
/// treating them as two lines counts the line twice and its braces as two levels.
fn append_the_lines_of(a_stretch: &AStretchOfTheFile, found: &mut Vec<(usize, String)>) {
    for (offset, one_line) in a_stretch.text().split('\n').enumerate() {
        let the_line = a_stretch.line_it_starts_on() + offset;
        let continues_the_last = found.last().is_some_and(|(on, _)| *on == the_line);
        let Some(last) = found.last_mut().filter(|_| continues_the_last) else {
            found.push((the_line, one_line.to_string()));
            continue;
        };
        last.1.push_str(one_line);
    }
}

#[test]
fn no_statement_in_this_repository_sits_too_deep() {
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
        "a statement may sit {THE_DEEPEST_A_STATEMENT_MAY_SIT} deep in a body and these do not: {found:?}"
    );
}

#[test]
fn a_statement_at_the_deepest_level_is_accepted() {
    let the_code = "fn two_deep() {\n    if ready {\n        if second {\n            go();\n        }\n    }\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "two levels is the deepest and `go` sits in it"
    );
}

#[test]
fn a_statement_three_deep_is_refused_and_named() {
    let the_code = "fn three_deep() {\n    if one {\n        if two {\n            if three {\n                go();\n            }\n        }\n    }\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec!["a_file.rs line 5 sits 3 deep in a body and the deepest is 2".to_string()],
        "a refusal a reader cannot match against the line they wrote teaches nothing"
    );
}

#[test]
fn a_brace_inside_a_string_does_not_make_a_level() {
    let the_code =
        "fn quoted() {\n    if one {\n        let the_text = \"{ { {\";\n        go();\n    }\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "a brace in a string is text, and a level counted out of it is a level nobody wrote"
    );
}

#[test]
fn a_match_arm_is_a_level_and_a_block_inside_an_arm_is_another() {
    let the_code = "fn matched(value: u8) {\n    match value {\n        1 => {\n            if one {\n                go();\n            }\n        }\n        _ => {}\n    }\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        vec!["a_file.rs line 5 sits 3 deep in a body and the deepest is 2".to_string()],
        "a match arm's block is a level and the `if` inside it is the next, so a statement under both is the third"
    );
}

#[test]
fn a_nesting_outside_a_function_is_not_a_body() {
    let the_code = "struct AThing {\n    a_field: u8,\n}\n\nimpl AThing {\n    fn a() -> u8 {\n        1\n    }\n}\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "a struct and an impl are not a body, and a rule that refused them would refuse every type in the domain for being a field of a field"
    );
}

#[test]
fn a_whole_function_on_one_line_is_not_three_deep() {
    let the_code = "fn one_line() { if a { if b { if c { go(); } } } }\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", the_code),
        Vec::<String>::new(),
        "three levels on one line is a line a reader cannot hold anyway, and this rule is about statements sitting deep — a separate rule about a body spending its lines is what catches this"
    );
}

/// Whether a line is only closing what the lines above it opened.
///
/// **A line that closes is not a statement in the block it closes**, and counting it would
/// report every function's last line as a statement sitting at the body's own depth.
fn closes_a_body(one_line: &str) -> bool {
    one_line.trim() == "}"
}
