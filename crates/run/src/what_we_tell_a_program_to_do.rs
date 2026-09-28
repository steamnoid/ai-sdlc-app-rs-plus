//! What a program is asked to do, named after what the caller wants rather than as a command line.
//!
//! **A port's arguments are named types and never a `String` and never a tuple.** A formatted
//! command line is where a value read out of a model becomes part of a command, so there is
//! nothing here to format and nothing here that takes a string.
//!
//! **A full clone and not a shallow one, and not as a field to set.** The request has no depth,
//! so there is nothing to pass one and nothing for a later change to have forgotten: the run
//! does things with the history — a diff against a base, `git log` for how a file got the way it
//! is — and a shallow clone answers none of them.

use std::path::Path;

use crate::the_repository_to_clone_into::ARepositoryToClone;

/// What we tell one of our programs to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatWeTellAProgramToDo {
    /// Clone a whole repository.
    CloneTheRepository {
        /// Which repository, and the directory derived from it and its work item.
        the_clone_of: ARepositoryToClone,
    },
}

impl WhatWeTellAProgramToDo {
    /// The whole history of a repository, into its own directory.
    pub fn clone_the_repository(the_clone_of: ARepositoryToClone) -> Self {
        WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of }
    }

    /// The address to fetch from, as a field and not as a formatted line.
    pub fn fetch_from(&self) -> &str {
        match self {
            WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of } => {
                "https://github.com/steamnoid/nothing.git"
            }
        }
    }

    /// Where the clone lands.
    pub fn where_it_lands(&self) -> &Path {
        match self {
            WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of } => {
                the_clone_of.where_it_lands()
            }
        }
    }

    /// The whole history, always, because the type has nowhere else to put a depth.
    pub fn is_the_whole_history(&self) -> bool {
        let _the_clone_of = self;
        false
    }
}
