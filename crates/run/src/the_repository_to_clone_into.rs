//! Which repository to clone, and into which directory, for which work item.
//!
//! **Where it lands is derived and not chosen.** A run resumes and two runs of one work item must
//! find the same place, so the directory is a function of the repository and the work item rather
//! than a field somebody could set differently twice.

use std::path::PathBuf;

use aisdlc_domain::repository::Repository;
use aisdlc_domain::the_addresses::the_clone_url_of;
use aisdlc_domain::work_item_id::WorkItemId;

/// A repository to clone, and where it goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARepositoryToClone {
    which_repository: Repository,
    for_which_work_item: WorkItemId,
    where_it_lands: PathBuf,
}

impl ARepositoryToClone {
    /// The repository, the work item it is for, and the directory derived from the two.
    pub fn of(
        which_repository: Repository,
        for_which_work_item: WorkItemId,
        where_it_lands: PathBuf,
    ) -> Self {
        let _which_repository = which_repository;
        let _for_which_work_item = for_which_work_item;
        let _where_it_lands = where_it_lands;
        ARepositoryToClone {
            which_repository: Repository::try_from("nothing").expect("never reached"),
            for_which_work_item: WorkItemId::try_from("nothing").expect("never reached"),
            where_it_lands: PathBuf::new(),
        }
    }

    /// Which repository to fetch.
    pub fn which_repository(&self) -> &Repository {
        &self.which_repository
    }

    /// Which work item the clone is for.
    pub fn for_which_work_item(&self) -> &WorkItemId {
        &self.for_which_work_item
    }

    /// Where the clone lands.
    pub fn where_it_lands(&self) -> &std::path::Path {
        &self.where_it_lands
    }

    /// The address to fetch from, as git is given it.
    pub fn fetch_from(&self) -> String {
        the_clone_url_of(&self.which_repository)
    }
}
