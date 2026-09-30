//! Each filing a creative run made becoming a work the product analyst answers.
//!
//! **The shell for now.** It builds nothing and reads nothing — the tests are what demand all of it.

use aisdlc_domain::repository::Repository;

use crate::an_issue_read::AnIssueRead;
use crate::an_issue_to_read::AnIssueToRead;

/// One piece of work the run will take up, **and the filing it answers.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheRunWillWorkOn {
    /// The work item, named after where the filing went.
    pub the_work_item: aisdlc_domain::work_item::WorkItem,
    /// The request it answers, **which is the filing read back and not what the run remembers.**
    pub the_work_request: Option<aisdlc_domain::commissioned_request::CommissionedRequest>,
}

impl WhatTheRunWillWorkOn {
    /// The name this piece of work is tracked by, **and it is where the filing went.**
    pub fn the_work_items_name(&self) -> aisdlc_domain::work_item_id::WorkItemId {
        self.the_work_item.the_work_item_id()
    }

    /// Who asked for it, **and a filing this system made says a model wrote it.**
    pub fn who_asked(&self) -> String {
        self.the_work_item.who_asked().to_string()
    }

    /// What this work answers, **and it is the filing's own words rather than what the run
    /// remembered writing.**
    pub fn what_it_answers(&self) -> &aisdlc_domain::commissioned_request::CommissionedRequest {
        self.the_work_request
            .as_ref()
            .expect("a work item built from a filing answers that filing")
    }

    /// Which discipline answers for it, **and an offer is the product analyst's question.**
    pub fn role_answers(&self) -> String {
        self.the_work_item.the_role_answering().to_string()
    }
}

/// A filing this system made that no work can be built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFilingNoWorkCanBeBuiltFrom(pub String);

/// The works the run will work on, **one per filing.**
pub fn the_work_each_filing_becomes(
    of_which_repository: &Repository,
    what_was_read: &[AnIssueRead],
    where_each_filing_went: &[String],
) -> Result<Vec<WhatTheRunWillWorkOn>, AFilingNoWorkCanBeBuiltFrom> {
    for (one_read, one_address) in what_was_read.iter().zip(where_each_filing_went) {
        AnIssueToRead::the_issue_an_address_returned(of_which_repository, one_address)
            .map_err(|not_an_issue| AFilingNoWorkCanBeBuiltFrom(not_an_issue.0))?;
        let _ = one_read;
    }
    Ok(Vec::new())
}
