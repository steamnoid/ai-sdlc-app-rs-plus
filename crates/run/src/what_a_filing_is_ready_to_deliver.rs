//! What a filing is ready to deliver.
//!
//! **The run is taken up answering, with what it delivers derived beside it.** Seven
//! collaborators do not fit one signature a reader can hold, so preparing a delivery and running
//! it are two functions: this one takes the work and the key, and the run takes the rest.

use std::fmt;

use aisdlc_domain::state_machine::StateMachineError;

use crate::a_credential::ACredential;
use crate::a_run_from_a_request_to_a_pull_request::WhatThereIsToDeliver;
use crate::the_work_a_filing_became_is_taken_up::the_work_a_filing_became_is_taken_up;
use crate::the_work_each_filing_becomes::WhatTheRunWillWorkOn;
use crate::what_there_is_to_deliver_for_one_run::{
    NothingToDeliverForOneRun, what_there_is_to_deliver_for_one_run,
};
use crate::where_a_run_stands::WhereARunStands;

/// What a filing is ready to deliver: the run taken up and answering, with its delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAFilingIsReadyToDeliver {
    /// The run taken up and answering.
    the_run: WhereARunStands,
    /// What it delivers.
    what_there_is_to_deliver: WhatThereIsToDeliver,
}

impl WhatAFilingIsReadyToDeliver {
    /// The run taken up and answering.
    pub fn the_run(&self) -> &WhereARunStands {
        &self.the_run
    }

    /// What it delivers.
    pub fn what_there_is_to_deliver(&self) -> &WhatThereIsToDeliver {
        &self.what_there_is_to_deliver
    }
}

/// Why what a filing became is not ready to deliver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AFilingThatDidNotDeliver {
    /// The work was never taken up, and the refusal travels whole.
    TheWorkWasNeverTakenUp {
        /// What the table refused.
        why_not: StateMachineError,
    },
    /// There was nothing to deliver, and the refusal travels whole.
    ThereWasNothingToDeliver {
        /// What could not be named.
        why_not: NothingToDeliverForOneRun,
    },
}

impl fmt::Display for AFilingThatDidNotDeliver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            Self::TheWorkWasNeverTakenUp { why_not } => {
                format!("the work was never taken up: {why_not}")
            }
            Self::ThereWasNothingToDeliver { why_not } => {
                format!("there was nothing to deliver: {why_not}")
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for AFilingThatDidNotDeliver {}

/// What a filing is ready to deliver, **or the refusal naming what is not ready.**
pub fn what_a_filing_is_ready_to_deliver(
    what_the_run_works_on: &WhatTheRunWillWorkOn,
    the_credential: &ACredential,
) -> Result<WhatAFilingIsReadyToDeliver, AFilingThatDidNotDeliver> {
    let what_there_is_to_deliver =
        what_there_is_to_deliver_for_one_run(what_the_run_works_on, the_credential)
            .map_err(|why_not| AFilingThatDidNotDeliver::ThereWasNothingToDeliver { why_not })?;
    let answering_work = the_answering_work(what_the_run_works_on);
    let the_run = the_work_a_filing_became_is_taken_up(&answering_work)
        .map_err(|why_not| AFilingThatDidNotDeliver::TheWorkWasNeverTakenUp { why_not })?;
    Ok(WhatAFilingIsReadyToDeliver {
        the_run,
        what_there_is_to_deliver,
    })
}

/// The work answering its request, **and it is the derivation that refused work answering
/// nothing** — so this holds what the derivation already let through.
fn the_answering_work(what_the_run_works_on: &WhatTheRunWillWorkOn) -> WhatTheRunWillWorkOn {
    let requested = what_the_run_works_on
        .the_work_request
        .clone()
        .expect("the derivation refused work answering nothing");
    let answering = what_the_run_works_on
        .the_work_item
        .clone()
        .answering(requested)
        .expect("a request is a fact, not a position");
    WhatTheRunWillWorkOn {
        the_work_item: answering,
        the_work_request: what_the_run_works_on.the_work_request.clone(),
    }
}
