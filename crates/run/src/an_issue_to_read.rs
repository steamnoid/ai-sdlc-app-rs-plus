//! Which issue, of which repository, and the refusals before a request is made.
//!
//! **A number and not an address, because a number is what a person types.** GitHub numbers issues
//! from one, so a number that could not be one has no address — and asking anyway turns a typo into
//! a refusal that reads as a missing request, which is a person being told their filing does not
//! exist when it does.
//!
//! **Refused here rather than sent**, because a mistyped number and a missing issue answer
//! identically from the far end, and a run that could not tell them apart would report "no such
//! issue" for somebody who simply typed `7 `.

use aisdlc_domain::repository::Repository;

/// A number that could not be an issue number, and the number that was given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ANumberThatIsNotAnIssueNumber {
    what_was_typed: i64,
}

impl ANumberThatIsNotAnIssueNumber {
    /// A number that could not be an issue number, and the number that was typed.
    pub fn of(what_was_typed: i64) -> Self {
        ANumberThatIsNotAnIssueNumber { what_was_typed }
    }
}

impl std::fmt::Display for ANumberThatIsNotAnIssueNumber {
    /// What was typed, and why it cannot be an issue number.
    ///
    /// **The number is in the message and not only in the field,** because a person reading a
    /// refusal is told what they typed and can act on it, while a caller reading a field has to
    /// know which field to look at.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} is not an issue number: GitHub numbers issues from one, and a number that is not \
             one has no address, so asking anyway would answer a typo in the reading of a filing \
             that exists",
            self.what_was_typed
        )
    }
}

/// Which issue of which repository, as a person named it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnIssueToRead {
    of_which_repository: Repository,
    which_issue: i64,
}

/// An address GitHub composed that is not an issue address, **quoted whole.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAddressThatIsNotAnIssue(pub String);

impl std::fmt::Display for AnAddressThatIsNotAnIssue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:?} is not an address of an issue — an issue and a pull request both have a number and \
             only one of them is the thing a filing produced",
            self.0
        )
    }
}

impl AnIssueToRead {
    /// The issue a person named, refused when the number could not be one.
    ///
    /// **Refused here rather than sent**, because a mistyped number and a missing issue answer
    /// identically, and a run that could not tell them apart would report "no such issue" for
    /// somebody who simply typed `7 `.
    pub fn the_issue_to_read(
        of_which_repository: Repository,
        which_issue: i64,
    ) -> Result<Self, ANumberThatIsNotAnIssueNumber> {
        if which_issue < 1 {
            return Err(ANumberThatIsNotAnIssueNumber::of(which_issue));
        }
        Ok(AnIssueToRead {
            of_which_repository,
            which_issue,
        })
    }

    /// The issue GitHub answered with, **read out of the address it composed.**
    ///
    /// **The address is the primary fact and the number is a reading of it** — the other direction
    /// is what `CommissionedIssue` did, **and an address composed from a number resolves to a page
    /// that does not exist while looking exactly like a link.**
    ///
    /// **An address that is not an issue is refused**, because `…/issues/10` and `…/pull/10` are
    /// different things **and a work item pointed at the wrong one is a work item about somebody
    /// else's pull request, with a number that was never wrong and an address that was.**
    ///
    /// **Refused for now** — the reading exists and the shape of the answer does not.
    pub fn the_issue_an_address_returned(
        of_which_repository: Repository,
        what_github_answered: &str,
    ) -> Result<Self, AnAddressThatIsNotAnIssue> {
        let _ = (of_which_repository, what_github_answered);
        Err(AnAddressThatIsNotAnIssue(String::from(
            what_github_answered,
        )))
    }

    /// Which issue was asked for, as a person typed it.
    pub fn which_issue(&self) -> i64 {
        self.which_issue
    }

    /// Which repository it is an issue of.
    pub fn of_which_repository(&self) -> &Repository {
        &self.of_which_repository
    }
}
