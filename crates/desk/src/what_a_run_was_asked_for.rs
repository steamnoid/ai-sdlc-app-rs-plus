//! What a person is asked for, **and the refusal when what they said cannot mean anything.**
//!
//! **This is the only place that decides what a person has to type.** Everything it says, it says
//! here, and everything it refuses, it refuses **by naming the argument** — so a person who got it
//! wrong is told which one.
//!
//! **It is a front door and not the pipeline.** The order the stages happen in lives in
//! `aisdlc_run::the_whole_of_a_run`; this reads arguments, refuses the ones that cannot mean
//! anything, and hands on. **A second copy of the pipeline here would be a second thing to keep in
//! step with the first, and the first is the one with tests on it.**
//!
//! **A run that was not told where it may write is not started anywhere on a person's disk** — so
//! the place is an argument and not a default, and a run that was not told what to work on is not
//! started at all.

use std::path::PathBuf;

use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;

/// What a person said, **and it is already-validated types because everything a person types is
/// something the domain has an opinion about.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatARunWasAskedFor {
    which_repository: Repository,
    where_it_may_write: PathBuf,
    what_it_should_do: CommissionedRequest,
}

impl WhatARunWasAskedFor {
    /// The repository the run works on.
    pub fn which_repository(&self) -> &Repository {
        &self.which_repository
    }

    /// Where the run may write, **and it is a path and not a string because a run that was told
    /// nowhere in particular is a run that writes where it happens to be started.**
    pub fn where_it_may_write(&self) -> &std::path::Path {
        &self.where_it_may_write
    }

    /// What the run should do, **in the words somebody typed.**
    pub fn what_it_should_do(&self) -> &CommissionedRequest {
        &self.what_it_should_do
    }
}

/// What a person said on a command line, **and the refusal when it cannot mean anything.**
///
/// **Every refusal names the argument**, because a person who typed `owner/name` where `owner` was
/// wanted cannot find the mistake by reading four sentences about a run — **and a front door that
/// explains itself at length is a front door nobody reads.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatWasAskedForThatCannotMeanAnything {
    /// Nothing was asked for at all, **and the refusal names every argument that is missing**
    /// rather than the first one, **because a person who typed one argument and was told that one
    /// was missing would be right about it and still not know what else to type.**
    NothingWasAskedFor {
        /// What was missing, **as a person would write it.**
        what_is_missing: String,
    },
    /// The repository was not `owner/name`.
    NotARepository {
        /// What was typed.
        what_was_said: String,
    },
    /// The place the run may write was not named.
    NowhereToWrite {
        /// What was typed, **and it is empty rather than absent because a person typed `--projects-root`
        /// with nothing after it.**
        what_was_said: String,
    },
    /// What the run should do was not typed, **and a run that does not know what to do has nothing to
    /// ask an agent about.**
    NothingToDo {
        /// What was typed.
        what_was_said: String,
    },
    /// Who reasons about the run was named by nobody here.
    NoOrganisationThatReasons {
        /// What was named.
        what_was_said: String,
    },
}

impl std::fmt::Display for WhatWasAskedForThatCannotMeanAnything {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let why_not = match self {
            Self::NothingWasAskedFor { what_is_missing } => {
                format!("a run needs three arguments: {what_is_missing}")
            }
            Self::NotARepository { what_was_said } => format!(
                "`--repository` is one owner and one name with a slash between them, and `{what_was_said}` is not that"
            ),
            Self::NowhereToWrite { what_was_said } => format!(
                "`--projects-root` is the directory a run may write in, and `{what_was_said}` is not one"
            ),
            Self::NothingToDo { what_was_said } => format!(
                "`--do` is what the run should change, in your own words, and `{what_was_said}` says nothing"
            ),
            Self::NoOrganisationThatReasons { what_was_said } => format!(
                "`LLM_ORGANISATION` is who reasons about the run — openai, ollama, \
                 openai_compatible or openai_responses — and `{what_was_said}` is none of them"
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for WhatWasAskedForThatCannotMeanAnything {}

/// Every argument a run needs, **and it is a function because a refusal that names them all is one
/// sentence rather than three, and a person who typed one of them is owed the other two.**
fn every_argument_a_run_needs() -> String {
    "`--repository owner/name`, `--projects-root DIR`, `--do \"one sentence\"`".to_string()
}

/// What a person said, **or the refusal when it cannot mean anything.**
pub fn what_a_run_was_asked_for(
    what_a_person_typed: &[String],
) -> Result<WhatARunWasAskedFor, WhatWasAskedForThatCannotMeanAnything> {
    let the_repository = the_repository_that_was_typed(what_a_person_typed)?;
    let where_it_may_write = where_the_run_may_write(what_a_person_typed)?;
    let what_it_should_do = what_the_run_should_do(what_a_person_typed)?;
    Ok(WhatARunWasAskedFor {
        which_repository: the_repository,
        where_it_may_write,
        what_it_should_do,
    })
}

/// The repository that was typed, **and a repository is two halves with a slash between them** —
/// which is the domain's own rule and not one this door repeats.
fn the_repository_that_was_typed(
    what_a_person_typed: &[String],
) -> Result<Repository, WhatWasAskedForThatCannotMeanAnything> {
    let what_was_said = what_follows(what_a_person_typed, "--repository");
    let what_was_said =
        what_was_said.ok_or(WhatWasAskedForThatCannotMeanAnything::NothingWasAskedFor {
            what_is_missing: every_argument_a_run_needs(),
        })?;
    Repository::try_from(what_was_said.as_str())
        .map_err(|_why_not| WhatWasAskedForThatCannotMeanAnything::NotARepository { what_was_said })
}

/// Where the run may write, **and nothing is invented here** — a run that was not told is a run
/// that writes wherever it happened to be started, **and that is somebody's home directory.**
fn where_the_run_may_write(
    what_a_person_typed: &[String],
) -> Result<PathBuf, WhatWasAskedForThatCannotMeanAnything> {
    let what_was_said = what_follows(what_a_person_typed, "--projects-root").unwrap_or_default();
    if what_was_said.trim().is_empty() {
        return Err(WhatWasAskedForThatCannotMeanAnything::NowhereToWrite { what_was_said });
    }
    Ok(PathBuf::from(what_was_said))
}

/// What the run should do, **and a sentence a person wrote is the only thing that becomes a
/// `CommissionedRequest`** — a run that was not told has nothing to ask an agent about.
fn what_the_run_should_do(
    what_a_person_typed: &[String],
) -> Result<CommissionedRequest, WhatWasAskedForThatCannotMeanAnything> {
    let what_was_said = what_follows(what_a_person_typed, "--do").unwrap_or_default();
    if what_was_said.trim().is_empty() {
        return Err(WhatWasAskedForThatCannotMeanAnything::NothingToDo { what_was_said });
    }
    let what_was_asked = format!("change what is described in {what_was_said}");
    CommissionedRequest::asked_for(String::from("the request you typed"), what_was_asked)
        .map_err(|_why_not| WhatWasAskedForThatCannotMeanAnything::NothingToDo { what_was_said })
}

/// What follows a flag, **and the flag with nothing after it is an empty answer rather than an
/// absent one** — a person typing `--projects-root` and pressing return meant to say where.
fn what_follows(what_a_person_typed: &[String], a_flag: &str) -> Option<String> {
    let where_the_flag_is = what_a_person_typed.iter().position(|one| one == a_flag)?;
    what_a_person_typed.get(where_the_flag_is + 1).cloned()
}
