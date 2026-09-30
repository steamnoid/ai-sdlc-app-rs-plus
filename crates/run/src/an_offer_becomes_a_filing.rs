//! An offer becoming a filing, and what GitHub is asked for when the thing does not exist yet.
//!
//! **A comment has an address and a filing does not**, because a filing is what gives itself one —
//! so a request carrying an address for a filing would be a request for a page that does not exist.
//!
//! **The shell for now.** It writes nothing and says nothing — the tests are what demand both.

use crate::a_git_hub_we_write_to::WhatGitHubWasAskedFor;
use crate::what_an_agent_offered::AWorkAnAgentOffered;

/// An offer that could not become a filing, quoting what the offer was.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnOfferThatCouldNotBeFiled(pub String);

/// The filing one offer becomes, **with nothing written, no address, and a person's name on it.**
///
/// **A `String` and not a filing, because the type is the whole claim**: a filing that carried an
/// address would be a filing for a page that does not exist yet.
pub fn the_filing_an_offer_becomes(one_offer: &AWorkAnAgentOffered) -> WhatGitHubWasAskedFor {
    let _ = one_offer;
    WhatGitHubWasAskedFor::a_filing_in(
        String::new(),
        String::new(),
        &crate::who_signs_a_comment::WhoSignsAComment::the_name_a_deployment_used("a person")
            .expect("a name"),
    )
}
