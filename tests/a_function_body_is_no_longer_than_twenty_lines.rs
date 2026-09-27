//! The gate for RULE 1.5: a function body is at most twenty lines.
//!
//! A body is counted from its opening brace to its closing one, and a line holding
//! documentation or holding nothing at all is not counted. Ten lines is a signal to look
//! rather than a signal to split: extraction adds names, and names cost the reader more
//! than lines do, so what a body over twenty lines usually holds is two things and the
//! second one wants a name from the glossary.
//!
//! **The walk is anchored on `fn` and not on any brace**, and that is the whole of why an
//! `impl` block of thirty lines is not refused: it is not a function body, and a gate that
//! refused it would be refusing something this rule does not name. Everything inside a
//! function's own body *is* judged, down to the last `{`, because the block inside a
//! function is a body somebody has to read — and that is where the strictness lives, and
//! where the cost is written down rather than discovered.
//!
//! The source is taken apart by the scanner in `src/scanning.rs` rather than searched for,
//! because a brace inside a string is a brace a value holds and a brace inside a comment is
//! nothing at all. Each stretch of code is the code of exactly one line, so every place in
//! a file is a line and an offset into it, and a body is a run of lines between two braces.
//!
//! Each test carries its own reasoning above it, and the assertions under it say what this
//! build found. The reasoning is why the rule exists and does not change from run to run;
//! the assertion is what the walk actually read.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use aisdlc_gates::scanning::{AStretchOfTheFile, HowTheFileIsWritten, WhatIsWrittenThere};

/// The most lines a body may hold, counted from its opening brace to its closing one.
///
/// There is no exception, because an exception is a hole the next person uses, and a test
/// function is a function and a fixture is a function too.
const THE_MOST_LINES_IN_A_BODY: usize = 20;

/// A line of code, which is a counted line like any other.
const A_LINE_OF_CODE: &str = "    let one = 1;";

/// A line of documentation, which is not counted: a reader is shown it and no instruction
/// is written on it.
const A_LINE_OF_DOCUMENTATION: &str = "    /// what a reader has to be told";

/// A line with nothing written on it, which is not counted either.
const AN_EMPTY_LINE: &str = "";

/// A place in a file: the line it is on, and how far into that line it is.
///
/// Both are needed because a file writes two of the things this gate looks for on one
/// line all the time — a `fn` and its body on one line, a semicolon and the brace that
/// follows it — and the earlier of the two is the one that decides what a declaration is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct APlaceInTheFile {
    /// The line the place is on, counted from 1.
    the_line: usize,
    /// How far into that line the place is written.
    how_far_into_it: usize,
}

impl APlaceInTheFile {
    /// A place on `the_line`, `how_far_into_it` characters from where that line starts.
    fn on(the_line: usize, how_far_into_it: usize) -> Self {
        Self {
            the_line,
            how_far_into_it,
        }
    }
}

/// A brace written in the code of a file: which way it opens, and where it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ABraceInTheCode {
    /// Whether this brace opens something rather than closing it.
    is_opening: bool,
    /// Where it is written.
    is_at: APlaceInTheFile,
}

/// A body written in the code of a file: the brace it opens with and the brace that closes
/// it. Both are held because a body is counted from one to the other, and a body that
/// knows only where it opens cannot be counted at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ABodyInTheCode {
    /// Where the brace that opens the body is written.
    opens_at: APlaceInTheFile,
    /// Where the brace that closes the body is written.
    closes_at: APlaceInTheFile,
}

/// A `fn` written in a file: the place it is written, and the name it was given.
#[derive(Debug, Clone, PartialEq, Eq)]
struct AFunctionWritten {
    /// Where the `fn` itself is written, which is what a body is looked for after.
    is_written_at: APlaceInTheFile,
    /// The name the function was given, which is what a refusal names.
    the_name: String,
}

/// One counted line, and the body it was counted in: the function that body is in, and the
/// place it opens.
///
/// A line inside a nested block is counted once for that block and once for the function
/// around it, and both of those are judged. That is why one line is written once per body
/// rather than once per file, and it is what lets a count be a count of one body.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ALineInABody {
    /// The function this body is in.
    the_function: String,
    /// Where the brace opening the body is written.
    the_body_opens_at: APlaceInTheFile,
    /// The line that was counted.
    on_line: usize,
}

/// A body, once its counted lines have been gathered: the function it is in, the line it
/// opens on, and how many lines were counted in it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ABody {
    /// The function this body is in, which is what a reader goes and looks at.
    the_function: String,
    /// The line the body opens on.
    opens_on_line: usize,
    /// How many lines were counted in it.
    lines_counted: usize,
}

/// A body and the file it was found in, because a refusal that cannot name the file to
/// open is a refusal nobody can act on.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ABodyInAFile {
    /// The file the body was read out of.
    file: PathBuf,
    /// The body itself.
    the_body: ABody,
}

/// The stretches of a file holding code, in the order the file writes them.
///
/// Every stretch of code is the code of exactly one line, because the reading stops at a
/// line ending, and that is what makes a place in a file a line and an offset into it
/// rather than one number.
fn the_code_stretches_in(source: &str) -> Vec<AStretchOfTheFile> {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .filter(|stretch| stretch.kind() == WhatIsWrittenThere::Code)
        .cloned()
        .collect()
}

/// The lines of the file with code written on them, in the order the file writes them.
///
/// **This is what "documentation and blank lines are not counted" is, as a list.** A line
/// holding only a `///` and a line holding nothing at all both have no stretch of code
/// starting on them, so neither is in this list, and a line holding a comment and code
/// after it is in it once: the two stretches that one comment splits the code into both
/// start on that line, and `dedup` is what folds them into the one line they are on.
fn the_lines_with_code_in(source: &str) -> Vec<usize> {
    let mut lines: Vec<usize> = the_code_stretches_in(source)
        .into_iter()
        .map(|stretch| stretch.line_it_starts_on())
        .collect();
    lines.dedup();
    lines
}

/// Every line counted in the body, and nothing else: the lines between its two braces that
/// have code written on them.
///
/// The two braces have code written on them, which is what "counted from `{` to `}` inclusive"
/// means, and comparing lines rather than counting stretches is what makes a line holding two
/// stretches — a comment and the code after it — count once.
fn the_lines_counted_in(body: &ABodyInTheCode, lines_with_code: &[usize]) -> Vec<usize> {
    lines_with_code
        .iter()
        .copied()
        .filter(|line| *line >= body.opens_at.the_line && *line <= body.closes_at.the_line)
        .collect()
}

/// A body begins where the body before it is not this one: the lines of one body are
/// written together, and that is the only thing this relies on. The function is part of the
/// question because a `fn` written inside another `fn` is walked twice — once as a block
/// inside the function around it, and once as the function it is — and those two are two
/// bodies that happen to open on the same line.
fn a_body_begins_at(at: usize, lines: &[ALineInABody]) -> bool {
    if at == 0 {
        return true;
    }
    lines[at].the_function != lines[at - 1].the_function
        || lines[at].the_body_opens_at != lines[at - 1].the_body_opens_at
}

/// Write one counted line into `into`, with the function it is in and the place its body
/// opens, so that a reader counting a body is counting one body and not the file.
fn write_the_lines_counted_in(
    the_function: &str,
    body: &ABodyInTheCode,
    lines_with_code: &[usize],
    into: &mut Vec<ALineInABody>,
) {
    for on_line in the_lines_counted_in(body, lines_with_code) {
        into.push(ALineInABody {
            the_function: the_function.to_string(),
            the_body_opens_at: body.opens_at,
            on_line,
        });
    }
}

/// Every place in the code of the file holding one of `these`, in the order the file writes
/// them, and the character written there.
///
/// **A brace inside a string is in a stretch of its own kind and is not here**, and neither
/// is a brace inside a line of documentation. That is the whole reason this walks stretches
/// rather than text: a `{` is a character a value holds, and a walk that counted it would
/// close a body in the middle of a line.
fn every_place_holding_one_of(source: &str, these: &str) -> Vec<(char, APlaceInTheFile)> {
    let mut places: Vec<(char, APlaceInTheFile)> = Vec::new();

    for stretch in the_code_stretches_in(source) {
        for (how_far_into_it, character) in stretch.text().char_indices() {
            if these.contains(character) {
                let is_at = APlaceInTheFile::on(stretch.line_it_starts_on(), how_far_into_it);
                places.push((character, is_at));
            }
        }
    }
    places
}

/// Every brace written in the code of the file, in the order the file writes them: which
/// way it opens, and where. Nothing pairs them here — pairing is what tells a body from a
/// list of braces, and `every_body_from` is where that happens.
fn the_braces_in_the_code_of(source: &str) -> Vec<ABraceInTheCode> {
    every_place_holding_one_of(source, "{}")
        .into_iter()
        .map(|(character, is_at)| ABraceInTheCode {
            is_opening: character == '{',
            is_at,
        })
        .collect()
}

/// Every semicolon written in the code of the file, in the order the file writes them.
///
/// A semicolon ends a statement, and a semicolon before any brace of a declaration ends the
/// declaration itself, which is how a `trait` method is recognised.
fn the_semicolons_in_the_code_of(source: &str) -> Vec<APlaceInTheFile> {
    every_place_holding_one_of(source, ";")
        .into_iter()
        .map(|(_, is_at)| is_at)
        .collect()
}

/// Every `fn` written in the code of the file, in the order the file writes them, each with
/// the place it is written and the name it was given.
///
/// A `fn` inside a string is text a value holds and a `fn` in a line of documentation is
/// something a reader is shown, and neither of them declares a function.
fn every_function_written_in(source: &str) -> Vec<AFunctionWritten> {
    let mut written: Vec<AFunctionWritten> = Vec::new();

    for stretch in the_code_stretches_in(source) {
        written.extend(the_functions_named_in(
            stretch.text(),
            stretch.line_it_starts_on(),
        ));
    }
    written
}

/// The `fn` written in one line of code, as the place it is written and the name after it.
///
/// A `fn` inside a longer word is not a declaration — `fnord` is a name and not a function —
/// so the two characters on either side of it decide, and that is the same question
/// `a_name_is_never_generic_vocabulary` asks about the last word of a name.
fn the_functions_named_in(code: &str, the_line: usize) -> Vec<AFunctionWritten> {
    let mut written: Vec<AFunctionWritten> = Vec::new();
    let mut from = 0;

    while let Some(at) = code[from..].find("fn") {
        let begins = from + at;
        from = begins + 2;
        if a_whole_word_is_at(code, begins) {
            let is_written_at = APlaceInTheFile::on(the_line, begins);
            written.push(AFunctionWritten {
                is_written_at,
                the_name: the_name_after(code, from),
            });
        }
    }
    written
}

/// Whether the `fn` at `at` is a whole word rather than two letters inside a longer name.
fn a_whole_word_is_at(code: &str, at: usize) -> bool {
    let before = code[..at].chars().next_back();
    let after = code[at + 2..].chars().next();

    !is_part_of_a_name(before) && !is_part_of_a_name(after)
}

/// Whether this character is one an identifier is made of, which is a letter, a digit or an
/// underscore and nothing else. Nothing at all is none of them, and that is how the
/// question is answered at the end of a line and at the end of the file.
fn is_part_of_a_name(character: Option<char>) -> bool {
    character.is_some_and(|one| one.is_alphanumeric() || one == '_')
}

/// The name written after a `fn`, which is the run of characters an identifier is made of
/// from there on.
///
/// The name is empty when nothing follows the `fn` on that line, which is a `fn` and its
/// name written on two lines. Nothing in this repository is written that way, and a refusal
/// naming a function by nothing would say nothing anybody could act on — so an empty name
/// is carried rather than invented, and the count of the body is unaffected either way.
fn the_name_after(code: &str, from: usize) -> String {
    code[from..]
        .trim_start()
        .chars()
        .take_while(|character| character.is_alphanumeric() || *character == '_')
        .collect()
}

/// The brace the body of the function declared at `is_written_at` opens with, and nothing at
/// all when the declaration has no body.
///
/// From the `fn` onwards the earlier of the two decides what a declaration is: an opening
/// brace is the body, and a semicolon before any brace is a function declared without one,
/// which is what a `trait` method is. **The walk is anchored on `fn` and not on any brace**
/// because an `impl` block of thirty lines is not a function body, and a gate that refused
/// it would be refusing something this rule does not name.
fn where_the_body_begins(
    is_written_at: APlaceInTheFile,
    braces: &[ABraceInTheCode],
    semicolons: &[APlaceInTheFile],
) -> Option<ABraceInTheCode> {
    let the_body_opens_at = the_first_opening_brace_after(is_written_at, braces);
    if a_semicolon_comes_first(is_written_at, semicolons, the_body_opens_at) {
        return None;
    }
    the_body_opens_at
}

/// The first brace after the declaration at `is_written_at` that opens something, and
/// nothing at all for a `fn` with no opening brace after it — a declaration that opens
/// nothing and closes nothing, which a compiled file does not have.
fn the_first_opening_brace_after(
    is_written_at: APlaceInTheFile,
    braces: &[ABraceInTheCode],
) -> Option<ABraceInTheCode> {
    braces
        .iter()
        .find(|brace| brace.is_opening && brace.is_at > is_written_at)
        .copied()
}

/// Whether a semicolon is written between the declaration at `is_written_at` and the brace
/// that would be its body, which is what a function declared without a body looks like.
///
/// `None` is earlier than every place there is, because it is where nothing was found: a
/// declaration with a semicolon after it and no brace after that has no body, and a
/// declaration with neither after it has no body either. The two are told apart by the
/// semicolon rather than by the brace, and that is the whole of the comparison below.
///
/// A semicolon inside a parameter's own type — `fn a(what: [u8; 4])` — is a semicolon the
/// gate cannot see past, and such a function is read as declared without a body. It is
/// written down rather than worked around: the fix is a bracket the walk would have to
/// understand, and this rule is worth a reader understanding rather than a walk guessing.
fn a_semicolon_comes_first(
    is_written_at: APlaceInTheFile,
    semicolons: &[APlaceInTheFile],
    the_body_opens_at: Option<ABraceInTheCode>,
) -> bool {
    let the_first = semicolons
        .iter()
        .copied()
        .find(|place| *place > is_written_at);

    the_first.is_some() && the_first < the_body_opens_at.map(|brace| brace.is_at)
}

/// The bodies of one function: the one its own `fn` opens, and every block written inside
/// it, in the order the file writes them. Nothing at all for a function declared without a
/// body, which is a `trait` method and has nothing in it to count.
///
/// **Every `{` inside a function body is judged, and that is where the strictness is.** A
/// `match` with twenty-five arms, an `impl` written inside a function, a struct literal
/// twenty-five lines long: each of them is a body somebody has to read and each of them is
/// refused. What it costs is a false alarm on a table of values written inline, and the
/// answer to that is the glossary's eighth naming rule — a table is a `const`, and a
/// `const` is not a function body — rather than an exception here.
fn the_bodies_of(
    function: &AFunctionWritten,
    braces: &[ABraceInTheCode],
    semicolons: &[APlaceInTheFile],
) -> Vec<ABodyInTheCode> {
    let Some(opens_with) = where_the_body_begins(function.is_written_at, braces, semicolons) else {
        return Vec::new();
    };

    every_body_from(the_place_of(opens_with, braces), braces)
}

/// Where in the list of braces this one is, which is what lets a walk that has found a brace
/// go on from it rather than read the whole list a second time.
fn the_place_of(opens_with: ABraceInTheCode, braces: &[ABraceInTheCode]) -> usize {
    braces
        .iter()
        .position(|brace| brace.is_at == opens_with.is_at)
        .expect("a brace that came out of this list is in this list")
}

/// Every body written inside the one that opens at `at` in this list, and that one itself
/// first, each paired with the brace that closes it.
///
/// A stack of opening braces is what pairs them, because the file writes them in order and
/// the last one opened is the first one closed — which is how a `match` arm and a block
/// written inside it come back as two bodies rather than as two halves of one. The bodies
/// therefore come out in the order their braces closed, which is the reverse of the order
/// they were written in, and turning them round is what puts a function's own body ahead of
/// what is written inside it.
///
/// The walk stops at the closing brace that empties the stack, and a file whose braces do not
/// balance ends the walk with the bodies it did find rather than reading to the end of it.
fn every_body_from(at: usize, braces: &[ABraceInTheCode]) -> Vec<ABodyInTheCode> {
    let mut unclosed: Vec<APlaceInTheFile> = Vec::new();
    let mut closed: Vec<ABodyInTheCode> = Vec::new();

    for brace in &braces[at..] {
        if brace.is_opening {
            unclosed.push(brace.is_at);
            continue;
        }
        closed.push(the_body_closing_at(brace, &mut unclosed));
        if unclosed.is_empty() {
            break;
        }
    }
    closed.reverse();
    closed
}

/// The body this closing brace closes, which is the one the last opening brace opened.
fn the_body_closing_at(
    closes: &ABraceInTheCode,
    unclosed: &mut Vec<APlaceInTheFile>,
) -> ABodyInTheCode {
    let opens_at = unclosed
        .pop()
        .expect("a closing brace closes the last opening brace, or the file does not compile");

    ABodyInTheCode {
        opens_at,
        closes_at: closes.is_at,
    }
}

/// Every counted line in every body of the file, and the body it was counted in.
///
/// A body is what the rule judges and the lines are what it counts, so one line is written
/// once for each body it lies in: a line inside a block written inside a function is
/// counted for that block and for the function, and both of those are judged. The lines of
/// one body are written together, and that is what makes a body recoverable from them.
fn the_lines_in_the_bodies_of(source: &str) -> Vec<ALineInABody> {
    let braces = the_braces_in_the_code_of(source);
    let semicolons = the_semicolons_in_the_code_of(source);
    let lines_with_code = the_lines_with_code_in(source);
    let mut lines: Vec<ALineInABody> = Vec::new();

    for function in every_function_written_in(source) {
        for body in the_bodies_of(&function, &braces, &semicolons) {
            write_the_lines_counted_in(&function.the_name, &body, &lines_with_code, &mut lines);
        }
    }
    lines
}

/// Every body in the file, in the order the file writes them, with the lines counted in
/// each of them. The lines of one body are written together, so a body is a run of lines
/// that begins where the body changes.
fn the_bodies_in(source: &str) -> Vec<ABody> {
    let lines = the_lines_in_the_bodies_of(source);
    let mut bodies: Vec<ABody> = Vec::new();

    for (at, line) in lines.iter().enumerate() {
        if a_body_begins_at(at, &lines) {
            bodies.push(ABody {
                the_function: line.the_function.clone(),
                opens_on_line: line.the_body_opens_at.the_line,
                lines_counted: 0,
            });
        }
        count_this_line_again(&mut bodies);
    }
    bodies
}

/// Count one more line in the body the last entry is, which is the body this line belongs
/// to: the lines of one body are written together, so the last entry is that body.
fn count_this_line_again(bodies: &mut [ABody]) {
    let the_body = bodies
        .last_mut()
        .expect("a body was begun before any line was counted in it");
    the_body.lines_counted += 1;
}

/// The bodies the rule refuses in this file, which are the ones counting more than twenty
/// lines, with the function, the line and the count so a failure can be acted on.
fn the_bodies_too_long_in(source: &str) -> Vec<ABody> {
    the_bodies_in(source)
        .into_iter()
        .filter(|body| body.lines_counted > THE_MOST_LINES_IN_A_BODY)
        .collect()
}

/// Where these tests run from: the manifest of the crate holding them when cargo built
/// this file, and the working directory when it did not. Both are read rather than
/// assumed, so the walk finds the workspace from wherever it was compiled.
fn the_directory_the_tests_run_from() -> PathBuf {
    if let Some(manifest_directory) = option_env!("CARGO_MANIFEST_DIR") {
        return PathBuf::from(manifest_directory);
    }
    match std::env::current_dir() {
        Ok(here) => here,
        Err(problem) => panic!("there is no directory to walk the workspace from: {problem}"),
    }
}

/// The path of the first entry named `wanted` at or above this directory, so the walk
/// finds the workspace from a test run inside a crate as well as from the root.
fn the_path_to(wanted: &str) -> PathBuf {
    let mut directory = the_directory_the_tests_run_from();
    loop {
        if directory.join(wanted).exists() {
            return directory.join(wanted);
        }
        match directory.parent() {
            Some(above) => directory = above.to_path_buf(),
            None => panic!(
                "no {wanted} above {}",
                the_directory_the_tests_run_from().display()
            ),
        }
    }
}

/// The text of the file at `path`, or a panic naming the file and what the read said.
///
/// A file that cannot be read is not a file with nothing in it. It is a file nobody has
/// looked at, and calling it clean is the one wrong answer here that leaves nothing to
/// argue with.
fn the_text_at(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(text) => text,
        Err(problem) => panic!("{} cannot be read as source: {problem}", path.display()),
    }
}

/// Every Rust file in the workspace: the crates, the gates' own library, and the gates
/// themselves. The gates are in the walk because a gate is code in this repository too,
/// and a rule applied only to other people's code is a rule about other people.
fn every_rust_file_in_the_workspace() -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    for directory in ["crates", "src", "tests"] {
        collect_rust_files(&the_path_to(directory), &mut found);
    }
    found
}

/// One directory's worth of the walk. A directory this gate cannot open is skipped rather
/// than fatal, because a gate that stops on an unreadable directory judges nothing at all
/// and says so by passing.
fn collect_rust_files(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, found);
        } else if path.extension().is_some_and(|ending| ending == "rs") {
            found.push(path);
        }
    }
}

/// Every body in every Rust file of the workspace, with the file each was found in. One
/// walk, so the count this gate reports and the refusals it makes come from one reading.
fn every_body_in_the_workspace() -> Vec<ABodyInAFile> {
    let mut found: Vec<ABodyInAFile> = Vec::new();
    for path in every_rust_file_in_the_workspace() {
        found.extend(
            the_bodies_in(&the_text_at(&path))
                .into_iter()
                .map(|the_body| ABodyInAFile {
                    file: path.clone(),
                    the_body,
                }),
        );
    }
    found
}

/// The bodies the rule refuses out of the ones already read, kept as its own step so that a
/// test can assert what the walk found before it asks what the walk refused.
fn the_bodies_too_long_in_the_workspace(bodies: &[ABodyInAFile]) -> Vec<ABodyInAFile> {
    bodies
        .iter()
        .filter(|one| one.the_body.lines_counted > THE_MOST_LINES_IN_A_BODY)
        .cloned()
        .collect()
}

/// What a refusal says when there is one, and what it says when there is none, because a
/// guard that reported nothing on a breach and nothing on a pass is indistinguishable from
/// a guard that read nothing at all.
fn what_a_refusal_says(refused: &[ABodyInAFile], bodies_found: usize) -> String {
    if refused.is_empty() {
        return format!(
            "every one of the {bodies_found} bodies in this workspace is at most twenty lines"
        );
    }
    let mut said = format!("a body over twenty lines, out of {bodies_found} bodies read:\n");

    for one in refused {
        writeln!(
            said,
            "{}:{} `fn {}` opens there and holds {} counted lines",
            one.file.display(),
            one.the_body.opens_on_line,
            one.the_body.the_function,
            one.the_body.lines_counted
        )
        .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

/// A function of exactly `counted` counted lines and nothing else inside it. The two braces
/// are counted like any other line, so a caller asking for twenty-one gets twenty-one.
fn a_function_of(counted: usize) -> String {
    a_function_of_with(counted, A_LINE_OF_CODE, 0)
}

/// A function of exactly `counted` counted lines, with `filler` written inside it once for
/// each of the `lines` asked for.
///
/// `filler` is a line that is not code — a line of documentation, or an empty line —
/// because those are the two the rule does not count, and it is written before the code so
/// that a fixture with both in it is the fixture for a body that is long and short at once.
fn a_function_of_with(counted: usize, filler: &str, lines: usize) -> String {
    let mut source = String::from("fn a_function() {\n");
    for _ in 0..lines {
        source.push_str(filler);
        source.push('\n');
    }
    for _ in 0..counted.saturating_sub(2) {
        source.push_str(A_LINE_OF_CODE);
        source.push('\n');
    }
    source.push_str("}\n");
    source
}

/// A function holding a block of twenty-five counted lines inside it: the block opens on
/// the second line and closes on the twenty-sixth, and the function's own body runs from
/// the first to the twenty-seventh. Both are over twenty, and the two of them are told
/// apart by the line each opens on.
fn a_function_holding_a_block_of_twenty_five_lines() -> String {
    let mut source = String::from("fn a_function() {\n    loop {\n");
    for _ in 0..23 {
        source.push_str(A_LINE_OF_CODE);
        source.push('\n');
    }
    source.push_str("    }\n}\n");
    source
}

/// A function of four counted lines in which one of them writes a brace inside a string a
/// value holds, because that is the case a walk reading text rather than stretches gets
/// wrong: it ends the body at the brace in the string and counts two lines where there are
/// four.
fn a_function_holding_a_brace_in_a_string() -> String {
    String::from("fn a_function() {\n    let kept = \"{ and }\";\n    let one = 1;\n}\n")
}

/// How many small methods the `impl` fixture holds, which is what makes its block thirty
/// lines long: the brace that opens it, two lines for each method, and the brace that
/// closes it.
const THE_METHODS_IN_THE_IMPLEMENTATION: usize = 14;

/// An `impl` block of thirty lines holding fourteen methods whose bodies are one line each.
///
/// The block is longer than the rule allows and is not a function body, and the fourteen
/// bodies inside it are shorter than the rule allows. Both halves of that are the same
/// assertion: the walk finds the methods and does not find the block.
fn an_implementation_of_thirty_lines() -> String {
    let mut source = String::from("impl AWorkItem {\n");
    for _ in 0..THE_METHODS_IN_THE_IMPLEMENTATION {
        source.push_str("    /// what this method is for\n    fn a_method() -> u8 { 1 }\n");
    }
    source.push_str("}\n");
    source
}

/// A body of exactly twenty-one counted lines is one too many, and the refusal names it
/// with the count, so a boundary asserted by the number is the number the gate uses.
#[test]
fn a_body_of_twenty_one_counted_lines_is_refused() {
    let source = a_function_of(21);

    let refused = the_bodies_too_long_in(&source);
    assert_eq!(refused.len(), 1, "one body is over twenty lines here");
    assert_eq!(
        refused[0].lines_counted, 21,
        "the boundary is exact: twenty-one counted lines is one too many"
    );
}

/// A body twice as long is refused on the same count and by the same rule, which is what
/// says the rule is a number and not a shape.
#[test]
fn a_body_of_forty_counted_lines_is_refused() {
    let refused = the_bodies_too_long_in(&a_function_of(40));

    assert_eq!(refused.len(), 1, "one body is over twenty lines here");
    assert_eq!(
        refused[0].lines_counted, 40,
        "forty counted lines is two twenties and one refusal"
    );
}

/// A `#[test]` is an attribute and a function, and the attribute is written above the
/// braces, so it changes nothing about what is between them. A rule with an exception for
/// tests is a rule the next person extends to the file they are writing.
#[test]
fn a_test_function_is_a_function_and_its_body_is_refused() {
    let mut source = String::from("#[test]\n");
    source.push_str(&a_function_of(21));

    let refused = the_bodies_too_long_in(&source);
    assert_eq!(refused.len(), 1, "a test body is counted like any other");
    assert_eq!(
        refused[0].lines_counted, 21,
        "the attribute is written above the braces and is not inside the body"
    );
}

/// A block written inside a function is a body somebody has to read, and the gate judges
/// it even when the function around it is not what is too long.
///
/// **What that costs, written down rather than discovered:** a long `impl` written inside a
/// function, a long `match`, and a struct literal twenty-five lines long are all caught as
/// well. That is deliberate — a reader's difficulty does not stop at the outermost brace —
/// and it is why a `const` is a table and a table is not a function body, which is what
/// `docs/domain-glossary.md` says in its eighth naming rule.
#[test]
fn a_block_of_twenty_five_lines_inside_a_function_is_refused() {
    let refused = the_bodies_too_long_in(&a_function_holding_a_block_of_twenty_five_lines());

    assert_eq!(
        refused.len(),
        2,
        "the function's own body and the block inside it are both over twenty lines"
    );
    assert_eq!(
        refused[0].opens_on_line, 1,
        "the function's own body is over twenty as well, and it is named on its own line"
    );
    assert_eq!(
        refused[1].opens_on_line, 2,
        "and so is the block inside it, because that is a body somebody has to read"
    );
}

/// The other side of the boundary: twenty counted lines is allowed, and a gate that refuses
/// it has moved the number rather than found a long body.
#[test]
fn a_body_of_exactly_twenty_counted_lines_is_not_refused() {
    let source = a_function_of(20);
    let bodies = the_bodies_in(&source);

    assert_eq!(bodies.len(), 1, "one body is in this source");
    assert_eq!(
        bodies[0].lines_counted, 20,
        "the two braces are counted, so twenty asked for is twenty counted"
    );
    assert!(
        the_bodies_too_long_in(&source).is_empty(),
        "twenty counted lines is the rule, and the rule is at most twenty"
    );
}

/// Thirty lines of documentation inside a body of twenty counted lines make a body fifty
/// lines long on the screen and twenty lines long to this rule, and the second is the one
/// that counts.
#[test]
fn documentation_in_a_body_is_not_counted() {
    let source = a_function_of_with(20, A_LINE_OF_DOCUMENTATION, 30);
    let bodies = the_bodies_in(&source);

    assert_eq!(bodies.len(), 1, "one body is in this source");
    assert_eq!(
        bodies[0].lines_counted, 20,
        "a line a reader is shown carries no instruction and is not counted"
    );
    assert!(
        the_bodies_too_long_in(&source).is_empty(),
        "fifty lines of screen is twenty lines of body"
    );
}

/// A blank line is not a line of code either, and thirty of them in the middle of a body
/// is the same trap as thirty lines of documentation with nothing to read instead.
#[test]
fn a_blank_line_in_a_body_is_not_counted() {
    let source = a_function_of_with(20, AN_EMPTY_LINE, 30);
    let bodies = the_bodies_in(&source);

    assert_eq!(bodies.len(), 1, "one body is in this source");
    assert_eq!(
        bodies[0].lines_counted, 20,
        "a line with nothing written on it is not counted"
    );
    assert!(
        the_bodies_too_long_in(&source).is_empty(),
        "fifty lines of screen is twenty lines of body"
    );
}

/// The smallest body there is: one line, one brace and its closing brace written on it.
#[test]
fn a_function_of_one_counted_line_is_not_refused() {
    let source = String::from("fn a_function() { let one = 1; }\n");

    let bodies = the_bodies_in(&source);
    assert_eq!(bodies.len(), 1, "one body is in this source");
    assert_eq!(bodies[0].lines_counted, 1, "both braces are on one line");
    assert!(
        the_bodies_too_long_in(&source).is_empty(),
        "one line is under twenty"
    );
}

/// A `trait` method is declared with a semicolon and no braces at all, so there is no body
/// to count. The fixture carries a real function after it, because a walk that ran on past
/// the semicolon would report that function's body twice and a walk that reported nothing
/// at all would pass the same assertion.
#[test]
fn a_trait_method_declared_without_a_body_has_nothing_to_count() {
    let source = String::from(
        "trait TheClock {\n    fn what_time_is_it(&self) -> u8;\n}\nfn a_function() {\n    let one = 1;\n}\n",
    );
    let bodies = the_bodies_in(&source);

    assert_eq!(
        bodies.len(),
        1,
        "a declaration with a semicolon and no braces has no body, and the function after it has one"
    );
    assert_eq!(
        bodies[0].the_function, "a_function",
        "the one body found is the function's, and the trait method's is not there"
    );
}

/// An `impl` block of thirty lines holding fourteen small methods is longer than the rule
/// allows and is not a function body, which is the whole reason the walk is anchored on
/// `fn`. The fourteen bodies are found and the block around them is not.
#[test]
fn an_implementation_of_thirty_lines_is_not_a_function_body() {
    let source = an_implementation_of_thirty_lines();

    assert_eq!(
        the_bodies_in(&source).len(),
        THE_METHODS_IN_THE_IMPLEMENTATION,
        "the methods are bodies and the block written around them is not"
    );
    assert!(
        the_bodies_too_long_in(&source).is_empty(),
        "a thirty-line `impl` block is not a function body, and a gate that refused it would be refusing something this rule does not name"
    );
}

/// A brace inside a string is a brace a value holds: it opens nothing and closes nothing.
/// A walk that matched on it would end the body early and count a body of two lines where
/// the body is four.
#[test]
fn a_brace_written_inside_a_string_is_not_a_brace_of_the_file() {
    let source = a_function_holding_a_brace_in_a_string();
    let bodies = the_bodies_in(&source);

    assert_eq!(
        bodies.len(),
        1,
        "the braces in the string did not end the body"
    );
    assert_eq!(
        bodies[0].lines_counted, 4,
        "the body is four counted lines, and the string did not close it early"
    );
    assert!(
        the_bodies_too_long_in(&source).is_empty(),
        "four counted lines is under twenty"
    );
}

/// Two slashes inside a string are the text a value holds, and the line carrying them is
/// one line. Read as a comment it would take the rest of the file for documentation, and a
/// body counted that way is a body counted with its own code missing from it.
#[test]
fn two_slashes_written_inside_a_string_are_one_line_and_not_two() {
    let source =
        String::from("fn a_function() { let address = \"https://github.com/owner/name\"; }\n");

    let bodies = the_bodies_in(&source);
    assert_eq!(bodies.len(), 1, "one body is in this source");
    assert_eq!(
        bodies[0].lines_counted, 1,
        "the two slashes are inside a string, and the line holding them is one line"
    );
    assert!(
        the_bodies_too_long_in(&source).is_empty(),
        "one counted line is under twenty"
    );
}

/// No body in this workspace is over twenty counted lines, and the walk says how many it
/// read before it judges any of them: a gate that read nothing and agreed with all of it
/// would be a lie told with a green tick.
#[test]
fn no_body_in_the_workspace_is_longer_than_twenty_lines() {
    let files = every_rust_file_in_the_workspace();
    let bodies = every_body_in_the_workspace();
    let refused = the_bodies_too_long_in_the_workspace(&bodies);

    assert!(
        !files.is_empty(),
        "the walk read no file at all, a pass not earned"
    );
    assert!(
        !bodies.is_empty(),
        "no body in the {} files the walk read",
        files.len()
    );
    assert!(
        refused.is_empty(),
        "{}",
        what_a_refusal_says(&refused, bodies.len())
    );
}
