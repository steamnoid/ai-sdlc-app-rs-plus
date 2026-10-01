//! What a model said its offer would restate, and what the run does about it.
//!
//! **The judgement of what counts as a duplicate is a model's; the rule about what happens afterwards
//! is not.**

use crate::the_filings_already_on_file::{AFilingAlreadyOnFile, TheFilingsAlreadyOnFile};

/// What a model said one of its offers would restate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAModelSaidWouldRestate {
    it_restates: Option<i64>,
}

/// What the run did with an offer that named a filing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheRunAnswers {
    /// Nothing on file that this restates, **and the offer is filed.**
    Filed,
    /// The offer restates a filing already on file, **and is not filed.**
    NotFiledBecauseItRestates(i64),
    /// The offer names a filing that is not on file, **and is refused.**
    RefusedBecauseItNamedAFilingThatIsNotOnFile(i64),
}

impl HowTheRunAnswers {
    /// Whether this offer reaches GitHub.
    pub fn is_filed(&self) -> bool {
        matches!(self, HowTheRunAnswers::Filed)
    }

    /// Whether it was held back because it repeats something already filed.
    pub fn is_not_filed_because_it_restates(&self) -> bool {
        matches!(self, HowTheRunAnswers::NotFiledBecauseItRestates(_))
    }

    /// Whether it was refused for naming a filing that is not there.
    pub fn is_refused_because_it_named_a_filing_that_is_not_on_file(&self) -> bool {
        matches!(
            self,
            HowTheRunAnswers::RefusedBecauseItNamedAFilingThatIsNotOnFile(_)
        )
    }
}

impl WhatAModelSaidWouldRestate {
    /// An offer that repeats this filing, **and the number is what it repeats.**
    pub fn the_filing_already_on_file(which: i64) -> Self {
        Self {
            it_restates: Some(which),
        }
    }

    /// An offer that repeats nothing, **which is nearly always the case.**
    pub fn nothing_already_on_file() -> Self {
        Self { it_restates: None }
    }

    /// What the run does with it, **given everything already on file.**
    pub fn how_the_run_answers(&self, what_is_on_file: AFilingAlreadyOnFile) -> HowTheRunAnswers {
        let mut every_one = TheFilingsAlreadyOnFile::none_yet();
        every_one.and_this_one_was_too(what_is_on_file);
        match self.it_restates {
            None => HowTheRunAnswers::Filed,
            Some(which) if every_one.whether_it_holds(which) => {
                HowTheRunAnswers::NotFiledBecauseItRestates(which)
            }
            Some(which) => HowTheRunAnswers::RefusedBecauseItNamedAFilingThatIsNotOnFile(which),
        }
    }
}
