//! The use cases, and the ports they are written against.
//!
//! A port is a question this crate cannot answer by itself. Each one is a trait declared
//! here and implemented in `aisdlc-outside`, so the adapters know the ports and the ports
//! know nothing about the adapters.
//!
//! **There is no `dyn` here.** The ports are generic and the composition root — the one
//! place a concrete type is named — monomorphises them, so no trait has to be made
//! object-safe and no `async` macro has to rewrite a signature. The cost is a longer type at
//! each boundary and a louder compile error when a test double drifts from the real thing.
//!
//! Nothing here may name an organisation, a filesystem or a runtime. Those are ports, and a
//! use case that says `reqwest` instead of `AGitHubWeReadFrom` has moved a decision out of
//! the type and into a line of code.

use std::fmt;

pub mod a_branch_goes_to_a_remote;
pub mod a_change_is_made_in_the_clone;
pub mod a_comment_is_left;
pub mod a_creative_agent_files_what_it_concluded;
pub mod a_creative_run_files_what_it_would_file;
pub mod a_credential;
pub mod a_filing_is_answered;
pub mod a_filing_is_read;
pub mod a_git_hub_we_write_to;
pub mod a_model_is_asked_for_an_artifact;
pub mod a_model_is_asked_how_it_rules;
pub mod a_path_in_the_project;
pub mod a_person_answers_at_a_terminal;
pub mod a_policy_answers_the_gate;
pub mod a_program_we_run;
pub mod a_pull_request_is_opened;
pub mod a_record_of_a_change;
pub mod a_ruling_is_recorded;
pub mod a_ruling_moves_the_work;
pub mod a_run_asks_what_is_in_its_own_checkout;
pub mod a_run_from_a_request_to_a_pull_request;
pub mod a_run_is_handed_back_to_work_again;
pub mod a_run_is_put_back_to_work_again;
pub mod a_run_is_stepped_forward;
pub mod a_run_is_walked_forward;
pub mod a_run_picks_up_the_work;
pub mod a_stage_is_worked_and_ruled_on;
pub mod an_agent_is_asked_what_it_would_file;
pub mod an_issue_read;
pub mod an_issue_to_read;
pub mod an_offer_becomes_a_filing;
pub mod how_this_deployment_is_set_up;
pub mod the_filings_already_on_file;
pub mod the_path_inside_the_project;
pub mod the_project_on_disk;
pub mod the_repository_to_clone_into;
pub mod the_run_is_delivered;
pub mod the_run_makes_a_checkout;
pub mod the_tests_are_run_in_a_container;
pub mod the_whole_of_a_run;
pub mod the_words_inside_the_endpoints_envelope;
pub mod the_work_each_filing_becomes;
pub mod what_a_model_said;
pub mod what_a_model_said_would_restate;
pub mod what_a_planner_planned;
pub mod what_a_stage_kept_is_written;
pub mod what_a_verification_concluded;
pub mod what_an_agent_has_produced;
pub mod what_an_agent_judged;
pub mod what_an_agent_offered;
pub mod what_could_go_wrong;
pub mod what_discovery_concluded;
pub mod what_the_agent_presents;
pub mod what_the_architecture_is;
pub mod what_the_product_analyst_concluded;
pub mod what_this_filing_is_about;
pub mod what_to_do_around_a_conflict;
pub mod what_to_repeat;
pub mod what_we_tell_a_program_to_do;
pub mod what_would_prove_the_work;
pub mod where_a_run_stands;
pub mod where_an_approved_artifact_leads;
pub mod where_an_artifact_of_this_run_landed;
pub mod whether_a_model_rules_here;
pub mod whether_an_agent_offers;
pub mod whether_anything_stands_there;
pub mod who_answers_the_gate;
pub mod who_signs_a_comment;

pub use the_path_inside_the_project::ThePathInsideTheProject;

/// The workspace this crate is in, which is the project the paths are read against.
pub fn the_project_this_run_is_in() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|the_workspace| the_workspace.parent())
        .map(std::path::Path::to_path_buf)
        .expect("a crate is inside a workspace")
}

pub use crate::the_project_on_disk::TheProjectOnDisk;

use crate::the_project_on_disk::AFileTheProjectDoesNotHave;

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::{AnArtifactVersion, Approval, ApprovalDecision, WhichArtifact};
use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::{StateMachineError, transition};
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;

/// Asking a model for an artifact, re-exported so this crate's root stays its one entry point.
///
/// **A re-export and not a new path, because a move that changes every call site is a move a
/// reader has to verify.** The module is where the code lives; the root is where a caller looks
/// for it, and the two answers being the same is worth more than the path being shorter.
pub use crate::a_model_is_asked_for_an_artifact::{
    AReasoningModel, AnAnswerIsNotYetAnArtifact, AnUnanswerableRequest,
    the_agent_asks_a_model_about_the_project, the_agent_asks_for_its_artifact,
};

/// Takes a work item off the queue and puts it in front of an agent.
///
/// **The move is the one the table allows and not a write to the stage.** `transition` is the
/// only door to a work item's state, so this calls it rather than assigning the stage; a
/// framework that wrote the stage itself would be a second door, and the second door is what
/// the biconditional between an agent and a stage exists to prevent.
///
/// **The agent is the caller's to name, and the stage requires one.** `AWAITING_AGENT_PICKUP`
/// is one of the two stages an agent must be holding, so the agent goes in with the move and
/// `transition` checks the biconditional itself. Nothing here decides which agent will do a
/// piece of work — that is the dispatcher's question and not this one's.
pub fn the_framework_picks_the_work_up(
    a_work_item: WorkItem,
    the_agent_who_will_do_it: AgentIdentifier,
) -> Result<WorkItem, StateMachineError> {
    transition(
        a_work_item,
        Stage::AwaitingAgentPickup,
        Some(the_agent_who_will_do_it),
    )
}

/// The agent takes the work item into its own hands.
///
/// **The move is refused for a second agent, and nothing here checks.** `transition` reads the
/// agent already holding the work item and the one named in the move, and `held_by` is the door
/// that refuses a substitution — so this passes both and lets the domain answer. A framework
/// that compared the two itself would be a second place where the rule lives, and the rule
/// already has a home.
pub fn the_agent_takes_the_work_item_into_its_own_hands(
    a_work_item: WorkItem,
    the_agent_taking_it: AgentIdentifier,
) -> Result<WorkItem, StateMachineError> {
    transition(
        a_work_item,
        Stage::InProgressByAgent,
        Some(the_agent_taking_it),
    )
}

/// An agent presents what it produced, putting the work item at the gate.
///
/// **The move is the one the table allows, and the artifact is not part of it.** A work item is
/// approved or refused at a gate, and which artifact is on the table is in the question — so
/// carrying the artifact here would be a second record of it, and a second record is a second
/// thing to keep in step with the first.
pub fn the_agent_presents_what_it_produced(
    a_work_item: WorkItem,
    _the_agent_presenting: AgentIdentifier,
    _which_artifact: &WhichArtifact,
) -> Result<WorkItem, StateMachineError> {
    transition(a_work_item, Stage::AwaitingHumanApproval, None)
}

/// The exact artifact and its version, put to somebody for a ruling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalQuestion {
    about_which_work_item: WorkItemId,
    about_which_artifact: WhichArtifact,
    about_which_version: AnArtifactVersion,
}

impl ApprovalQuestion {
    /// Which work item this is about, read from the work item and never supplied.
    pub fn about_which_work_item(&self) -> WorkItemId {
        self.about_which_work_item.clone()
    }

    /// Which artifact, exactly.
    pub fn about_which_artifact(&self) -> WhichArtifact {
        self.about_which_artifact.clone()
    }

    /// Which version, exactly.
    pub fn about_which_version(&self) -> AnArtifactVersion {
        self.about_which_version
    }
}

/// Somebody who rules on an exact artifact at an exact version.
pub trait APersonToDecide {
    /// The ruling this person gives about one question.
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision;
}

/// A work item that is not standing at the gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotStandingAtTheGate {
    the_stage_it_stands_at: Stage,
}

impl NotStandingAtTheGate {
    /// Why this work item cannot be put to somebody, naming the stage it is standing at.
    ///
    /// **The stage's own `Display` and not a second list of names here.** A first version
    /// wanted `Stage::as_str`, and adding it wrote the six stage names a second time in the one
    /// file `the_specification_and_the_code_have_not_drifted_apart` reads them from — which that
    /// gate caught. **`Stage` already knew its own name.**
    pub fn why_it_could_not_be_asked(&self) -> String {
        format!(
            "a work item at {} is not standing at AWAITING_HUMAN_APPROVAL, and a finished one has nothing left to approve",
            self.the_stage_it_stands_at
        )
    }

    /// The refusal in a sentence, **and it is here because every caller that keeps this refusal has
    /// had to flatten it to a string to write it down.**
    ///
    /// **A refusal a reader cannot read is a refusal a reader has to look up**, and looking one up
    /// means opening the file that raised it — **which is a place to send somebody rather than a
    /// thing to tell them.**
    fn as_a_sentence(&self) -> String {
        self.why_it_could_not_be_asked()
    }

    /// This refusal, for a work item standing somewhere else.
    pub fn because(the_stage_it_stands_at: Stage) -> Self {
        Self {
            the_stage_it_stands_at,
        }
    }
}

impl fmt::Display for NotStandingAtTheGate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_a_sentence())
    }
}

impl std::error::Error for NotStandingAtTheGate {}

/// The question put to somebody standing at the gate.
pub fn the_question_put_to_somebody(
    the_work_item: &WorkItem,
    which_artifact: &WhichArtifact,
    the_version: AnArtifactVersion,
) -> Result<ApprovalQuestion, NotStandingAtTheGate> {
    if the_work_item.the_stage_it_stands_at() != Stage::AwaitingHumanApproval {
        return Err(NotStandingAtTheGate::because(
            the_work_item.the_stage_it_stands_at(),
        ));
    }
    Ok(ApprovalQuestion {
        about_which_work_item: the_work_item.the_work_item_id(),
        about_which_artifact: which_artifact.clone(),
        about_which_version: the_version,
    })
}

/// The gate records what the human decided.
pub fn the_gate_records_what_the_human_decided(
    the_person: &mut impl APersonToDecide,
    a_question: &ApprovalQuestion,
) -> Result<Approval, NotStandingAtTheGate> {
    let a_ruling = the_person.how_they_rule_on(a_question);
    Approval::what_the_ruling_left_behind(&a_ruling)
        .ok_or(NotStandingAtTheGate::because(Stage::AwaitingHumanApproval))
}

/// The gate records what the human decided, refusing a work item not standing at it.
pub fn the_gate_records_what_the_human_decided_at(
    the_person: &mut impl APersonToDecide,
    the_work_item: &WorkItem,
    a_question: &ApprovalQuestion,
) -> Result<Approval, NotStandingAtTheGate> {
    if the_work_item.the_stage_it_stands_at() != Stage::AwaitingHumanApproval {
        return Err(NotStandingAtTheGate::because(
            the_work_item.the_stage_it_stands_at(),
        ));
    }
    let a_ruling = the_person.how_they_rule_on(a_question);
    Approval::what_the_ruling_left_behind(&a_ruling)
        .ok_or(NotStandingAtTheGate::because(Stage::AwaitingHumanApproval))
}

/// A run closes a work item, holding the record of somebody's ruling.
///
/// **The record is an argument and the refusal is a type.** The gate produces an `Approval` only
/// after an affirmative ruling, so this cannot be called with a refusal in hand at all — and a
/// function that took a boolean and checked it would be a rule a caller could pass through by
/// lying about the boolean.
///
/// **The move is the one the table allows**, which is also the only edge into `DONE`. A work
/// item that is not standing at the gate is refused by that table rather than by a check here,
/// so one work item's record cannot close another.
pub fn the_run_closes_the_work_item(
    a_work_item: WorkItem,
    _the_record_of_a_ruling: &Approval,
) -> Result<WorkItem, StateMachineError> {
    transition(a_work_item, Stage::Done, None)
}

/// What the agent found in the project, asked before it asks about any of it.
pub fn the_agent_asks_what_is_in_the_project(
    the_project: &mut impl TheProjectOnDisk,
) -> Vec<String> {
    the_project.what_is_in_it()
}

/// What the agent read out of the project.
pub fn the_agent_reads(
    the_project: &mut impl TheProjectOnDisk,
    a_path: &ThePathInsideTheProject,
) -> Result<String, AFileTheProjectDoesNotHave> {
    let the_text = the_project.the_text_of(a_path);
    the_text.ok_or_else(|| AFileTheProjectDoesNotHave::because(a_path))
}
