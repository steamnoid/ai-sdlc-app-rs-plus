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
    the_projects_root
        .join(&what_is_addressed.owner)
        .join(&what_is_addressed.name)
        .join(a_work_item.0.as_str())
        .join("repository")
}
