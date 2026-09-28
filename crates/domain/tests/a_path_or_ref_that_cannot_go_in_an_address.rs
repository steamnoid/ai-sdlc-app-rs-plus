//! A path or a ref that would change which ref is fetched, is refused.
//!
//! **A path holding `?` or `#` changes which ref is fetched, so the address would name a
//! different thing than the one it was built from.** That is not a display problem and not a
//! validation nicety: the address is what `git` is handed, and a repository called
//! `owner/name?x` produces an address that fetches something else.
//!
//! **The boundary in `a_path_in_the_project` is about project paths and does not apply here**,
//! so `://` is not the only thing to refuse. A rule that drew the line at a scheme would let
//! every one of these through, because none of them has a scheme.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::the_addresses::{
    AStringThatCannotGoInAnAddress, a_string_that_can_go_in, the_address_of, the_read_address_of,
};

fn the_addressed() -> Repository {
    Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves")
}

#[test]
fn an_ordinary_name_goes_into_an_address() {
    let the_name = a_string_that_can_go_in("crates/run/src/lib.rs", "a file's path")
        .expect("an ordinary path");

    assert_eq!(the_name.as_str(), "crates/run/src/lib.rs");
}

#[test]
fn a_path_holding_a_question_mark_is_refused() {
    let refusal = a_string_that_can_go_in("crates/run?x=1", "a file's path");

    assert!(
        refusal.is_err(),
        "**because the `?` starts a query and the address stops being the address of the file it was built from**"
    );
}

#[test]
fn a_path_holding_a_hash_is_refused() {
    let refusal = a_string_that_can_go_in("crates/run#readme", "a file's path");

    assert!(
        refusal.is_err(),
        "because the `#` starts a fragment, and a fragment is not a path"
    );
}

#[test]
fn a_ref_holding_a_question_mark_is_refused() {
    let refusal = the_read_address_of(&the_addressed(), "main?x=1", "README.md");

    assert!(
        refusal.is_err(),
        "**and the ref is the half that matters most**, because a ref with a query in it is a different ref from the one the caller named"
    );
}

#[test]
fn an_ordinary_ref_and_file_still_produce_an_address() {
    let the_address = the_read_address_of(&the_addressed(), "refs/pull/42/head", "README.md")
        .expect("an ordinary ref and an ordinary file");

    assert_eq!(
        the_address,
        "https://github.com/steamnoid/ai-sdlc-app-rs-plus/blob/refs/pull/42/head/README.md",
        "**and the ordinary case still produces the ordinary address**, because a rule that refuses the interesting cases and forgets the plain one is a rule that broke the thing it was protecting"
    );
}

#[test]
fn the_address_of_a_repository_is_refused_for_a_name_that_cannot_be_addressed() {
    let refusal: AStringThatCannotGoInAnAddress =
        a_string_that_can_go_in("own?er/name", "a repository's name")
            .expect_err("a question mark in a name is a different repository");

    assert!(
        refusal.why_not().contains('?'),
        "**because a refusal that does not quote the character is a refusal the caller cannot match against what it built**"
    );
}

#[test]
fn a_refusal_names_what_was_being_built() {
    let refusal =
        a_string_that_can_go_in("crates/run?x=1", "a file's path").expect_err("it is not one");

    assert!(
        refusal.what_was_being_built().contains("a file's path"),
        "**because a path and a ref are refused by the same rule and a refusal that does not say which one was being built leaves the caller to guess**"
    );
}
