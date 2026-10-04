//! The run in one page, **written from the run's own state and not from a model.**
//!
//! **A run holds its artifacts and its rulings and nothing writes them down in a form a person
//! reads.** A trace is a machine's account of a run; **this is a page**, and it exists so somebody
//! can find out what a run did without reading one.
//!
//! **No model is asked for any of it, and that is the whole of the design.** A summary written by a
//! model can flatter the run that produced it, **and the one artefact a person reads after a run is
//! the one that must not.** Every line here comes out of the run's state, and a line that could not
//! would be a line nobody could check.
//!
//! **A refused run's page is written too, and it is the one anybody reads.** A refusal leaves no
//! `Approval` anywhere else in this system, **so this page is the last place a reader can find out
//! who stopped the run and why.**

use std::fmt::Write;

use crate::a_path_in_the_project::APathInTheProject;
use crate::the_path_inside_the_project::the_path_a_file_will_be_written_at;
use crate::the_project_on_disk::TheProjectOnDisk;
use crate::what_a_stage_kept_is_written::WhatCouldNotBeWritten;
use crate::where_a_run_stands::WhereARunStands;

/// Where the page lands and what it says, **and the two come back together because a caller that got
/// one without the other has either a page nobody can read or a place nobody can find.**
pub struct ThePageOfARun {
    where_the_page_landed: crate::the_path_inside_the_project::ThePathInsideTheProject,
    what_the_page_says: String,
}

impl ThePageOfARun {
    /// Where the page is, **and it is a resolved path because that is where the file is.**
    pub fn where_the_page_landed(
        &self,
    ) -> &crate::the_path_inside_the_project::ThePathInsideTheProject {
        &self.where_the_page_landed
    }

    /// Exactly what the page says, **and never a rendering of it** — the page is what a reader opens.
    pub fn what_the_page_says(&self) -> &str {
        &self.what_the_page_says
    }
}

/// The run is written down in one page, **and the page is beside the clone with the artifacts.**
pub fn the_summary_of_a_run_is_written(
    the_project: &mut impl TheProjectOnDisk,
    the_run: &WhereARunStands,
) -> Result<ThePageOfARun, WhatCouldNotBeWritten> {
    let what_the_page_says = the_page_is_composed(the_run);
    let inside_the_project = the_path_a_file_will_be_written_at(
        the_project.where_it_lives(),
        &where_the_page_lands_in(the_run)?,
    )
    .map_err(the_place_cannot_hold_the_page)?;
    the_project
        .a_file_is_written(&inside_the_project, &what_the_page_says)
        .map_err(the_project_refused_the_page)?;
    Ok(ThePageOfARun {
        where_the_page_landed: inside_the_project,
        what_the_page_says,
    })
}

/// The page's own words, **and it is composed here rather than asked for, because every line in it
/// is a fact the run already holds.**
fn the_page_is_composed(the_run: &WhereARunStands) -> String {
    let mut the_page = String::new();
    the_run_is_named(&mut the_page, the_run);
    the_request_is_quoted(&mut the_page, the_run);
    the_artifacts_are_listed(&mut the_page, the_run);
    the_rulings_are_listed(&mut the_page, the_run);
    the_page
}

/// The run and the work it was done for, **and the stage is named because "did it finish" is the
/// first question anybody has.**
fn the_run_is_named(the_page: &mut String, the_run: &WhereARunStands) {
    let the_work_item = the_run.the_work_item_it_is();
    let _ = writeln!(
        the_page,
        "# {}",
        the_work_item.the_work_item_id().0.as_str()
    );
    let _ = writeln!(
        the_page,
        "- repository: {}",
        the_work_item.the_repository().name
    );
    let _ = writeln!(the_page, "- stage: {}", the_run.the_stage_it_stands_at());
}

/// The request in the words somebody typed, **and neither field is paraphrased — a summary of a
/// request is a different request.**
fn the_request_is_quoted(the_page: &mut String, the_run: &WhereARunStands) {
    let Some(the_request) = the_run.the_work_item_it_is().the_request_it_answers() else {
        return;
    };
    let _ = writeln!(the_page, "\n## What was asked for\n");
    let _ = writeln!(the_page, "**{}**\n", the_request.what_was_asked());
    let _ = writeln!(the_page, "{}\n", the_request.what_was_said_besides());
}

/// Every artifact and the place it landed, **and the place is the resolved path because a reader
/// clicking a relative one has to know where the reader is.**
fn the_artifacts_are_listed(the_page: &mut String, the_run: &WhereARunStands) {
    let _ = writeln!(the_page, "## What was produced\n");
    for one in the_run.everything_this_run_wrote() {
        let _ = writeln!(
            the_page,
            "- `{}` v{} — {}",
            one.which_artifact.0,
            one.the_version.0,
            one.where_it_landed.as_str()
        );
    }
    let _ = writeln!(the_page);
}

/// Every gate and who ruled at it, **and a refusal is named as one — because a list of three rulings
/// that says nothing about them reads as three approvals.**
fn the_rulings_are_listed(the_page: &mut String, the_run: &WhereARunStands) {
    let _ = writeln!(the_page, "## Who ruled, and why\n");
    for one in the_run.every_ruling_this_run_received() {
        let _ = writeln!(
            the_page,
            "- `{}` v{} **{}** by {} — {}",
            one.which_artifact.0,
            one.the_version.0,
            what_the_answer_was(one.approved),
            one.decided_by.0,
            one.reason
        );
    }
    let _ = writeln!(the_page);
}

/// What an answer was, **and it is a word rather than a boolean because a page is read by a person.**
fn what_the_answer_was(approved: bool) -> &'static str {
    if approved { "approved" } else { "refused" }
}

/// The directory the page lands in, **and it is the run's own directory rather than a file beside
/// an artifact — a page is not an artifact and naming it one would put it in the artifact table.**
fn where_the_page_lands_in(
    the_run: &WhereARunStands,
) -> Result<APathInTheProject, WhatCouldNotBeWritten> {
    let the_work_item = the_run.the_work_item_it_is();
    let where_the_project_is =
        aisdlc_domain::where_an_artifact_lands::where_the_project_of_a_work_item_is(
            std::path::Path::new(""),
            &the_work_item.the_repository(),
            &the_work_item.the_work_item_id(),
        );
    let as_written = where_the_project_is
        .join(format!("{THE_NAME_OF_THE_PAGE}.md"))
        .to_str()
        .unwrap_or_default()
        .to_string();
    crate::a_path_in_the_project::a_path_in_the_project(&as_written).map_err(the_name_is_not_a_path)
}

/// The page's own name is not a path, **and the shape rule's refusal travels whole.**
fn the_name_is_not_a_path(
    why_not: crate::a_path_in_the_project::AStringThatIsNotAPath,
) -> WhatCouldNotBeWritten {
    WhatCouldNotBeWritten::ThePathIsNotAPath {
        why_not: Box::new(why_not),
    }
}
/// The project refused the page, **and the variant names the world fact rather than the layer, so a
/// caller can tell which of the two it was without opening the refusal inside.**
fn the_project_refused_the_page(
    why_not: crate::the_project_on_disk::TheProjectRefusedToWrite,
) -> WhatCouldNotBeWritten {
    use crate::the_project_on_disk::TheProjectRefusedToWrite as WhatTheProjectSaid;
    match &why_not {
        WhatTheProjectSaid::ThereIsAlreadySuchAFile { .. } => the_file_is_already_there(why_not),
        WhatTheProjectSaid::ThereWasNoRoomForIt { .. } => the_disk_had_no_room(why_not),
    }
}

/// The file is already there, **and the refusal travels whole.**
fn the_file_is_already_there(
    why_not: crate::the_project_on_disk::TheProjectRefusedToWrite,
) -> WhatCouldNotBeWritten {
    WhatCouldNotBeWritten::ThereIsAlreadySuchAFile {
        why_not: Box::new(why_not),
    }
}

/// The disk had no room, **and the refusal travels whole.**
fn the_disk_had_no_room(
    why_not: crate::the_project_on_disk::TheProjectRefusedToWrite,
) -> WhatCouldNotBeWritten {
    WhatCouldNotBeWritten::ThereWasNoRoomForIt {
        why_not: Box::new(why_not),
    }
}

/// The page's name, **and it is one and not a version — a run has one page and rewriting it over
/// the last is exactly the thing a version exists to prevent.**
const THE_NAME_OF_THE_PAGE: &str = "summary";

/// The path was refused, **and the refusal travels whole.**
fn the_place_cannot_hold_the_page(
    why_not: crate::the_path_inside_the_project::APathOutsideTheProject,
) -> WhatCouldNotBeWritten {
    WhatCouldNotBeWritten::ThePathWouldNotLandInside {
        why_not: Box::new(why_not),
    }
}
