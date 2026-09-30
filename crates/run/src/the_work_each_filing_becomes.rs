//! Each filing a creative run made becoming a work the product analyst answers.
//!
//! **The shell for now.** It builds nothing and reads nothing — the tests are what demand all of it.

use aisdlc_domain::repository::Repository;

use crate::a_filing_is_answered::TheWork;
use crate::an_issue_read::AnIssueRead;
use crate::an_issue_to_read::AnIssueToRead;
use crate::what_this_filing_is_about::{TheRoleThatAnswers, WhatThisFilingIsAbout};

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
    let mut what_the_run_will_work_on = Vec::new();
    for (one_read, one_address) in what_was_read.iter().zip(where_each_filing_went) {
        what_the_run_will_work_on.push(the_work_the_product_analyst_answers(
            of_which_repository,
            one_read,
            one_address,
        )?);
    }
    Ok(what_the_run_will_work_on)
}

/// The work one filing becomes, **for the product analyst and at the beginning.**
///
/// **The filing is read, and the address is checked against what it was read at** — a read filing
/// paired with the wrong address is a work item about the wrong issue, **and both halves are real, so
/// nothing downstream could tell.**
fn the_work_the_product_analyst_answers(
    of_which: &Repository,
    one_read: &AnIssueRead,
    one_address: &str,
) -> Result<WhatTheRunWillWorkOn, AFilingNoWorkCanBeBuiltFrom> {
    if one_read.where_it_answers() != one_address.trim() {
        return Err(AFilingNoWorkCanBeBuiltFrom(format!(
            "{} was read at {} and paired with {}",
            one_read.what_was_asked(),
            one_read.where_it_answers(),
            one_address
        )));
    }
    let the_issue = AnIssueToRead::the_issue_an_address_returned(of_which, one_address)
        .map_err(|not_an_issue| AFilingNoWorkCanBeBuiltFrom(not_an_issue.0))?;
    let what_a_run_was_told = WhatThisFilingIsAbout::about_a_filing_a_model_made(
        &the_name_it_was_filed_at(of_which, the_issue.which_issue()),
        TheRoleThatAnswers::try_from("PO").expect("a discipline"),
        of_which.clone(),
    );
    let the_work = TheWork::a_filing_becomes(one_read, &what_a_run_was_told);
    Ok(WhatTheRunWillWorkOn {
        the_work_item: the_work.work_item,
        the_work_request: Some(the_work.requested),
    })
}

/// The name a work item is tracked by, **which is the repository and the number GitHub returned.**
fn the_name_it_was_filed_at(of_which: &Repository, which_issue: i64) -> String {
    format!("{of_which}#{which_issue}")
}
