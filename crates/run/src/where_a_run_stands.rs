//! Where a run stands, and the one rule about who may move a work item.
//!
//! **A run is a state and a state is one value.** Eight stages, seven gates, a suite that ran twice
//! — **and none of it is any use unless a reader can ask a run where it is and be told one thing.**
//!
//! **Every move goes through `transition()` and this module writes nothing.** Recording a move and
//! commissioning are two functions, **and one function answering both could only answer one** —
//! which is exactly how a register froze at one stage while a real run went through eight gates and
//! opened a pull request.

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::{StateMachineError, transition};
use aisdlc_domain::work_item::WorkItem;

/// Where a run stands, **and nothing about how it got there.**
///
/// **`WhichArtifact` is `Option` and that is the whole of "before the first gate"** — a run that
/// has not reached an artifact has not reached a gate, **and naming one anyway would put a name in a
/// record for work nobody has presented.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereARunStands {
    the_work_item: WorkItem,
    which_artifact_it_is_on: Option<WhichArtifact>,
}

impl WhereARunStands {
    /// A run that has begun and is standing at its first stage.
    ///
    /// **The artifact is an argument and is not derived**, because which stage a run is at is the
    /// framework's business **and which artifact it is producing is the pipeline's** — a function
    /// that worked the second out from the first would be a second ordering nobody chose.
    pub fn at_the_first_stage(the_work_item: WorkItem, which_artifact: &str) -> Self {
        Self {
            the_work_item,
            which_artifact_it_is_on: Some(
                WhichArtifact::try_from(which_artifact).expect("a name is not blank"),
            ),
        }
    }

    /// A run that has not reached an artifact yet, **and says so by naming none.**
    pub fn before_anything(the_work_item: WorkItem) -> Self {
        Self {
            the_work_item,
            which_artifact_it_is_on: None,
        }
    }

    /// Which artifact this run is on, **and `None` before the first gate.**
    pub fn which_artifact_it_is_on(&self) -> Option<&WhichArtifact> {
        self.which_artifact_it_is_on.as_ref()
    }

    /// Which stage the work stands at, **read and never written.**
    pub fn the_stage_it_stands_at(&self) -> Stage {
        self.the_work_item.the_stage_it_stands_at()
    }

    /// Who holds the work, **and `None` at every stage where no agent may hold it.**
    pub fn who_holds_it(&self) -> Option<AgentIdentifier> {
        self.the_work_item.the_agent_holding_it()
    }

    /// The work item itself, **and a reader asking a run where it is gets this and not the
    /// fields** — a run that handed out its work item would hand out its only writer.
    fn the_work_item(&self) -> &WorkItem {
        &self.the_work_item
    }

    /// The work item, **read only and by reference.**
    ///
    /// **Read only is the whole of what this exposes.** A caller that needs to move the work goes
    /// through the moves below, **because a work item handed out by value is a second writer** — and
    /// the table is the only thing allowed to write a stage.
    pub fn the_work_item_it_is(&self) -> &WorkItem {
        &self.the_work_item
    }

    /// This run at another stage, **and it is the only place a move is made.**
    fn moved_to(
        self,
        to: Stage,
        the_agent: Option<AgentIdentifier>,
    ) -> Result<Self, StateMachineError> {
        let moved = transition(self.the_work_item, to, the_agent)?;
        Ok(Self {
            the_work_item: moved,
            which_artifact_it_is_on: self.which_artifact_it_is_on,
        })
    }

    /// This run at another stage **and on another artifact**, **and it is the only place an
    /// artifact is written.**
    ///
    /// **The stage and the artifact move together because they are one fact about the work.** A run
    /// handed to the product analyst while still standing on `DiscoveryReport` would route back to
    /// the product analysis on the next step, **which is the router defect that cost the sibling a
    /// run while its suite stayed green** — so the two cannot be set apart here even though a
    /// reader might expect them to be.
    fn moved_onto(
        self,
        to: Stage,
        the_agent: Option<AgentIdentifier>,
        which_artifact_it_is_on: WhichArtifact,
    ) -> Result<Self, StateMachineError> {
        let moved = transition(self.the_work_item, to, the_agent)?;
        Ok(Self {
            the_work_item: moved,
            which_artifact_it_is_on: Some(which_artifact_it_is_on),
        })
    }
}

/// The work is handed to an agent, **and the move and the handover are one fact.**
///
/// **An agent holding work that is still standing at `AWAITING_AGENT_PICKUP` is a state the table
/// forbids, and checking that in two places would be two places to forget.**
pub fn the_work_is_handed_to_an_agent(
    the_run: &WhereARunStands,
    to_which_agent: AgentIdentifier,
) -> Result<WhereARunStands, StateMachineError> {
    let _ = the_run.the_work_item();
    the_run
        .clone()
        .moved_to(Stage::InProgressByAgent, Some(to_which_agent))
}

/// The work is presented for a ruling, **and the agent lets go at that moment.**
///
/// **A work item at a gate with an agent still holding it is a state the invariant forbids**, and
/// the refusal would come from a check reading a field nobody set.
pub fn the_work_is_presented_for_a_ruling(
    the_run: &WhereARunStands,
) -> Result<WhereARunStands, StateMachineError> {
    the_run.clone().moved_to(Stage::AwaitingHumanApproval, None)
}

/// The work is handed to the next agent, **and the run moves onto whatever that agent produces.**
///
/// **The artifact is an argument and is not derived from the stage**, because the router's table is
/// where the pipeline's order lives, **and a function that worked the second artifact out from the
/// stage would be a second ordering nobody chose.**
pub fn the_work_is_handed_to_the_next_agent(
    the_run: &WhereARunStands,
    to_which_agent: AgentIdentifier,
    which_artifact_it_produces: WhichArtifact,
) -> Result<WhereARunStands, StateMachineError> {
    the_run.clone().moved_onto(
        Stage::AwaitingAgentPickup,
        Some(to_which_agent),
        which_artifact_it_produces,
    )
}

/// The run is over, **and a refusal is the only thing that reaches it.**
///
/// **A refused delivery does not reach it either**, which is a decision about the delivery and not
/// about this function — **so nothing here says which refusal it was**, and the caller that knows
/// is the one that decides.
pub fn the_run_ends_on_a_refusal(
    the_run: &WhereARunStands,
) -> Result<WhereARunStands, StateMachineError> {
    the_run.clone().moved_to(Stage::Done, None)
}

/// The work item is delivered, and the run is over.
///
/// **`AWAITING_HUMAN_APPROVAL -> DONE` is the only edge into `DONE` and it comes from a gate**, so a
/// delivery cannot finish a run by announcing itself — **it puts what it delivered to a person, and
/// the table's own edge is what carries the work home.** The delivery node is where that edge is
/// taken; this function is only where it is legal.
pub fn the_work_is_delivered_so_it_is_done(
    the_run: &WhereARunStands,
) -> Result<WhereARunStands, StateMachineError> {
    the_run.clone().moved_to(Stage::Done, None)
}
