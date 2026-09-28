//! Where the clone of one work item lands, derived and never chosen.
//!
//! **A destination a caller picks is a different directory on the second run of the same work
//! item**, so a resumed run looks in the wrong place, finds nothing, and the run that has no
//! answer for that writes over its own work.
//!
//! **One clone, many agents, and the artifacts beside it and not inside it.** The clone is
//! somebody else's repository and must stay a faithful checkout of it, so it gets a directory of
//! its own inside a directory that belongs to this system.

use std::path::PathBuf;

use crate::repository::Repository;
use crate::work_item_id::WorkItemId;

/// The directory one work item's clone lands in, derived from three things and nothing else.
pub fn where_the_clone_of_a_work_item_lands(
    the_projects_root: &std::path::Path,
    what_is_addressed: &Repository,
    a_work_item: &WorkItemId,
) -> PathBuf {
    let _the_projects_root = the_projects_root;
    let _what_is_addressed = what_is_addressed;
    let _a_work_item = a_work_item;
    PathBuf::new()
}
