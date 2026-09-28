//! One repository is known by five strings, and none of them is interchangeable.
//!
//! **They are five functions rather than one with a flag, because the choice has to be
//! visible at the call site.** One function and a boolean is a function whose two meanings are
//! told apart by a reader reading a call, and the four of these differ by one suffix.
//!
//! **The read address carries the ref as a parameter and not the repository's default
//! branch**, because reading a file at a pull request's head is the question that address
//! exists to answer.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::the_addresses::{
    Addresses, the_address_of, the_clone_url_of, the_push_address_of, the_read_address_of,
};

fn the_addressed() -> Repository {
    Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves")
}

#[test]
fn the_address_a_person_reads_has_no_git_on_the_end() {
    let the_address = the_address_of(&the_addressed());

    assert_eq!(
        the_address, "https://github.com/steamnoid/ai-sdlc-app-rs-plus",
        "**because this string ends up in a summary**, and the suffix is a thing `git` needs and a person does not want"
    );
}

#[test]
fn the_address_git_fetches_from_is_the_same_string_with_git_on_the_end() {
    let the_address = the_clone_url_of(&the_addressed());

    assert_eq!(
        the_address, "https://github.com/steamnoid/ai-sdlc-app-rs-plus.git",
        "**and it carries no credential ever, because the signature is what guarantees it** — the function is handed a `&Repository` and returns a `String`, so there is nothing in scope that could hold a token"
    );
}

#[test]
fn the_address_written_to_is_a_different_place_again() {
    let the_address = the_push_address_of(&the_addressed());

    assert_eq!(
        the_address, "git@github.com:steamnoid/ai-sdlc-app-rs-plus.git",
        "**over SSH, and separately from every read address** — a repository that is read from one place and written to another is a fork, and a caller that writes to the read address has written to somebody else's repository"
    );
}

#[test]
fn the_read_address_carries_the_ref_it_was_given() {
    let at_a_pull_request = the_read_address_of(&the_addressed(), "refs/pull/42/head", "README.md")
        .expect("a ref and a path that can go in an address");
    let at_the_default =
        the_read_address_of(&the_addressed(), "main", "README.md").expect("so can these");

    assert_eq!(
        at_a_pull_request,
        "https://github.com/steamnoid/ai-sdlc-app-rs-plus/blob/refs/pull/42/head/README.md",
        "**the ref is a parameter and not the repository's default branch**, because reading a file at a pull request's head is the question this address exists to answer"
    );
    assert_ne!(
        at_a_pull_request, at_the_default,
        "and the same file at two refs is two different things"
    );
}

#[test]
fn a_clone_has_two_addresses_and_only_one_of_them_is_writable() {
    let the_addresses = Addresses::of(&the_addressed());

    assert_eq!(
        the_addresses.fetch_from,
        the_clone_url_of(&the_addressed()),
        "**`git clone` sets `origin` and nothing else**"
    );
    assert_eq!(
        the_addresses.origin_of_the_code,
        the_address_of(&the_addressed()),
        "**so without the second, a reader running `git remote -v` months later cannot tell a fork from a mystery** — and this system's own copy is a fork"
    );
}
