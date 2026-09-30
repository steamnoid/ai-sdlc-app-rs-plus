//! A ruling that reached the gate, and the record left behind.
//!
//! **A model's ruling is a decision and never a record.** The gate's own recorder takes what a
//! person decided, so a model could rule and the ruling went nowhere — **and a run carrying a
//! decision nobody can point at is a run whose next step has no evidence behind it.**
//!
//! **The record names whoever answered, and that is the only difference between a person and a
//! model.** A record that said "approved" and left the name off would be evidence with no author,
//! **and a record that put a person's name on a model's ruling would be a forged audit trail.**

use aisdlc_domain::approval::{Approval, ApprovalDecision};

/// The record a ruling left behind, and `None` when it refused.
///
/// **`None` is the whole shape of a refusal.** A refusal is a decision and not an approval, and a
/// record of a refusal would be a record of something that was approved — **which is the one thing
/// a record must not be.** The word already existed; this only gives it a door.
pub fn the_gate_records_the_ruling_it_was_given(the_ruling: &ApprovalDecision) -> Option<Approval> {
    Approval::what_the_ruling_left_behind(the_ruling)
}
