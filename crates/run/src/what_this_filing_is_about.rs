//! What a filing is about, and who answers for it — both of which a run is told.
//!
//! **Two values were written into the code.** `A-1` and `PO` sat in a function that builds the
//! work, so every run in history was a work item called `A-1` accountable to the product analyst,
//! whatever was filed. **A record that says the same thing every time is not a record of this run.**
//!
//! **The role is the discipline that answers and not the one that notices.** An issue nobody had
//! looked at yet has a discipline waiting, and only the work has one answering.

use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item_id::WorkItemId;

use crate::a_filing_is_answered::TheWork;
use crate::an_issue_read::AnIssueRead;

/// A work item id that is not one, and what was given instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWorkItemsIdThatIsNotOne(pub String);

/// The name a work item will carry, given by a run and refused when it is blank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWorkItemsId(String);

impl TheWorkItemsId {
    /// The name a run was told, refused when it is not one.
    ///
    /// **A blank id is a work nothing can be found by**, and a run that went ahead would leave a
    /// record nothing points at.
    pub fn try_from(what_a_run_was_told: &str) -> Result<Self, AWorkItemsIdThatIsNotOne> {
        let it_is_a_name = WorkItemId::try_from(what_a_run_was_told);
        match it_is_a_name {
            Ok(_) => Ok(TheWorkItemsId(what_a_run_was_told.to_string())),
            Err(_) => Err(AWorkItemsIdThatIsNotOne(what_a_run_was_told.to_string())),
        }
    }

    /// The name, word for word, **as the run wrote it and not as a type re-spells it.**
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A discipline that is not one, and what was given instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADisciplineThatIsNotOne(pub String);

/// The discipline answering for this work, given by a run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRoleThatAnswers(String);

impl TheRoleThatAnswers {
    /// The discipline a run was told answers, refused when it is not one.
    pub fn try_from(what_a_run_was_told: &str) -> Result<Self, ADisciplineThatIsNotOne> {
        let it_is_a_discipline = Role::try_from(what_a_run_was_told);
        match it_is_a_discipline {
            Ok(_) => Ok(TheRoleThatAnswers(what_a_run_was_told.to_string())),
            Err(_) => Err(ADisciplineThatIsNotOne(what_a_run_was_told.to_string())),
        }
    }

    /// The discipline, word for word, **as the run wrote it.**
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// What a run was told this filing is about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatThisFilingIsAbout {
    /// Which work item this run is.
    pub which_work_item: TheWorkItemsId,
    /// Which discipline answers for it.
    pub which_role_answers: TheRoleThatAnswers,
    /// Who asked, **and a run that filed its own ideas says a model asked rather than signing a
    /// person's name.**
    pub who_asked: WhoCommissioned,
    /// What is being worked on, **never one the code that builds the work item names itself.**
    pub which_repository: Repository,
}

impl AnIssueRead {
    /// The work this filing becomes, **under everything a run was told about it.**
    ///
    /// **The whole of what the run was told and not two fields of it** — because a seam taking a name
    /// and a discipline with no room for who asked is exactly how a work item came to name `"a
    /// person"` whatever the run was told, and **the name is what the seam carries.**
    pub fn the_work_it_becomes(&self, what_a_run_was_told: &WhatThisFilingIsAbout) -> TheWork {
        TheWork::a_filing_becomes(self, what_a_run_was_told)
    }
}

impl WhatThisFilingIsAbout {
    /// What a run was told about a filing **a model made**, and every one of those is a fact rather
    /// than a choice: the name is where it was filed, **the discipline is the product analyst, the
    /// commissioner is a model, and the repository is the one it was filed in.**
    ///
    /// **The commissioner is not a parameter**, because a filing this system made was made by this
    /// system — **and a parameter here would be a way for a caller to write somebody else's name on
    /// a filing nobody else wrote.**
    pub fn about_a_filing_a_model_made(
        which_work_item: &str,
        which_role_answers: TheRoleThatAnswers,
        of_which_repository: Repository,
    ) -> Self {
        WhatThisFilingIsAbout {
            which_work_item: TheWorkItemsId::try_from(which_work_item).expect("a name"),
            which_role_answers,
            who_asked: WhoCommissioned::try_from("llm agent").expect("a name"),
            which_repository: of_which_repository,
        }
    }
}
