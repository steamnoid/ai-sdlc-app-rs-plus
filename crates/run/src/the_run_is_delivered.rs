//! The run is delivered, and the work item is finished only when a person says so.
//!
//! **A branch on a remote is not a delivery, and a pull request nobody ruled on is not one either.**
//! Both halves of the delivery exist — `a_branch_is_pushed` and `a_pull_request_is_opened` — **and
//! nothing joined them to a run or moved the work item afterwards.**
//!
//! **`DONE` is reachable from one stage only, and that stage is a gate.** The transition table has
//! no edge from `IN_PROGRESS_BY_AGENT` into `DONE`, **so a delivery cannot finish a run by itself** —
//! it presents what it delivered, and a person rules on it.

use std::fmt;

use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::StateMachineError;

use crate::a_branch_goes_to_a_remote::{
    ABranchToPush, APushedBranch, WhyABranchDidNotGo, a_branch_is_pushed,
};
use crate::a_credential::ACredential;
use crate::a_program_we_run::AProgramWeRun;
use crate::a_pull_request_is_opened::{
    APullRequestToOpen, APullRequestWasNotOpened, AnOpenedPullRequest, a_pull_request_is_opened,
};
use crate::the_repository_to_clone_into::ARepositoryToClone;
use crate::where_a_run_stands::{
    WhereARunStands, the_work_is_delivered_so_it_is_done, the_work_is_presented_for_a_ruling,
};
use crate::{
    APersonToDecide, NotStandingAtTheGate, the_gate_records_what_the_human_decided,
    the_question_put_to_somebody,
};

/// What a delivery left behind, **and the pull request and the run are both in it because a delivery
/// is one fact about two things and splitting it would let a caller report a branch as a delivery.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatADeliveryLeft {
    which_branch_went: APushedBranch,
    which_pull_request_was_opened: AnOpenedPullRequest,
    where_the_work_stands: WhereARunStands,
    was_the_delivery_approved: bool,
}

impl WhatADeliveryLeft {
    /// The branch that went, **and it is what `git` said rather than what was asked for.**
    pub fn which_branch_went(&self) -> &APushedBranch {
        &self.which_branch_went
    }

    /// The pull request GitHub opened, **with the number and the address it composed.**
    pub fn which_pull_request(&self) -> &AnOpenedPullRequest {
        &self.which_pull_request_was_opened
    }

    /// The run as it stands, **and it is the work item and not the delivery.**
    pub fn where_the_work_stands(&self) -> &WhereARunStands {
        &self.where_the_work_stands
    }

    /// Whether a person approved the delivery, **and `false` is a delivery that is not finished rather
    /// than a delivery that did not happen.**
    pub fn was_it_approved(&self) -> bool {
        self.was_the_delivery_approved
    }
}

/// Why a run was not delivered, **and each is a fact about a different thing.**
///
/// **The two that came from elsewhere keep the type that refused them**, so a reader told "the branch
/// did not go" knows it was `git` and not the run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ADeliveryThatDidNotHappen {
    /// The work is standing nowhere a delivery could be made from, **and that is the run's own fact.**
    ///
    /// **The stage is named rather than the move refused**, because a reader told
    /// `AWAITING_AGENT_PICKUP may not be moved to DONE` would go looking for a fault in the table,
    /// **and there is none: the fault would be in whatever tried to deliver from there.**
    NotStandingAtTheDelivery {
        /// Which stage it is standing at.
        where_it_stands: Stage,
    },
    /// The branch did not go, **and `git`'s own refusal travels whole.**
    TheBranchDidNotGo {
        /// What `git` said.
        why_not: WhyABranchDidNotGo,
    },
    /// The pull request did not open, **and GitHub's own refusal travels whole.**
    ThePullRequestDidNotOpen {
        /// What GitHub said.
        why_not: APullRequestWasNotOpened,
    },
    /// The work could not be presented for a ruling, **and the domain's own refusal is kept rather
    /// than written out.**
    ///
    /// **A person who refused is not here.** Refusing leaves no `Approval` **because refusing is a
    /// decision and not a record**, and the delivery that was refused is returned rather than
    /// refused — **so a reader is sent to the delivery and not to a fault in the gate.**
    TheWorkCouldNotBePresented {
        /// What the transition table said.
        why_not: StateMachineError,
    },
    /// The question could not be put to a person, **and the gate's own refusal is kept.**
    ///
    /// **A different refusal from the one above**, because the table refused a move and the gate
    /// refused a question — **and those two send a reader to two different files.**
    TheQuestionCouldNotBeAsked {
        /// What the gate said.
        why_not: NotStandingAtTheGate,
    },
}

impl fmt::Display for ADeliveryThatDidNotHappen {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            ADeliveryThatDidNotHappen::NotStandingAtTheDelivery { where_it_stands } => format!(
                "the work is standing at {where_it_stands} and nothing is standing at the delivery, \
                 so there is nothing to deliver"
            ),
            ADeliveryThatDidNotHappen::TheBranchDidNotGo { why_not } => {
                format!("the branch did not go: {why_not}")
            }
            ADeliveryThatDidNotHappen::ThePullRequestDidNotOpen { why_not } => {
                format!("the pull request did not open: {why_not}")
            }
            ADeliveryThatDidNotHappen::TheWorkCouldNotBePresented { why_not } => {
                format!("the delivery could not be presented for a ruling: {why_not}")
            }
            ADeliveryThatDidNotHappen::TheQuestionCouldNotBeAsked { why_not } => {
                format!(
                    "the delivery could not be put to a person: {}",
                    why_not.why_it_could_not_be_asked()
                )
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for ADeliveryThatDidNotHappen {}

/// Deliver a run: push its branch, open its pull request, and put the delivery to a person.
///
/// **The order is push, open, present, rule, and each step refuses before the next is attempted** —
/// a pull request offered against a head that does not exist is a request GitHub answers with a
/// number and an error, **and the number would be in a delivery a reader could open.**
///
/// **A refusal at the gate leaves the work item standing at it rather than at `DONE`, and that
/// asymmetry is the point.** A refused delivery happened and was not accepted; **a work item at
/// `DONE` says delivered**, and putting a refusal there would record a decision as a delivery.
pub fn the_run_is_delivered(
    the_run: &WhereARunStands,
    the_program: &impl AProgramWeRun,
    of_which_clone: &ARepositoryToClone,
    which_branch: &ABranchToPush,
    what_to_open: &APullRequestToOpen,
    the_credential: Option<&ACredential>,
    whoever_answers_the_gate: &mut impl APersonToDecide,
) -> Result<WhatADeliveryLeft, ADeliveryThatDidNotHappen> {
    let where_the_work_stands = the_run.the_stage_it_stands_at();
    if where_the_work_stands != Stage::InProgressByAgent {
        return Err(nowhere_near_the_delivery(where_the_work_stands));
    }
    let which_branch_went = a_branch_is_pushed(the_program, of_which_clone, which_branch)
        .map_err(the_branch_did_not_go)?;
    let which_pull_request_was_opened =
        a_pull_request_is_opened(the_program, what_to_open, the_credential)
            .map_err(the_pull_request_did_not_open)?;
    let (where_the_work_stands, was_the_delivery_approved) =
        the_delivery_is_put_to_a_person(the_run, whoever_answers_the_gate)?;
    Ok(WhatADeliveryLeft {
        which_branch_went,
        which_pull_request_was_opened,
        where_the_work_stands,
        was_the_delivery_approved,
    })
}

/// The delivery is presented and a person rules on it, **and the run moves to `DONE` only when the
/// ruling was yes.**
fn the_delivery_is_put_to_a_person(
    the_run: &WhereARunStands,
    whoever_answers_the_gate: &mut impl APersonToDecide,
) -> Result<(WhereARunStands, bool), ADeliveryThatDidNotHappen> {
    let which_artifact = the_run
        .which_artifact_it_is_on()
        .ok_or(this_run_is_on_no_artifact())?;
    let presented =
        the_work_is_presented_for_a_ruling(the_run).map_err(the_work_could_not_be_presented)?;
    let a_question = the_question_put_to_somebody(
        presented.the_work_item_it_is(),
        which_artifact,
        the_version_the_delivery_is_on(),
    )
    .map_err(the_question_could_not_be_asked)?;
    let what_the_person_left_behind =
        the_gate_records_what_the_human_decided(whoever_answers_the_gate, &a_question);
    the_delivery_is_finished_because_of(&presented, what_the_person_left_behind.is_ok())
}

/// The delivery is finished when a person approved it, **and left standing at the gate when they
/// did not.**
///
/// **A refusal and an absent gate both come back as `Err` from the gate, and telling them apart is
/// the whole of this function.** `the_gate_records_what_the_human_decided` answers `Err` when no
/// `Approval` was left behind, **and a refusal leaves no `Approval` because refusing is a decision
/// and not a record** — so `Err` here means *the person said no*, not *the gate was not there*. A
/// gate that could not be asked at all is refused earlier, by `the_question_put_to_somebody`, **so
/// the two never reach the same line.**
///
/// **A refused delivery happened and was not accepted, and `DONE` says delivered** — so the work
/// item stays at the gate, **which is the one place a reader will find it and read the refusal off
/// the record beside it.**
fn the_delivery_is_finished_because_of(
    the_presented: &WhereARunStands,
    was_it_approved: bool,
) -> Result<(WhereARunStands, bool), ADeliveryThatDidNotHappen> {
    if !was_it_approved {
        return Ok((the_presented.clone(), false));
    }
    let finished = the_work_is_delivered_so_it_is_done(the_presented)
        .map_err(the_work_could_not_be_presented)?;
    Ok((finished, true))
}

/// The version the delivery is on, **and it is one because a delivery has never been revised.**
fn the_version_the_delivery_is_on() -> aisdlc_domain::approval::AnArtifactVersion {
    aisdlc_domain::approval::AnArtifactVersion(1)
}

/// `git`'s refusal, **kept as `git` wrote it.**
fn the_branch_did_not_go(why_not: WhyABranchDidNotGo) -> ADeliveryThatDidNotHappen {
    ADeliveryThatDidNotHappen::TheBranchDidNotGo { why_not }
}

/// GitHub's refusal, **kept as GitHub wrote it.**
fn the_pull_request_did_not_open(why_not: APullRequestWasNotOpened) -> ADeliveryThatDidNotHappen {
    ADeliveryThatDidNotHappen::ThePullRequestDidNotOpen { why_not }
}

/// Nothing is standing at the delivery, **and the stage travels whole because the delivery's own
/// stage is the question and not the move.**
fn nowhere_near_the_delivery(where_it_stands: Stage) -> ADeliveryThatDidNotHappen {
    ADeliveryThatDidNotHappen::NotStandingAtTheDelivery { where_it_stands }
}

/// The work could not be presented for a ruling, **and the stage it was standing at travels with it
/// because the table's refusal says which two stages and this one says which of ours.**
fn the_work_could_not_be_presented(the_refusal: StateMachineError) -> ADeliveryThatDidNotHappen {
    ADeliveryThatDidNotHappen::TheWorkCouldNotBePresented {
        why_not: the_refusal,
    }
}

/// The question could not be put, **and the stage the work was standing at travels with it** — the
/// gate's own type carries the stage and this refusal carries it as words, **so a reader is told
/// both facts in one sentence rather than having to look the type up.**
fn the_question_could_not_be_asked(the_refusal: NotStandingAtTheGate) -> ADeliveryThatDidNotHappen {
    ADeliveryThatDidNotHappen::TheQuestionCouldNotBeAsked {
        why_not: the_refusal,
    }
}

/// The run is on no artifact, **and there is no gate to put a delivery to.**
fn this_run_is_on_no_artifact() -> ADeliveryThatDidNotHappen {
    ADeliveryThatDidNotHappen::TheQuestionCouldNotBeAsked {
        why_not: NotStandingAtTheGate::because(Stage::InProgressByAgent),
    }
}
