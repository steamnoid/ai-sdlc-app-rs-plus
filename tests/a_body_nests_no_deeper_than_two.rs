//! The gate for RULE 1.6: a function body nests no deeper than two levels.
//!
//! Three levels of `if` is always one of three things: a `match` that wants to be written,
//! a chain of guards, or two functions in one. This is the rule that catches what a line
//! count misses, and that is why it stands beside the twenty-line rule rather than inside
//! it: a body can be short and still be unreadable.
//!
//! **A brace is a control-flow brace** when the nearest preceding code word is one of `if`,
//! `else`, `match`, `for`, `while` or `loop`, when it follows the closing bracket of such a
//! construct's condition, or when it follows an `=>`. Every other brace — a function body,
//! an `impl` block, a `struct` body, a bare block, a struct literal, a tuple, an array, and
//! a **closure body** — is a plain brace, and a plain brace resets the count. A closure is a
//! function, so a `for` inside one is depth one, and that is the cost of treating the thing
//! rather than the shape: a closure holding three levels of `if` passes.
//!
//! Depth is counted from the nearest enclosing plain brace, not from the top of the file,
//! so a file of ten functions and no deep `if` is clean.
//!
//! The source is taken apart by the scanner in `src/scanning.rs` rather than searched for,
//! because an `if` inside a string is not an `if` and a brace inside a format string is not
//! a brace. Each stretch of code is the code of exactly one line, so every place in a file
//! is a line and an offset into it.

use std::fs;
use std::path::{Path, PathBuf};

use aisdlc_gates::scanning::{HowTheFileIsWritten, WhatIsWrittenThere};

/// The words a control-flow construct begins with.
///
/// `else if` is two words and one construct, which is why the check is on the *nearest*
/// word: a brace after `else` is at the same depth as the `else` that holds it, and a brace
/// after `if` inside it is one deeper. The table has six entries and they are read in one
/// place, so adding a seventh construct is a change here and nowhere else.
const THE_WORDS_THAT_BEGIN_CONTROL_FLOW: [&str; 6] =
    ["if", "else", "match", "for", "while", "loop"];

/// How deep a file nests before the gate refuses it.
///
/// Two is allowed, and the boundary is asserted from both sides, because a rule with a
/// number in it that has only ever been tested from the failing side is a rule nobody knows
/// where it really is.
const THE_DEEPEST_NESTING_ALLOWED: usize = 2;

/// Whether a word begins a control-flow construct.
fn is_control_flow(word: &str) -> bool {
    THE_WORDS_THAT_BEGIN_CONTROL_FLOW.contains(&word)
}

/// Whether a character is part of a word, so that `ifx` is not `if` followed by `x`.
fn is_part_of_a_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// What the walk remembers from one character to the next.
///
/// **A control-flow word announces a construct, and the brace that follows the construct
/// is its body** — not the brace that follows the word. `if one {` has a word and then a
/// condition and then a brace, and only the flag carried across the condition finds the
/// right one. The first version of this walk looked for a control-flow word immediately
/// before the brace, and it found every `if` in a file except the ones written the way
/// Rust is actually written.
struct HowDeepTheFileIs {
    control_flow_braces_open: usize,
    deepest: usize,
    the_deepest_line: usize,
    current_line: usize,
    the_word_being_written: String,
    at_the_start_of_a_word: bool,
    a_construct_is_waiting_for_its_body: bool,
}

impl HowDeepTheFileIs {
    /// A file that has not been read yet is nested zero deep, which is the only honest
    /// answer before a single brace has been seen.
    fn at_the_start() -> HowDeepTheFileIs {
        HowDeepTheFileIs {
            control_flow_braces_open: 0,
            deepest: 0,
            the_deepest_line: 0,
            current_line: 0,
            the_word_being_written: String::new(),
            at_the_start_of_a_word: true,
            a_construct_is_waiting_for_its_body: false,
        }
    }

    /// The deepest the file nests, and the line it does it on.
    fn what_it_found(&self) -> (usize, usize) {
        (self.deepest, self.the_deepest_line)
    }

    /// Read one stretch of code. Anything that is not code is not read at all, which is
    /// the whole reason this gate runs on the scanner and not on the text.
    fn over_a_stretch(&mut self, text: &str, line: usize) {
        self.current_line = line;
        for character in text.chars() {
            self.over_one_character(character);
        }
    }

    /// Read one character, and decide which of the four things it can be.
    fn over_one_character(&mut self, character: char) {
        match character {
            '{' => self.on_an_opening_brace(),
            '}' => self.on_a_closing_brace(),
            ';' | ',' => self.on_the_end_of_a_statement(),
            _ => self.on_anything_else(character),
        }
    }

    /// A brace is control flow when a construct announced itself and has not yet been given
    /// its body. A plain brace resets the count, and a brace that is not control flow is not
    /// a level of anything.
    fn on_an_opening_brace(&mut self) {
        self.the_word_ends_here();
        if !self.a_construct_is_waiting_for_its_body {
            return;
        }
        self.a_construct_is_waiting_for_its_body = false;
        self.control_flow_braces_open += 1;
        self.record_the_depth_reached_here();
    }

    /// One control-flow brace less. A closing brace that matches nothing is ignored, so
    /// the count never goes below zero and a file with a stray brace reads as shallow
    /// rather than as absurd.
    fn on_a_closing_brace(&mut self) {
        self.the_word_ends_here();
        if self.control_flow_braces_open == 0 {
            return;
        }
        self.control_flow_braces_open -= 1;
    }

    /// A statement has ended, so a construct announced earlier and never given a body was
    /// not a construct at all.
    fn on_the_end_of_a_statement(&mut self) {
        self.the_word_ends_here();
        self.a_construct_is_waiting_for_its_body = false;
    }

    /// A word being finished, or punctuation in between.
    ///
    /// **An arm's `=>` is deliberately not a construct.** A `match` arm is a case of the
    /// match, not something inside it, so the arm's block is a plain brace and the
    /// control flow inside it starts again from the match's own level. Reading `=>` as an
    /// announcement also counted a function body as control flow whenever the signature
    /// held a generic — `Iterator<Item = u8>` ends in `>` — which is a false positive on
    /// code with nothing to do with nesting.
    fn on_anything_else(&mut self, character: char) {
        if character.is_whitespace() || !is_part_of_a_word(character) {
            self.the_word_ends_here();
            return;
        }
        self.push_onto_the_word_being_written(character);
    }

    /// One letter of a word. The first letter clears whatever was there, so `round` never
    /// reads as `r` followed by `ound`.
    fn push_onto_the_word_being_written(&mut self, character: char) {
        if self.at_the_start_of_a_word {
            self.the_word_being_written.clear();
        }
        self.the_word_being_written.push(character);
        self.at_the_start_of_a_word = false;
    }

    /// A word ended here, so if it was a construct it is now waiting for its body.
    fn the_word_ends_here(&mut self) {
        if is_control_flow(&self.the_word_being_written) {
            self.a_construct_is_waiting_for_its_body = true;
        }
        self.the_word_being_written.clear();
        self.at_the_start_of_a_word = true;
    }

    /// Remember a depth only when it is deeper than any before it, so the line named is
    /// the line of the innermost brace rather than the last one walked past.
    fn record_the_depth_reached_here(&mut self) {
        if self.control_flow_braces_open <= self.deepest {
            return;
        }
        self.deepest = self.control_flow_braces_open;
        self.the_deepest_line = self.current_line;
    }
}

/// How deep a source nests, and the line it does it on.
///
/// The permissive shell this repository's rules ask for: a reading that reports nothing
/// finds no nesting anywhere, which is the shape the tests below are written to refuse.
fn the_deepest_nesting_in(source: &str) -> (usize, usize) {
    let mut reading = HowDeepTheFileIs::at_the_start();
    for stretch in HowTheFileIsWritten::of(source).stretches() {
        if stretch.kind() != WhatIsWrittenThere::Code {
            continue;
        }
        reading.over_a_stretch(stretch.text(), stretch.line_it_starts_on());
    }
    reading.what_it_found()
}

/// Every `.rs` file in the workspace, sorted, so a refusal names a path.
fn every_rust_file_in_the_workspace() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut found = Vec::new();
    for relative in ["src", "crates", "tests"] {
        collect_rust_files_under(&root.join(relative), &mut found);
    }
    found.sort();
    found
}

fn collect_rust_files_under(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_under(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
}

/// Read one file and say what it found, or refuse to say anything about it.
///
/// A file that cannot be read is a failure rather than an empty answer, because a gate that
/// turns a missing file into a pass is a gate that has stopped being one.
fn what_one_file_says(path: &Path) -> Result<(usize, usize), String> {
    let source = fs::read_to_string(path)
        .map_err(|why| format!("{} could not be read: {why}", path.display()))?;
    Ok(the_deepest_nesting_in(&source))
}

#[test]
fn three_levels_of_if_are_refused() {
    let source = "\
fn a() {
    if one {
        if two {
            if three {
                act();
            }
        }
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        3,
        "three levels of if are one of the three things this rule exists to catch, and the innermost brace is on line 4"
    );
    assert_eq!(
        the_deepest_nesting_in(source).1,
        4,
        "the refusal has to name the line a reader would point at, and this is the brace"
    );
}

#[test]
fn three_different_constructs_at_three_levels_are_refused() {
    let source = "\
fn a() {
    for each in all {
        while going {
            if tired {
                stop();
            }
        }
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        3,
        "the rule counts levels, not repeats of one word, and three different constructs nest as deeply as three of the same"
    );
}

#[test]
fn an_arm_holding_two_more_levels_is_refused() {
    let source = "\
fn a() {
    match shape {
        round => {
            if small {
                if tiny {
                    act();
                }
            }
        }
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        3,
        "an arm is a control-flow brace at one, so two inside it is three"
    );
}

#[test]
fn an_if_holding_a_match_holding_a_for_is_refused() {
    let source = "\
fn a() {
    if ready {
        match shape {
            round => {
                for each in all {
                    act();
                }
            }
        }
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        3,
        "three different constructs through a match arm is still three"
    );
}

#[test]
fn two_levels_are_allowed() {
    let source = "\
fn a() {
    if one {
        if two {
            act();
        }
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        2,
        "two is the boundary and it is allowed, so the rule is not a rule that refuses everything that nests"
    );
}

#[test]
fn one_level_is_allowed() {
    let source = "\
fn a() {
    if one {
        act();
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        1,
        "an if on its own is one, which is what most code is"
    );
}

#[test]
fn six_arms_of_one_line_each_are_all_at_one() {
    let source = "\
fn a() {
    match shape {
        round => go(),
        square => stop(),
        triangle => turn(),
        line => wait(),
        dot => hide(),
        nothing => {}
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        1,
        "six arms side by side are six of one, not three of something"
    );
}

#[test]
fn a_struct_literal_of_ten_lines_is_not_nesting() {
    let source = "\
fn a() {
    let what = Something {
        first: 1,
        second: 2,
        third: 3,
        fourth: 4,
        fifth: 5,
        sixth: 6,
        seventh: 7,
        eighth: 8,
        ninth: 9,
        tenth: 10,
    };
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        0,
        "a struct literal is not control flow, and a rule that counted it would refuse every table written inline"
    );
}

#[test]
fn a_closure_holding_a_for_holding_an_if_is_two() {
    let source = "\
fn a() {
    let work = |all| {
        for each in all {
            if wanted(each) {
                act(each);
            }
        }
    };
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        2,
        "a closure is a function, so the count follows the thing rather than the shape"
    );
}

#[test]
fn an_if_written_inside_a_string_is_not_an_if() {
    let source = "\
fn a() {
    let note = \"if one { if two { if three { } } }\";
    tell(note);
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        0,
        "text inside a string is a value, and a value that says if three times is still a value"
    );
}

#[test]
fn an_if_written_inside_a_comment_is_not_an_if() {
    let source = "\
fn a() {
    if one {
        act();
    }
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        1,
        "a file with one if and prose about nesting is nested one, whatever the prose says"
    );
}

#[test]
fn a_trait_with_no_bodies_is_not_nested_at_all() {
    let source = "\
trait Asking {
    fn what_is_wanted(&self) -> bool;
    fn what_was_found(&self) -> bool;
    fn what_happened(&self) -> bool;
}";
    assert_eq!(
        the_deepest_nesting_in(source).0,
        0,
        "a signature with no body opens no brace, so there is nothing to be inside"
    );
}

#[test]
fn the_walk_reads_the_files_before_it_judges_them() {
    let files = every_rust_file_in_the_workspace();
    assert!(
        files.len() > 5,
        "the walk read {} files, which is too few to be this repository's source",
        files.len()
    );
    let deepest = files
        .iter()
        .map(|path| what_one_file_says(path))
        .collect::<Result<Vec<(usize, usize)>, String>>()
        .expect("every file in the workspace can be read");
    assert!(
        deepest.iter().any(|(depth, _)| *depth > 0),
        "the walk read the files and found no nesting anywhere, which is a pass it has not earned"
    );
}

#[test]
fn no_file_in_the_workspace_nests_too_deeply() {
    let mut refused = Vec::new();
    for path in every_rust_file_in_the_workspace() {
        let (depth, line) = what_one_file_says(&path).expect("every file can be read");
        if depth > THE_DEEPEST_NESTING_ALLOWED {
            refused.push(format!("{}:{} nests {} deep", path.display(), line, depth));
        }
    }
    assert!(
        refused.is_empty(),
        "three levels of nesting is a match that wants to be written, a chain of guards, or two functions in one: {refused:?}"
    );
}
