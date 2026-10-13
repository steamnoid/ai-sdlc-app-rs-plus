//! What a creative run reads from its own checkout, before any model is asked.
//!
//! **The copy comes first and the listing is read off the copy.** A run that asked a model about
//! a repository before copying it reasons about a description, which is the defect the checkout
//! exists to prevent. The checkout travels with what it saw, because a listing without its copy
//! is a directory nobody can point at.

use std::fmt;

use aisdlc_domain::a_clone::ACloneOnDisk;

use crate::a_program_we_run::AProgramWeRun;
use crate::a_run_asks_what_is_in_its_own_checkout::{
    NotTheRunItsOwnCheckout, the_run_asks_what_is_in_its_own_checkout,
};
use crate::the_project_on_disk::TheProjectOnDisk;
use crate::the_run_makes_a_checkout::{HowCloningWent, the_run_makes_a_checkout};
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// What a creative run read from its own checkout: the copy and what was in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatACreativeRunRead {
    /// The copy the run made.
    the_checkout: ACloneOnDisk,
    /// What was in it, as the run saw it.
    what_it_saw: Vec<String>,
}

impl WhatACreativeRunRead {
    /// The copy the run made.
    pub fn the_checkout(&self) -> &ACloneOnDisk {
        &self.the_checkout
    }

    /// What was in it, as the run saw it.
    pub fn what_it_saw(&self) -> &[String] {
        &self.what_it_saw
    }
}

/// Why a creative run could not read its own checkout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ACheckoutThatCouldNotBeRead {
    /// The repository was never copied, and how the cloning went travels whole.
    TheRepositoryWasNeverCopied {
        /// How the cloning went.
        how_the_cloning_went: HowCloningWent,
    },
    /// The project lives somewhere else, and the two places travel whole.
    NotTheRunItsOwnCheckout {
        /// The two places that differ.
        why_not: NotTheRunItsOwnCheckout,
    },
}

impl ACheckoutThatCouldNotBeRead {
    /// How the cloning went, and nothing when the copy was made.
    pub fn how_the_cloning_went(&self) -> Option<&HowCloningWent> {
        match self {
            Self::TheRepositoryWasNeverCopied {
                how_the_cloning_went,
            } => Some(how_the_cloning_went),
            Self::NotTheRunItsOwnCheckout { .. } => None,
        }
    }
}

impl fmt::Display for ACheckoutThatCouldNotBeRead {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            Self::TheRepositoryWasNeverCopied {
                how_the_cloning_went,
            } => format!("the repository was never copied: {how_the_cloning_went}"),
            Self::NotTheRunItsOwnCheckout { why_not } => why_not.because(),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for ACheckoutThatCouldNotBeRead {}

/// What a creative run reads from its own checkout, **and nothing is asked before the copy.**
///
/// **One project and not two handles to it.** The directory is asked whether anything stands at
/// the destination through the same project the listing is read off — a separate seam answering
/// it is a second answer, and two answers to one question are two that can disagree.
pub fn what_a_creative_run_reads_from_its_own_checkout(
    the_program: &mut impl AProgramWeRun,
    the_project: &mut impl TheProjectOnDisk,
    what_it_is_asked_to_clone: &WhatWeTellAProgramToDo,
) -> Result<WhatACreativeRunRead, ACheckoutThatCouldNotBeRead> {
    let how_the_cloning_went =
        the_run_makes_a_checkout(the_program, the_project, what_it_is_asked_to_clone);
    let the_checkout = the_copy_the_run_made(how_the_cloning_went)?;
    let what_it_saw = what_is_in_the_checkout(&the_checkout, the_project)?;
    Ok(WhatACreativeRunRead {
        the_checkout,
        what_it_saw,
    })
}

/// The copy the run made, **or the refusal carrying how the cloning went.**
fn the_copy_the_run_made(
    how_the_cloning_went: HowCloningWent,
) -> Result<ACloneOnDisk, ACheckoutThatCouldNotBeRead> {
    let the_clone = how_the_cloning_went.the_clone().cloned();
    the_clone.ok_or(ACheckoutThatCouldNotBeRead::TheRepositoryWasNeverCopied {
        how_the_cloning_went,
    })
}

/// What is in the checkout, **read off the copy and never off another directory.**
fn what_is_in_the_checkout(
    the_checkout: &ACloneOnDisk,
    the_project: &mut impl TheProjectOnDisk,
) -> Result<Vec<String>, ACheckoutThatCouldNotBeRead> {
    the_run_asks_what_is_in_its_own_checkout(the_checkout, the_project)
        .map_err(|why_not| ACheckoutThatCouldNotBeRead::NotTheRunItsOwnCheckout { why_not })
}
