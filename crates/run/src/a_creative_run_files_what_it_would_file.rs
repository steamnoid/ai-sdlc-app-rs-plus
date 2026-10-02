//! A creative run from a deployment's flag to the works it will take up.
//!
//! **The flag is read before the model and before GitHub**, because a model asked costs money and a
//! filing made lands in a real repository — **and a deployment that did not ask must get neither a
//! question nor an issue.**
//!
//! **The filings are read back before they become work.** A work item built from what the run
//! remembers writing carries text GitHub never had, **and the two part company the moment a person
//! edits one word.**

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_domain::repository::Repository;

use crate::AReasoningModel;
use crate::a_creative_agent_files_what_it_concluded::a_creative_agent_files_what_it_concluded;
use crate::a_filing_is_read::{AFilingIsReadFrom, the_filing_a_filing_was_read_back};
use crate::a_git_hub_we_write_to::AGitHubWeWriteTo;
use crate::the_work_each_filing_becomes::{WhatTheRunWillWorkOn, the_work_each_filing_becomes};
use crate::whether_an_agent_offers::WhetherAnAgentOffers;

/// A flag that is off, **and the words that would open it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFlagThatIsOff(pub String);

/// The works the run will take up, **and nothing at all when the flag is off.**
///
/// **The flag first, and everything else after it.** A run that asked a model and then found the flag
/// off has already paid for an answer nobody wanted **and may already have made a filing in somebody's
/// repository**, and neither can be taken back by reading the flag afterwards.
pub fn a_creative_run_files_what_it_would_file(
    the_model: &mut impl AReasoningModel,
    somewhere: &mut impl AGitHubWeWriteTo,
    somewhere_else: &mut impl AFilingIsReadFrom,
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    may_an_agent_offer: WhetherAnAgentOffers,
) -> Result<Vec<WhatTheRunWillWorkOn>, AFlagThatIsOff> {
    if may_an_agent_offer != WhetherAnAgentOffers::Yes {
        return Err(AFlagThatIsOff(String::from(
            "this deployment did not ask for an agent that offers work — set \
             AISDLC_AN_AGENT_OFFERS=yes to have one file issues of its own",
        )));
    }
    let where_each_filing_went = a_creative_agent_files_what_it_concluded(
        the_model,
        somewhere,
        somewhere_else,
        of_which_repository,
        what_the_run_saw,
    )
    .map_err(|could_not_file| AFlagThatIsOff(could_not_file.0))?;
    the_work_the_run_will_work_on(somewhere_else, of_which_repository, &where_each_filing_went)
        .map_err(|no_work| AFlagThatIsOff(no_work.0))
}

/// Every filing read back and turned into a work, **and the reading happens before the building.**
fn the_work_the_run_will_work_on(
    somewhere_else: &mut impl AFilingIsReadFrom,
    of_which_repository: &Repository,
    where_each_filing_went: &[AWhereItWasFiled],
) -> Result<Vec<WhatTheRunWillWorkOn>, AFlagThatIsOff> {
    let mut what_was_read = Vec::new();
    for where_it_went in where_each_filing_went {
        what_was_read.push(
            the_filing_a_filing_was_read_back(somewhere_else, where_it_went.as_str())
                .map_err(AFlagThatIsOff)?,
        );
    }
    let where_each_went_as_a_string: Vec<String> = where_each_filing_went
        .iter()
        .map(|where_it_went| where_it_went.as_str().to_string())
        .collect();
    the_work_each_filing_becomes(
        of_which_repository,
        &what_was_read,
        &where_each_went_as_a_string,
    )
    .map_err(|no_work| AFlagThatIsOff(no_work.0))
}
