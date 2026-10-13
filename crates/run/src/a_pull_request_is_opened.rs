//! A pull request, opened over the API and answered with the number and the address GitHub
//! composed.
//!
//! **A branch on a remote is not a delivery.** A run that pushed a branch and stopped has left
//! something nobody can see, **and the ports table has claimed `AGitHubWeWriteTo` asks for a
//! pull request since before it did.**
//!
//! **A credential is refused before a word reaches GitHub**, because an empty credential is a
//! request that goes out claiming to be somebody it is not, and a host may answer it
//! differently from a request that makes no claim at all.

use aisdlc_domain::repository::Repository;

use crate::a_credential::ACredential;
use crate::a_program_we_run::AProgramWeRun;
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// The change a run offers, **with nothing of a model's in any of it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APullRequestToOpen {
    where_the_code_was_written: Repository,
    where_the_head_is: Repository,
    which_branch: String,
    the_base: String,
    the_title: String,
    the_body: String,
}

impl APullRequestToOpen {
    /// The request a run makes of its own change, **refused when the title or the body is blank.**
    ///
    /// **A title a reviewer cannot read and a body nobody can check are both refused here**, and a
    /// blank one is a field nobody filled rather than a sentence somebody wrote.
    pub fn the_request_a_run_makes(
        where_the_code_was_written: Repository,
        where_the_head_is: Repository,
        which_branch: &str,
        the_base: &str,
        the_title: &str,
        the_body: &str,
    ) -> Result<Self, ARequestThatIsNotAPullRequest> {
        a_word_says_something(the_title, A_TITLE_NOBODY_CAN_FIND_IN_A_LIST)?;
        a_word_says_something(the_body, A_BODY_NOBODY_CAN_CHECK)?;
        Ok(Self {
            where_the_code_was_written,
            where_the_head_is,
            which_branch: which_branch.to_string(),
            the_base: the_base.to_string(),
            the_title: the_title.to_string(),
            the_body: the_body.to_string(),
        })
    }

    /// Where the request goes, **and that is the repository the code was written in.**
    pub fn where_it_goes(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/pulls",
            self.where_the_code_was_written.owner, self.where_the_code_was_written.name
        )
    }

    /// The title reviewers read first, **and it is the filing's because a title composed by a
    /// model is prose nobody checked.**
    pub fn the_title(&self) -> &str {
        self.the_title.as_str()
    }

    /// The body reviewers check, **and it is the filing's words with the address they were read
    /// at.**
    pub fn the_body(&self) -> &str {
        self.the_body.as_str()
    }

    /// The body GitHub is given, **built by the JSON library and never by a hand.**
    ///
    /// **The head names the fork and the base names what is being changed.** A head naming the
    /// wrong repository is somebody else's branch, **and a base naming the run's own fork asks
    /// GitHub to merge a fork into itself.**
    pub fn the_body_github_is_given(&self) -> String {
        let what_is_asked = serde_json::json!({
            "title": self.the_title,
            "body": self.the_body,
            "head": format!("{}:{}", self.where_the_head_is.owner, self.which_branch),
            "base": self.the_base,
        });
        what_is_asked.to_string()
    }
}

/// Why a title is refused, **and it is a list a reviewer is reading.**
const A_TITLE_NOBODY_CAN_FIND_IN_A_LIST: &str =
    "a pull request with no title is a page nobody can find in a list";

/// Why a body is refused, **and a change is checked by reading it.**
const A_BODY_NOBODY_CAN_CHECK: &str = "a pull request with no body is a change nobody can check";

/// Whether a word says anything, **refused with the reason that word would be refused for.**
fn a_word_says_something(
    one_word: &str,
    why_it_would_be_refused: &str,
) -> Result<(), ARequestThatIsNotAPullRequest> {
    if one_word.trim().is_empty() {
        return Err(ARequestThatIsNotAPullRequest(String::from(
            why_it_would_be_refused,
        )));
    }
    Ok(())
}

/// A request that is not one, **carrying the reason in the words a person can act on.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARequestThatIsNotAPullRequest(pub String);

impl std::fmt::Display for ARequestThatIsNotAPullRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "this is not a pull request to open: {}", self.0)
    }
}

impl std::error::Error for ARequestThatIsNotAPullRequest {}

/// A pull request GitHub really opened, **with the number and the address it composed.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnOpenedPullRequest {
    /// Which number GitHub gave it.
    pub which_number: i64,
    /// Where it answers, word for word as GitHub composed it.
    pub where_it_answers: String,
}

/// Why a pull request a run wanted is not open.
///
/// **Four refusals and not one, because one request can be refused for four different facts and
/// a caller needs to tell them apart in order to answer.** A run that cannot tell "this
/// deployment has no key" from "GitHub said no" is a run that retries the wrong thing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum APullRequestWasNotOpened {
    /// This deployment has no credential, **and nothing was asked of GitHub.**
    ThereWasNoCredential,
    /// GitHub refused, **with its own words quoted whole.**
    GitHubRefusedIt {
        /// What GitHub wrote, and both halves of it.
        what_github_said: String,
    },
    /// GitHub answered and it carried no number, **so it is not a pull request.**
    ThereWasNoNumberInIt {
        /// What GitHub wrote, because a reader has to see what was mistaken for one.
        what_github_said: String,
    },
    /// The answer GitHub gave carried no address, **which is the door a delivery is recorded at.**
    ThereWasNoAddressInIt {
        /// What GitHub wrote.
        what_github_said: String,
    },
}

impl APullRequestWasNotOpened {
    /// GitHub refused, **with its own words quoted whole.**
    pub fn github_refused_it(what_github_said: &str) -> Self {
        APullRequestWasNotOpened::GitHubRefusedIt {
            what_github_said: what_github_said.to_string(),
        }
    }

    /// GitHub answered and it carried no number, **so it is not a pull request.**
    pub fn there_was_no_number_in_it(what_github_said: &str) -> Self {
        APullRequestWasNotOpened::ThereWasNoNumberInIt {
            what_github_said: what_github_said.to_string(),
        }
    }

    /// GitHub answered with a number and no address, **which is the door a delivery is recorded
    /// at.**
    pub fn there_was_no_address_in_it(what_github_said: &str) -> Self {
        APullRequestWasNotOpened::ThereWasNoAddressInIt {
            what_github_said: what_github_said.to_string(),
        }
    }
}

impl std::fmt::Display for APullRequestWasNotOpened {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let the_words = match self {
            APullRequestWasNotOpened::ThereWasNoCredential => {
                String::from("this deployment supplied no credential, and nothing was asked")
            }
            APullRequestWasNotOpened::GitHubRefusedIt { what_github_said }
            | APullRequestWasNotOpened::ThereWasNoNumberInIt { what_github_said }
            | APullRequestWasNotOpened::ThereWasNoAddressInIt { what_github_said } => {
                format!("the pull request was not opened, and GitHub said: {what_github_said}")
            }
        };
        write!(formatter, "{the_words}")
    }
}

impl std::error::Error for APullRequestWasNotOpened {}

/// The request a run makes of its own change, **which is the one and only way to make one.**
pub fn the_request_a_run_makes_of_its_own_change(
    where_the_code_was_written: Repository,
    where_the_head_is: Repository,
    which_branch: &str,
    the_base: &str,
    the_title: &str,
    the_body: &str,
) -> Result<APullRequestToOpen, ARequestThatIsNotAPullRequest> {
    APullRequestToOpen::the_request_a_run_makes(
        where_the_code_was_written,
        where_the_head_is,
        which_branch,
        the_base,
        the_title,
        the_body,
    )
}

/// The pull request is opened, **and the number and the address come from what GitHub wrote.**
pub fn a_pull_request_is_opened(
    the_program: &impl AProgramWeRun,
    what_to_open: &APullRequestToOpen,
    the_credential: Option<&ACredential>,
) -> Result<AnOpenedPullRequest, APullRequestWasNotOpened> {
    if the_credential.is_none() {
        return Err(APullRequestWasNotOpened::ThereWasNoCredential);
    }
    let what_github_said = the_program.run(&WhatWeTellAProgramToDo::open_a_pull_request(
        &what_to_open.where_it_goes(),
        &what_to_open.the_body_github_is_given(),
    ));
    if !what_github_said.did_it_work() {
        let what_it_refused_with = what_github_said.the_words().trim();
        return Err(APullRequestWasNotOpened::github_refused_it(
            what_it_refused_with,
        ));
    }
    what_github_opened(what_github_said.the_words())
}

/// What GitHub answered is read as a pull request, **or the half that is missing is named.**
///
/// **A refusal per missing half and not one refusal, because a number without an address is a
/// delivery a reader cannot open** and an address without a number is a page nobody can find in
/// a list — so the two are told apart.
fn what_github_opened(
    what_github_said: &str,
) -> Result<AnOpenedPullRequest, APullRequestWasNotOpened> {
    let what_it_came_back_as = what_github_wrote_as_json(what_github_said)?;
    let which_number = the_number_in(&what_it_came_back_as, what_github_said)?;
    let where_it_answers = the_address_in(&what_it_came_back_as, what_github_said)?;
    Ok(AnOpenedPullRequest {
        which_number,
        where_it_answers,
    })
}

/// What GitHub wrote, read by the JSON library and never by a hand.
fn what_github_wrote_as_json(
    what_github_said: &str,
) -> Result<serde_json::Value, APullRequestWasNotOpened> {
    let not_a_number = APullRequestWasNotOpened::there_was_no_number_in_it;
    serde_json::from_str(what_github_said).map_err(|_| not_a_number(what_github_said))
}

/// The number GitHub gave it, **and a `2xx` carrying none is not a pull request.**
fn the_number_in(
    what_it_came_back_as: &serde_json::Value,
    what_github_said: &str,
) -> Result<i64, APullRequestWasNotOpened> {
    what_it_came_back_as
        .get("number")
        .and_then(serde_json::Value::as_i64)
        .ok_or_else(|| APullRequestWasNotOpened::there_was_no_number_in_it(what_github_said))
}

/// The address GitHub composed, **and it is the only address there is.**
fn the_address_in(
    what_it_came_back_as: &serde_json::Value,
    what_github_said: &str,
) -> Result<String, APullRequestWasNotOpened> {
    what_it_came_back_as
        .get("html_url")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| APullRequestWasNotOpened::there_was_no_address_in_it(what_github_said))
}
