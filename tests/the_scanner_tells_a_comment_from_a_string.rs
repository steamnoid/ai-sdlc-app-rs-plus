//! The rule a walk of a source file has to hold before any gate may be built on it.
//!
//! A gate that reads a `.rs` file has to know where the code stops and the words begin, and
//! a text search cannot know that: `let address = "https://github.com";` holds two slashes
//! and has no comment in it, and three fixtures here hold them inside strings on purpose.
//!
//! **Six gates will call this walk, so it is one walk in the library and not six in the
//! gates.** A private helper in a test binary is reported dead the moment the test that does
//! not use it is compiled, and the only ways to stop that are `#[allow(dead_code)]`, which a
//! gate in this repository refuses, or a copy.

use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// A source with a scheme in a string and a comment beside it.
///
/// Two things at once on purpose: a fixture holding only a comment would pass if the walk had
/// stopped reading altogether, and one holding only a string would pass if the walk had
/// decided everything is a comment.
const A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT: &str = r#"
let address = "https://github.com/owner/name";
// and this is a comment
"#;

#[test]
fn a_slash_slash_inside_a_string_is_not_a_comment() {
    let read = HowTheFileIsWritten::of(A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT);

    assert_eq!(
        read.the_lines_a_comment_starts_on(),
        vec![3],
        "the two slashes in the address are inside a string and the two on the next line are a comment, and a gate that cannot tell them is a gate that will refuse this repository's own tests"
    );
}

#[test]
fn the_walk_puts_every_byte_in_exactly_one_stretch() {
    let read = HowTheFileIsWritten::of(A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT);

    let the_bytes: usize = read.stretches().iter().map(|one| one.text().len()).sum();
    assert_eq!(
        the_bytes,
        A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT.len(),
        "a stretch that drops or repeats a byte is a walk that cannot be trusted to report a line, because a line is counted from the bytes that went before it"
    );
}

#[test]
fn a_code_stretch_is_one_line_and_not_one_character() {
    let read = HowTheFileIsWritten::of("let one = 1; let two = 2;\nlet three = 3;\n");

    let the_lines_of_code: Vec<&str> = read
        .stretches()
        .iter()
        .filter(|one| one.kind() == WhatIsWrittenThere::Code)
        .map(AStretchOfTheFile::text)
        .collect();
    assert_eq!(
        the_lines_of_code,
        vec!["let one = 1; let two = 2;", "let three = 3;"],
        "a stretch of code is the code of one line, so a place in a file is a line and an offset into it rather than one number"
    );
    let the_line_endings: Vec<&str> = read
        .stretches()
        .iter()
        .filter(|one| one.kind() == WhatIsWrittenThere::TheSpaceBetween)
        .map(AStretchOfTheFile::text)
        .collect();
    assert_eq!(
        the_line_endings,
        vec!["\n", "\n"],
        "and a line ending belongs to the space between, because a stretch of code that carried its own ending could not be told from one that did not"
    );
}

#[test]
fn a_documentation_comment_is_a_comment() {
    for source in ["/// what a reader needs", "//! what a module reader needs"] {
        let read = HowTheFileIsWritten::of(source);
        assert_eq!(
            read.the_lines_a_comment_starts_on(),
            vec![1],
            "{source:?} is documentation, and documentation is a comment that happens to be required"
        );
    }
}

#[test]
fn a_block_comment_that_nests_is_one_comment() {
    let read = HowTheFileIsWritten::of("/* outer /* inner */ still outer */\nlet after = 1;\n");

    assert_eq!(
        read.the_lines_a_comment_starts_on(),
        vec![1],
        "block comments nest in Rust unlike in C, and a walk that stops at the first close treats the rest of the file as code the gate believed it had read"
    );
    let what_follows = read
        .stretches()
        .iter()
        .find(|one| one.text().contains("after"))
        .map(AStretchOfTheFile::kind);
    assert_eq!(
        what_follows,
        Some(WhatIsWrittenThere::Code),
        "and the code after a nested comment is code, which is the other half of the same claim"
    );
}

#[test]
fn a_lifetime_is_not_a_character() {
    let read = HowTheFileIsWritten::of("let binding = 'a;\nlet other = 'b;\n");

    assert!(
        !read
            .stretches()
            .iter()
            .any(|one| one.kind() == WhatIsWrittenThere::ACharacter),
        "a quote followed by a name and no closing quote is a lifetime, and a walk that treats it as a character swallows the rest of the file — which is a gate that reports a comment on a file with none"
    );
}

#[test]
fn a_raw_lifetime_is_not_a_string() {
    let read = HowTheFileIsWritten::of("let kept = r#'name;\n");

    assert!(
        !read
            .stretches()
            .iter()
            .any(|one| one.kind() == WhatIsWrittenThere::AString),
        "a raw lifetime is a third case and it is neither a character nor a string"
    );
}

#[test]
fn a_character_literal_is_a_character() {
    let read = HowTheFileIsWritten::of("let letter = 'x';\n");

    assert!(
        read.stretches()
            .iter()
            .any(|one| one.kind() == WhatIsWrittenThere::ACharacter),
        "a quote, a character and a quote is a character, and the difference from a lifetime is the closing quote"
    );
}

#[test]
fn a_slash_inside_a_character_is_not_a_comment() {
    let read = HowTheFileIsWritten::of("let divider = '/';\n");

    assert_eq!(
        read.the_lines_a_comment_starts_on(),
        Vec::<usize>::new(),
        "a slash inside a character is a character, and a walk that calls it a comment invents a comment in a file that has none"
    );
}

#[test]
fn every_spelling_of_a_string_is_a_string() {
    for source in [
        r#"let kept = "a // b";"#,
        r#"let kept = r"a // b";"#,
        r##"let kept = r#"a "// b"#;"##,
        r#"let kept = b"a // b";"#,
        r#"let kept = c"a // b";"#,
        r##"let kept = cr#"a // b"#;"##,
    ] {
        let read = HowTheFileIsWritten::of(source);
        let holds_a_string = read
            .stretches()
            .iter()
            .any(|one| one.kind() == WhatIsWrittenThere::AString);
        let holds_a_comment = read.is_there_a_comment_anywhere();
        assert!(holds_a_string, "{source:?} holds a string");
        assert!(
            !holds_a_comment,
            "{source:?} holds a string and not a comment"
        );
    }
}

#[test]
fn a_line_is_counted_from_the_file_and_not_from_the_reading() {
    let read = HowTheFileIsWritten::of("\n\n\n// the fourth line says this\n");

    assert_eq!(
        read.the_lines_a_comment_starts_on(),
        vec![4],
        "a comment is named by the line a reader would point at, and three blank lines before it are three lines"
    );
}

#[test]
fn a_string_spanning_two_lines_keeps_the_line_after_it() {
    let read = HowTheFileIsWritten::of("let kept = \"a\nb\";\nlet after = 1;\n");

    let the_line_after = read
        .stretches()
        .iter()
        .find(|one| one.text().contains("after"))
        .map(AStretchOfTheFile::line_it_starts_on);
    assert_eq!(
        the_line_after,
        Some(3),
        "a real line ending inside a string is a real line ending, and the code after the string is on the third"
    );
}

#[test]
fn a_backslash_escape_is_not_a_line_ending() {
    let read = HowTheFileIsWritten::of("let kept = \"a\\nb\";\nlet after = 1;\n");

    let the_line_after = read
        .stretches()
        .iter()
        .find(|one| one.text().contains("after"))
        .map(AStretchOfTheFile::line_it_starts_on);
    assert_eq!(
        the_line_after,
        Some(2),
        "an escaped n is two characters and not a line ending, and a walk that counted it as one would name every later line one too high"
    );
}

#[test]
fn a_file_with_nothing_in_it_is_no_stretches_at_all() {
    for source in ["", "   \n\n", "\t"] {
        let read = HowTheFileIsWritten::of(source);
        assert!(
            !read.is_there_a_comment_anywhere(),
            "{source:?} is white space and holds no comment"
        );
    }
}

#[test]
fn the_walk_reads_the_file_that_documents_it() {
    let the_file_holding_the_walk = concat!(env!("CARGO_MANIFEST_DIR"), "/src/scanning.rs");
    let source = std::fs::read_to_string(the_file_holding_the_walk)
        .expect("the file that holds the walk is a file that can be read");

    let read = HowTheFileIsWritten::of(&source);
    let mut what_was_found = Vec::new();
    for stretch in read.stretches() {
        if stretch.kind() != WhatIsWrittenThere::AComment {
            continue;
        }
        what_was_found.push(stretch.text().to_string());
    }

    let not_documentation: Vec<&String> = what_was_found
        .iter()
        .filter(|one| !one.starts_with("///") && !one.starts_with("//!"))
        .collect();
    assert!(
        not_documentation.is_empty(),
        "the file that documents the walk holds a comment that is not documentation, and a walk that cannot be run against its own author is a walk nobody has checked: {not_documentation:?}"
    );
    assert!(
        what_was_found.len() > 20,
        "the walk found {} comments in the file that documents it, which is too few to be a file with this much documentation",
        what_was_found.len()
    );
}

#[test]
fn the_readers_find_this_checkout_and_read_a_file_in_it() {
    let the_root = aisdlc_gates::reading_the_checkout::the_root_of_the_checkout();

    assert!(
        the_root.join("crates/domain/Cargo.toml").is_file(),
        "the root is the directory holding crates/, and this one is {}",
        the_root.display()
    );
    let the_manifest =
        aisdlc_gates::reading_the_checkout::the_text_at(&the_root.join("crates/domain/Cargo.toml"));
    assert!(
        the_manifest.contains("aisdlc-domain"),
        "and the reader read the file it was pointed at rather than answering about a path: {the_manifest}"
    );
}

#[test]
fn a_file_that_cannot_be_read_is_a_failure_and_not_an_empty_one() {
    let a_directory = aisdlc_gates::reading_the_checkout::the_root_of_the_checkout().join("crates");

    let result =
        std::panic::catch_unwind(|| aisdlc_gates::reading_the_checkout::the_text_at(&a_directory));

    assert!(
        result.is_err(),
        "a directory is not a file, and a reader that answers about it has reported nothing at all"
    );
}

/// The two kinds of thing a person writes for a person to read, told apart line by line.
///
/// **This repository allows `///` and `//!` and refuses `//`.** They are the same characters
/// at the start, and a walk that reported one thing for all of them could not judge a file
/// against that rule — it would have to re-read the text to find out, which is the boundary
/// it exists to hold.
///
/// **Per line and not per stretch**, because a stretch is a *run* of comment lines and the
/// question is still per line. Three `//!` lines are one stretch; a `//!` line followed by a
/// `//` line is a different stretch. Either way a rule that called a whole run documentation
/// would pass a file with a stray `//` in it.
#[test]
fn a_stretch_says_which_of_its_lines_only_comment() {
    let the_code = "//! about the crate\n// a thought\n/// about the thing\n/* a block */\n";

    let only_commenting: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
        .stretches()
        .iter()
        .map(AStretchOfTheFile::the_lines_that_only_comment)
        .filter(|it| !it.is_empty())
        .collect();

    assert_eq!(
        only_commenting,
        vec![vec![2], vec![4]],
        "the walk gives a run of `//!` its own stretch and a run of `//` another, so these are line 2 and line 4 — the two this repository refuses, with 1 and 3 documenting"
    );
}

#[test]
fn a_stretch_that_is_not_a_comment_has_no_lines_that_only_comment() {
    let the_code = "let the_answer = 42; // and a thought\n";

    let found: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
        .stretches()
        .iter()
        .map(AStretchOfTheFile::the_lines_that_only_comment)
        .filter(|it| !it.is_empty())
        .collect();

    assert_eq!(
        found,
        vec![vec![1]],
        "the one line with a comment on it is the one to refuse, and the code on it is not"
    );
}

#[test]
fn a_run_of_documentation_documents_every_line_of_it() {
    let the_code = "//! one\n//! two\n//! three\n";

    let found: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
        .stretches()
        .iter()
        .map(AStretchOfTheFile::the_lines_that_only_comment)
        .filter(|it| !it.is_empty())
        .collect();

    assert_eq!(
        found,
        Vec::<Vec<usize>>::new(),
        "three lines about the crate and none of them is a comment"
    );
}

#[test]
fn a_slash_inside_a_string_is_not_a_comment_and_is_never_refused() {
    let the_code = "let the_path = \"src/lib.rs // not a comment\";\n";

    let found: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
        .stretches()
        .iter()
        .map(AStretchOfTheFile::the_lines_that_only_comment)
        .filter(|it| !it.is_empty())
        .collect();

    assert_eq!(
        found,
        Vec::<Vec<usize>>::new(),
        "a path inside a string is a string, and this repository does not refuse strings"
    );
}
