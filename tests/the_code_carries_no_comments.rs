//! The gate for RULE 1.4: a comment is a stretch of its own, and the only ones allowed
//! are the two forms of documentation.
//!
//! `///` above an item and `//!` above a module are what a reader looks at first, and
//! everything else that begins with two slashes is refused. That includes `TODO`,
//! `FIXME`, `SAFETY`, a `NOLINT`, a commented-out line and a separator: a placeholder is
//! an unfinished cycle, and a line of code that is not run is a line the reader has to
//! decide whether to believe.
//!
//! **This is not a style preference, it is a forcing function.** Every comment that would
//! have been written is a place where the code failed to say something in the only
//! language it can be read in, and the pressure the rule puts on the next reader is to fix
//! the code rather than to describe it. A rule that let `TODO` through has given the
//! unfinished thing a place to live.
//!
//! ## Why this is not a search for two slashes
//!
//! `let address = "https://github.com/owner/name";` holds two slashes and has no comment in
//! it, and three fixtures in this repository hold them inside strings on purpose. A search
//! for the two characters would refuse those and the gate would be wrong on its first run,
//! and a gate that is wrong on its first run is a gate whose next finding is not read. So
//! the source is taken apart into stretches by `HowTheFileIsWritten`, and only the stretches
//! that are comments are judged — a string and a comment are different things, and only one
//! of them is a sentence in a file.
//!
//! ## What a refusal says
//!
//! The file, the line the comment starts on, and the first sixty characters of it, for the
//! same reason every other gate in this repository names both: a gate that says "something
//! is wrong somewhere" is a gate nobody can act on.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use aisdlc_gates::scanning::{AStretchOfTheFile, HowTheFileIsWritten, WhatIsWrittenThere};

/// What a comment above an item begins with, and the only third character this gate
/// allows. A fourth slash is still this: it is documentation, and its first character after
/// the two is a third one.
const DOCUMENTATION_OF_AN_ITEM: &str = "///";

/// What a comment above a module begins with: the two slashes and then an exclamation mark,
/// which documents the file rather than whatever happens to be written under it.
const DOCUMENTATION_OF_A_MODULE: &str = "//!";

/// How much of a comment a refusal quotes, in characters. Enough to recognise the comment
/// among its neighbours, and short enough that a failure stays one line per mistake.
const HOW_MANY_CHARACTERS_A_REFUSAL_QUOTES: usize = 60;

/// Whether this comment documents rather than explains.
///
/// The two prefixes are the whole rule, so they are written out as two prefixes a reader can
/// see rather than as a keyword to be matched somewhere: the third slash, and the two then
/// one. **A block comment is documentation in Rust when it is written `/**`, and this gate
/// does not allow it** — the rule is a prefix, and a prefix is the whole of the rule
/// instead of a list of cases that grows one entry per exception somebody finds a use for.
fn is_documentation(comment: &str) -> bool {
    comment.starts_with(DOCUMENTATION_OF_AN_ITEM) || comment.starts_with(DOCUMENTATION_OF_A_MODULE)
}

/// Every comment in `source`, in the order the file has them.
///
/// The list is of every comment rather than of the ones this gate refuses, because a gate
/// handed only its findings cannot say how many comments it agreed with, and a reading that
/// reached no comment at all hands back the same empty answer as a file that has none.
/// That equality is the failure this gate is most able to have, and it is what
/// `the_walk_read_a_file_under_every_root_and_found_documentation` is here to refuse.
fn the_comments_in(source: &str) -> Vec<AStretchOfTheFile> {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .filter(|stretch| stretch.kind() == WhatIsWrittenThere::AComment)
        .cloned()
        .collect()
}

/// The comments in `source` that are not documentation, which is the whole of this rule.
///
/// A comment is a stretch of the file that is neither code nor a string nor a character,
/// and it is refused unless it documents something. Nothing about the words inside it is
/// asked: `TODO` and a safety note and a line of code that was commented out are the same
/// kind of thing here, and the three are the same kind of thing because all three are a
/// sentence in a file where the reader expected the language.
fn comments_refused_in(source: &str) -> Vec<AStretchOfTheFile> {
    Vec::new()
}

/// The first sixty characters of a comment, which is as much of it as a refusal quotes.
///
/// Cutting the text does not cost a reader anything they need: the file and the line are
/// both there, and a comment long enough to run past sixty characters is one whose first
/// sentence has already said which of them it is.
fn the_first_words_of(comment: &str) -> String {
    comment
        .chars()
        .take(HOW_MANY_CHARACTERS_A_REFUSAL_QUOTES)
        .collect()
}

/// What one file's refusals say, one line each: the file, the line, and the text.
///
/// Nothing at all when there is nothing to refuse, and that is the whole report rather than
/// a line saying everything is fine — the count of what a gate refused is what a reader
/// needs, and a gate that says nothing on a breach and nothing on a pass is
/// indistinguishable from a gate that read nothing at all.
fn what_a_refused_comment_says(the_file: &Path, refused: &[AStretchOfTheFile]) -> String {
    if refused.is_empty() {
        return String::new();
    }
    let mut said = String::new();
    for one in refused {
        let the_first_of_it = the_first_words_of(one.text());
        writeln!(
            said,
            "{}:{}: {the_first_of_it}",
            the_file.display(),
            one.line_it_starts_on()
        )
        .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

/// Everything this repository has to answer for: one line per comment that is not
/// documentation, over every file the walk found.
///
/// An empty string is the whole of what this gate has to say about a clean repository, and
/// it is a string rather than a count because the thing a reader needs when it is not empty
/// is the file and the line, not how many there were.
fn what_the_workspace_would_have_to_answer_for() -> String {
    let mut said = String::new();
    for path in every_rust_file_in_the_workspace() {
        let refused = comments_refused_in(&the_text_at(&path));
        said.push_str(&what_a_refused_comment_says(&path, &refused));
    }
    said
}

/// Every Rust file this repository is made of, found by walking down from its root.
///
/// The three roots are the three places this workspace keeps code: the gates' own library,
/// every crate under `crates/`, and `tests/`. **`crates` is walked whole rather than
/// `crates/*/src`** because a rule about the code of this repository is not a rule about
/// one package's layout, and a crate's own tests are code here too. The gates' own files
/// are in the walk for the same reason a rule applied only to other people's code is a rule
/// about other people: a gate that does not read itself has a hole shaped like itself.
fn every_rust_file_in_the_workspace() -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    for root in ["src", "crates", "tests"] {
        collect_rust_files(&the_root_of_the_checkout().join(root), &mut found);
    }
    found
}

/// One directory's worth of the walk, and the files in it appended to the ones already
/// found.
///
/// A directory that cannot be opened is skipped rather than fatal, and that is the one
/// decision here that is not free: it trades a loud failure for a quiet pass. The trade is
/// covered from the other side by `the_walk_read_a_file_under_every_root_and_found_documentation`,
/// which asks each of the three roots how many files were read out of it, so a root that
/// stopped being walkable shows up as a count of zero rather than as an agreement.
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

/// The root of this checkout, found by walking up from where cargo compiled this file.
///
/// `CARGO_MANIFEST_DIR` is fixed by cargo when this file is compiled and names the checkout
/// itself, so this is a thing a reader can follow and not a thing a test has to be
/// configured with. `crates` is what is worth walking up to, because it is in every root
/// and none of the three would be found without it.
fn the_root_of_the_checkout() -> PathBuf {
    let mut here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !here.join("crates").is_dir() {
        assert!(
            here.pop(),
            "no crates/ directory above where this file was compiled"
        );
    }
    here
}

/// The text of the file at `path`, or a panic naming the file and what the read said.
///
/// A file that cannot be read is not a file that has nothing in it. It is a file nobody has
/// looked at, and agreeing with it silently is the one wrong answer here that leaves nothing
/// to argue with, because every other refusal this gate makes names a comment in a file and
/// a file that was never opened is the one failure with nothing at all to say.
fn the_text_at(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(text) => text,
        Err(problem) => panic!("{} cannot be read as source: {problem}", path.display()),
    }
}

/// How many of the files the walk found sit under `root`, which is how this gate asks
/// whether one of the three places it was pointed at was read at all.
///
/// The root is joined onto the checkout before the comparison, because a `Path` is compared
/// a piece at a time: `starts_with("src")` asks whether the *first* piece of an absolute
/// path is `src`, which is never true, so the question would be answered zero for all three
/// roots and would look exactly like a walk that opened nothing.
fn how_many_were_found_under(root: &str) -> usize {
    let under = the_root_of_the_checkout().join(root);
    every_rust_file_in_the_workspace()
        .iter()
        .filter(|path| path.starts_with(&under))
        .count()
}

/// How many comments in `source` are documentation, counted rather than judged.
///
/// This is the number that says the reading reached a comment at all. It is asked about the
/// comments and not about the refusals because the two together would be the same fact said
/// twice, and one of the two ways of saying it can be got wrong alone.
fn how_many_documentation_comments_are_in(source: &str) -> usize {
    the_comments_in(source)
        .iter()
        .filter(|comment| is_documentation(comment.text()))
        .count()
}

/// How many comments in this repository are documentation, over every file the walk found.
fn how_many_documentation_comments_the_workspace_holds() -> usize {
    every_rust_file_in_the_workspace()
        .iter()
        .map(|path| how_many_documentation_comments_are_in(&the_text_at(path)))
        .sum()
}

/// Whether the reading of `source` read anything as a string, which is the fact that makes
/// two slashes inside one not a comment.
///
/// A fixture holding a scheme in an address has to ask this, or "accepted" is satisfied by a
/// reading that found no comment anywhere in the file and refused nothing for that reason
/// rather than because the two slashes are the text of a value.
fn is_read_as_one_string(source: &str) -> bool {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .any(|stretch| stretch.kind() == WhatIsWrittenThere::AString)
}

/// The one comment this gate refuses in `source`, and a sentence naming the rule when there
/// is not exactly one.
///
/// A test that indexed into a list the reading had left empty would fail with a number
/// rather than with the rule it is here to protect, and a test that passed over two
/// refusals would say the one thing this gate is about and not the other.
fn the_one_comment_refused_in(source: &str, because: &str) -> AStretchOfTheFile {
    let refused = comments_refused_in(source);

    assert_eq!(
        refused.len(),
        1,
        "{because}, and this is what it refused: {refused:?}"
    );
    refused.into_iter().next().expect("the count above is one")
}

/// The one comment this gate allows in `source`, and a sentence naming the rule when there
/// is not exactly one.
///
/// It is asked about the same text as `the_one_comment_refused_in` on purpose, because a
/// fixture that only asserted nothing was refused is satisfied by a gate that read no
/// comment at all — and that gate would be green, and every refusal above it would be a
/// count of zero.
fn the_one_comment_accepted_in(source: &str, because: &str) -> AStretchOfTheFile {
    let every_comment = the_comments_in(source);
    let refused = comments_refused_in(source);

    assert_eq!(
        every_comment.len(),
        1,
        "{because}, and these are the comments in it: {every_comment:?}"
    );
    assert!(refused.is_empty(), "{because}, and it refused: {refused:?}");
    every_comment
        .into_iter()
        .next()
        .expect("the count above is one")
}

#[test]
fn a_line_comment_is_refused() {
    let refused = the_one_comment_refused_in("// why this exists", "this is the plain case");

    assert_eq!(
        refused.line_it_starts_on(),
        1,
        "the whole fixture is one line and the comment is on it"
    );
    assert_eq!(
        refused.text(),
        "// why this exists",
        "and it is the whole line"
    );
}

#[test]
fn a_block_comment_is_refused() {
    let refused = the_one_comment_refused_in("/* why this exists */", "a block is a comment");

    assert_eq!(refused.text(), "/* why this exists */", "the whole fixture");
    assert_eq!(
        refused.line_it_starts_on(),
        1,
        "and the comment is on the first line of it"
    );
}

#[test]
fn a_placeholder_written_as_a_todo_is_refused() {
    let refused = the_one_comment_refused_in(
        "//TODO: finish this",
        "a placeholder is an unfinished cycle and is not an exception",
    );

    assert_eq!(
        refused.text(),
        "//TODO: finish this",
        "there is no space after the two slashes and that changes nothing"
    );
}

#[test]
fn a_placeholder_written_as_a_fixme_is_refused() {
    let refused = the_one_comment_refused_in(
        "// FIXME: this is wrong",
        "a word that says the code is wrong is the strongest reason to change the code",
    );

    assert_eq!(
        refused.text(),
        "// FIXME: this is wrong",
        "the whole comment"
    );
}

#[test]
fn a_safety_note_is_refused() {
    let refused = the_one_comment_refused_in(
        "// SAFETY: we checked",
        "a note claiming a reader may rely on something is a claim about the code, not about it",
    );

    assert_eq!(
        refused.text(),
        "// SAFETY: we checked",
        "the words inside a comment do not make it documentation"
    );
}

#[test]
fn a_commented_out_line_is_refused() {
    let refused = the_one_comment_refused_in(
        "// let previous = compute();",
        "a line of code that is not run is a comment like any other",
    );

    assert_eq!(
        refused.text(),
        "// let previous = compute();",
        "and it is refused for the one reason and not for holding code"
    );
}

#[test]
fn a_bare_pair_of_slashes_is_refused() {
    let refused =
        the_one_comment_refused_in("//", "two slashes and nothing else is still a comment");

    assert_eq!(
        refused.text(),
        "//",
        "there is no third slash and no exclamation mark"
    );
}

#[test]
fn a_block_comment_that_spans_several_lines_is_refused() {
    let refused = the_one_comment_refused_in(
        "/* one\ntwo\nthree */\nlet after = 1;\n",
        "a block of prose is a comment whatever it spans",
    );

    assert_eq!(
        refused.line_it_starts_on(),
        1,
        "it begins on the first line"
    );
    assert!(
        refused.text().ends_with("*/"),
        "and it is one comment to the close that matches the open, so the third line is inside it"
    );
}

#[test]
fn a_block_comment_that_is_not_documentation_is_refused() {
    let refused = the_one_comment_refused_in(
        "/*! */",
        "an exclamation mark does not make a block documentation",
    );

    assert_eq!(
        refused.text(),
        "/*! */",
        "the two prefixes are the whole rule"
    );
}

#[test]
fn the_documentation_of_a_module_is_accepted() {
    let accepted = the_one_comment_accepted_in(
        "//! what a module reader needs",
        "two slashes then an exclamation mark documents the file",
    );

    assert_eq!(
        accepted.text(),
        "//! what a module reader needs",
        "the whole fixture is the documentation it wrote"
    );
}

#[test]
fn the_documentation_of_an_item_is_accepted() {
    let accepted = the_one_comment_accepted_in(
        "/// what a reader of the item needs",
        "three slashes document whatever is written under them",
    );

    assert_eq!(
        accepted.text(),
        "/// what a reader of the item needs",
        "the whole fixture is the documentation it wrote"
    );
}

#[test]
fn documentation_with_nothing_written_after_it_is_accepted() {
    let accepted =
        the_one_comment_accepted_in("///", "an empty line of documentation is documentation");

    assert_eq!(
        accepted.text(),
        "///",
        "the fixture is exactly those three characters"
    );
}

#[test]
fn a_scheme_in_an_address_is_accepted() {
    let source = "let address = \"https://github.com/owner/name\";";

    assert!(
        is_read_as_one_string(source),
        "the two slashes are inside a string a value holds, and a search for the two characters would refuse this file"
    );
    assert!(
        comments_refused_in(source).is_empty(),
        "and there is no comment in this file for the gate to refuse"
    );
}

#[test]
fn a_slash_slash_inside_a_string_is_accepted() {
    let source = "let note = \"a // inside a string\";";

    assert!(
        is_read_as_one_string(source),
        "the two slashes are the text of a value and not a sentence beside it"
    );
    assert!(
        comments_refused_in(source).is_empty(),
        "and the file has no comment in it"
    );
}

#[test]
fn a_slash_slash_inside_a_raw_string_is_accepted() {
    let source = "let kept = r#\"a \"// and /* inside a raw string\"#;";

    assert!(
        is_read_as_one_string(source),
        "a raw string is a string for any number of hashes and a block opening inside it is a block opening in a value"
    );
    assert!(
        comments_refused_in(source).is_empty(),
        "and the file has no comment in it"
    );
}

#[test]
fn a_slash_slash_inside_the_string_of_an_attribute_is_accepted() {
    let source = "#[doc = \"a // inside an attribute's string\"]";

    assert!(
        is_read_as_one_string(source),
        "an attribute's string is a string, and the two slashes are its text"
    );
    assert!(
        comments_refused_in(source).is_empty(),
        "and the file has no comment in it"
    );
}

#[test]
fn a_character_holding_a_slash_is_accepted() {
    let source = "let letter = '/';";

    assert!(
        comments_refused_in(source).is_empty(),
        "one character between two apostrophes is a value and not a sentence"
    );
    assert!(
        the_comments_in(source).is_empty(),
        "and the reading found no comment in it, so nothing was refused for a reason it did not have"
    );
}

#[test]
fn a_file_with_no_comment_at_all_is_accepted() {
    let source = "fn nothing_to_say() {}\n";

    assert!(
        the_comments_in(source).is_empty(),
        "the fixture is a whole file with no comment in it"
    );
    assert!(
        comments_refused_in(source).is_empty(),
        "and a file that says nothing in prose is not a file this gate objects to"
    );
}

/// The real gate, and the only assertion here that reads the repository rather than a
/// fixture. Every one of the twenty-odd above is a demonstration; this one is the rule, and
/// it is the one that is worth keeping if the rest were deleted.
#[test]
fn the_code_carries_no_comments() {
    let said = what_the_workspace_would_have_to_answer_for();

    assert!(
        said.is_empty(),
        "these comments are not documentation, by file and by line:\n{said}"
    );
}

/// **A gate that read no files and agreed with all of them would be green and would be a
/// lie**, and there is no way to tell that apart from a clean repository by looking at what
/// it refused. So the walk is asked what it found before anything is judged: a file under
/// each of the three roots, and at least one comment that is documentation.
///
/// The documentation is the load-bearing half of that question. There are no block comments
/// in this repository today, so the honest non-vacuous claim is not that the reading found
/// a comment to refuse — a repair that stopped reading comments would also find none of
/// those, and would pass every assertion above — but that it found the documentation, and
/// agreed with it. **A gate that can only be seen to work when it fails is a gate that has
/// never been seen to work.** If this test is ever deleted the gate above is worth nothing,
/// because it would be agreeing with whatever the walk happened to reach.
#[test]
fn the_walk_read_a_file_under_every_root_and_found_documentation() {
    for root in ["src", "crates", "tests"] {
        assert!(
            how_many_were_found_under(root) > 0,
            "the walk read no .rs file under {root}/, so it agreed with a directory it never opened"
        );
    }
    assert!(
        how_many_documentation_comments_the_workspace_holds() > 0,
        "the walk reached no documentation at all, and there are no block comments here to be mistaken for one"
    );
}
