//! An offer becoming a filing, and what GitHub is asked for when the thing does not exist yet.
//!
//! **A comment has an address and a filing does not**, because a filing is what gives itself one —
//! so a request carrying an address for a filing would be a request for a page that does not exist.
//!
//! **The shell for now.** It writes nothing and says nothing — the tests are what demand both.

use crate::a_git_hub_we_write_to::WhatGitHubWasAskedFor;
use crate::what_an_agent_offered::AWorkAnAgentOffered;

/// The filing one offer becomes, **carrying the words and every address the run derived.**
///
/// **The evidence travels with the filing**, because the person who reads it is the one who has to
/// check it, **and a confident claim with no way to check it is worse than no claim at all** — the
/// grounding is invisible to a reader unless the address reaches the page.
pub fn the_filing_an_offer_becomes(
    of_which_repository: &aisdlc_domain::repository::Repository,
    one_offer: &AWorkAnAgentOffered,
) -> WhatGitHubWasAskedFor {
    WhatGitHubWasAskedFor::a_filing_in(of_which_repository, the_body_of(one_offer))
}

/// What the issue will say, **in the order a person reads it: what it is, what it wants, and where
/// to look.**
fn the_body_of(one_offer: &AWorkAnAgentOffered) -> String {
    use std::fmt::Write;
    let mut the_body = one_offer.what_it_asks_for.clone();
    for one_file in &one_offer.what_it_concluded_from {
        let _ = writeln!(the_body, "\n{} — {}", one_file.path, one_file.link);
    }
    the_body
}
