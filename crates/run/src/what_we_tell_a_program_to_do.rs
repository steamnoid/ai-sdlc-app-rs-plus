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

use aisdlc_domain::repository::Repository;
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
        /// Where the model is, which is a deployment fact and not a secret.
        the_endpoint: String,
        /// Which model, which is a deployment fact and not a secret.
        the_model: String,
        /// The question and the shape of the answer, built here and not by the model.
        the_body: String,
    },
    /// Ask a model what work is worth filing, from the listing the run showed it.
    AskAModelWhatItWouldFile {
        /// Where the model is, which is a deployment fact and not a secret.
        the_endpoint: String,
        /// Which model, which is a deployment fact and not a secret.
        the_model: String,
        /// The question and the shape of the answer, built here and not by the model.
        the_body: String,
    },
    /// Every filing already on file, so a run can tell a new finding from a repeated one.
    ListTheFilingsAlreadyOnFile {
        /// Which repository's filings are being listed.
        of_which_repository: Repository,
    },

    /// Read one filing, as a person named it.
    ReadTheFiling {
        /// Which filing, and of which repository.
        which_filing: crate::an_issue_to_read::AnIssueToRead,
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
            WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile {
                of_which_repository,
            } => {
                format!("{}/{}", of_which_repository.owner, of_which_repository.name)
            }
            WhatWeTellAProgramToDo::AskAModelHowItRules { the_body, .. }
            | WhatWeTellAProgramToDo::AskAModelWhatItWouldFile { the_body, .. } => the_body.clone(),
            WhatWeTellAProgramToDo::ReadTheFiling { which_filing } => {
                which_filing.which_issue().to_string()
            }
        }
    }

    /// Ask GitHub for every filing already open on one repository.
    ///
    /// **A listing and not a page number**, because a run that reads the first thirty and asks a
    /// model to weigh them **is a run that never saw filing thirty-one** — and the tracker this is
    /// for already holds more than thirty.
    ///
    /// **By reference**, because the run reads the repository and never writes to it.
    ///
    /// **Named after the filings and not after the repository**, because the vocabulary gate refuses
    /// a name ending on `Repository` — **it is the word for a persistence abstraction wearing a
    /// domain word**, and this is not that. The repository is where the filings live; **it is not
    /// what the run is asking for.**
    pub fn list_the_filings_already_on_file(whose_filings: &Repository) -> Self {
        WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile {
            of_which_repository: whose_filings.clone(),
        }
    }

    /// Read one filing by the number and repository a person named.
    pub fn read_the_filing(which_filing: crate::an_issue_to_read::AnIssueToRead) -> Self {
        WhatWeTellAProgramToDo::ReadTheFiling { which_filing }
    }

    /// Ask a model how it rules on an exact artifact.
    pub fn ask_a_model_how_it_rules(the_endpoint: &str, the_model: &str, the_body: &str) -> Self {
        WhatWeTellAProgramToDo::AskAModelHowItRules {
            the_endpoint: the_endpoint.to_string(),
            the_model: the_model.to_string(),
            the_body: the_body.to_string(),
        }
    }

    /// Ask a model what work is worth filing.
    pub fn ask_a_model_what_it_would_file(
        the_endpoint: &str,
        the_model: &str,
        the_body: &str,
    ) -> Self {
        WhatWeTellAProgramToDo::AskAModelWhatItWouldFile {
            the_endpoint: the_endpoint.to_string(),
            the_model: the_model.to_string(),
            the_body: the_body.to_string(),
        }
    }

    /// This request, read as a question about what a model would file.
    ///
    /// **A question a run cannot classify is a question no flag can hold back**, and refusing to ask
    /// one is the whole of what `AISDLC_AN_AGENT_OFFERS` means.
    pub fn is_a_question_about_what_it_would_file(&self) -> bool {
        matches!(
            self,
            WhatWeTellAProgramToDo::AskAModelWhatItWouldFile { .. }
        )
    }

    /// This request, read as a question about a ruling.
    pub fn is_a_question_about_a_ruling(&self) -> bool {
        matches!(self, WhatWeTellAProgramToDo::AskAModelHowItRules { .. })
    }

    /// The question this request carries, for a request that carries one.
    ///
    /// **Both questions and not only the ruling one.** It answered the ruling question only, **so a
    /// test could read what an agent was told and never what a creative agent was told** — and the
    /// grounding rule, the file listing and the filings already on file all live in the second
    /// question. **A door that opens for one of two questions is half a door.**
    pub fn what_it_is_asked_about(&self) -> &str {
        match self {
            WhatWeTellAProgramToDo::AskAModelHowItRules { the_body, .. }
            | WhatWeTellAProgramToDo::AskAModelWhatItWouldFile { the_body, .. } => the_body,
            _ => "",
        }
    }

    /// Whether this request is answered by `gh` and not by `git`.
    ///
    /// **A listing is `gh` and a clone is `git`,** and one function that asked both used to carry
    /// both vocabularies in one match **— so adding a `gh` request meant counting the lines of a
    /// `git` function, which is the coupling this reports.
    pub fn is_answered_by_gh(&self) -> bool {
        matches!(
            self,
            WhatWeTellAProgramToDo::ReadTheFiling { .. }
                | WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile { .. }
                | WhatWeTellAProgramToDo::AskAModelHowItRules { .. }
                | WhatWeTellAProgramToDo::AskAModelWhatItWouldFile { .. }
        )
    }

    /// Which repository a listing is about, for a request that is a listing.
    pub fn the_repository_it_lists(&self) -> Option<&Repository> {
        match self {
            WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile {
                of_which_repository,
            } => Some(of_which_repository),
            _ => None,
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
            WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile { .. } => None,
            WhatWeTellAProgramToDo::AskAModelHowItRules { .. }
            | WhatWeTellAProgramToDo::AskAModelWhatItWouldFile { .. }
            | WhatWeTellAProgramToDo::ReadTheFiling { .. } => None,
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
            | WhatWeTellAProgramToDo::AskAModelHowItRules { .. }
            | WhatWeTellAProgramToDo::AskAModelWhatItWouldFile { .. }
            | WhatWeTellAProgramToDo::ReadTheFiling { .. }
            | WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile { .. } => None,
        }
    }
}
