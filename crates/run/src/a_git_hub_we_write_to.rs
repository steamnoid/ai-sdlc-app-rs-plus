//! Writing to GitHub, which always needs a credential and is therefore a different port from
//! reading.
//!
//! **A comment is this system's handwriting and a reader cannot tell it from a person's.** So
//! every comment carries who wrote it, and the name is one this system was told — because a
//! comment that claims a person wrote it is the same forgery as a record naming a person for a
//! decision no person made.

use aisdlc_domain::commissioned_request::AWhereItWasFiled;

/// What GitHub answered, and the address it answered with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereGitHubAnswered {
    the_address: AWhereItWasFiled,
}

impl WhereGitHubAnswered {
    /// GitHub's answer, at the address it composed.
    pub fn an_answer_at(where_github_says_it_went: &str) -> Self {
        WhereGitHubAnswered {
            the_address: AWhereItWasFiled::the_address_the_filing_returned(
                where_github_says_it_went,
            )
            .expect("GitHub answered with an address"),
        }
    }

    /// GitHub's answer, at the address it composed.
    pub fn an_address_at(what_github_said: &str) -> Self {
        WhereGitHubAnswered {
            the_address: AWhereItWasFiled::the_address_the_filing_returned(what_github_said)
                .expect("GitHub answered with an address"),
        }
    }

    /// The address, word for word as GitHub composed it.
    pub fn the_address(&self) -> &AWhereItWasFiled {
        &self.the_address
    }
}

/// One thing asked of GitHub, carrying the address, the words and the author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatGitHubWasAskedFor {
    where_it_is_going: Option<AWhereItWasFiled>,
    which_repository_it_is_filed_in: Option<aisdlc_domain::repository::Repository>,
    what_is_written: String,
    who_wrote_this: String,
    what_it_is_called: WhatKindOfThingGitHubIsAskedFor,
    what_the_filing_is_called: Option<String>,
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
            which_repository_it_is_filed_in: None,
            what_is_written: what_is_written.to_string(),
            who_wrote_this: who_wrote_this.to_string(),
            what_it_is_called: WhatKindOfThingGitHubIsAskedFor::AComment,
            what_the_filing_is_called: None,
        }
    }

    /// A filing that does not exist yet, **in the repository it will be filed in and not at an
    /// address.**
    ///
    /// **No signer and not a signer, because a filing's author is not a fact about the deployment.**
    /// A comment's writer comes from configuration, because a person's name does; **a filing this
    /// system writes is by definition written by this system**, and a signer parameter here would be
    /// a way for a caller to write somebody else's name on a filing nobody else wrote.
    pub fn a_filing_in(
        of_which_repository: &aisdlc_domain::repository::Repository,
        what_the_filing_is_called: String,
        what_is_written: String,
    ) -> Self {
        Self {
            where_it_is_going: None,
            which_repository_it_is_filed_in: Some(of_which_repository.clone()),
            what_is_written,
            who_wrote_this: A_FILING_THIS_SYSTEM_WROTE.to_string(),
            what_it_is_called: WhatKindOfThingGitHubIsAskedFor::AFilingNotYetMade,
            what_the_filing_is_called: Some(what_the_filing_is_called),
        }
    }

    /// What this filing is called, **and nothing for a comment.**
    ///
    /// **A title and a body and no title is a filing nobody can find in a list of fifty**, and an
    /// issue titled with the first words of its own paragraph is an issue a person cannot scan.
    pub fn what_the_filing_is_called(&self) -> Option<&str> {
        self.what_the_filing_is_called.as_deref()
    }

    /// Which repository this filing is filed in, **and nothing for a comment.**
    ///
    /// **`gh issue create` is asked for a repository by name and not by address**, because the filing
    /// does not exist yet — **so the repository and the address are two different facts and only one
    /// of them is known before the filing is made.**
    pub fn which_repository_it_is_filed_in(
        &self,
    ) -> Option<&aisdlc_domain::repository::Repository> {
        self.which_repository_it_is_filed_in.as_ref()
    }
}

/// Who a filing this system wrote says wrote it, **and it is a fact and not a configuration.**
///
/// **`llm agent` and not a person, ever** — the same rule `autopilot:` follows for an approval and
/// the same one #131 applied to the commissioning party: **a record naming a person for something a
/// system did looks exactly like the real thing at the moment somebody needs to tell them apart.**
/// This one carries a title and a body and an author's name, **which is the shape a reader trusts
/// most.**
pub const A_FILING_THIS_SYSTEM_WROTE: &str = "llm agent";

/// Writing to GitHub, which is where a branch, a pull request and a comment all go.
pub trait AGitHubWeWriteTo {
    /// Do what it was asked, **and answer with what GitHub answered.**
    ///
    /// **A port that returns nothing cannot hand back an address**, and a filing whose address was
    /// thrown away is a filing in a real repository that nobody can find — **so the answer travels
    /// back through the port, for a comment as much as for a filing**, because a comment's address
    /// was read earlier and is still the answer to where it went.
    fn it_is_asked_to(
        &mut self,
        what_it_is_asked_for: &WhatGitHubWasAskedFor,
    ) -> Result<WhereGitHubAnswered, String>;
}
