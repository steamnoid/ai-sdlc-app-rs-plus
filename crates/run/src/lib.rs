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

pub mod a_comment_is_left;
pub mod a_credential;
pub mod a_filing_is_answered;
pub mod a_git_hub_we_write_to;
pub mod a_model_is_asked_how_it_rules;
pub mod a_path_in_the_project;
pub mod a_program_we_run;
pub mod a_run_asks_what_is_in_its_own_checkout;
pub mod an_issue_read;
pub mod an_issue_to_read;
pub mod how_this_deployment_is_set_up;
pub mod the_path_inside_the_project;
pub mod the_project_on_disk;
pub mod the_repository_to_clone_into;
pub mod the_run_makes_a_checkout;
pub mod what_we_tell_a_program_to_do;
pub mod whether_a_model_rules_here;
pub mod whether_anything_stands_there;
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
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;

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

/// A model that answers with a validated artifact.
pub trait AReasoningModel {
    /// What the model says when asked for one artifact.
    fn what_it_says_when_asked_for(
        &mut self,
        which_artifact: &WhichArtifact,
        which_organisation: WhichOrganisationReasons,
        the_question: &str,
    ) -> String;
}

/// What a model said when it was asked for an artifact, before anything checked it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAnswerIsNotYetAnArtifact {
    which_artifact: WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
    the_question: String,
    what_the_model_said: String,
}

impl AnAnswerIsNotYetAnArtifact {
    /// Which artifact the model was asked for.
    pub fn which_artifact_was_asked_for(&self) -> WhichArtifact {
        self.which_artifact.clone()
    }

    /// The same answer, with what the project holds written into the question.
    ///
    /// **A copy and not a mutation**, because the answer came back from a call already spent and
    /// a model cannot be asked the same question twice for one listing.
    #[must_use]
    pub fn with_the_project_named_in_the_question(
        &self,
        what_is_in_it: &[String],
    ) -> AnAnswerIsNotYetAnArtifact {
        let mut the_answer = self.clone();
        the_answer.the_question = format!(
            "{}\n\nthe project holds:\n{}",
            self.the_question,
            what_is_in_it.join("\n")
        );
        the_answer
    }

    /// Which organisation's model answered.
    pub fn who_reasoned(&self) -> WhichOrganisationReasons {
        self.who_reasoned
    }

    /// The question the model was given.
    pub fn the_question_it_was_given(&self) -> String {
        self.the_question.clone()
    }

    /// The words the model said, unaltered.
    pub fn what_the_model_said(&self) -> String {
        self.what_the_model_said.clone()
    }
}

/// An agent asks a model for the artifact its work item needs.
///
/// **The question is the request somebody typed, and not a summary of it.** A model asked a
/// question nobody asked is a model answering a question of its own, and the request on a work
/// item is the only record that this piece of work is this person's work.
///
/// **The return type is an answer and not an artifact**, so a caller cannot get hold of
/// something with a name that could be mistaken for a plan until something has checked one. A
/// work item with no request cannot be asked about, so it is refused here rather than sent to a
/// model with nothing to answer.
pub fn the_agent_asks_for_its_artifact(
    the_model: &mut impl AReasoningModel,
    the_work_item: &WorkItem,
    which_artifact: &WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
) -> Result<AnAnswerIsNotYetAnArtifact, AnUnanswerableRequest> {
    let the_request = the_work_item.the_request_it_answers();
    let Some(requested) = the_request else {
        return Err(AnUnanswerableRequest::because(
            which_artifact,
            "the work item answers no request",
        ));
    };
    let what_the_model_said =
        the_model.what_it_says_when_asked_for(which_artifact, who_reasoned, &requested.title);
    Ok(AnAnswerIsNotYetAnArtifact {
        which_artifact: which_artifact.clone(),
        who_reasoned,
        the_question: requested.title,
        what_the_model_said,
    })
}

/// A work item whose request a model cannot be asked about, named with the artifact and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnUnanswerableRequest {
    which_artifact: WhichArtifact,
    why_not: String,
}

impl AnUnanswerableRequest {
    /// The artifact the model was about to be asked for.
    pub fn which_artifact_was_asked_for(&self) -> WhichArtifact {
        self.which_artifact.clone()
    }

    /// Why no question could be built.
    pub fn why_it_could_not_be_asked(&self) -> String {
        self.why_not.clone()
    }

    /// This refusal, for a caller that has the artifact and the reason.
    pub fn because(which_artifact: &WhichArtifact, why_not: &str) -> Self {
        Self {
            which_artifact: which_artifact.clone(),
            why_not: why_not.to_string(),
        }
    }
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

    /// This refusal, for a work item standing somewhere else.
    pub fn because(the_stage_it_stands_at: Stage) -> Self {
        Self {
            the_stage_it_stands_at,
        }
    }
}

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

/// The agent asks a model about the project, and the question carries what the project holds.
///
/// **The question is the request and the listing**, because a model asked a question with no
/// repository in it is answering about a repository it has not seen. **The listing is read once
/// and the read has to be used**, or the port is opened, the answer is read, and it is thrown
/// away — which is the shape a call site takes when the thing it reads is not yet needed.
///
/// **A project holding nothing is refused before the call and not after.** A model asked about
/// a repository with nothing in it is answering about nothing, and a refusal that has already
/// spent a call has paid for it.
pub fn the_agent_asks_a_model_about_the_project(
    the_model: &mut impl AReasoningModel,
    the_project: &mut impl TheProjectOnDisk,
    the_work_item: &WorkItem,
    which_artifact: &WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
) -> Result<AnAnswerIsNotYetAnArtifact, AnUnanswerableRequest> {
    let what_is_in_it = the_project.what_is_in_it();
    if what_is_in_it.is_empty() {
        return Err(AnUnanswerableRequest::because(
            which_artifact,
            "the project holds nothing, and a model asked about a repository with nothing in it is answering about nothing",
        ));
    }
    the_agent_asks_a_question_naming(
        the_model,
        the_work_item,
        which_artifact,
        who_reasoned,
        &what_is_in_it,
    )
}

/// The same call, with what the project holds written into the question.
fn the_agent_asks_a_question_naming(
    the_model: &mut impl AReasoningModel,
    the_work_item: &WorkItem,
    which_artifact: &WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
    what_is_in_it: &[String],
) -> Result<AnAnswerIsNotYetAnArtifact, AnUnanswerableRequest> {
    let the_answer =
        the_agent_asks_for_its_artifact(the_model, the_work_item, which_artifact, who_reasoned)?;
    Ok(the_answer.with_the_project_named_in_the_question(what_is_in_it))
}
