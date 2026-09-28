//! The rules: what a work item is, and which moves are legal.
//!
//! This crate has no dependencies at all — not "few", not "only serde". Nothing. The
//! manifest has no `[dependencies]` section, and that is the one rule in this repository
//! the compiler enforces without being asked.
//!
//! Nothing here may reach for a model, a filesystem, a clock, or a process. Reading the
//! repository, writing files, running tests and keeping state are all deterministic and
//! all live outside this crate, behind a port.

mod role;
mod stage;
mod work_item_id;

pub use role::{ARoleThatNobodyNamed, EVERY_ROLE, Role};
pub use stage::{AStageThatNobodyNamed, EVERY_STAGE, Stage};
pub use work_item_id::{AWorkItemThatNamesNothing, WorkItemId};
