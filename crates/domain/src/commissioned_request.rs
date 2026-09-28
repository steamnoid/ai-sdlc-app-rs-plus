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

impl TryFrom<(String, String)> for CommissionedRequest {
    type Error = ARequestThatSaysNothing;

    fn try_from(the_words: (String, String)) -> Result<Self, Self::Error> {
        let (title, body) = the_words;
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
