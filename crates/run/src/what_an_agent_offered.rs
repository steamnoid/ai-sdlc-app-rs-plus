//! What an agent offers a human: work it concluded is worth filing, and the files behind it.
//!
//! **An offer that names no file is a guess, and a guess that gets filed looks exactly like a
//! finding.** A discovery report describes a repository that exists; an offer asks for a change to
//! something that does not exist yet — **and asking for a change to a file that is not there is
//! indistinguishable, in an issue, from asking for a real one.**

use aisdlc_domain::repository::Repository;

use crate::what_an_agent_judged::WhatAnAgentJudged;
use crate::what_discovery_concluded::{EvidenceFile, WhatDiscoveryConcluded};

/// One piece of work an agent concluded is worth filing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWorkAnAgentOffered {
    /// What the filing is called.
    pub title: String,
    /// What the filing asks for, in words.
    pub what_it_asks_for: String,
    /// Which files of the repository it concluded that from, **each with an address.**
    pub what_it_concluded_from: Vec<EvidenceFile>,
}

/// An offer nothing was concluded from, so nothing can be filed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnOfferNothingWasConcludedFrom(pub String);

/// What an agent offered, and what it said about its own reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAnAgentOffered {
    /// The work worth filing.
    pub offers: Vec<AWorkAnAgentOffered>,
    /// What it found, what it could not settle, and what it expects next.
    pub judged: WhatAnAgentJudged,
}

impl WhatAnAgentOffered {
    /// The evidence behind an offer, **and the only way an offer gets any.**
    ///
    /// **Every path is checked before any address is built**, exactly as a discovery report's
    /// evidence is, so a set containing one bad path produces no evidence at all rather than a
    /// filing that is partly clickable.
    pub fn the_evidence_it_concluded_from(
        of_which_repository: &Repository,
        the_paths: &[&str],
    ) -> Result<Vec<EvidenceFile>, AnOfferNothingWasConcludedFrom> {
        if the_paths.is_empty() {
            return Err(AnOfferNothingWasConcludedFrom(
                "an offer that names no file".to_string(),
            ));
        }
        WhatDiscoveryConcluded::everything_empty()
            .what_it_concluded_from(of_which_repository, the_paths)
            .map_err(|a_path| AnOfferNothingWasConcludedFrom(a_path.0))
    }

    /// What an agent offered, **refused when any single offer names no file.**
    ///
    /// **One fault is enough and every offer is checked**, because a run that files four issues and
    /// gets one of them wrong has filed an issue about a file that is not there — **and the three
    /// beside it being right is no reason to file it.**
    ///
    /// **No offers at all is not a fault.** An agent that concluded there is nothing worth filing has
    /// told the truth, **and it still has to say so in words**, because an empty offer list and an
    /// agent that was never asked look identical afterwards.
    pub fn an_agent_offered(
        offers: Vec<(String, String, Vec<EvidenceFile>)>,
        judged: WhatAnAgentJudged,
    ) -> Result<WhatAnAgentOffered, AnOfferNothingWasConcludedFrom> {
        let what_was_offered: Vec<AWorkAnAgentOffered> = offers
            .into_iter()
            .map(
                |(title, what_it_asks_for, what_it_concluded_from)| AWorkAnAgentOffered {
                    title,
                    what_it_asks_for,
                    what_it_concluded_from,
                },
            )
            .collect();
        the_fault_in(&what_was_offered)?;
        Ok(WhatAnAgentOffered {
            offers: what_was_offered,
            judged,
        })
    }
}

/// The first thing wrong with an offer, **and every offer is looked at because one fault is enough.**
fn the_fault_in(
    what_was_offered: &[AWorkAnAgentOffered],
) -> Result<(), AnOfferNothingWasConcludedFrom> {
    for one_offer in what_was_offered {
        something_to_ask_for(one_offer)?;
        files_behind_it(one_offer)?;
    }
    Ok(())
}

/// An offer with no title or nothing to ask for, **and neither is a filing anybody can rule on.**
fn something_to_ask_for(
    one_offer: &AWorkAnAgentOffered,
) -> Result<(), AnOfferNothingWasConcludedFrom> {
    if one_offer.title.trim().is_empty() || one_offer.what_it_asks_for.trim().is_empty() {
        return Err(AnOfferNothingWasConcludedFrom(
            "an offer with no title, or nothing to ask for".to_string(),
        ));
    }
    Ok(())
}

/// An offer with no file behind it, **which is a guess that would be filed as a finding.**
fn files_behind_it(one_offer: &AWorkAnAgentOffered) -> Result<(), AnOfferNothingWasConcludedFrom> {
    if one_offer.what_it_concluded_from.is_empty() {
        return Err(AnOfferNothingWasConcludedFrom(format!(
            "{}, which names no file",
            one_offer.title
        )));
    }
    Ok(())
}
