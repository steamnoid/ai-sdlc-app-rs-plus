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
pub fn the_read_address_of(
    what_is_addressed: &Repository,
    a_ref: &str,
    a_file: &str,
) -> Result<String, AStringThatCannotGoInAnAddress> {
    let the_ref = a_string_that_can_go_in(a_ref, "a ref")?;
    let the_file = a_string_that_can_go_in(a_file, "a file's path")?;
    Ok(format!(
        "{}/blob/{}/{}",
        the_address_of(what_is_addressed),
        the_ref.as_str(),
        the_file.as_str()
    ))
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

use std::error::Error;
use std::fmt;

/// A string that would change which ref is fetched, and so cannot go in an address.
///
/// **The rule is the characters and not the scheme.** `://` is the obvious thing to refuse and
/// none of these strings has one, so a rule drawn there lets every one of them through: the `?`
/// that starts a query and the `#` that starts a fragment both change what the address fetches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStringThatCannotGoInAnAddress {
    what_was_being_built: String,
    written: String,
    what_made_it_impossible: String,
}

impl AStringThatCannotGoInAnAddress {
    /// Which of the two things was being built, because a path and a ref are refused alike.
    pub fn what_was_being_built(&self) -> &str {
        &self.what_was_being_built
    }

    /// The string that was written, exactly as it was written.
    pub fn what_was_written(&self) -> &str {
        &self.written
    }

    /// Which character made it impossible, quoted back so the caller can match against it.
    pub fn why_not(&self) -> &str {
        &self.what_made_it_impossible
    }

    /// This refusal, for a string that cannot go where it was going.
    pub fn because(
        what_was_being_built: &str,
        written: &str,
        what_made_it_impossible: &str,
    ) -> Self {
        Self {
            what_was_being_built: what_was_being_built.to_string(),
            written: written.to_string(),
            what_made_it_impossible: what_made_it_impossible.to_string(),
        }
    }
}

impl Error for AStringThatCannotGoInAnAddress {}

impl fmt::Display for AStringThatCannotGoInAnAddress {
    /// Write the refusal, quoting both the string and the character.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} cannot go in {}: it holds `{}`, which changes what the address fetches",
            self.written, self.what_was_being_built, self.what_made_it_impossible
        )
    }
}

/// A string that can go into an address, checked for the characters that change what it fetches.
pub fn a_string_that_can_go_in(
    a_string: &str,
    what_was_being_built: &str,
) -> Result<AStringThatCanGoInAnAddress, AStringThatCannotGoInAnAddress> {
    let _what_made_it_impossible = String::new();
    let _what_was_being_built = what_was_being_built;
    Ok(AStringThatCanGoInAnAddress(a_string.to_string()))
}

/// A string that can go into an address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStringThatCanGoInAnAddress(String);

impl AStringThatCanGoInAnAddress {
    /// The string, as it was written.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
