//! A filing that was read, and the comment that is written back on it.
//!
//! **Two working ends and no use joining them are two proofs and not a system.** The read exists
//! and the comment exists, and a run that never joins them has demonstrated a repository and a
//! filing rather than a thing that works on a filing.
//!
//! **The comment goes to the address that was read, and never to one composed from the number.**
//! An issue that was transferred answers at an address that does not carry the number asked for —
//! **and a comment composed from a number is a comment on a page that has nothing to do with the
//! one the run read.**

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;

use crate::a_comment_is_left::{ANowhereToLeaveAComment, the_run_leaves_a_comment};
use crate::a_git_hub_we_write_to::AGitHubWeWriteTo;
use crate::a_run_picks_up_the_work::a_run_picks_up_the_work_it_was_given;
use crate::an_issue_read::AnIssueRead;
use crate::what_this_filing_is_about::WhatThisFilingIsAbout;
use crate::who_signs_a_comment::WhoSignsAComment;

/// The work a read filing becomes, and the words the person filed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWork {
    /// The work item, and everything a stage later needs to know.
    pub work_item: WorkItem,
    /// The request as the person filed it.
    pub requested: aisdlc_domain::commissioned_request::CommissionedRequest,
}

impl TheWork {
    /// The work a read filing becomes, **under everything a run was told about it and nothing the
    /// code adds on its own.**
    pub fn a_filing_becomes(
        what_was_read: &AnIssueRead,
        what_a_run_was_told: &WhatThisFilingIsAbout,
    ) -> Self {
        a_filing_is_answered_into_work(what_was_read, what_a_run_was_told)
    }
}

impl TheWork {
    /// The work this run takes up, **along the one edge the table allows out of where it stands.**
    pub fn the_run_picks_it_up(
        &self,
    ) -> Result<WorkItem, aisdlc_domain::state_machine::StateMachineError> {
        a_run_picks_up_the_work_it_was_given(&self.work_item)
    }
}

/// A filing that was read, written back on as a comment.
pub fn a_filing_is_answered(
    somewhere: &mut impl AGitHubWeWriteTo,
    what_was_read: &AnIssueRead,
    what_the_run_would_say: &str,
    who_signed: &WhoSignsAComment,
    what_a_run_was_told: &WhatThisFilingIsAbout,
) -> Result<(), ANowhereToLeaveAComment> {
    let the_work = a_filing_is_answered_into_work(what_was_read, what_a_run_was_told);
    the_run_leaves_a_comment(
        somewhere,
        &the_work.requested,
        what_the_run_would_say,
        who_signed,
    )
}

/// A work item standing where work begins, and the caller names who and what.
fn a_work_item_standing_at(what_a_run_was_told: &WhatThisFilingIsAbout) -> WorkItem {
    let the_role =
        Role::try_from(what_a_run_was_told.which_role_answers.as_str()).expect("a discipline");
    WorkItem::standing_at(
        WorkItemId::try_from(what_a_run_was_told.which_work_item.as_str()).expect("a name"),
        what_a_run_was_told.who_asked.clone(),
        what_a_run_was_told.which_repository.clone(),
        the_role,
        Stage::Idle,
    )
    .expect("a work item at a stage")
}

/// The work a read filing becomes, carrying the address GitHub answered at.
///
/// **The address is the one that was read and never one composed from the number**, because a
/// filing that was transferred answers at an address that does not carry the number asked for.
pub fn a_filing_is_answered_into_work(
    what_was_read: &AnIssueRead,
    what_a_run_was_told: &WhatThisFilingIsAbout,
) -> TheWork {
    let where_it_answers =
        AWhereItWasFiled::the_address_the_filing_returned(what_was_read.where_it_answers())
            .expect("an address that was read is an address");
    let requested =
        aisdlc_domain::commissioned_request::CommissionedRequest::asked_for_and_filed_at(
            what_was_read.what_was_asked().to_string(),
            what_was_read.what_was_said_besides().to_string(),
            where_it_answers,
        )
        .expect("a filing somebody wrote is a request");
    let work_item = a_work_item_standing_at(what_a_run_was_told);
    TheWork {
        work_item,
        requested,
    }
}
