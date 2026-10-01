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
    /// Which filing already on file this offer restates, **or `None` when it is a new finding.**
    pub it_restates: Option<i64>,
}

/// What a model said it would file, **in the shape it said it in — paths and not addresses.**
///
/// **The raw answer and the report are two types**, because an address a model may invent is worse
/// than none, **and the address is derived here from a path this run verified was really there.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAModelSaidItWouldFile {
    /// What the filing is called.
    pub title: String,
    /// What the filing asks for, in words.
    pub what_it_asks_for: String,
    /// Which files it says it concluded that from.
    pub paths: Vec<String>,
    /// Which filing already on file it restates, **or `None` for an offer that is not a
    /// restatement.**
    pub it_restates: Option<i64>,
}

/// An offer naming a file the run never showed the model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnOfferThatNamedAFileTheRunNeverShowed(pub String);

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
    /// The offers a model offered, **every path checked against the listing the run really showed.**
    ///
    /// **Checked here and not after**, because an address derived from a path that is not there is an
    /// address that resolves to nothing — **and one that looks exactly like a link.** The address rule
    /// stops a model inventing *where* a file is; this stops it inventing *that a file is*.
    pub fn the_offers_a_model_offered(
        of_which_repository: &Repository,
        what_the_run_saw: &[String],
        what_the_model_said: Vec<WhatAModelSaidItWouldFile>,
        judged: WhatAnAgentJudged,
    ) -> Result<WhatAnAgentOffered, AnOfferThatNamedAFileTheRunNeverShowed> {
        WhatAnAgentOffered::an_agent_offered(
            what_the_model_said
                .into_iter()
                .map(|one| one_as_an_offer(of_which_repository, what_the_run_saw, one))
                .collect::<Result<Vec<_>, _>>()?,
            judged,
        )
        .map_err(|a_fault| AnOfferThatNamedAFileTheRunNeverShowed(a_fault.0))
    }

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
        offers: Vec<AWorkAnAgentOffered>,
        judged: WhatAnAgentJudged,
    ) -> Result<WhatAnAgentOffered, AnOfferNothingWasConcludedFrom> {
        let what_was_offered = offers;
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

/// One thing a model said, as one offer with the evidence the run verified behind it.
///
/// **A named offer and not a four-tuple**, **because a fourth field turned `(title, asks_for,
/// evidence, restates)` into four positions a reader has to count** — and the run then rebuilt the
/// same struct from it, **which is the one place `it_restates` could quietly have been dropped.**
fn one_as_an_offer(
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    what_it_said: WhatAModelSaidItWouldFile,
) -> Result<AWorkAnAgentOffered, AnOfferThatNamedAFileTheRunNeverShowed> {
    let what_it_concluded_from =
        the_evidence_behind(of_which_repository, what_the_run_saw, &what_it_said.paths)?;
    Ok(AWorkAnAgentOffered {
        title: what_it_said.title,
        what_it_asks_for: what_it_said.what_it_asks_for,
        what_it_concluded_from,
        it_restates: what_it_said.it_restates,
    })
}

/// The evidence behind one offer, **every path of which the run really showed the model.**
fn the_evidence_behind(
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    the_paths: &[String],
) -> Result<Vec<EvidenceFile>, AnOfferThatNamedAFileTheRunNeverShowed> {
    if the_paths.is_empty() {
        return Err(AnOfferThatNamedAFileTheRunNeverShowed(
            "an offer that names no file".to_string(),
        ));
    }
    the_paths
        .iter()
        .map(|one_path| one_file_the_run_showed(of_which_repository, what_the_run_saw, one_path))
        .collect()
}

/// One path, refused unless the run really showed it to the model, and given its derived address.
fn one_file_the_run_showed(
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    one_path: &String,
) -> Result<EvidenceFile, AnOfferThatNamedAFileTheRunNeverShowed> {
    if !what_the_run_saw.contains(one_path) {
        return Err(AnOfferThatNamedAFileTheRunNeverShowed(format!(
            "{one_path}, which the run never showed the model"
        )));
    }
    WhatDiscoveryConcluded::one_file(of_which_repository, one_path)
        .map_err(|a_path| AnOfferThatNamedAFileTheRunNeverShowed(a_path.0))
}
