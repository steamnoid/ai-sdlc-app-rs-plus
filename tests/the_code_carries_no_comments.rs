//! No source file carries a comment.
//!
//! **Documentation is not a comment and is allowed; a comment is refused.** `///` and `//!`
//! say why the code is as it is, in the language a reader thinks in. `//` says something
//! about the code that the code did not manage to say, and a place where that is true is a
//! place where the code failed to be read. `/* */` is refused for the same reason and no
//! softer one.
//!
//! **The refusal names the file and the line.** A gate that says "there is a comment
//! somewhere" is a gate nobody can act on, and this one can be acted on by deleting a line.
//!
//! **This gate is judged by its own rule.** It lives in a `.rs` file like everything else, so
//! every comment in it is one it would have refused, and the walk that tells a `///` from a
//! `//` is the walk this repository reads itself with.

use aisdlc_gates::a_stretch_of_the_file::WhatIsWrittenThere;
use aisdlc_gates::reading_the_checkout;
use aisdlc_gates::scanning::HowTheFileIsWritten;

/// Every source file's refusals, given its name and its text.
fn what_is_wrong_with(a_name: &str, the_code: &str) -> Vec<String> {
    let _the_name_nobody_read = a_name;
    let _the_code_nobody_read = the_code;
    Vec::new()
}

/// Every `.rs` file in this checkout, skipping the ones a build leaves behind.
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
fn no_file_in_this_repository_carries_a_comment() {
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
        "a comment is a place the code failed to say something: {found:?}"
    );
}

#[test]
fn a_line_comment_is_refused_and_named() {
    let what_it_said = "let the_answer = 42; // the answer\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", what_it_said),
        vec!["a_file.rs line 1 carries a comment".to_string()],
        "a comment on the right of a line is still a comment, and the refusal names the line so it can be deleted"
    );
}

#[test]
fn a_block_comment_is_refused_and_named() {
    let what_it_said = "/* one\n   two */\nlet the_answer = 42;\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", what_it_said),
        vec!["a_file.rs line 1 carries a comment".to_string()],
        "a block comment is refused for the same reason and no softer one, and the line it starts on is the line a reader looks at first"
    );
}

#[test]
fn documentation_is_never_refused() {
    for what_it_says in [
        "//! about the crate\n",
        "/// about the thing\nlet the_answer = 42;\n",
        "    /// indented documentation\n    fn a() {}\n",
        "//! one\n//! two\n//! three\n",
    ] {
        assert_eq!(
            what_is_wrong_with("a_file.rs", what_it_says),
            Vec::<String>::new(),
            "{what_it_says:?} is documentation, and a rule that made documentation expensive would be a rule against saying why"
        );
    }
}

#[test]
fn a_slash_inside_a_string_is_a_string_and_is_never_refused() {
    let what_it_says = "let the_path = \"a//b\";\nlet the_other = \"/* not a comment */\";\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", what_it_says),
        Vec::<String>::new(),
        "a path inside a string is a string, and refusing it would refuse the first line of every test in this repository"
    );
}

#[test]
fn a_comment_among_documentation_is_still_refused() {
    let what_it_said = "//! about the crate\n// a thought\n/// about the thing\n";

    assert_eq!(
        what_is_wrong_with("a_file.rs", what_it_said),
        vec!["a_file.rs line 2 carries a comment".to_string()],
        "one stray comment in a file of documentation is the case a rule that only looked at whole stretches would pass, and it is the common one"
    );
}

#[test]
fn the_walk_was_actually_read() {
    let found: Vec<usize> = HowTheFileIsWritten::of("// a comment\n")
        .stretches()
        .iter()
        .flat_map(|it| it.the_lines_that_only_comment())
        .collect();

    assert_eq!(
        found,
        vec![1],
        "the walk finds the one comment in this string, and a gate built on a walk that found nothing would pass everything"
    );
    assert_eq!(
        HowTheFileIsWritten::of("//! docs\n").stretches()[0].kind(),
        WhatIsWrittenThere::AComment,
        "and documentation is still a comment as far as the kind goes, which is why the walk answers a second question about it"
    );
}
