//! A public GitHub repository, named by its owner and by its name.
//!
//! A repository is **two fields and not one `owner/name` string**, and the reason is a
//! failure this type is here to prevent: a string carrying a slash is either half a
//! repository with a slash in the wrong place or both halves pasted into one field, and
//! nothing in the string says which. Two fields make the question answerable, and the
//! constructor refuses rather than guessing.
//!
//! **The five addresses of a repository are named in the glossary and are not built
//! here.** `the_address_of`, `the_clone_url_of`, `the_push_address_of`,
//! `the_read_address_of` and `Addresses` are each a function with a caller that does not
//! exist yet, and a function nobody calls is a name waiting to disagree with itself. They
//! arrive with the first caller that needs one, and `Display` is the one string of the
//! five that is already here, because it is the two halves as a person writes them and
//! not a derived address.
//!
//! Nothing here may reach outside this crate. There is no reading of a clock, no name of
//! a model and no filesystem, and the crate holding this file has no dependencies at all.

use std::fmt;

/// The first of the two reasons a half names nothing, in the words the refusal is written
/// in rather than in the words the type is named by.
///
/// **Two reasons and one type.** A half that is blank names nothing, and a half carrying a
/// slash is both halves pasted into one field, and a caller told "that is not a repository"
/// has the same question whichever half it left out — three types would make it choose
/// which one to ask before it knows anything about the answer. What differs between the two
/// is the advice, and the advice belongs where a caller reads it.
const THE_REASON_A_BLANK_HALF_NAMES_NOTHING: &str = "a half that trims to nothing names nothing";

/// The second of the two reasons, in the same words and for the same reader.
const THE_REASON_A_SLASH_IN_A_HALF_IS_BOTH_HALVES: &str =
    "a half carrying a slash is both halves pasted into one field";

/// A repository that is not a repository, and the half that was refused.
///
/// **A refusal and not an `Option`, and it carries the half as it was written.** The words
/// matter for the same reason a model is corrected better when it is handed its own
/// spelling back, and they matter more here: a half this system silently trimmed is a half
/// a lookup will miss, so a caller that sent `"  x  "` is given `"  x  "` back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AHalfThatIdentifiesNothing {
    what_was_looked_for: String,
    why_it_names_nothing: &'static str,
}

impl AHalfThatIdentifiesNothing {
    /// The refusal for a half that is blank, and that half as it was written.
    pub fn for_a_blank_half(half: &str) -> AHalfThatIdentifiesNothing {
        AHalfThatIdentifiesNothing {
            what_was_looked_for: half.to_string(),
            why_it_names_nothing: THE_REASON_A_BLANK_HALF_NAMES_NOTHING,
        }
    }

    /// The refusal for a half carrying a slash, and that half as it was written.
    pub fn for_a_half_carrying_a_slash(half: &str) -> AHalfThatIdentifiesNothing {
        AHalfThatIdentifiesNothing {
            what_was_looked_for: half.to_string(),
            why_it_names_nothing: THE_REASON_A_SLASH_IN_A_HALF_IS_BOTH_HALVES,
        }
    }

    /// The half that was refused, exactly as the caller wrote it.
    pub fn what_was_looked_for(&self) -> &str {
        &self.what_was_looked_for
    }
}

impl fmt::Display for AHalfThatIdentifiesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} is not half a repository: {}",
            self.what_was_looked_for, self.why_it_names_nothing
        )
    }
}

impl std::error::Error for AHalfThatIdentifiesNothing {}

/// A public GitHub repository, named by its owner and by its name.
///
/// **Two fields rather than one string, and the constructor takes two arguments rather
/// than one.** There is no `TryFrom<&str>` and there is no `from`, and the absence is the
/// point: a repository is two halves, so no single string is one, and a one-argument door
/// would have to guess where the split falls on every call. `github/rails` is refused
/// rather than split, because a caller who pasted both halves into one field has said
/// something about their own input and deserves to be told what they said.
///
/// **`Debug`, `Clone`, `PartialEq` and `Eq`, and no `Ord`, no `PartialOrd` and no `Hash`.**
/// A repository is a name somebody typed, and two names that are equal are the same
/// repository — a register that cannot compare two cannot tell a second run from the
/// first. What is refused is order: alphabetical order is an order on spellings and not on
/// repositories, nothing in the domain puts repositories in a row, and a `Hash` with no key
/// behind it is a door with no room on the other side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    owner: String,
    name: String,
}

impl Repository {
    /// The repository these two halves name, or the refusal naming the half that does not.
    ///
    /// **A slash is refused before either half is examined for being blank**, so a half
    /// that is only a slash is answered with the more specific of the two reasons rather
    /// than with whichever test happened to run first.
    pub fn new(owner: &str, name: &str) -> Result<Repository, AHalfThatIdentifiesNothing> {
        if owner.contains('/') || name.contains('/') {
            return Err(AHalfThatIdentifiesNothing::for_a_half_carrying_a_slash(
                the_half_of_the_two_carrying_the_slash(owner, name),
            ));
        }
        if owner.trim().is_empty() || name.trim().is_empty() {
            return Err(AHalfThatIdentifiesNothing::for_a_blank_half(
                the_blank_half_of_the_two(owner, name),
            ));
        }
        Ok(Repository {
            owner: owner.to_string(),
            name: name.to_string(),
        })
    }
}

/// The half of the two carrying a slash, which is the one the refusal quotes back.
///
/// **The owner wins a tie**, and that is a decision rather than an accident: a pair where
/// both halves carry a slash is refused either way, and the one being reported is the
/// first one a reader of the two would go and look at.
fn the_half_of_the_two_carrying_the_slash<'as_written>(
    owner: &'as_written str,
    name: &'as_written str,
) -> &'as_written str {
    if owner.contains('/') {
        return owner;
    }
    name
}

/// The half of the two that is blank, which is the one the refusal quotes back.
fn the_blank_half_of_the_two<'as_written>(
    owner: &'as_written str,
    name: &'as_written str,
) -> &'as_written str {
    if owner.trim().is_empty() {
        return owner;
    }
    name
}

impl fmt::Display for Repository {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.owner, self.name)
    }
}
