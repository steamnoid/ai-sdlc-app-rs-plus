//! A filing reaching GitHub, and the address it printed.
//!
//! **The shell for now.** It runs nothing and reads nothing — the tests are what demand both.

use aisdlc_run::a_git_hub_we_write_to::WhatGitHubWasAskedFor;

/// A filing GitHub made no address for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFilingWithNowhereToPoint(pub String);

/// Writing a filing to GitHub with `gh`, which is a program and not a shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AFilingReachingGitHub;

impl AFilingReachingGitHub {
    /// What `gh` would be run with, **nothing for now.**
    pub fn the_arguments_of(&self, what_git_hub_is_asked: &WhatGitHubWasAskedFor) -> Vec<String> {
        let _ = what_git_hub_is_asked;
        Vec::new()
    }

    /// The address GitHub printed, **refused for now.**
    pub fn what_the_address_it_returned(
        &self,
        what_github_printed: &str,
    ) -> Result<aisdlc_domain::commissioned_request::AWhereItWasFiled, AFilingWithNowhereToPoint>
    {
        let _ = what_github_printed;
        Err(AFilingWithNowhereToPoint(String::from(what_github_printed)))
    }
}
