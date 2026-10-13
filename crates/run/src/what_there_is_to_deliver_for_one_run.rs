//! What one filing's run delivers, derived from what the run works on.
//!
//! **The branch is named after the work, the title is the filing's, and the body is the filing's
//! words with the address they were read at**, because a title composed by a model is prose
//! nobody checked on the one page every reviewer reads first.

use std::fmt;

use aisdlc_domain::commissioned_request::CommissionedRequest;

use crate::a_branch_goes_to_a_remote::{ABranchToPush, ANameABranchCannotTake};
use crate::a_credential::ACredential;
use crate::a_pull_request_is_opened::{
    APullRequestToOpen, ARequestThatIsNotAPullRequest, the_request_a_run_makes_of_its_own_change,
};
use crate::a_run_from_a_request_to_a_pull_request::WhatThereIsToDeliver;
use crate::the_work_each_filing_becomes::WhatTheRunWillWorkOn;

/// Why there is nothing to deliver for one run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NothingToDeliverForOneRun {
    /// The run answers nothing, and a delivery about nothing is about nobody's request.
    NoRequestToAnswer,
    /// The branch cannot be named, and the refusal travels whole.
    BranchCannotBeNamed {
        /// What git refused.
        why_not: ANameABranchCannotTake,
    },
    /// The request is not a pull request, and the refusal travels whole.
    RequestIsNotAPullRequest {
        /// What GitHub would refuse.
        why_not: ARequestThatIsNotAPullRequest,
    },
}

impl fmt::Display for NothingToDeliverForOneRun {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            Self::NoRequestToAnswer => String::from(
                "the run answers nothing, and a delivery about nothing is about nobody's request",
            ),
            Self::BranchCannotBeNamed { why_not } => {
                format!("the branch cannot be named: {why_not}")
            }
            Self::RequestIsNotAPullRequest { why_not } => {
                format!("the request is not a pull request: {why_not}")
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for NothingToDeliverForOneRun {}

/// What one filing's run delivers, **or the refusal naming what it cannot name.**
pub fn what_there_is_to_deliver_for_one_run(
    what_the_run_works_on: &WhatTheRunWillWorkOn,
    the_credential: &ACredential,
) -> Result<WhatThereIsToDeliver, NothingToDeliverForOneRun> {
    let requested = what_the_run_works_on
        .the_work_request
        .as_ref()
        .ok_or(NothingToDeliverForOneRun::NoRequestToAnswer)?;
    let which_branch =
        ABranchToPush::a_branch_named(&the_branch_for_the_work(what_the_run_works_on))
            .map_err(|why_not| NothingToDeliverForOneRun::BranchCannotBeNamed { why_not })?;
    let what_to_open = the_request_the_run_opens(what_the_run_works_on, &which_branch, requested)?;
    Ok(WhatThereIsToDeliver::for_this_run(
        which_branch,
        what_to_open,
        the_credential.clone(),
    ))
}

/// The request the run opens, **about its own change on its own branch.**
fn the_request_the_run_opens(
    what_the_run_works_on: &WhatTheRunWillWorkOn,
    which_branch: &ABranchToPush,
    requested: &CommissionedRequest,
) -> Result<APullRequestToOpen, NothingToDeliverForOneRun> {
    the_request_a_run_makes_of_its_own_change(
        what_the_run_works_on.the_work_item.the_repository(),
        what_the_run_works_on.the_work_item.the_repository(),
        which_branch.which_branch(),
        THE_BRANCH_THE_CHANGE_GOES_ONTO,
        requested.what_was_asked(),
        &the_body_the_run_delivers(requested),
    )
    .map_err(|why_not| NothingToDeliverForOneRun::RequestIsNotAPullRequest { why_not })
}

/// The branch for the work, **named after the work it delivers.**
fn the_branch_for_the_work(what_the_run_works_on: &WhatTheRunWillWorkOn) -> String {
    format!(
        "aisdlc/{}",
        what_the_run_works_on.the_work_item.the_work_item_id()
    )
}

/// The body the run delivers, **the filing's words, or its title when it said nothing besides,
/// with the address they were read at beside them.**
fn the_body_the_run_delivers(requested: &CommissionedRequest) -> String {
    let what_was_said = what_was_said_besides_the_title(requested);
    match requested.where_it_was_filed() {
        Some(where_it_was_filed) => filed_besides(what_was_said, where_it_was_filed.as_str()),
        None => String::from(what_was_said),
    }
}

/// What was said besides the title, **or the title when nothing was said besides it.**
fn what_was_said_besides_the_title(requested: &CommissionedRequest) -> &str {
    let what_was_said = requested.what_was_said_besides();
    if what_was_said.trim().is_empty() {
        requested.what_was_asked()
    } else {
        what_was_said
    }
}

/// Words filed beside the address they were read at.
fn filed_besides(what_was_said: &str, where_it_was_filed: &str) -> String {
    format!("{what_was_said}\n\nFiled as {where_it_was_filed}")
}

/// What the change goes onto.
const THE_BRANCH_THE_CHANGE_GOES_ONTO: &str = "main";
