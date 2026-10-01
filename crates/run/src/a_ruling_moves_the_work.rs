//! A ruling, and the move it makes in the work item's own state.
//!
//! **An approval is the only move that ends the work, and a refusal sends it back.** Both edges are
//! the table's own — **so this file chooses between two destinations the specification already
//! wrote down and invents neither.**
//!
//! **A work whose ruling was a refusal is not delivered**, and a refusal that ended the work would
//! record a delivery nobody made. **A refused work goes back to whoever was holding it**, read off
//! the work itself rather than named here — **a file that chose which agent gets it back would be
//! choosing an agent, and choosing an agent is not this file's business.**

use aisdlc_domain::stage::Stage;
use aisdlc_domain::state_machine::{StateMachineError, transition};
use aisdlc_domain::work_item::WorkItem;

/// The work moved where its ruling says it goes, **and nowhere else.**
///
/// **Both moves leave no agent holding it**, because `AWAITING_AGENT_PICKUP` and `DONE` are two of
/// the four stages that hold none — and a move that carried the agent along would be refused by the
/// invariant, **which is the point of an invariant.**
pub fn the_ruling_moves_the_work(
    the_work: &WorkItem,
    was_it_approved: bool,
) -> Result<WorkItem, StateMachineError> {
    if was_it_approved {
        return transition(the_work.clone(), Stage::Done, None);
    }
    let who_was_holding_it = the_work.the_agent_holding_it();
    let who_answers = crate::a_run_picks_up_the_work::the_agent_answering_for(the_work).ok();
    let who_gets_it_back = who_was_holding_it.or(who_answers);
    transition(
        the_work.clone(),
        Stage::AwaitingAgentPickup,
        who_gets_it_back,
    )
}
