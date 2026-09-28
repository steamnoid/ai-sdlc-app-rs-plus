//! A string that cannot be a path, and a path that lands outside the project.
//!
//! **Two questions with two answers, and only the second was ever about confinement.** Whether a
//! string can be a path at all and whether the path that results lands inside the project are
//! not the same question, and a rule that only answers the second lets `not a path at all`
//! through to the disk, which is where the operating system finds out rather than where
//! anything can refuse it.
//!
//! **The shape rule comes first because a model once wrote a 433-byte sentence into a path
//! field**, and the operating system raised about that instead of answering.

use aisdlc_run::a_path_in_the_project::{
    AStringThatIsNotAPath, THE_LONGEST_A_COMPONENT_MAY_BE, a_path_in_the_project,
};

#[test]
fn an_ordinary_path_is_a_path() {
    let the_path =
        a_path_in_the_project("crates/run/src/lib.rs").expect("an ordinary path is a path");

    assert_eq!(
        the_path.as_str(),
        "crates/run/src/lib.rs",
        "and it reads back what it was given, because a rule that rewrites a correct path turns a working read into a refused one"
    );
}

#[test]
fn a_string_of_only_whitespace_is_not_a_path() {
    assert!(
        a_path_in_the_project("   ").is_err(),
        "**a blank is not a short path**, and a path that trims to nothing names nothing on any filesystem"
    );
}

#[test]
fn an_empty_segment_is_not_a_path() {
    assert!(
        a_path_in_the_project("crates//run").is_err(),
        "**an empty segment is not a directory**, and the operating system would read it as one and answer about a place that does not exist"
    );
}

#[test]
fn a_component_longer_than_the_bound_is_not_a_path() {
    let a_sentence = "x".repeat(THE_LONGEST_A_COMPONENT_MAY_BE + 1);

    let refusal = a_path_in_the_project(&a_sentence);

    assert!(
        refusal.is_err(),
        "**a 433-byte sentence in a path field is what this rule is for**, and the refusal quotes the input back so the model that wrote it can match against what it produced"
    );
}

#[test]
fn a_refusal_quotes_the_input_back() {
    let refusal: AStringThatIsNotAPath =
        a_path_in_the_project("").expect_err("a blank is not a path");

    assert_eq!(
        refusal.what_was_written(),
        "",
        "**a refusal the model that wrote the input cannot match against what it produced spends the model a turn it has already spent**"
    );
}

#[test]
fn a_refusal_says_why() {
    let refusal = a_path_in_the_project("  ").expect_err("whitespace is not a path");

    assert!(
        !refusal.why_not().is_empty(),
        "because a refusal that does not say why leaves the caller to guess between a blank, a bad character and a long component"
    );
}
