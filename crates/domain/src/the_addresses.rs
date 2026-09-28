//! One repository is known by five strings, and none of them is interchangeable.
//!
//! **Five functions rather than one with a flag**, because the choice has to be visible at the
//! call site. One function and a boolean is a function whose two meanings are told apart by a
//! reader reading a call, and these differ by one suffix.
//!
//! **The clone address carries no credential ever, because the signature is what guarantees
//! it**: a `&Repository` in and a `String` out, so there is nothing in scope that could hold a
//! token.

use crate::repository::Repository;

/// What a person reads.
pub fn the_address_of(what_is_addressed: &Repository) -> String {
    format!(
        "https://github.com/{}/{}",
        what_is_addressed.owner, what_is_addressed.name
    )
}

/// What `git` fetches from, and the same address with `.git` on the end.
pub fn the_clone_url_of(what_is_addressed: &Repository) -> String {
    format!("{}.git", the_address_of(what_is_addressed))
}

/// Where a repository is written, which is a different place from where it is read.
pub fn the_push_address_of(what_is_addressed: &Repository) -> String {
    format!(
        "git@github.com:{}/{}.git",
        what_is_addressed.owner, what_is_addressed.name
    )
}

/// Where one file of one repository at one ref lives.
pub fn the_read_address_of(what_is_addressed: &Repository, a_ref: &str, a_file: &str) -> String {
    format!(
        "{}/blob/{}/{}",
        the_address_of(what_is_addressed),
        a_ref,
        a_file
    )
}

/// The two addresses of a clone: where this system's copy lives, and where the code was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Addresses {
    /// Where this system's copy lives, and may be pushed to.
    pub fetch_from: String,
    /// Where the code was written, and may only be read.
    pub origin_of_the_code: String,
}

impl Addresses {
    /// The two addresses one repository has, before the clone that carries them.
    pub fn of(what_is_addressed: &Repository) -> Self {
        Self {
            fetch_from: the_clone_url_of(what_is_addressed),
            origin_of_the_code: the_address_of(what_is_addressed),
        }
    }
}
