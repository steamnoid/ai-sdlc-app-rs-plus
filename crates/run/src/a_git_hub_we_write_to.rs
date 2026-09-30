//! Writing to GitHub, which always needs a credential and is therefore a different port from
//! reading.
//!
//! **A comment is this system's handwriting and a reader cannot tell it from a person's.** So
//! every comment carries who wrote it, and the name is one this system was told — because a
//! comment that claims a person wrote it is the same forgery as a record naming a person for a
//! decision no person made.

use aisdlc_domain::commissioned_request::AWhereItWasFiled;

/// One thing asked of GitHub, carrying the address, the words and the author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatGitHubWasAskedFor {
    where_it_is_going: Option<AWhereItWasFiled>,
    what_is_written: String,
    who_wrote_this: String,
    what_it_is_called: WhatKindOfThingGitHubIsAskedFor,
}

/// What kind of thing GitHub is being asked for, **because one port carries two and a run has to
/// be able to tell them apart.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatKindOfThingGitHubIsAskedFor {
    /// A comment on a filing that already exists and therefore has an address.
    AComment,
    /// A filing that does not exist yet and therefore has none.
    AFilingNotYetMade,
}

impl WhatGitHubWasAskedFor {
    /// Whether this is a filing that has not been made yet, **and whether it carries an address.**
    ///
    /// **The two are the same question**, because a filing that existed would have been read first —
    /// so a request that is not a comment has nowhere to go, and that is not a gap in the type.
    pub fn is_a_filing_not_yet_made(&self) -> bool {
        self.what_it_is_called == WhatKindOfThingGitHubIsAskedFor::AFilingNotYetMade
    }

    /// Where this is going, word for word as the filing returned it, **and nothing for a filing that
    /// has not been made.**
    pub fn where_it_is_going(&self) -> Option<&AWhereItWasFiled> {
        self.where_it_is_going.as_ref()
    }

    /// The words being left, unaltered.
    pub fn what_is_written(&self) -> &str {
        &self.what_is_written
    }

    /// Who wrote this, and a reader who cannot weigh a comment will read it as a person.
    pub fn who_wrote_this(&self) -> &str {
        &self.who_wrote_this
    }

    /// One thing to be asked of GitHub, for a use case to hand an implementation.
    ///
    /// **A door and not three public fields**, because a struct whose fields are private to the
    /// crate that declares the port is a port nobody outside it can answer — and the compiler
    /// found that here rather than a reader.
    pub fn a_comment_at(
        where_it_is_going: AWhereItWasFiled,
        what_is_written: &str,
        who_wrote_this: &crate::who_signs_a_comment::WhoSignsAComment,
    ) -> Self {
        Self {
            where_it_is_going: Some(where_it_is_going),
            what_is_written: what_is_written.to_string(),
            who_wrote_this: who_wrote_this.to_string(),
            what_it_is_called: WhatKindOfThingGitHubIsAskedFor::AComment,
        }
    }

    /// A filing that does not exist yet, in the repository it will be filed in and not at an
    /// address.
    pub fn a_filing_in(
        of_which_repository: String,
        what_is_written: String,
        who_wrote_this: &crate::who_signs_a_comment::WhoSignsAComment,
    ) -> Self {
        let _ = of_which_repository;
        Self {
            where_it_is_going: None,
            what_is_written,
            who_wrote_this: who_wrote_this.to_string(),
            what_it_is_called: WhatKindOfThingGitHubIsAskedFor::AFilingNotYetMade,
        }
    }
}

/// Writing to GitHub, which is where a branch, a pull request and a comment all go.
pub trait AGitHubWeWriteTo {
    /// Do what it was asked, or say why it could not be done.
    fn it_is_asked_to(
        &mut self,
        what_it_is_asked_for: &WhatGitHubWasAskedFor,
    ) -> Result<(), String>;
}
