//! An artifact lands beside the clone, and never inside it.
//!
//! **The clone is somebody else's repository and must stay a faithful checkout of it.** Nothing of
//! ours goes into its working tree — **and an artifact that landed inside the clone would be a change
//! to that repository**, which is the one thing this system's boundary exists to prevent.
//!
//! **The place is derived and not chosen**, exactly as the clone's place is: a destination a caller
//! picks is a different directory on the second run of the same work item, **so a resumed run looks
//! in the wrong place, finds nothing, and the run that has no answer for that writes over its own
//! work.**
//!
//! **The version is in the file's name, and that is the point.** An approval names the exact artifact
//! *and version* it ruled on, **and it stops being valid the moment that artifact changes** — so a
//! second production must be a *second file*, or the approval a person gave the first one would point
//! at a document they never read.

use std::path::PathBuf;

use aisdlc_domain::approval::{AnArtifactVersion, WhichArtifact};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::where_an_artifact_lands::{
    AnArtifactThisProjectCannotHold, where_an_artifact_lands,
};
use aisdlc_domain::where_the_clone_of_a_work_item_lands::where_the_clone_of_a_work_item_lands;
use aisdlc_domain::work_item_id::WorkItemId;

/// Where this deployment keeps projects, **and it is an argument because a deployment chooses it.**
fn the_projects_root() -> PathBuf {
    PathBuf::from("/var/aisdlc/projects")
}

/// The repository this work item is about, **and the function is named for what it is *for* rather
/// than for the type it returns** — the naming guard refuses a name that ends on a domain type,
/// because a name after a type claims nothing about which one.
fn what_this_work_is_about() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

fn the_work_item() -> WorkItemId {
    WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name")
}

fn version(one: u32) -> AnArtifactVersion {
    AnArtifactVersion::try_from(one).expect("a version counts from one")
}

/// Where one artifact lands, **and the version is in the name by default.**
fn where_this_artifact_lands(
    which_artifact: &str,
    the_version: u32,
) -> Result<PathBuf, AnArtifactThisProjectCannotHold> {
    where_an_artifact_lands(
        &the_projects_root(),
        &what_this_work_is_about(),
        &the_work_item(),
        &WhichArtifact::try_from(which_artifact).expect("a name is not blank"),
        &version(the_version),
    )
}

#[test]
fn an_artifact_lands_in_the_project_directory_beside_the_clone() {
    let where_it_lands = where_this_artifact_lands("DiscoveryReport", 1);

    assert_eq!(
        where_it_lands,
        Ok(PathBuf::from(
            "/var/aisdlc/projects/steamnoid/wellbeing-tracker-public/WELLBEINGT-42/DiscoveryReport.v1.json"
        )),
        "**because the artifacts are siblings of the clone and not children of it** — a run that puts \\
         its own work inside somebody else's working tree is a run making a change to a repository it \\
         was asked to analyse. It landed at {where_it_lands:?}"
    );
}

#[test]
fn an_artifact_lands_in_the_same_directory_the_clone_was_derived_into() {
    let where_the_clone_lands = where_the_clone_of_a_work_item_lands(
        &the_projects_root(),
        &what_this_work_is_about(),
        &the_work_item(),
    );
    let where_it_lands = where_this_artifact_lands("Requirements", 1).expect("an ordinary name");

    assert_eq!(
        where_it_lands.parent(),
        where_the_clone_lands.parent(),
        "**because the two paths are one directory and not two derivations that happen to agree** — \\
         an artifact in a directory beside the project is an artifact nobody will look for. The clone \\
         is at {where_the_clone_lands:?} and the artifact at {where_it_lands:?}"
    );
}

#[test]
fn the_same_artifact_at_two_versions_is_two_files() {
    let the_first = where_this_artifact_lands("Requirements", 1).expect("an ordinary name");
    let the_second = where_this_artifact_lands("Requirements", 2).expect("an ordinary name");

    assert_ne!(
        the_first, the_second,
        "**because an approval names the exact artifact and version it ruled on** — and a second \\
         production written over the first would leave that approval naming a document the person \\
         never read. The first is {the_first:?} and the second {the_second:?}"
    );
}

#[test]
fn an_artifact_that_would_land_inside_the_clone_is_refused() {
    let refused = where_this_artifact_lands("repository/README.md", 1);

    assert!(
        matches!(
            refused,
            Err(AnArtifactThisProjectCannotHold::ItWouldLandInTheClone { .. })
        ),
        "**because the clone is a faithful checkout of somebody else's repository** — a file of ours \\
         inside its working tree is a change to that repository, and the change would be pushed with \\
         the branch. It refused: {refused:?}"
    );
}

#[test]
fn an_artifact_that_would_escape_the_project_directory_is_refused() {
    let refused = where_this_artifact_lands("../../../../etc/passwd", 1);

    assert!(
        matches!(
            refused,
            Err(AnArtifactThisProjectCannotHold::ItWouldLeaveTheProject { .. })
        ),
        "**because a run's artifacts belong to the run's own directory** — a path that climbs out of \\
         it is a file written somewhere this system does not own, and the refusal is here rather than \\
         in a writer because the path is what is wrong. It refused: {refused:?}"
    );
}
