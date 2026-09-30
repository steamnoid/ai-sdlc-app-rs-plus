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

use crate::a_git_hub_we_write_to::AGitHubWeWriteTo;
use crate::a_model_is_asked_how_it_rules::WhereAModelIs;
use crate::a_program_we_run::AProgramWeRun;
use crate::an_agent_is_asked_what_it_would_file;

/// A creative agent that could not file what it concluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACreativeAgentCouldNotFile(pub String);

/// A creative agent filing what it concluded, **and every address it comes back with.**
///
/// **The whole chain in one function**: the listing becomes a question, the question an answer, the
/// answer offers grounded in the listing, the offers filings, **and each filing's address comes back
/// out of the port that asked GitHub.** A path split across five seams is five places for two of them
/// to disagree about what it takes.
pub fn a_creative_agent_files_what_it_concluded(
    the_model: &impl AProgramWeRun,
    somewhere: &mut impl AGitHubWeWriteTo,
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    where_the_model_is: &WhereAModelIs,
) -> Result<Vec<AWhereItWasFiled>, ACreativeAgentCouldNotFile> {
    let what_it_concluded =
        an_agent_is_asked_what_it_would_file::an_agent_is_asked_what_it_would_file(
            the_model,
            of_which_repository,
            what_the_run_saw,
            where_the_model_is,
        )
        .map_err(|could_not_offer| ACreativeAgentCouldNotFile(could_not_offer.0))?;
    one_filing_per_offer(somewhere, of_which_repository, &what_it_concluded.offers)
}

/// One filing per offer, **and one address per filing, because an offer with no address is an offer a
/// person can never look at again.**
fn one_filing_per_offer(
    somewhere: &mut impl AGitHubWeWriteTo,
    of_which_repository: &Repository,
    what_was_offered: &[crate::what_an_agent_offered::AWorkAnAgentOffered],
) -> Result<Vec<AWhereItWasFiled>, ACreativeAgentCouldNotFile> {
    let mut where_each_one_went = Vec::new();
    for one_offer in what_was_offered {
        let what_it_is_asked_for = crate::an_offer_becomes_a_filing::the_filing_an_offer_becomes(
            of_which_repository,
            one_offer,
        );
        let where_it_went = somewhere
            .it_is_asked_to(&what_it_is_asked_for)
            .map_err(ACreativeAgentCouldNotFile)?;
        where_each_one_went.push(where_it_went.the_address().clone());
    }
    Ok(where_each_one_went)
}
