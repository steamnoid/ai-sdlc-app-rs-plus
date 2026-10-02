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

use crate::AReasoningModel;
use crate::a_git_hub_we_write_to::AGitHubWeWriteTo;
use crate::an_agent_is_asked_what_it_would_file;
use crate::what_a_model_said_would_restate::{HowTheRunAnswers, WhatAModelSaidWouldRestate};

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
    the_model: &mut impl AReasoningModel,
    somewhere: &mut impl AGitHubWeWriteTo,
    somewhere_else: &mut impl crate::a_filing_is_read::AFilingIsReadFrom,
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
) -> Result<Vec<AWhereItWasFiled>, ACreativeAgentCouldNotFile> {
    let what_is_already_on_file = somewhere_else
        .the_filings_already_on_file_of(of_which_repository)
        .map_err(ACreativeAgentCouldNotFile)?;
    let what_it_concluded =
        an_agent_is_asked_what_it_would_file::an_agent_is_asked_what_it_would_file(
            the_model,
            &what_is_already_on_file,
            of_which_repository,
            what_the_run_saw,
        )
        .map_err(|could_not_offer| ACreativeAgentCouldNotFile(could_not_offer.0))?;
    one_filing_per_offer(
        somewhere,
        of_which_repository,
        &what_it_concluded.offers,
        &what_is_already_on_file,
    )
}

/// One filing per offer, **and one address per filing, because an offer with no address is an offer a
/// person can never look at again.**
fn one_filing_per_offer(
    somewhere: &mut impl AGitHubWeWriteTo,
    of_which_repository: &Repository,
    what_was_offered: &[crate::what_an_agent_offered::AWorkAnAgentOffered],
    what_is_already_on_file: &crate::the_filings_already_on_file::TheFilingsAlreadyOnFile,
) -> Result<Vec<AWhereItWasFiled>, ACreativeAgentCouldNotFile> {
    let mut where_each_one_went = Vec::new();
    for one_offer in what_was_offered {
        let where_one_went = the_address_of_an_offer_that_is_filed(
            somewhere,
            of_which_repository,
            one_offer,
            what_is_already_on_file,
        )?;
        where_each_one_went.extend(where_one_went);
    }
    Ok(where_each_one_went)
}

/// The address one offer was filed at, **or `None` because it restates a filing already on file.**
///
/// **Three answers and not two**, **because a restatement of a filing that is *not* on file is not a
/// restatement** — it is a model reporting a filing it never read, and filing it would put a number
/// nobody can open into a filing. **The third answer is an error and the second is a silence**, and
/// the difference is whether the model lied or merely agreed with itself.
fn the_address_of_an_offer_that_is_filed(
    somewhere: &mut impl AGitHubWeWriteTo,
    of_which_repository: &Repository,
    one_offer: &crate::what_an_agent_offered::AWorkAnAgentOffered,
    what_is_already_on_file: &crate::the_filings_already_on_file::TheFilingsAlreadyOnFile,
) -> Result<Option<AWhereItWasFiled>, ACreativeAgentCouldNotFile> {
    let what_it_would_restate = match one_offer.it_restates {
        None => WhatAModelSaidWouldRestate::nothing_already_on_file(),
        Some(which) => WhatAModelSaidWouldRestate::the_filing_already_on_file(which),
    };
    let is_this_offer_filed =
        whether_this_offer_is_filed(&what_it_would_restate, what_is_already_on_file)
            .map_err(ACreativeAgentCouldNotFile)?;
    if !is_this_offer_filed {
        return Ok(None);
    }
    Ok(Some(the_address_github_was_asked_for(
        somewhere,
        of_which_repository,
        one_offer,
    )?))
}

/// Whether one offer reaches GitHub at all, **or the fault naming a filing the model invented.**
///
/// **Three arms and all three are expressions**, **because a `return` inside a match arm sits three
/// deep and a body may sit two** — and the fix that satisfies the reader is three expressions and a
/// name, not a brace.
fn whether_this_offer_is_filed(
    what_it_would_restate: &WhatAModelSaidWouldRestate,
    what_is_already_on_file: &crate::the_filings_already_on_file::TheFilingsAlreadyOnFile,
) -> Result<bool, String> {
    let verdict = what_it_would_restate.how_the_run_answers(what_is_already_on_file);
    if let HowTheRunAnswers::RefusedBecauseItNamedAFilingThatIsNotOnFile(which) = verdict {
        return Err(the_fault_about(which));
    }
    Ok(!matches!(
        verdict,
        HowTheRunAnswers::NotFiledBecauseItRestates(_)
    ))
}

/// Why one offer was refused, **naming the filing it invented so a person can open it and see
/// nothing is there.**
fn the_fault_about(which: i64) -> String {
    format!(
        "the offer restates filing #{which}, which is not on file, so this run cannot tell \
         whether it is a new finding or a repeated one"
    )
}

/// The address GitHub printed for one filing, **asked for once per offer that is filed at all.**
fn the_address_github_was_asked_for(
    somewhere: &mut impl AGitHubWeWriteTo,
    of_which_repository: &Repository,
    one_offer: &crate::what_an_agent_offered::AWorkAnAgentOffered,
) -> Result<AWhereItWasFiled, ACreativeAgentCouldNotFile> {
    let what_it_is_asked_for = crate::an_offer_becomes_a_filing::the_filing_an_offer_becomes(
        of_which_repository,
        one_offer,
    );
    let where_it_went = somewhere
        .it_is_asked_to(&what_it_is_asked_for)
        .map_err(ACreativeAgentCouldNotFile)?;
    Ok(where_it_went.the_address().clone())
}
