//! Writing to GitHub, which always needs a credential and is therefore a different port from
//! reading.
//!
//! **A comment is this system's handwriting and a reader cannot tell it from a person's.** So
//! every comment carries who wrote it, and the name is one this system was told — because a
//! comment that claims a person wrote it is the same forgery as a record naming a person for a
//! decision no person made.

use aisdlc_domain::approval::WhoDecided;
use aisdlc_domain::commissioned_request::AWhereItWasFiled;

/// One thing asked of GitHub, carrying the address, the words and the author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatGitHubWasAskedFor {
    where_it_is_going: AWhereItWasFiled,
    what_is_written: String,
    who_wrote_this: String,
}

impl WhatGitHubWasAskedFor {
    /// Where this is going, word for word as the filing returned it.
    pub fn where_it_is_going(&self) -> &AWhereItWasFiled {
        &self.where_it_is_going
    }

    /// The words being left, unaltered.
    pub fn what_is_written(&self) -> &str {
        &self.what_is_written
    }

    /// Who wrote this, and a reader who cannot weigh a comment will read it as a person.
    pub fn who_wrote_this(&self) -> &str {
        &self.who_wrote_this
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
