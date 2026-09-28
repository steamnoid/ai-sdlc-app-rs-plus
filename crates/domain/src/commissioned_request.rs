//! What somebody asked for, in the words they used.
//!
//! **Text, not an address, and what this type does not have is most of what it is.** It
//! derives nothing: no repository, no issue number, no URL, no `…/issues/N`. The
//! implementation this domain was read from derived that last one from an owner and a
//! number, and **the repository it was pointed at has no issues at all** — so every
//! summary it produced was full of links to pages that do not exist, and a summary full
//! of them is worse than a summary with none.
//!
//! Two `String`s, two arguments, and **nothing is trimmed, lowercased or re-wrapped**.
//! The words are the words: a title a person typed and a title a run reads back nine reads
//! later are the same title, the space somebody left at the end of it included.
//!
//! Nothing here may reach outside this crate. There is no reading of a clock, no name of
//! a model and no filesystem, and the crate holding this file has no dependencies at all.

use std::fmt;

/// A request with no title and no body, and the two words as they were written.
///
/// **A refusal and not an `Option`, and it quotes both halves back.** Which of the two
/// was empty is the whole of the answer a caller needs before it asks again, and quoting
/// them untrimmed is the same rule a repository's halves follow: a value this system
/// silently tidies is a value a lookup will miss.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARequestThatSaysNothing {
    the_title_written: String,
    the_body_written: String,
}

impl ARequestThatSaysNothing {
    /// The refusal for a request with no title and no body, and both words as written.
    pub fn for_a_title_and_a_body_that_are_both_empty(
        title: &str,
        body: &str,
    ) -> ARequestThatSaysNothing {
        ARequestThatSaysNothing {
            the_title_written: title.to_string(),
            the_body_written: body.to_string(),
        }
    }

    /// The title as the caller wrote it, trimmed by nobody.
    pub fn the_title_written(&self) -> &str {
        &self.the_title_written
    }

    /// The body as the caller wrote it, trimmed by nobody.
    pub fn the_body_written(&self) -> &str {
        &self.the_body_written
    }
}

impl fmt::Display for ARequestThatSaysNothing {
    /// The refusal written out, with both halves in their `Debug` form.
    ///
    /// **The `Debug` form and not the halves themselves**, because a refusal here is
    /// always two halves that are both blank, and a title of spaces written as a body of a
    /// tab is the same sentence twice. The whitespace is still visible as whitespace
    /// characters rather than as an empty field, which is the part that tells a caller it
    /// sent something and that what it sent was nothing.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "a request with the title {:?} and the body {:?} says nothing",
            self.the_title_written, self.the_body_written
        )
    }
}

impl std::error::Error for ARequestThatSaysNothing {}

/// What somebody asked for, in the words they used.
///
/// **Two fields and two arguments, and no `TryFrom<&str>` and no `from`.** A request is
/// two pieces of text and there is no single string that is one, so the absence of a
/// one-argument door is the same point the absence of one makes on a repository.
///
/// **No `Display`, and that is a decision rather than an oversight.** A repository is
/// written `owner/name` and one string is how every one of its five addresses is built;
/// a request is a title and a body, and writing them as one string is the re-wrapping this
/// type exists to refuse. A reader who wants a sentence writes it, with the punctuation
/// and the newlines they mean.
///
/// **The refusal is for a request with no title *and* no body, and nothing else.** A title
/// on its own says what was asked and the body is a question the analysis asks later; the
/// question this type is choosing not to answer is whether a body is long enough to plan
/// against, which is a stage's question and not a type's.
///
/// **`Debug`, `Clone`, `PartialEq` and `Eq`, and no `Ord`, no `PartialOrd` and no `Hash`.**
/// Two requests are equal when their words are equal, which is what lets a run tell a
/// second request from the first. What is refused is order: requests have no order that is
/// not an order on spellings, nothing puts them in a row, and a `Hash` with no key behind
/// it is a door with no room on the other side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommissionedRequest {
    title: String,
    body: String,
}

impl CommissionedRequest {
    /// The request these two pieces of text are, or the refusal when there are none.
    ///
    /// **Both halves are examined for emptiness together, because either one on its own
    /// says what was asked.** A title with no body is a request, and a body with no title
    /// is a request, and refusing either would be refusing a question a person can answer.
    pub fn new(title: &str, body: &str) -> Result<CommissionedRequest, ARequestThatSaysNothing> {
        if title.trim().is_empty() && body.trim().is_empty() {
            return Err(
                ARequestThatSaysNothing::for_a_title_and_a_body_that_are_both_empty(title, body),
            );
        }
        Ok(CommissionedRequest {
            title: title.to_string(),
            body: body.to_string(),
        })
    }

    /// The title somebody used, exactly as they typed it.
    pub fn the_title_of(&self) -> &str {
        &self.title
    }

    /// The body somebody used, exactly as they typed it.
    pub fn the_body_of(&self) -> &str {
        &self.body
    }
}
