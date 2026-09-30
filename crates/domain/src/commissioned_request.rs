//! What somebody asked for, in the words they used, and where those words were typed.
//!
//! **Text first, and the filing beside the words rather than instead of them.** The words are the
//! request; the filing is where they were typed. A stage that reads the words nine times must
//! read the same nine times, and a type that carried a number instead of a sentence made a
//! product analyst read the repository while the sentence went to the planner.
//!
//! **An address this system composed would be a link to a page it had not checked.** The only door
//! here takes a whole address, and a derived `…/issues/N` resolves to a page that does not exist
//! in a repository with no issues — a false reference in the audit trail is the one thing a
//! traceability chain exists to prevent.
//!
//! **And nothing watches for a second door.** Adding `an_address_composed_of(what, 41)` to this
//! module was tried and every test in this file still passed, so the claim above is held by the
//! code and not by the suite. **A claim about a door that does not exist is the one kind of claim
//! a test cannot make from the outside**, because it has nothing to name.

use std::fmt;

/// Where a request was filed, as the address the filing itself returned.
///
/// **No `Debug`, and this is the second type in the repository with that.** A filing address is
/// what a work item is traced by, and a derived `Debug` on a value a run prints and records is one
/// brace away from a log — **so the declaration carries no derive and the only text it can become
/// is its own `Display`**, which is the address because an address is not a secret and pretending
/// otherwise would make the audit trail unreadable.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AWhereItWasFiled(String);

impl std::fmt::Debug for AWhereItWasFiled {
    /// The address in full, because an audit trail a reader cannot follow is not an audit trail.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, formatter)
    }
}

impl AWhereItWasFiled {
    /// The address a filing returned, or a refusal when it was not one.
    ///
    /// **A whole address and not a number to build one from.** The composition is the whole defect
    /// and this is the only door it has — **and see the module's own note: nothing watches for a
    /// second one.**
    pub fn the_address_the_filing_returned(
        the_address: &str,
    ) -> Result<Self, AWhereItWasFiledThatNamesNothing> {
        if the_address.trim().is_empty() {
            return Err(AWhereItWasFiledThatNamesNothing(the_address.to_string()));
        }
        Ok(AWhereItWasFiled(the_address.to_string()))
    }

    /// The address, word for word as the filing returned it.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AWhereItWasFiled {
    /// The address in full, because **an audit trail a reader cannot follow is not an audit
    /// trail** — and an address is not a secret, so hiding it protects nothing and costs the one
    /// thing a filing is for.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An address that names nothing, and a link to nothing in an audit trail is worse than no link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWhereItWasFiledThatNamesNothing(pub String);

impl fmt::Display for AWhereItWasFiledThatNamesNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} names no place a request was filed", self.0)
    }
}

impl std::error::Error for AWhereItWasFiledThatNamesNothing {}

/// Somebody's request, and never a bare string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommissionedRequest {
    /// What they called it.
    pub title: String,
    /// What they said.
    pub body: String,
    /// Where it was filed, and nothing at all when nobody filed it.
    pub filed_as: Option<AWhereItWasFiled>,
}

/// A request with no title and no body, so nothing was asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARequestThatSaysNothing;

impl CommissionedRequest {
    /// What somebody asked for, in the words they used, or a refusal when they said nothing.
    ///
    /// **A named door and not a tuple**, because `asked_for(body, title)` compiles and is
    /// wrong: two fields of the same type, in an order nobody has to look at. A caller who
    /// swaps them gets a request whose title is a paragraph, and nothing anywhere reports
    /// it. Two names the reader can see beat one type the compiler cannot check.
    pub fn asked_for(title: String, body: String) -> Result<Self, ARequestThatSaysNothing> {
        let says_nothing = title.trim().is_empty() && body.trim().is_empty();
        if says_nothing {
            return Err(ARequestThatSaysNothing);
        }
        Ok(CommissionedRequest {
            title,
            body,
            filed_as: None,
        })
    }

    /// What somebody asked for, in the words they used, and where it was filed.
    ///
    /// **A second door and not a settable field**, so a caller cannot fill the address in after
    /// the fact from a number it happened to have.
    pub fn asked_for_and_filed_at(
        title: String,
        body: String,
        filed_as: AWhereItWasFiled,
    ) -> Result<Self, ARequestThatSaysNothing> {
        let the_request = Self::asked_for(title, body)?;
        Ok(CommissionedRequest {
            filed_as: Some(filed_as),
            ..the_request
        })
    }

    /// What was asked for, in the words somebody used.
    ///
    /// **The product analyst reads this and nothing else can**, and a request that could not be read
    /// is a request no stage after filing has anything to work on.
    pub fn what_was_asked(&self) -> &str {
        &self.title
    }

    /// What was said besides the ask, **which may be nothing.**
    pub fn what_was_said_besides(&self) -> &str {
        &self.body
    }

    /// Where this request was filed, and nothing when nobody filed it.
    pub fn where_it_was_filed(&self) -> Option<&AWhereItWasFiled> {
        self.filed_as.as_ref()
    }
}

impl fmt::Display for ARequestThatSaysNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a request with no title and no body says nothing was asked for")
    }
}

impl std::error::Error for ARequestThatSaysNothing {}
