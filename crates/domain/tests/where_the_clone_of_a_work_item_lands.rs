//! Where the clone of one work item lands, derived and never chosen.
//!
//! **The destination is a field and not a convention because it is derived, not chosen.** A
//! destination a caller picks is a different directory on the second run of the same work item,
//! so a resumed run looks in the wrong place and finds nothing, and the run that has no answer
//! for that re-clones over its own work.
//!
//! **Two runs of the same work item land in the same place, and two work items never share one.**
//! That pair is the whole argument: the first is why a run may resume, and the second is why one
//! work item's clone is not another work item's.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::where_the_clone_of_a_work_item_lands::where_the_clone_of_a_work_item_lands;
use aisdlc_domain::work_item_id::WorkItemId;

fn the_projects_root() -> std::path::PathBuf {
    std::path::PathBuf::from("/projects")
}

fn the_addressed() -> Repository {
    Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves")
}

fn this_work_item() -> WorkItemId {
    WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank")
}

#[test]
fn two_runs_of_the_same_work_item_land_in_the_same_place() {
    let first = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &the_addressed(),
        &this_work_item(),
    );
    let second = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &the_addressed(),
        &this_work_item(),
    );

    assert_eq!(
        first, second,
        "**because a run resumes**, and a destination chosen by the caller is a different directory on the second run — so a resumed run looks in the wrong place and finds nothing"
    );
}

#[test]
fn two_work_items_never_share_one_place() {
    let this_one = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &the_addressed(),
        &this_work_item(),
    );
    let another = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &the_addressed(),
        &WorkItemId::try_from("WELLBEINGT-13").expect("a name is not blank"),
    );

    assert_ne!(
        this_one, another,
        "**because one work item's clone is not another's**, and two work items sharing a directory is two runs writing over each other"
    );
}

#[test]
fn two_repositories_never_share_one_place() {
    let this_one = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &the_addressed(),
        &this_work_item(),
    );
    let another = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &Repository::try_from("steamnoid/other").expect("two halves"),
        &this_work_item(),
    );

    assert_ne!(
        this_one, another,
        "and the same work item name in two repositories is two pieces of work"
    );
}

#[test]
fn the_place_names_the_owner_the_repository_and_the_work_item() {
    let the_place = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &the_addressed(),
        &this_work_item(),
    );

    let as_written = the_place.to_string_lossy().to_string();
    assert!(
        as_written.contains("steamnoid")
            && as_written.contains("ai-sdlc-app-rs-plus")
            && as_written.contains("WELLBEINGT-5"),
        "**and it says so in the path**, because a place whose components cannot be read off it is a place a person cannot find — the three are the owner, the repository and the work item, in that order"
    );
}

#[test]
fn the_clone_lands_in_a_directory_of_its_own_named_for_what_it_is() {
    let the_place = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &the_addressed(),
        &this_work_item(),
    );

    assert!(
        the_place.ends_with("repository"),
        "**and the clone is in a directory called `repository`**, because the project's directory holds the clone *and* the artifacts beside it — a clone in the project's own directory would have artifacts written into its working tree, and that clone is somebody else's repository"
    );
}
