//! The scanner tells a comment from a string, and a lifetime from a character.
//!
//! A gate that reads source is looking for a comment, and the two ways it can be wrong are
//! opposites that make each other look harmless. A file whose only comment is inside a
//! string reported as clean loses a real finding. A file with no comment at all reported
//! as full of them teaches its reader to stop believing the gate, and then the first
//! mistake costs nothing to make and everything to catch. Both are asserted here, on
//! fixtures chosen so that a reading which gets one wrong cannot get the other right.
//!
//! **The block comment that nests and the lifetime are the two that matter most.** A
//! nested block comment is *one* comment, and a lifetime is *not* a character literal.
//! A scanner that reports a comment on a file that has none is worse than no scanner,
//! because it is trusted.
//!
//! Each test carries its own reasoning above it, and the assertions under it say what the
//! reading found. The two are separated on purpose: the reasoning is why the rule exists
//! and does not change from run to run, and the assertion is what this build found.

use aisdlc_gates::scanning::{AStretchOfTheFile, HowTheFileIsWritten, WhatIsWrittenThere};

/// What the file is made of, in the order the file has it, the space between included.
///
/// Naming the whole list rather than one part of it is what catches a reading that finds
/// the right kind in the wrong number of pieces, which is a different mistake from finding
/// the wrong kind and the one a single-kind assertion cannot see.
fn what_the_file_is_made_of(source: &str) -> Vec<WhatIsWrittenThere> {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .map(AStretchOfTheFile::kind)
        .collect()
}

/// Assert that the file is made of exactly these kinds, in this order, and say why when it
/// is not. One call rather than a spelled-out list keeps the expected shape next to the
/// fixture it is about, which is the only place a reader looks for it.
fn assert_the_file_is_made_of(
    source: &str,
    these_and_no_others: &[WhatIsWrittenThere],
    because: &str,
) {
    assert_eq!(
        what_the_file_is_made_of(source),
        these_and_no_others,
        "{because}"
    );
}

/// The stretches written in one kind, in the order the file has them, so that a test can
/// say what one of them says without counting the file's white space first.
fn the_stretches_written_in(source: &str, kind: WhatIsWrittenThere) -> Vec<AStretchOfTheFile> {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .filter(|stretch| stretch.kind() == kind)
        .cloned()
        .collect()
}

/// The one stretch written in a kind, and a sentence naming the rule when there is not
/// exactly one. A test that indexed into a list the reading had left empty would fail with
/// a number rather than with the rule it is here to protect.
fn the_one_stretch_written_in(source: &str, kind: WhatIsWrittenThere) -> AStretchOfTheFile {
    let found = the_stretches_written_in(source, kind);

    assert_eq!(
        found.len(),
        1,
        "this fixture is about the one {kind:?} in it"
    );
    found.into_iter().next().expect("the count above is one")
}

/// The code of a one-stretch file, which is what most of the fixtures here are: a line
/// that is nothing but code.
fn the_code_of(source: &str) -> AStretchOfTheFile {
    the_one_stretch_written_in(source, WhatIsWrittenThere::Code)
}

/// What was written after the first stretch of a kind, skipping the space between.
///
/// A stretch is not the whole file, and the line a comment starts on is not the line the
/// code after it is on. Skipping the space is what makes this the code rather than the
/// line ending, which is the difference the last three fixtures are about.
fn what_was_written_after(source: &str, kind: WhatIsWrittenThere) -> AStretchOfTheFile {
    let read = HowTheFileIsWritten::of(source);
    let stretches = read.stretches();
    let where_the_kind_is = stretches
        .iter()
        .position(|stretch| stretch.kind() == kind)
        .unwrap_or_else(|| panic!("this fixture has no {kind:?} in it at all"));
    let after = stretches[where_the_kind_is + 1..]
        .iter()
        .find(|stretch| stretch.kind() != WhatIsWrittenThere::TheSpaceBetween);

    after
        .cloned()
        .unwrap_or_else(|| panic!("nothing follows the {kind:?} but the space between"))
}

/// The file read back out of its own stretches, which has to be the file: a reading that
/// dropped a character or invented one would pass every test that only asks what kind
/// something is, and a comment is only ever half the question.
fn the_file_read_back_out(source: &str) -> String {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .map(AStretchOfTheFile::text)
        .collect()
}

/// Two slashes inside a string are the text a value holds, and a scan for two slashes
/// finds them on a file that has no comment in it at all.
///
/// This is the finding that hides: the gate believes the file is clean, says nothing, and
/// the comment that was really there is never seen by anybody.
#[test]
fn a_slash_slash_inside_a_string_is_not_a_comment() {
    let source = "let address = \"https://github.com/owner/name\";";

    assert_the_file_is_made_of(
        source,
        &[
            WhatIsWrittenThere::Code,
            WhatIsWrittenThere::TheSpaceBetween,
            WhatIsWrittenThere::AString,
            WhatIsWrittenThere::Code,
        ],
        "the two slashes are inside a string a value holds",
    );
    let the_string = the_one_stretch_written_in(source, WhatIsWrittenThere::AString);
    assert_eq!(
        the_string.text(),
        "\"https://github.com/owner/name\"",
        "one whole address"
    );
}

/// A line comment that begins with a third slash documents what it is written above, and it
/// is still a comment and still nobody's code.
///
/// The three forms are not looked for separately, because they begin with the same two
/// characters: a reading that wanted to tell them apart would have to know which file it
/// was in, and the answer would not change what it is.
#[test]
fn a_documentation_line_is_a_comment() {
    assert_the_file_is_made_of(
        "/// what a reader needs",
        &[WhatIsWrittenThere::AComment],
        "three slashes are a comment a reader is shown",
    );
}

/// An exclamation mark after the two slashes documents the module rather than the line, and
/// it is a comment in exactly the same way.
#[test]
fn the_documentation_of_a_module_is_a_comment() {
    assert_the_file_is_made_of(
        "//! what a module reader needs",
        &[WhatIsWrittenThere::AComment],
        "an exclamation mark after the slashes is still a comment",
    );
}

/// The plain case, which the other eleven exist to be different from.
#[test]
fn a_line_comment_is_a_comment() {
    assert_the_file_is_made_of(
        "// why this exists",
        &[WhatIsWrittenThere::AComment],
        "a line comment is what this reading is for",
    );
}

/// Block comments nest, so the first close is not the end of the comment.
///
/// A reading that stopped at the first close would call the rest of the comment code, and
/// would then report any two slashes inside it as a comment of its own — a comment inside
/// a comment, on a file that has one. This and the lifetime below are the two a gate is
/// most often wrong about.
#[test]
fn a_block_comment_that_nests_is_one_comment() {
    let source = "/* outer /* inner */ still outer */";
    let comment = the_one_stretch_written_in(source, WhatIsWrittenThere::AComment);

    assert_eq!(comment.text(), source, "the whole line is the one comment");
    assert_the_file_is_made_of(
        source,
        &[WhatIsWrittenThere::AComment],
        "the words after the inner close are still the comment",
    );
}

/// An apostrophe and a name is a lifetime, and nothing on the line closes it.
///
/// A reading that took it for a character would go on looking for an apostrophe that is
/// not there, report the rest of the file as one long character, and hide every comment in
/// it. The whole line is code here, because nothing in it is anything else.
#[test]
fn a_lifetime_is_not_a_character() {
    let source = "let binding = 'a;";

    assert_the_file_is_made_of(source, &[WhatIsWrittenThere::Code], "nothing closes it");
    assert_eq!(
        the_code_of(source).text(),
        source,
        "the apostrophe is the compiler's"
    );
}

/// An apostrophe, one character, and an apostrophe: the closing apostrophe is the whole
/// difference from a lifetime, and a letter outside ASCII is more than one byte wide.
#[test]
fn a_character_between_apostrophes_is_a_character() {
    let source = "let letter = 'x';";

    assert_the_file_is_made_of(
        source,
        &[
            WhatIsWrittenThere::Code,
            WhatIsWrittenThere::TheSpaceBetween,
            WhatIsWrittenThere::ACharacter,
            WhatIsWrittenThere::Code,
        ],
        "a closing apostrophe is what makes it a character",
    );
    assert_eq!(
        the_one_stretch_written_in(source, WhatIsWrittenThere::ACharacter).text(),
        "'x'"
    );
}

/// A raw string with no hashes is closed by the next quote whatever precedes it, and
/// nothing inside it is an escape. The two slashes are part of what the string says.
#[test]
fn a_raw_string_keeps_a_slash_slash_inside_it() {
    let source = "let kept = r\"a // b\";";

    assert_the_file_is_made_of(
        source,
        &[
            WhatIsWrittenThere::Code,
            WhatIsWrittenThere::TheSpaceBetween,
            WhatIsWrittenThere::AString,
            WhatIsWrittenThere::Code,
        ],
        "the two slashes are inside a raw string",
    );
    assert_eq!(
        the_one_stretch_written_in(source, WhatIsWrittenThere::AString).text(),
        "r\"a // b\""
    );
}

/// One hash is closed by a quote and one hash, so the quote in the middle is a character
/// of the string and not the end of anything. This is what the hashes are for.
#[test]
fn a_raw_string_with_hashes_keeps_a_quote_inside_it() {
    let source = "let kept = r#\"a \"// b\"#;";

    assert_the_file_is_made_of(
        source,
        &[
            WhatIsWrittenThere::Code,
            WhatIsWrittenThere::TheSpaceBetween,
            WhatIsWrittenThere::AString,
            WhatIsWrittenThere::Code,
        ],
        "the two slashes are inside a string carrying a hash",
    );
    assert_eq!(
        the_one_stretch_written_in(source, WhatIsWrittenThere::AString).text(),
        "r#\"a \"// b\"#"
    );
}

/// A `b` in front of a quote is a prefix and not a word, so the string opens after it and
/// not at it. The slashes are still inside the value.
#[test]
fn a_byte_string_keeps_a_slash_slash_inside_it() {
    let source = "let kept = b\"a // b\";";

    assert_the_file_is_made_of(
        source,
        &[
            WhatIsWrittenThere::Code,
            WhatIsWrittenThere::TheSpaceBetween,
            WhatIsWrittenThere::AString,
            WhatIsWrittenThere::Code,
        ],
        "the two slashes are inside a byte string",
    );
    assert_eq!(
        the_one_stretch_written_in(source, WhatIsWrittenThere::AString).text(),
        "b\"a // b\""
    );
}

/// An `r`, a hash and an apostrophe is a raw lifetime, and it is the third thing that looks
/// like a string and is not one.
///
/// A reading that took the `r` and the hash for a raw string would look for a closing quote
/// that is not there and report the rest of the file as one long string — a file with no
/// comments in it and no code either.
#[test]
fn a_raw_lifetime_is_not_a_string() {
    let source = "let kept = r#'name;";

    assert_the_file_is_made_of(source, &[WhatIsWrittenThere::Code], "nothing here closes");
    assert_eq!(
        the_code_of(source).text(),
        source,
        "the r and the hash are the compiler's"
    );
}

/// A line ending inside a string is still a line, so the code after the string is on the
/// line a reader would point at and not on the one the string began on.
#[test]
fn the_code_after_a_string_that_continues_onto_the_next_line_is_on_that_next_line() {
    let source = "let first = 1;\nlet second = 2;\nlet kept = \"a\\\nb\";\n";
    let the_string = the_one_stretch_written_in(source, WhatIsWrittenThere::AString);

    assert_eq!(
        the_string.line_it_starts_on(),
        3,
        "the string begins on the third line"
    );
    let after = what_was_written_after(source, WhatIsWrittenThere::AString);
    assert_eq!(
        after.line_it_starts_on(),
        4,
        "and the code after it is on the fourth"
    );
}

/// A comment is named by the line it begins on, and the lines it spans are counted, so the
/// code after it is on the line it really is on rather than the one it started on.
#[test]
fn the_code_after_a_comment_that_spans_five_lines_is_on_the_fifth() {
    let source = "/* one\ntwo\nthree\nfour */\nlet after = 1;\n";
    let comment = the_one_stretch_written_in(source, WhatIsWrittenThere::AComment);

    assert_eq!(
        comment.line_it_starts_on(),
        1,
        "it begins on the first line"
    );
    assert_eq!(
        comment.text(),
        "/* one\ntwo\nthree\nfour */",
        "and spans four of them"
    );
    let after = what_was_written_after(source, WhatIsWrittenThere::AComment);
    assert_eq!(
        after.line_it_starts_on(),
        5,
        "so the code after it is on the fifth"
    );
}

/// A backslash and an `n` is one escape and not a line, so the code after the string is on
/// the line the string is on. The other half of this is that the string is not ended by
/// the `n`, which the text below shows.
#[test]
fn an_escape_is_not_a_line() {
    let source = "let first = 1;\nlet kept = \"a\\nb\";\nlet after = 2;\n";
    let the_string = the_one_stretch_written_in(source, WhatIsWrittenThere::AString);

    assert_eq!(the_string.text(), "\"a\\nb\"", "the n is inside the string");
    assert_eq!(
        the_string.line_it_starts_on(),
        2,
        "the string begins on the second line"
    );
    let after = what_was_written_after(source, WhatIsWrittenThere::AString);
    assert_eq!(
        after.line_it_starts_on(),
        2,
        "and the code after it is on that same line"
    );
}

/// The stretches of a file have to read back as the file.
///
/// A reading that dropped a character or invented one would pass every test above, because
/// each of those asks what kind something is and never asks whether all of the file is
/// there. A gate that quotes a stretch has to be quoting the file.
#[test]
fn every_stretch_together_is_the_file_itself() {
    for source in [
        "let address = \"https://github.com/owner/name\";",
        "/* outer /* inner */ still outer */",
        "let kept = r#\"a \"// b\"#;",
        "let binding = 'a;",
        "let kept = r#'name;",
        "let first = 1;\nlet kept = \"a\\\nb\";\n",
        "/* one\ntwo\nthree\nfour */\nlet after = 1;\n",
        "//! a module\n\nlet x = 1; // and a line\n",
    ] {
        assert_eq!(
            the_file_read_back_out(source),
            source,
            "{source:?} reads back as itself"
        );
    }
}

/// A file with no comment in it has to be able to say so, because that is the answer a gate
/// gives when it has found nothing — and an answer that is wrong here is the one nobody
/// checks.
#[test]
fn a_file_with_no_comment_says_so() {
    let read =
        HowTheFileIsWritten::of("let address = \"https://github.com/owner/name\";\nlet b = 'a;\n");

    assert!(
        !read.is_there_a_comment_anywhere(),
        "there is no comment in this file"
    );
    assert!(
        read.the_lines_a_comment_starts_on().is_empty(),
        "and so no line to name"
    );
}

/// A file with comments in it names every one of them by the line it starts on, which is
/// what a gate reports and what a reader then goes and looks at.
#[test]
fn a_file_names_every_line_a_comment_starts_on() {
    let read = HowTheFileIsWritten::of("//! what a module reader needs\n\nlet x = 1; // why\n");

    assert!(read.is_there_a_comment_anywhere());
    assert_eq!(
        read.the_lines_a_comment_starts_on(),
        [1, 3],
        "both lines are named"
    );
}
