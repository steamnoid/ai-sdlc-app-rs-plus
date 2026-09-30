//! A creative agent filing what it concluded, and getting each address back.
//!
//! **A port that returns nothing cannot hand back an address**, and a filing whose address was
//! thrown away is a filing in a real repository that nobody can find — **so `it_is_asked_to` answers
//! with what GitHub answered, for a comment as much as for a filing.**
//!
//! **The shell for now.** Every function has the shape it will keep and none of the behaviour — the
//! tests are what demand any of it.

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_domain::repository::Repository;

use crate::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhereGitHubAnswered};
use crate::a_model_is_asked_how_it_rules::WhereAModelIs;
use crate::a_program_we_run::AProgramWeRun;

/// A creative agent that could not file what it concluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACreativeAgentCouldNotFile(pub String);

/// A creative agent filing what it concluded, **and the addresses it comes back with.**
pub fn a_creative_agent_files_what_it_concluded(
    the_model: &impl AProgramWeRun,
    somewhere: &mut impl AGitHubWeWriteTo,
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    where_the_model_is: &WhereAModelIs,
) -> Result<Vec<AWhereItWasFiled>, ACreativeAgentCouldNotFile> {
    let _ = (
        the_model,
        somewhere,
        of_which_repository,
        what_the_run_saw,
        where_the_model_is,
    );
    Ok(Vec::new())
}
