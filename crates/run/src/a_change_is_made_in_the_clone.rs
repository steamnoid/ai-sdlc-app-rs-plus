//! A change the plan named, written into the clone and recorded in its history.
//!
//! **Only where the plan named it, and that is the whole rule.** A plan a person approved is a
//! list of files, **and an implementation writing anything else is a change nobody approved** — a
//! file outside the clone is a file in somebody else's project.
//!
//! **A write identical to what is already there is refused at the writing**, because a change
//! nobody can see is not a change: this repository's history records a run that wrote a file
//! byte-identical to the one already there, committed nothing, **and was caught only by a
//! `nothing to commit` from GitHub forty-five minutes later.**

use crate::a_path_in_the_project::a_path_in_the_project;
use crate::a_program_we_run::AProgramWeRun;
use crate::a_record_of_a_change::AChangeIsRecorded;
use crate::what_a_planner_planned::WhatAPlannerPlanned;
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// One file a plan named, **with the words a person would read if they opened it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFileToWrite {
    where_it_is: String,
    what_it_says: String,
}

impl AFileToWrite {
    /// The file, **refused when the path could not be one at all.**
    ///
    /// **The shape rule and not the confinement one, and the difference is the point.** Every
    /// path here is *relative to the repository*, because that is how a plan names a file, **and
    /// two relative paths are compared to find out whether the plan named this one.** What has to
    /// be refused before anything is written is a path that could be an address somewhere else —
    /// absolute, or stepping out with `..` — **and a path refused here is refused where a person
    /// can be told, not by the operating system afterwards.**
    pub fn the_file_at(
        where_it_is: &str,
        what_it_says: &str,
    ) -> Result<Self, AFileOutsideTheProject> {
        let a_path = a_path_in_the_project(where_it_is).map_err(|not_a_path| {
            AFileOutsideTheProject(not_a_path.what_was_written().to_string())
        })?;
        Ok(Self {
            where_it_is: a_path.as_str().to_string(),
            what_it_says: what_it_says.to_string(),
        })
    }

    /// Where the file is, **relative to the repository and in the spelling the plan used.**
    pub fn where_it_is(&self) -> &str {
        &self.where_it_is
    }

    /// What it says, **word for word as a person wrote it.**
    pub fn what_it_says(&self) -> &str {
        &self.what_it_says
    }
}

/// A path this system will not write to, **quoted whole.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFileOutsideTheProject(pub String);

impl std::fmt::Display for AFileOutsideTheProject {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} is not a path this system will write to: a repository this system writes into is \
             somebody else's working tree, and a path that steps out of it is a file in somebody \
             else's project",
            self.0
        )
    }
}

impl std::error::Error for AFileOutsideTheProject {}

/// What was written and the commit that records it, **both read out of what `git` wrote.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatWasWritten {
    files_written: Vec<String>,
    the_commit: String,
}

impl WhatWasWritten {
    /// What was written, **one path per file and never the words.**
    pub fn files_written_ref(&self) -> &[String] {
        &self.files_written
    }

    /// The commit, **the one `git` reported and never one this system composed.**
    pub fn the_commit(&self) -> &str {
        &self.the_commit
    }
}

/// Why a change the plan named could not be made, **and each is a different fact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AChangeCouldNotBeMade {
    /// The plan did not name this file, **quoted whole so a person can see what was wanted.**
    AFileThePlanDidNotName {
        /// Which file was asked for.
        which_file: String,
    },
    /// The words are what is already there, **so the change nobody can see is refused here.**
    AWriteThatChangesNothing {
        /// Which file was written to.
        which_file: String,
    },
    /// The disk would not take this path, **which only the disk can say.**
    AFileWasNotWritten {
        /// Which file was not written.
        which_file: String,
    },
    /// `git` would not commit, **with its own words because that sentence is the only one.**
    TheChangeWasNotCommitted {
        /// What `git` wrote.
        what_git_said: String,
    },
}

impl AChangeCouldNotBeMade {
    /// The plan named no such file, **quoted whole so a person can see what was wanted.**
    pub fn a_file_the_plan_did_not_name(which_file: &str) -> Self {
        AChangeCouldNotBeMade::AFileThePlanDidNotName {
            which_file: which_file.to_string(),
        }
    }

    /// These words are what the file already says, **so the change nobody can see is refused.**
    pub fn a_write_that_changes_nothing(which_file: &str) -> Self {
        AChangeCouldNotBeMade::AWriteThatChangesNothing {
            which_file: which_file.to_string(),
        }
    }

    /// The disk would not take this path, **which only the disk can say.**
    pub fn a_file_was_not_written(which_file: &str) -> Self {
        AChangeCouldNotBeMade::AFileWasNotWritten {
            which_file: which_file.to_string(),
        }
    }

    /// `git` would not commit, **with its own words because that sentence is the only one.**
    pub fn the_change_was_not_committed(what_git_said: &str) -> Self {
        AChangeCouldNotBeMade::TheChangeWasNotCommitted {
            what_git_said: what_git_said.to_string(),
        }
    }
}

impl std::fmt::Display for AChangeCouldNotBeMade {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let why_not = match self {
            AChangeCouldNotBeMade::AFileThePlanDidNotName { which_file } => format!(
                "the plan named no {which_file}, and a change a person did not approve is not a \
                 change this system may make"
            ),
            AChangeCouldNotBeMade::AWriteThatChangesNothing { which_file } => format!(
                "{which_file} already says this, and a change nobody can see is not a change — a run \
                 that wrote it and committed nothing was caught by the far end forty-five minutes \
                 later"
            ),
            AChangeCouldNotBeMade::AFileWasNotWritten { which_file } => format!(
                "{which_file} was not written, and a repository this system writes into is \
                 somebody else's working tree — a path that steps out of the clone is a file in \
                 somebody else's project"
            ),
            AChangeCouldNotBeMade::TheChangeWasNotCommitted { what_git_said } => {
                format!("nothing was recorded, and git said: {what_git_said}")
            }
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for AChangeCouldNotBeMade {}

/// The change is made and recorded, **or the one reason it could not be.**
///
/// **Every file is checked against the plan before any of them is written**, because a run that
/// wrote four approved files and then refused a fifth has already changed somebody's repository
/// for nothing — **and the approval is a list, so the check is a comparison against a list.**
pub fn a_change_is_made_in_the_clone(
    the_git: &impl AProgramWeRun,
    the_project: &mut impl AChangeIsRecorded,
    the_plan: &WhatAPlannerPlanned,
    every_file: &[AFileToWrite],
) -> Result<WhatWasWritten, AChangeCouldNotBeMade> {
    for one_file in every_file {
        the_plan_named(one_file, the_plan)?;
    }
    for one_file in every_file {
        the_write_is_a_change(the_project, one_file)?;
    }
    let the_commit = the_change_is_committed(the_git, every_file)?;
    Ok(WhatWasWritten {
        files_written: every_file
            .iter()
            .map(|one| one.where_it_is().to_string())
            .collect(),
        the_commit,
    })
}

/// Whether the plan named this file, **and a list compared to a list and nothing else.**
fn the_plan_named(
    one_file: &AFileToWrite,
    the_plan: &WhatAPlannerPlanned,
) -> Result<(), AChangeCouldNotBeMade> {
    if the_plan
        .the_paths_it_named()
        .contains(&one_file.where_it_is().to_string())
    {
        return Ok(());
    }
    Err(AChangeCouldNotBeMade::a_file_the_plan_did_not_name(
        one_file.where_it_is(),
    ))
}

/// The one refusal at the writing, **and it is the one this repository was burned by.**
///
/// **The place is checked before the words, and the other way round is wrong.** A file whose words
/// are what it already holds is refused whether or not the path is safe, **but a file at a path
/// the disk will not take has to be refused by the disk's own answer rather than a guess made
/// here** — so the two refusals arrive in the order the two facts are known.
fn the_write_is_a_change(
    the_project: &mut impl AChangeIsRecorded,
    one_file: &AFileToWrite,
) -> Result<(), AChangeCouldNotBeMade> {
    if the_write_changes_nothing(the_project, one_file) {
        return Err(AChangeCouldNotBeMade::a_write_that_changes_nothing(
            one_file.where_it_is(),
        ));
    }
    let was_it_written =
        the_project.one_file_is_written(one_file.where_it_is(), one_file.what_it_says());
    was_it_written
        .map_err(|not_written| AChangeCouldNotBeMade::a_file_was_not_written(&not_written.0))
}

/// Whether these words are what the file already says, **read before the write and not after.**
fn the_write_changes_nothing(
    the_project: &impl AChangeIsRecorded,
    one_file: &AFileToWrite,
) -> bool {
    let what_it_holds = the_project.what_it_holds_of(one_file.where_it_is());
    what_it_holds.as_deref() == Some(one_file.what_it_says())
}

/// The commit, **read out of what `git` wrote and refused when there is no commit in it.**
fn the_change_is_committed(
    the_git: &impl AProgramWeRun,
    every_file: &[AFileToWrite],
) -> Result<String, AChangeCouldNotBeMade> {
    let every_path: Vec<&str> = every_file.iter().map(AFileToWrite::where_it_is).collect();
    let what_git_said = the_git.run(&WhatWeTellAProgramToDo::record_the_change(&every_path));
    the_commit_out_of(what_git_said.the_words())
}

/// The commit inside what `git` wrote, **and a refusal when it wrote no commit.**
///
/// **`[branch 9f1c2a4] the subject` and nothing else, and the commit is the last word before the
/// bracket** — because the branch name carries slashes and spaces are not in it, **while the
/// text after the bracket is a sentence and a sentence is not an identifier.** The words `git`
/// prints are a fact about what happened; **a commit this system composed would be a record of
/// something that never happened**, and `nothing to commit, working tree clean` carries no
/// bracket and so no commit at all.
fn the_commit_out_of(what_git_wrote: &str) -> Result<String, AChangeCouldNotBeMade> {
    let the_first_line = what_git_wrote.lines().next().unwrap_or_default();
    let Some(what_is_written) = the_first_line.strip_prefix('[') else {
        return Err(the_git_recorded_nothing(what_git_wrote));
    };
    let Some((before_the_bracket, _after_it)) = what_is_written.split_once(']') else {
        return Err(the_git_recorded_nothing(what_git_wrote));
    };
    let the_commit = before_the_bracket
        .split_whitespace()
        .next_back()
        .unwrap_or_default();
    if the_commit.is_empty() {
        return Err(the_git_recorded_nothing(what_git_wrote));
    }
    Ok(the_commit.to_string())
}

/// The refusal for words that carry no commit, **quoted whole because that sentence is the only
/// thing that says so.**
fn the_git_recorded_nothing(what_git_wrote: &str) -> AChangeCouldNotBeMade {
    AChangeCouldNotBeMade::the_change_was_not_committed(what_git_wrote.trim())
}
