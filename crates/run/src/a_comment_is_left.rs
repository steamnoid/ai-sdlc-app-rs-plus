//! A run leaves a comment on the filing its work answers.
//!
//! **A request nobody filed has nowhere to be commented on, and that is a refusal and not a
//! comment on a made-up place.** A comment on a composed address is a comment on a page that does
//! not exist, and **a link to nothing in an audit trail is worse than no link at all.**

use aisdlc_domain::commissioned_request::CommissionedRequest;

use crate::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhatGitHubWasAskedFor};
use crate::who_signs_a_comment::WhoSignsAComment;

/// A request that was filed nowhere, so there is nowhere to leave a comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ANowhereToLeaveAComment {
    the_work: String,
}

impl ANowhereToLeaveAComment {
    /// Why the comment was refused, naming the work and the fact that nobody filed it.
    ///
    /// **Two reasons land here and they are not the same thing**: a request nobody filed, and a
    /// write that failed. **The reason is the string either way**, because a caller reading one
    /// message can act on it and a caller reading a category has to ask which.
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

/// The run leaves a comment, refusing a request nobody filed **before it writes anything**.
///
/// **The order is the rule, and it is the destination guard's rule again.** A refusal that arrives
/// after the write is a comment nobody can take back, so the filing is asked about first and the
/// comment is only written once there is somewhere to write it.
pub fn the_run_leaves_a_comment(
    somewhere: &mut impl AGitHubWeWriteTo,
    the_request: &CommissionedRequest,
    what_it_would_say: &str,
    who_wrote_this: &WhoSignsAComment,
) -> Result<(), ANowhereToLeaveAComment> {
    let Some(filed_as) = the_request.where_it_was_filed() else {
        return Err(ANowhereToLeaveAComment::because(the_request));
    };
    let what_it_is_asked_for =
        WhatGitHubWasAskedFor::a_comment_at(filed_as.clone(), what_it_would_say, who_wrote_this);
    somewhere
        .it_is_asked_to(&what_it_is_asked_for)
        .map_err(|why_it_failed| ANowhereToLeaveAComment {
            the_work: why_it_failed,
        })
}
