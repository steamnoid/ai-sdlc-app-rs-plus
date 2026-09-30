//! A comment, turned into what `gh` is run with, so that one really is left.
//!
//! **A port with no answer is a port nothing can reach.** `AGitHubWeWriteTo` had exactly one
//! implementor and it was a test double, so a comment had never been written anywhere — and the use
//! case was proved against a place that accepts anything, which is the shape that hides a broken
//! translation.
//!
//! **No shell, one argument each, and the address is one argument.** A comment body is prose, and
//! prose has dashes and colons and backticks in it; a body re-split into its lines is a body
//! somebody else has assembled.

use aisdlc_run::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhatGitHubWasAskedFor};

/// Writing to GitHub with `gh`, which is a program and not a shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ACommentReachingGitHub;

impl ACommentReachingGitHub {
    /// What `gh` would be run with, one thing each, and nothing at all when there is no comment.
    pub fn the_arguments_of(&self, what_git_hub_is_asked: &WhatGitHubWasAskedFor) -> Vec<String> {
        vec![
            "issue".to_string(),
            "comment".to_string(),
            what_git_hub_is_asked
                .where_it_is_going()
                .as_str()
                .to_string(),
            "--body".to_string(),
            what_git_hub_is_asked.what_is_written().to_string(),
        ]
    }
}

impl AGitHubWeWriteTo for ACommentReachingGitHub {
    /// Run `gh` with no shell, and report whether the comment was left.
    ///
    /// **A refusal comes back as a refusal and not as a quiet success.** A caller told a comment
    /// was written when it was not is a reader of a record that does not exist, and this system's
    /// whole claim is that a record can be followed.
    fn it_is_asked_to(
        &mut self,
        what_git_hub_is_asked: &WhatGitHubWasAskedFor,
    ) -> Result<(), String> {
        let the_arguments = self.the_arguments_of(what_git_hub_is_asked);
        let what_it_ran = std::process::Command::new("gh")
            .args(&the_arguments)
            .output();
        let Ok(what_it_ran) = what_it_ran else {
            return Err(String::from(
                "gh could not be started, so no comment was left",
            ));
        };
        if what_it_ran.status.success() {
            return Ok(());
        }
        let why_not = String::from_utf8_lossy(&what_it_ran.stderr);
        Err(format!(
            "gh refused to leave the comment, and said: {}",
            why_not.trim()
        ))
    }
}
