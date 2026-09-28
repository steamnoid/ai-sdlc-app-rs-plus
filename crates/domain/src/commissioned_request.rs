//! What somebody asked for, in the words they used.
//!
//! **Text, not an address.** The words arrive with the work, the work item keeps them, and a
//! stage that reads them nine reads later reads what the first one read.

use std::fmt;

/// Somebody's request, and never a bare string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommissionedRequest {
    /// What they called it.
    pub title: String,
    /// What they said.
    pub body: String,
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
        Ok(CommissionedRequest { title, body })
    }
}

impl fmt::Display for ARequestThatSaysNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a request with no title and no body says nothing was asked for")
    }
}

impl std::error::Error for ARequestThatSaysNothing {}
