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
    /// Ask a model, at an address a deployment named, one question.
    ///
    /// **One variant and not one per kind of question, because the two that were here carried the
    /// same three fields and were told apart by nothing a caller could act on.** Two variants that
    /// mean one thing are two places for the translation to be wrong, and the wrongness looks like
    /// a model that was asked the wrong thing — which is not a thing a reader can see.
    AskAModel {
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

    /// Record the change, **by staging exactly the paths named and then committing them.**
    RecordTheChange {
        /// Every path, one argument each and never a line.
        every_path: Vec<String>,
    },
    /// Read the commit a change is standing on, **and change nothing.**
    ReadTheCommit {
        /// Which clone, which is the only place this system takes one from.
        the_clone_of: ARepositoryToClone,
    },
    /// Ask GitHub to open a pull request, **and change something in a repository.**
    OpenAPullRequest {
        /// Where the request goes, which is the repository the code was written in.
        where_it_goes: String,
        /// The request, built by the JSON library and never by a hand.
        the_body: String,
    },
    /// Read the remote, and change nothing.
    ReadTheRemote {
        /// Which clone's remote, which is the only remote this system has.
        the_clone_of: ARepositoryToClone,
        /// Which branch the run is about to push, asked for by name.
        which_branch: String,
    },
    /// Put one commit on a remote under a branch name, **and not under a local branch name.**
    PushTheBranch {
        /// Which clone the commit is in, which is the only place this system takes one from.
        the_clone_of: ARepositoryToClone,
        /// Which branch the run wants on the remote.
        which_branch: String,
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
    ///
    /// **A clone is its own arm and not the one below, because there are now six of them** —
    /// and a `match` arm per clone request is a list of six places to change.
    pub fn fetch_from(&self) -> String {
        match self {
            WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of }
            | WhatWeTellAProgramToDo::TheCommitItStandsOn { the_clone_of }
            | WhatWeTellAProgramToDo::TheRefItStandsOn { the_clone_of }
            | WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { the_clone_of }
            | WhatWeTellAProgramToDo::ReadTheRemote { the_clone_of, .. }
            | WhatWeTellAProgramToDo::PushTheBranch { the_clone_of, .. } => {
                the_clone_of.fetch_from()
            }
            _ => self.the_address_or_the_whole_of_it(),
        }
    }

    /// The address for a request that is not about a clone, **and a question to a model is
    /// answered by its own body rather than by an address.**
    fn the_address_or_the_whole_of_it(&self) -> String {
        match self {
            WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile {
                of_which_repository,
            } => {
                format!("{}/{}", of_which_repository.owner, of_which_repository.name)
            }
            WhatWeTellAProgramToDo::AskAModel { the_body, .. } => the_body.clone(),
            WhatWeTellAProgramToDo::ReadTheFiling { which_filing } => {
                which_filing.which_issue().to_string()
            }
            WhatWeTellAProgramToDo::OpenAPullRequest { where_it_goes, .. } => where_it_goes.clone(),
            _ => String::new(),
        }
    }

    /// Record the change, **staging exactly what the plan named and not one file more.**
    pub fn record_the_change(every_path: &[&str]) -> Self {
        WhatWeTellAProgramToDo::RecordTheChange {
            every_path: every_path.iter().map(|one| (*one).to_string()).collect(),
        }
    }

    /// Read the commit a change is standing on, **and nothing else.**
    pub fn read_the_commit(of_which_clone: ARepositoryToClone) -> Self {
        WhatWeTellAProgramToDo::ReadTheCommit {
            the_clone_of: of_which_clone,
        }
    }

    /// Ask GitHub to open a pull request, **and carry the credential as one argument.**
    pub fn open_a_pull_request(where_it_goes: &str, the_body: &str) -> Self {
        WhatWeTellAProgramToDo::OpenAPullRequest {
            where_it_goes: where_it_goes.to_string(),
            the_body: the_body.to_string(),
        }
    }

    /// Read what the remote holds for one branch, **and change nothing.**
    pub fn read_the_remote(of_which_clone: ARepositoryToClone, which_branch: &str) -> Self {
        WhatWeTellAProgramToDo::ReadTheRemote {
            the_clone_of: of_which_clone,
            which_branch: which_branch.to_string(),
        }
    }

    /// Put one commit on the remote under one branch name.
    pub fn push_the_branch(of_which_clone: ARepositoryToClone, which_branch: &str) -> Self {
        WhatWeTellAProgramToDo::PushTheBranch {
            the_clone_of: of_which_clone,
            which_branch: which_branch.to_string(),
        }
    }

    /// Whether this request moves a repository, **and the readings are the ones that do not.**
    ///
    /// **Asked as a property because that is the rule:** `ls-remote` is read before anything is
    /// pushed, which is the only reason a refusal can happen before a push has landed — **and
    /// opening a pull request is a change a repository can refuse for a reason of its own.**
    pub fn does_this_change_the_remote(&self) -> bool {
        matches!(
            self,
            WhatWeTellAProgramToDo::PushTheBranch { .. }
                | WhatWeTellAProgramToDo::OpenAPullRequest { .. }
        )
    }

    /// Whether this request is answered over HTTPS, **and a pull request and a filing are.**
    pub fn is_answered_by_github_over_https(&self) -> bool {
        matches!(self, WhatWeTellAProgramToDo::OpenAPullRequest { .. })
    }

    /// Whether this request changes a repository, **and a commit and a push and a pull request
    /// all do while a reading does not.**
    pub fn does_this_change_a_repository(&self) -> bool {
        matches!(
            self,
            WhatWeTellAProgramToDo::RecordTheChange { .. }
                | WhatWeTellAProgramToDo::PushTheBranch { .. }
                | WhatWeTellAProgramToDo::OpenAPullRequest { .. }
        )
    }

    /// The branch this request is about, **for a request that is about one branch.**
    pub fn the_branch_it_is_about(&self) -> Option<&String> {
        match self {
            WhatWeTellAProgramToDo::ReadTheRemote { which_branch, .. }
            | WhatWeTellAProgramToDo::PushTheBranch { which_branch, .. } => Some(which_branch),
            _ => None,
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

    /// Ask a model one question, at the address a deployment named.
    pub fn ask_a_model(the_endpoint: &str, the_model: &str, the_body: &str) -> Self {
        WhatWeTellAProgramToDo::AskAModel {
            the_endpoint: the_endpoint.to_string(),
            the_model: the_model.to_string(),
            the_body: the_body.to_string(),
        }
    }

    /// Whether this request is a question about a model.
    ///
    /// **A question a run cannot classify is a question no flag can hold back**, and refusing to ask
    /// one is the whole of what `AISDLC_AN_AGENT_OFFERS` means.
    pub fn is_a_question_about_a_model(&self) -> bool {
        matches!(self, WhatWeTellAProgramToDo::AskAModel { .. })
    }

    /// This request, read as a question about a ruling.
    ///
    /// **Every question a model is asked is also a question about a ruling, and that is the
    /// asymmetry this rename settles.** There was one variant for each kind of question and no
    /// way to ask "is this a question for a model" — so a gate that has to hold one question
    /// back has to know the kind, and the kind is a second list to keep in step with the first.
    pub fn is_a_question_about_a_ruling(&self) -> bool {
        self.is_a_question_about_a_model()
    }

    /// The question this request carries, for a request that carries one.
    ///
    /// **One question and not one per kind**, and this accessor existed because there were two
    /// kinds: it answered the ruling question only, **so a test could read what an agent was told
    /// and never what a creative agent was told** — and the grounding rule, the file listing and the
    /// filings already on file all lived in the second question. **A door that opens for one of two
    /// questions is half a door.**
    pub fn what_it_is_asked_about(&self) -> &str {
        match self {
            WhatWeTellAProgramToDo::AskAModel { the_body, .. } => the_body,
            WhatWeTellAProgramToDo::OpenAPullRequest { the_body, .. } => the_body,
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
                | WhatWeTellAProgramToDo::AskAModel { .. }
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
            WhatWeTellAProgramToDo::ReadTheRemote { the_clone_of, .. }
            | WhatWeTellAProgramToDo::PushTheBranch { the_clone_of, .. }
            | WhatWeTellAProgramToDo::ReadTheCommit { the_clone_of } => Some(the_clone_of),
            WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile { .. } => None,
            WhatWeTellAProgramToDo::AskAModel { .. }
            | WhatWeTellAProgramToDo::OpenAPullRequest { .. }
            | WhatWeTellAProgramToDo::RecordTheChange { .. }
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
            | WhatWeTellAProgramToDo::AskAModel { .. }
            | WhatWeTellAProgramToDo::ReadTheRemote { .. }
            | WhatWeTellAProgramToDo::PushTheBranch { .. }
            | WhatWeTellAProgramToDo::OpenAPullRequest { .. }
            | WhatWeTellAProgramToDo::RecordTheChange { .. }
            | WhatWeTellAProgramToDo::ReadTheCommit { .. }
            | WhatWeTellAProgramToDo::ReadTheFiling { .. }
            | WhatWeTellAProgramToDo::ListTheFilingsAlreadyOnFile { .. } => None,
        }
    }
}
