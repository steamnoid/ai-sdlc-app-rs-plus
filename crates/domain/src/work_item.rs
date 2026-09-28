//! One tracked unit of work against a repository.
//!
//! **The fields are private, so the doors below are the only ways in.** A work item that
//! could be built by assigning its fields is a work item that can be built at a stage
//! needing an agent while holding nobody — and a checkpoint carrying exactly that was read
//! back as a real work item.

use std::fmt;

use crate::agent_identifier::AgentIdentifier;
use crate::commissioned_request::CommissionedRequest;
use crate::repository::Repository;
use crate::role::Role;
use crate::stage::Stage;
use crate::who_commissioned::WhoCommissioned;
use crate::work_item_id::WorkItemId;

/// The first-class domain object, and the thing every other rule is about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItem {
    /// The name it is tracked by.
    work_item_id: WorkItemId,
    /// Whoever asked for it.
    who_commissioned: WhoCommissioned,
    /// What it is worked on.
    repository: Repository,
    /// The discipline accountable for the result.
    role: Role,
    /// Where it stands, and only `transition` may move it.
    stage: Stage,
    /// The agent holding it, or nobody.
    agent: Option<AgentIdentifier>,
    /// What was asked for, once somebody has said it.
    the_request: Option<CommissionedRequest>,
}

/// The door said no.
///
/// **One refusal type for all three doors**, because a caller that has just been told "this
/// cannot exist" has one question, and three types would make it choose which one to ask.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnUncheckedWorkItem {
    /// Why this work item cannot exist as it was described.
    pub why: String,
}

impl fmt::Display for AnUncheckedWorkItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.why)
    }
}

impl std::error::Error for AnUncheckedWorkItem {}

impl WorkItem {
    /// A work item standing at a stage, held by nobody.
    ///
    /// **Refuses a stage that needs an agent**, because nobody is holding it.
    pub fn standing_at(
        work_item_id: WorkItemId,
        who_commissioned: WhoCommissioned,
        the_repository: Repository,
        the_role: Role,
        the_stage: Stage,
    ) -> Result<Self, AnUncheckedWorkItem> {
        let needs_an_agent = the_stage.an_agent_must_be_holding_it();
        if needs_an_agent {
            return Err(AnUncheckedWorkItem {
                why: format!("{the_stage} needs an agent and this door holds nobody"),
            });
        }
        Ok(WorkItem {
            work_item_id,
            who_commissioned,
            repository: the_repository,
            role: the_role,
            stage: the_stage,
            agent: None,
            the_request: None,
        })
    }

    /// A work item held at a stage, by an agent.
    ///
    /// **A second way to be born and not a step on the first**, because the first refuses
    /// every stage an agent must hold — a work item that could only be chained onto would
    /// be one whose only reachable position is a stage nobody may hold, and the second
    /// refusal would then refuse every work item ever built.
    ///
    /// **Refuses holding a stage where nobody may hold.**
    pub fn held_by(
        work_item_id: WorkItemId,
        who_commissioned: WhoCommissioned,
        the_repository: Repository,
        the_role: Role,
        the_stage: Stage,
        agent: AgentIdentifier,
    ) -> Result<Self, AnUncheckedWorkItem> {
        let no_agent_may_hold = !the_stage.an_agent_must_be_holding_it();
        if no_agent_may_hold {
            return Err(AnUncheckedWorkItem {
                why: format!("{the_stage} is a stage no agent may hold"),
            });
        }
        Ok(WorkItem {
            work_item_id,
            who_commissioned,
            repository: the_repository,
            role: the_role,
            stage: the_stage,
            agent: Some(agent),
            the_request: None,
        })
    }

    /// The same work item, carrying the request.
    ///
    /// **Refuses nothing**: a request is a fact, not a position.
    pub fn answering(self, the_request: CommissionedRequest) -> Result<Self, AnUncheckedWorkItem> {
        let mut answered = self;
        answered.the_request = Some(the_request);
        Ok(answered)
    }
}
