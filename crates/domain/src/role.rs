//! The disciplines accountable for a work item, and the closed set of them.
//!
//! A role is never a bare string, and it is written with its own name in capitals. The
//! variant is the discipline and the writing of it is the capitals, so the two are one
//! concept with two spellings and the capitals exist in exactly one place.
//!
//! Nothing here may reach outside this crate. There is no parsing of a file, no reading of
//! a clock and no name of a model, and the crate holding this file has no dependencies at
//! all — which is the rule every other boundary in the repository rests on.

use std::convert::TryFrom;
use std::fmt;

/// Every role, in the order the specification lists them.
///
/// The specification's order rather than any other, so a caller crossing one thing with
/// every role does not write a second list to keep in step. The tests transcribe the five
/// rather than import this, because a test that reads its own expectation from the code it
/// checks proves nothing.
///
/// **The name is in capitals because Rust writes a constant that way**, and an `allow` is
/// refused in this repository, so the writing is the one thing a constant cannot opt out
/// of. The glossary's `EveryRole` normalises to the same name — it keeps letters and
/// digits and drops the rest — so the two are one name to the vocabulary guard.
pub const EVERY_ROLE: [Role; 5] = [
    Role::Product,
    Role::Development,
    Role::Quality,
    Role::Security,
    Role::Architecture,
];

/// A name that is not one of the five roles.
///
/// **A refusal and not an `Option`, and it carries the name that was looked for.** A
/// caller asking "is this a role" has the same question whichever way the answer came
/// out, and an `Option` would make it choose which of two shapes to hold before it knows
/// anything. The name matters for the same reason a model is corrected better when it is
/// handed its own spelling back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARoleThatNobodyNamed {
    what_was_looked_for: String,
}

impl ARoleThatNobodyNamed {
    /// The refusal for a name that is not one of the five.
    pub fn for_a_name(name: &str) -> ARoleThatNobodyNamed {
        ARoleThatNobodyNamed {
            what_was_looked_for: name.to_string(),
        }
    }

    /// The name that was looked for and is not one of the five.
    pub fn what_was_looked_for(&self) -> &str {
        &self.what_was_looked_for
    }
}

impl fmt::Display for ARoleThatNobodyNamed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} is not one of the five roles",
            self.what_was_looked_for
        )
    }
}

impl std::error::Error for ARoleThatNobodyNamed {}

/// The discipline accountable for a work item.
///
/// **The variant is the discipline and the capitals are the writing, and they are two
/// spellings of one concept rather than two concepts.** `Product` and `PO` answer the same
/// question about a work item, and a reader who had to remember which spelling answers it
/// would be holding a distinction the domain does not make. The writing is what a
/// specification, a prompt or a document says, and the variant is what Rust is able to
/// hold, and one `Display` and one `TryFrom` join them.
///
/// **The variant is the discipline word rather than the acronym, and the gate is what
/// decided it.** `cargo clippy --workspace --all-targets -- -D warnings` denies
/// `upper_case_acronyms`, and that lint refuses `DEV`, `SEC` and `ARCH` as variant names
/// on this toolchain with "contains a capitalized acronym". A `#[allow]` for it is
/// refused by `tests/no_allow_silences_a_lint`, so the variant could not have been the
/// acronym even for one member.
///
/// **Two of the five would have slipped through, and that is the more interesting half.**
/// `PO` and `QA` are two letters, the lint ignores an acronym that short, and a variant
/// written `PO` is accepted. So the reason the set is written one way rather than two is
/// not that every acronym is refused — writing `PO` and `QA` as variants and `Product` and
/// `Quality` as variants, on the strength of a length, is a spelling a reader has to look
/// up to learn which rule decided it. The glossary's table gives each discipline in the
/// same row as the acronym it writes, and one round — the discipline is the variant and
/// the acronym is the writing — is the one `Stage` uses as well.
///
/// There is no `AI`. A handoff is not a discipline, and a set whose own definition
/// excludes one of its members is not a set of disciplines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    /// Product: accountable for what should be built.
    Product,
    /// Development: accountable for code.
    Development,
    /// Quality: accountable for test strategy and verification.
    Quality,
    /// Security: accountable for risk.
    Security,
    /// Architecture: accountable for structure.
    Architecture,
}

impl TryFrom<&str> for Role {
    type Error = ARoleThatNobodyNamed;

    /// The role this name writes, or the refusal quoting the name back.
    ///
    /// **The name is compared exactly as written, and a name with a space in it is a
    /// different name.** A reading that trimmed would answer about a role nobody wrote
    /// down, and the caller would never learn that its own spelling was wrong — which is
    /// the one thing a model writing a role name most needs to be told.
    ///
    /// The discipline word is refused as well, and on purpose: `Product` is the variant,
    /// not the written form, so a name that parses has to be the acronym a document
    /// writes. The vocabulary guard normalises a name to letters and digits, so to it
    /// `Product` and `PO` are one name — which is precisely why the two cannot also be
    /// two names to anything that reads a name as a name.
    fn try_from(name: &str) -> Result<Role, ARoleThatNobodyNamed> {
        let role = match name {
            "PO" => Role::Product,
            "DEV" => Role::Development,
            "QA" => Role::Quality,
            "SEC" => Role::Security,
            "ARCH" => Role::Architecture,
            _ => return Err(ARoleThatNobodyNamed::for_a_name(name)),
        };
        Ok(role)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let written = match self {
            Role::Product => "PO",
            Role::Development => "DEV",
            Role::Quality => "QA",
            Role::Security => "SEC",
            Role::Architecture => "ARCH",
        };
        write!(formatter, "{written}")
    }
}
