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
    /// Read the commit a clone is standing on.
    TheCommitItStandsOn {
        /// Which clone, and where it landed.
        the_clone_of: ARepositoryToClone,
    },
    /// Read the ref a clone is standing on, which may be a branch and may be `HEAD`.
    TheRefItStandsOn {
        /// Which clone, and where it landed.
        the_clone_of: ARepositoryToClone,
    },
    /// Name the address the code was written in, which may only be read.
    NameWhereTheCodeWasWrittenIn {
        /// Which clone, and where it landed.
        the_clone_of: ARepositoryToClone,
    },
    /// Ask a model how it rules on an exact artifact at an exact version.
    AskAModelHowItRules {
        /// The whole question, built here and not by the model.
        the_question: String,
    },
}

impl WhatWeTellAProgramToDo {
    /// The whole history of a repository, into its own directory.
    pub fn clone_the_repository(the_clone_of: ARepositoryToClone) -> Self {
        WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of }
    }

    /// Which commit the clone at this place is standing on.
    pub fn the_commit_it_stands_on(the_clone_of: ARepositoryToClone) -> Self {
        WhatWeTellAProgramToDo::TheCommitItStandsOn { the_clone_of }
    }

    /// Which ref the clone at this place is standing on, which may be `HEAD`.
    pub fn the_ref_it_stands_on(the_clone_of: ARepositoryToClone) -> Self {
        WhatWeTellAProgramToDo::TheRefItStandsOn { the_clone_of }
    }

    /// Name the address the code was written in, once the copy this run made exists.
    pub fn name_where_the_code_was_written_in(the_clone_of: ARepositoryToClone) -> Self {
        WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { the_clone_of }
    }

    /// This request, read as a question about where the code was written in.
    pub fn is_naming_where_the_code_was_written_in(&self) -> bool {
        matches!(
            self,
            WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { .. }
        )
    }

    /// The address to fetch from, derived from the repository and not a formatted line.
    pub fn fetch_from(&self) -> String {
        match self {
            WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of }
            | WhatWeTellAProgramToDo::TheCommitItStandsOn { the_clone_of }
            | WhatWeTellAProgramToDo::TheRefItStandsOn { the_clone_of }
            | WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { the_clone_of } => {
                the_clone_of.fetch_from()
            }
            WhatWeTellAProgramToDo::AskAModelHowItRules { the_question } => the_question.clone(),
        }
    }

    /// Ask a model how it rules on an exact artifact.
    pub fn ask_a_model_how_it_rules(the_question: String) -> Self {
        WhatWeTellAProgramToDo::AskAModelHowItRules { the_question }
    }

    /// This request, read as a question about a ruling.
    pub fn is_a_question_about_a_ruling(&self) -> bool {
        matches!(self, WhatWeTellAProgramToDo::AskAModelHowItRules { .. })
    }

    /// The question this request carries, for a request that carries one.
    pub fn what_it_is_asked_about(&self) -> &str {
        match self {
            WhatWeTellAProgramToDo::AskAModelHowItRules { the_question } => the_question,
            _ => "",
        }
    }

    /// Which clone the question is about, when this request is about a clone already made.
    pub fn the_clone_it_is_about(&self) -> Option<&ARepositoryToClone> {
        match self {
            WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of }
            | WhatWeTellAProgramToDo::TheCommitItStandsOn { the_clone_of }
            | WhatWeTellAProgramToDo::TheRefItStandsOn { the_clone_of }
            | WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { the_clone_of } => {
                Some(the_clone_of)
            }
            WhatWeTellAProgramToDo::AskAModelHowItRules { .. } => None,
        }
    }

    /// This request, read as a question about the commit a clone stands on.
    pub fn is_the_commit_of_the_clone(&self) -> Option<&ARepositoryToClone> {
        match self {
            WhatWeTellAProgramToDo::TheCommitItStandsOn { the_clone_of } => Some(the_clone_of),
            _ => None,
        }
    }

    /// This request, read as a question about the ref a clone stands on.
    pub fn is_the_ref_of_the_clone(&self) -> Option<&ARepositoryToClone> {
        match self {
            WhatWeTellAProgramToDo::TheRefItStandsOn { the_clone_of } => Some(the_clone_of),
            _ => None,
        }
    }

    /// Where the clone lands.
    pub fn where_it_lands(&self) -> &Path {
        self.the_clone_it_is_about()
            .expect("every request this system can make is about a clone")
            .where_it_lands()
    }

    /// How much history this request carries, and nothing at all when it asks about no clone.
    ///
    /// **`Some(true)` for a clone, because the type has nowhere to put a depth** — there is no
    /// field to hold one, so a shallow clone is not something this system can ask for and is not
    /// something a later change has to remember not to add.
    ///
    /// **`None` for a request that is not about cloning**, and this is why the answer is an
    /// `Option` and not a `bool`. Three of the four requests this system can make are about a
    /// commit, a ref or a second address, and **a `bool` says *no* for all three**, which a caller
    /// reads as *a shallow clone* on a request that is not a clone at all. **Yes and *not a
    /// question* are the same value in a `bool`, and one of them is a lie.**
    pub fn how_much_history_it_carries(&self) -> Option<bool> {
        match self {
            WhatWeTellAProgramToDo::CloneTheRepository { .. } => Some(true),
            WhatWeTellAProgramToDo::TheCommitItStandsOn { .. }
            | WhatWeTellAProgramToDo::TheRefItStandsOn { .. }
            | WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { .. }
            | WhatWeTellAProgramToDo::AskAModelHowItRules { .. } => None,
        }
    }
}
