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
    let mut found = Vec::new();
    for (starts_on, spent) in the_functions_in(the_code) {
        if spent <= THE_MOST_A_BODY_MAY_SPEND {
            continue;
        }
        found.push(format!(
            "{a_name} line {starts_on} spends {spent} lines of code and the most is {THE_MOST_A_BODY_MAY_SPEND}"
        ));
    }
    found
}

/// Every `fn` in the code and the lines its body spends, as `(the line it starts on, the
/// lines the body spends)`.
///
/// **The braces are counted over `Code` lines only**, so a `{` in a string or a character
/// cannot end a body early or leave it open forever. **The signature is not counted**,
/// because the rule is about a body: a function with a long argument list has a long
/// signature and a short body, and telling those apart is the whole point of the rule.
fn the_functions_in(the_code: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut a_reading = AReading::new();
    for (the_line, one_line) in the_lines_of_code(the_code) {
        if let Some(one) = a_reading.of_one_line(the_line, &one_line) {
            found.push(one);
        }
    }
    found
}

/// How deep the walk is inside a function, and which body it is inside.
///
/// **A type because the depth and the body cannot be kept apart.** The rule is three
/// statements that touch each other — how deep, which body, and whether this line is the one
/// that closed it — and three separate locals in one function is three ways to get one of
/// them out of step with the other two.
struct AReading {
    how_deep: usize,
    where_a_function_began: Option<usize>,
    a_body: Option<(usize, usize)>,
}

impl AReading {
    /// A reading that is nowhere inside a function.
    fn new() -> Self {
        AReading {
            how_deep: 0,
            where_a_function_began: None,
            a_body: None,
        }
    }

    /// The function this line finished, or nothing if it finished none.
    fn of_one_line(&mut self, the_line: usize, one_line: &str) -> Option<(usize, usize)> {
        let opened = one_line.matches('{').count();
        let closed = one_line.matches('}').count();
        self.remember_where_a_function_began(the_line, one_line);
        let began_a_body = self.entered_a_body(opened, closed);
        if self.a_body.is_some() && self.how_deep == 0 {
            self.where_a_function_began = None;
            return self.a_body.take();
        }
        self.count_this_line(began_a_body);
        None
    }

    /// Add this line to the body it is inside, if it is a line of one.
    fn count_this_line(&mut self, began_a_body: bool) {
        if began_a_body {
            return;
        }
        if let Some(body) = self.a_body.as_mut() {
            body.1 += 1;
        }
    }

    /// Note a `fn` on this line, if there was not one already being followed.
    fn remember_where_a_function_began(&mut self, the_line: usize, one_line: &str) {
        if self.where_a_function_began.is_none() && one_line.contains("fn ") {
            self.where_a_function_began = Some(the_line);
        }
    }

    /// Whether this line opened the body, and the depth it leaves the walk at.
    fn entered_a_body(&mut self, opened: usize, closed: usize) -> bool {
        let begins = self.a_body.is_none() && self.where_a_function_began.is_some() && opened > 0;
        if begins {
            self.how_deep = opened.saturating_sub(closed);
            self.a_body = self
                .where_a_function_began
                .map(|it_began_on| (it_began_on, 0));
        } else if self.a_body.is_some() {
            self.how_deep = self.how_deep.saturating_sub(closed) + opened;
        }
        begins
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
fn a_closure_is_not_counted_as_a_function_of_its_own() {
    let the_closure = (1..=THE_MOST_A_BODY_MAY_SPEND + 1)
        .map(|one| format!("    the_{one}();"))
        .collect::<Vec<String>>()
        .join("\n");

    let the_code = format!(
        "fn holds_a_closure() {{\n    let it = |the_input| {{\n{the_closure}\n    }};\n    it;\n}}\n"
    );

    let found = what_is_wrong_with("a_file.rs", &the_code);

    assert_eq!(
        found,
        vec![format!(
            "a_file.rs line 1 spends {} lines of code and the most is {THE_MOST_A_BODY_MAY_SPEND}",
            THE_MOST_A_BODY_MAY_SPEND + 4
        )],
        "one refusal, naming line 1 — the function. The closure inside it is not a `fn` and is not counted as one; what is refused here is the function that holds it, which is a body over the limit whether or not it closes over anything. A rule that counted closures would have reported a second refusal naming line 2."
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
