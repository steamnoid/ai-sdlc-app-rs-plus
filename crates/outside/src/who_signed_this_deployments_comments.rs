//! The name on a comment, read out of the deployment that is writing it.
//!
//! **A port that answers is not a thing anybody calls.** `ACommentReachingGitHub` existed and
//! `WhoSignsAComment` existed, and until this reader existed there was no way for a deployment to
//! put itself on a comment — so the only way to reach the adapter was to invent a name, and
//! inventing a name is the forgery `WhoSignsAComment` was written to remove.
//!
//! **An absent variable is a refusal and not a default, and the refusal comes before the write.** A
//! comment with an invented author is worse than no comment, because it looks like a record.

use aisdlc_run::who_signs_a_comment::WhoSignsAComment;

use crate::the_environment::TheEnvironment;

/// Who signs the comments this deployment writes, or the variable it left unset.
///
/// **The name is a fact about the deployment and never a value this system chose**, for the same
/// reason the credential is not invented: a comment with an author nobody chose is a record that
/// looks like evidence.
pub fn who_signed_these_comments(
    what_the_process_was_told: &TheEnvironment,
) -> Result<WhoSignsAComment, String> {
    let what_it_named = what_the_process_was_told
        .the_variable_a_deployment_set("AISDLC_WHO_SIGNS_A_COMMENT")
        .ok_or_else(|| {
            String::from(
                "AISDLC_WHO_SIGNS_A_COMMENT is not set, and a comment signed with a name this \
                 system chose is a record that looks like evidence",
            )
        })?;
    WhoSignsAComment::the_name_a_deployment_used(what_it_named).map_err(|why_not| {
        format!(
            "AISDLC_WHO_SIGNS_A_COMMENT is '{}', and a deployment that named nobody cannot have a \
             comment written under a name this system chose",
            why_not.0
        )
    })
}
