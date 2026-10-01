//! A filing, read out of GitHub by `gh`.
//!
//! **This is the half that was missing while the writing half was finished.** A comment could go
//! back to a filing that nothing could bring in — and a system with one end closed is a system that
//! cannot be run, only demonstrated.
//!
//! **A pull request at the address is refused, and here that is the likely reading rather than a
//! corner:** six of the seven entries GitHub answers with in the repository this is pointed at are
//! pull requests.

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_domain::repository::Repository;

use aisdlc_run::a_program_we_run::AProgramWeRun;
use aisdlc_run::an_issue_read::AnIssueRead;
use aisdlc_run::an_issue_to_read::AnIssueToRead;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// Reading a filing with `gh`, which is a program and not a shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AFilingIsRead;

impl AFilingIsRead {
    /// What `gh` would be run with, one thing each, to read one filing.
    pub fn the_arguments_of(&self, which_filing: &AnIssueToRead) -> Vec<String> {
        the_arguments_of(&WhatWeTellAProgramToDo::read_the_filing(
            which_filing.clone(),
        ))
    }

    /// The filing, or the reason there was not one to read.
    pub fn the_filing_is_read(
        &mut self,
        which_filing: &AnIssueToRead,
    ) -> Result<AnIssueRead, aisdlc_run::an_issue_read::WhyAnAnswerIsNotAFiling> {
        let the_answer = self.run(&WhatWeTellAProgramToDo::read_the_filing(
            which_filing.clone(),
        ));
        AnIssueRead::the_issue_read(the_answer.the_words())
    }
}

impl aisdlc_run::a_filing_is_read::AFilingIsReadFrom for AFilingIsRead {
    /// The filing at one address, **asked for by address because a run that filed one holds that and
    /// not a number** — asking for the number again would be this system guessing at a fact GitHub
    /// already composed.
    fn the_filing_at(
        &mut self,
        where_it_answers: &AWhereItWasFiled,
    ) -> Result<AnIssueRead, String> {
        let (of_which, which_issue) = what_an_address_names(where_it_answers.as_str())?;
        let which_filing = AnIssueToRead::the_issue_to_read(of_which, which_issue)
            .map_err(|not_a_number| not_a_number.to_string())?;
        self.the_filing_is_read(&which_filing)
            .map_err(|why_not| why_not.to_string())
    }
}

/// What one of GitHub's own addresses names, **read out of it and never composed into one.**
///
/// **Both halves come from the address**, because a run that just filed something holds that address
/// and nothing else — **and asking for the repository and the number separately would be this system
/// guessing at two facts GitHub had already written down.**
fn what_an_address_names(where_it_answers: &str) -> Result<(Repository, i64), String> {
    let the_address = where_it_answers.trim();
    let Some((the_repository_part, the_number)) = the_address.rsplit_once("/issues/") else {
        return Err(format!("{the_address} is not the address of an issue"));
    };
    let which_issue = the_number
        .parse::<i64>()
        .map_err(|_| format!("{the_address} carries no issue number"))?;
    let (the_owner, the_name) = the_owner_and_the_name_out_of(the_repository_part, the_address)?;
    let of_which = Repository::try_from(format!("{the_owner}/{the_name}").as_str())
        .map_err(|two_halves| two_halves.to_string())?;
    Ok((of_which, which_issue))
}

/// The owner and the name, **from either form the address is held in.**
///
/// **GitHub's `html_url` carries a scheme and `gh issue create` prints one, and this system stores
/// the bare `owner/name/issues/N`** — a real run answered with the bare form and this parser was
/// written for the other, **so it now reads the last two segments rather than a shape.**
fn the_owner_and_the_name_out_of<'a>(
    the_repository_part: &'a str,
    the_address: &'a str,
) -> Result<(&'a str, &'a str), String> {
    let what_is_left = match the_repository_part.rsplit_once("github.com/") {
        Some((_scheme, what_is_after)) => what_is_after,
        None => the_repository_part,
    };
    let Some((the_owner, the_name)) = what_is_left.rsplit_once('/') else {
        return Err(format!("{the_address} names no repository"));
    };
    if the_owner.is_empty() || the_name.is_empty() {
        return Err(format!("{the_address} names no repository"));
    }
    Ok((the_owner, the_name))
}

impl AProgramWeRun for AFilingIsRead {
    /// Run `gh` with no shell, and read the filing out of what it wrote.
    ///
    /// **Both streams and not one, because a refusal from the far end arrives on either** and a
    /// reader that is told only about the stream it did not expect is told nothing.
    fn run(
        &self,
        what_it_is_asked_to_do: &WhatWeTellAProgramToDo,
    ) -> aisdlc_run::a_program_we_run::WhatTheProgramSaid {
        let the_arguments = the_arguments_of(what_it_is_asked_to_do);
        let Ok(what_it_ran) = std::process::Command::new("gh")
            .args(&the_arguments)
            .output()
        else {
            return aisdlc_run::a_program_we_run::WhatTheProgramSaid::of(
                false,
                "gh could not be started, so no filing was read",
            );
        };
        let the_words = what_was_written(&what_it_ran);
        aisdlc_run::a_program_we_run::WhatTheProgramSaid::of(
            what_it_ran.status.success(),
            &the_words,
        )
    }
}

/// What `gh` would be run with, one thing each, and nothing at all when there is no filing to read.
fn the_arguments_of(what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> Vec<String> {
    let WhatWeTellAProgramToDo::ReadTheFiling { which_filing } = what_it_is_asked_to_do else {
        return Vec::new();
    };
    let the_repository = which_filing.of_which_repository();
    vec![
        "api".to_string(),
        format!(
            "repos/{}/{}/issues/{}",
            the_repository.owner,
            the_repository.name,
            which_filing.which_issue()
        ),
    ]
}

/// What a program wrote, and **the answer itself when it wrote one.**
///
/// **The two streams are not concatenated**, because a program that fails writes a complaint on one
/// and a body on the other — `gh` answers a missing filing with `{"message":"Not Found","status":
/// "404"}` on stdout and `gh: Not Found (HTTP 404)` on stderr — **and joining them produces
/// something that is not an answer at all**, so a reader would report that it cannot read a filing
/// that is merely not there. The complaint is kept for when there is no body to read.
fn what_was_written(what_it_ran: &std::process::Output) -> String {
    let the_answer = String::from_utf8_lossy(&what_it_ran.stdout)
        .trim()
        .to_string();
    if !the_answer.is_empty() {
        return the_answer;
    }
    String::from_utf8_lossy(&what_it_ran.stderr)
        .trim()
        .to_string()
}
