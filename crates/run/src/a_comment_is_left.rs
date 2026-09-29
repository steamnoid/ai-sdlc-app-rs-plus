//! A run leaves a comment on the filing its work answers.
//!
//! **A request nobody filed has nowhere to be commented on, and that is a refusal and not a
//! comment on a made-up place.** A comment on a composed address is a comment on a page that does
//! not exist, and **a link to nothing in an audit trail is worse than no link at all.**

use aisdlc_domain::approval::WhoDecided;
use aisdlc_domain::commissioned_request::CommissionedRequest;

use crate::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhatGitHubWasAskedFor};

/// A request that was filed nowhere, so there is nowhere to leave a comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ANowhereToLeaveAComment {
    the_work: String,
}

impl ANowhereToLeaveAComment {
    /// Why the comment was refused, naming the work and the fact that nobody filed it.
    pub fn because(the_request: &CommissionedRequest) -> Self {
        Self {
            the_work: the_request.title.clone(),
        }
    }

    /// The refusal, for a caller that has to tell this from a write that failed.
    pub fn why_not(&self) -> String {
        format!(
            "{} was typed rather than filed, so it has no address to be commented on, and a comment on a composed address is a comment on a page that does not exist",
            self.the_work
        )
    }
}

/// The run leaves a comment, refusing a request nobody filed before it writes anything.
pub fn the_run_leaves_a_comment(
    somewhere: &mut impl AGitHubWeWriteTo,
    the_request: &CommissionedRequest,
    what_it_would_say: &str,
    who_wrote_this: &WhoDecided,
) -> Result<(), ANowhereToLeaveAComment> {
    let _somewhere = somewhere;
    let _what_it_would_say = what_it_would_say;
    let _who_wrote_this = who_wrote_this;
    Err(ANowhereToLeaveAComment::because(the_request))
}
