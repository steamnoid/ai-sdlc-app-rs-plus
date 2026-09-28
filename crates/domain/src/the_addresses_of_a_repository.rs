//! One repository is known by five strings, and none of them is interchangeable.
//!
//! **Five functions rather than one with a flag**, because the choice has to be visible at the
//! call site. One function and a boolean is a function whose two meanings are told apart by a
//! reader reading a call, and these differ by one suffix.
//!
//! **The clone address carries no credential ever, because the signature is what guarantees
//! it**: a `&Repository` in and a `String` out, so there is nothing in scope that could hold a
//! token.

use std::error::Error;
use std::fmt;

use crate::repository::Repository;

/// What a person reads.
pub fn the_address_of(a_repository: &Repository) -> String {
    let _the_owner = a_repository.owner.clone();
    let _the_name = a_repository.name.clone();
    String::new()
}

/// What `git` fetches from, and the same address with `.git` on the end.
pub fn the_clone_url_of(a_repository: &Repository) -> String {
    let _the_owner = a_repository.owner.clone();
    let _the_name = a_repository.name.clone();
    String::new()
}

/// Where a repository is written, which is a different place from where it is read.
pub fn the_push_address_of(a_repository: &Repository) -> String {
    let _the_owner = a_repository.owner.clone();
    let _the_name = a_repository.name.clone();
    String::new()
}

/// Where one file of one repository at one ref lives.
pub fn the_read_address_of(a_repository: &Repository, a_ref: &str, a_file: &str) -> String {
    let _the_owner = a_repository.owner.clone();
    let _the_name = a_repository.name.clone();
    let _the_ref = a_ref;
    let _the_file = a_file;
    String::new()
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
    pub fn of(a_repository: &Repository) -> Self {
        Self {
            fetch_from: the_clone_url_of(a_repository),
            origin_of_the_code: the_address_of(a_repository),
        }
    }
}

/// A file or a ref that cannot be put in an address, named for what cannot go in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStringThatCannotGoInAnAddress(pub String);

impl Error for AStringThatCannotGoInAnAddress {}

impl fmt::Display for AStringThatCannotGoInAnAddress {
    /// Write the refusal, quoting the string back.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} cannot go in an address: a path holding `?` or `#` changes which ref is fetched",
            self.0
        )
    }
}
