//! A run goes from a request to a pull request, and the work item is finished.
//!
//! **A whole run stops at the delivery and a delivery finishes a run, and nothing joined the two.**
//! `the_whole_of_a_run` ends standing at the delivery, and `the_run_is_delivered` takes a run already
//! standing there — **so this repository could walk seven gates and stop**, which is a run that did all
//! the work and delivered none of it.
//!
//! **The delivery is the eighth gate, and it is a gate.** A delivery pushes a branch and opens a pull
//! request, **neither of which is a stage a person rules on** — so the delivery is presented, and
//! **a delivery that finished a run without asking would put a decision in the record that nobody
//! made.**

use std::fmt;

use aisdlc_domain::stage::Stage;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;

use crate::APersonToDecide;
use crate::a_branch_goes_to_a_remote::ABranchToPush;
use crate::a_credential::ACredential;
use crate::a_model_is_asked_for_an_artifact::AReasoningModel;
use crate::a_program_we_run::AProgramWeRun;
use crate::a_pull_request_is_opened::{APullRequestToOpen, AnOpenedPullRequest};
use crate::the_repository_to_clone_into::ARepositoryToClone;
use crate::the_run_is_delivered::{
    ADeliveryThatDidNotHappen, WhatADeliveryLeft, the_run_is_delivered,
};
use crate::the_whole_of_a_run::{AWholeRunThatCouldNotBeHad, the_whole_of_a_run};
use crate::where_a_run_stands::WhereARunStands;

/// Everything one delivery needs, **and it is one value rather than four arguments.**
///
/// **The four are one fact about one delivery**, so a caller holding them separately could push a
/// branch and open a pull request about a different one — **and this is the only place the pairing is
/// stated.**
#[derive(Clone, PartialEq, Eq)]
pub struct WhatThereIsToDeliver {
    of_which_clone: ARepositoryToClone,
    which_branch: ABranchToPush,
    what_to_open: APullRequestToOpen,
    the_credential: ACredential,
}

impl WhatThereIsToDeliver {
    /// What this run will push and open.
    ///
    /// **It cannot fail and that is deliberate.** The four are already-validated types — a branch name
    /// `git` refused, a request with a title and a body, a key a deployment supplied — **so a
    /// constructor here that could be refused would be a `Result` whose error nobody could ever
    /// produce**, and a refusal nobody can produce is a place a reader would look for a fault.
    pub fn for_this_run(
        of_which_clone: ARepositoryToClone,
        which_branch: ABranchToPush,
        what_to_open: APullRequestToOpen,
        the_credential: ACredential,
    ) -> Self {
        Self {
            of_which_clone,
            which_branch,
            what_to_open,
            the_credential,
        }
    }

    fn the_clone(&self) -> &ARepositoryToClone {
        &self.of_which_clone
    }

    fn the_branch(&self) -> &ABranchToPush {
        &self.which_branch
    }

    fn the_request(&self) -> &APullRequestToOpen {
        &self.what_to_open
    }

    fn the_key(&self) -> &ACredential {
        &self.the_credential
    }
}

/// What a run left behind, **and the pull request is named because it is the run's whole record.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatARunLeft {
    where_the_work_stands: WhereARunStands,
    which_pull_request_was_opened: Option<AnOpenedPullRequest>,
}

impl WhatARunLeft {
    /// The run as it stands, **and a run that was refused is at `DONE` with no pull request.**
    pub fn where_the_work_stands(&self) -> &WhereARunStands {
        &self.where_the_work_stands
    }

    /// The pull request GitHub opened, **and nothing when the run was refused or the delivery failed.**
    pub fn which_pull_request(&self) -> &AnOpenedPullRequest {
        self.which_pull_request_was_opened
            .as_ref()
            .expect("a caller asks this only of a run that delivered")
    }
}

/// Why a run did not finish, **and each is a fact about a different thing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ARunThatDidNotFinish {
    /// The run itself could not be had, **and the whole run's own refusal travels whole.**
    TheRun {
        /// What stopped it.
        why_not: Box<AWholeRunThatCouldNotBeHad>,
    },
    /// The delivery did not happen, **and the delivery's own refusal travels whole.**
    TheDelivery {
        /// What stopped it.
        why_not: Box<ADeliveryThatDidNotHappen>,
    },
}

impl fmt::Display for ARunThatDidNotFinish {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            ARunThatDidNotFinish::TheRun { why_not } => {
                format!("the run could not be had: {why_not}")
            }
            ARunThatDidNotFinish::TheDelivery { why_not } => {
                format!("the delivery did not happen: {why_not}")
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for ARunThatDidNotFinish {}

/// Run a whole pipeline and deliver what it produced, **and stop the moment a person says no.**
///
/// **The delivery happens only if the run reached it**, and the test is the run's own stage — **a run
/// refused at the third gate is at `DONE` and never hands its work to the delivery agent**, which is
/// the one thing that separates a run that was stopped from a run that delivered nothing.
pub fn a_run_from_a_request_to_a_pull_request(
    the_run: &WhereARunStands,
    a_model: &mut impl AReasoningModel,
    who_reasoned: WhichOrganisationReasons,
    whoever_answers_the_gates: &mut impl APersonToDecide,
    what_there_is: &WhatThereIsToDeliver,
    the_program: &impl AProgramWeRun,
    whoever_answers_the_delivery: &mut impl APersonToDecide,
) -> Result<WhatARunLeft, ARunThatDidNotFinish> {
    let the_run =
        the_whole_pipeline_is_run(the_run, a_model, who_reasoned, whoever_answers_the_gates)?;
    if the_run.the_stage_it_stands_at() == Stage::Done {
        return Ok(a_run_stopped_with_nothing_delivered(the_run));
    }
    what_the_run_produced_is_delivered(
        &the_run,
        what_there_is,
        the_program,
        whoever_answers_the_delivery,
    )
}

/// The whole pipeline is run, **and its own refusal is boxed rather than copied.**
fn the_whole_pipeline_is_run(
    the_run: &WhereARunStands,
    a_model: &mut impl AReasoningModel,
    who_reasoned: WhichOrganisationReasons,
    whoever_answers_the_gates: &mut impl APersonToDecide,
) -> Result<WhereARunStands, ARunThatDidNotFinish> {
    let the_whole = the_whole_of_a_run(the_run, a_model, who_reasoned, whoever_answers_the_gates);
    the_whole.map_err(|why_not| ARunThatDidNotFinish::TheRun {
        why_not: Box::new(why_not),
    })
}

/// What the run produced is delivered, **and the delivery's own refusal is boxed rather than copied.**
fn what_the_run_produced_is_delivered(
    the_run: &WhereARunStands,
    what_there_is: &WhatThereIsToDeliver,
    the_program: &impl AProgramWeRun,
    whoever_answers_the_delivery: &mut impl APersonToDecide,
) -> Result<WhatARunLeft, ARunThatDidNotFinish> {
    let delivered = the_run_is_delivered(
        the_run,
        the_program,
        what_there_is.the_clone(),
        what_there_is.the_branch(),
        what_there_is.the_request(),
        Some(what_there_is.the_key()),
        whoever_answers_the_delivery,
    );
    let delivered = delivered.map_err(|why_not| ARunThatDidNotFinish::TheDelivery {
        why_not: Box::new(why_not),
    })?;
    Ok(a_run_finished_with(&delivered))
}

/// A run refused in the middle, **and it is finished with nothing delivered** — which is the same
/// stage as a delivered run and **the only thing telling the two apart is the pull request.**
fn a_run_stopped_with_nothing_delivered(the_run: WhereARunStands) -> WhatARunLeft {
    WhatARunLeft {
        where_the_work_stands: the_run,
        which_pull_request_was_opened: None,
    }
}

/// A run that delivered, **and what it left names GitHub's own pull request.**
fn a_run_finished_with(delivered: &WhatADeliveryLeft) -> WhatARunLeft {
    let where_the_work_stands = delivered.where_the_work_stands().clone();
    let which_pull_request_was_opened = Some(delivered.which_pull_request().clone());
    WhatARunLeft {
        where_the_work_stands,
        which_pull_request_was_opened,
    }
}
