//! A filing reaching GitHub, and the address it printed.
//!
//! **A comment has nowhere new to point and a filing has exactly one place to point.** `gh issue
//! create` prints the address of what it made, **and a run that filed an issue into a real repository
//! and threw the address away has done the one thing a person cannot undo and cannot find** — so the
//! address comes back out of here and nothing else produces one.
//!
//! **No shell, and the repository and the title and the body are one argument each.** A title has
//! dashes and colons in it and a body is prose with backticks; **one string re-split into two is one
//! string somebody else assembled.**

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_run::a_git_hub_we_write_to::WhatGitHubWasAskedFor;

/// A filing GitHub made no address for, quoting what it said instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFilingWithNowhereToPoint(pub String);

impl std::fmt::Display for AFilingWithNowhereToPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "the filing was made and GitHub said {:?}, and a filing with no address is a filing in a \
             repository nobody can find — there is exactly one address of an issue and it is the one \
             `gh` prints",
            self.0
        )
    }
}

/// Writing a filing to GitHub with `gh`, which is a program and not a shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AFilingReachingGitHub;

impl AFilingReachingGitHub {
    /// What `gh` would be run with, one thing each.
    ///
    /// **`--repo` and not an address, because the filing does not exist yet.** The repository and the
    /// address are two different facts and **only one of them is known before the filing is made.**
    pub fn the_arguments_of(&self, what_git_hub_is_asked: &WhatGitHubWasAskedFor) -> Vec<String> {
        let Some(of_which_repository) = what_git_hub_is_asked.which_repository_it_is_filed_in()
        else {
            return Vec::new();
        };
        let Some(what_the_filing_is_called) = what_git_hub_is_asked.what_the_filing_is_called()
        else {
            return Vec::new();
        };
        vec![
            String::from("issue"),
            String::from("create"),
            String::from("--repo"),
            of_which_repository.to_string(),
            String::from("--title"),
            String::from(what_the_filing_is_called),
            String::from("--body"),
            what_git_hub_is_asked.what_is_written().to_string(),
        ]
    }

    /// The address GitHub printed, **and a refusal when it printed nothing.**
    ///
    /// **GitHub's words and not one this system composed from a repository and a number** — that
    /// composition is what `CommissionedIssue` did, and it resolved to a page that did not exist
    /// while looking exactly like a link.
    pub fn what_the_address_it_returned(
        &self,
        what_github_printed: &str,
    ) -> Result<AWhereItWasFiled, AFilingWithNowhereToPoint> {
        let what_it_printed = what_github_printed.trim();
        if what_it_printed.is_empty() {
            return Err(AFilingWithNowhereToPoint(String::from("nothing at all")));
        }
        AWhereItWasFiled::the_address_the_filing_returned(what_it_printed)
            .map_err(|named_nothing| AFilingWithNowhereToPoint(named_nothing.to_string()))
    }

    /// The filing this makes, **and the address it comes back with.**
    ///
    /// **A refusal comes back as a refusal and not as a quiet success**, and it quotes `gh` —
    /// **because a caller told a filing was made when it was not is a person reading a record of
    /// something that does not exist.** This is the tier that runs the program; describing the
    /// arguments is not running them.
    pub fn the_filing_it_makes(
        &self,
        what_git_hub_is_asked: &WhatGitHubWasAskedFor,
    ) -> Result<AWhereItWasFiled, AFilingWithNowhereToPoint> {
        let what_it_ran = std::process::Command::new("gh")
            .args(self.the_arguments_of(what_git_hub_is_asked))
            .output()
            .map_err(|why_it_could_not_run| {
                AFilingWithNowhereToPoint(why_it_could_not_run.to_string())
            })?;
        if !what_it_ran.status.success() {
            return Err(AFilingWithNowhereToPoint(
                String::from_utf8_lossy(&what_it_ran.stderr).to_string(),
            ));
        }
        self.what_the_address_it_returned(&String::from_utf8_lossy(&what_it_ran.stdout))
    }
}
