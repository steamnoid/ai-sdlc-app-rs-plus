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
    /// Every filing already on file, **asked for by GitHub and read out of what it wrote.**
    ///
    /// **`--state all` and not `open`**, because the point is not to duplicate what somebody is
    /// working on **but what somebody already answered.** Filing thirty-one of the same observation
    /// because the earlier ones were closed is the same duplication with a longer fuse.
    fn the_filings_already_on_file_of(
        &mut self,
        of_which_repository: &aisdlc_domain::repository::Repository,
    ) -> Result<aisdlc_run::the_filings_already_on_file::TheFilingsAlreadyOnFile, String> {
        let the_answer = self.run(&WhatWeTellAProgramToDo::list_the_filings_already_on_file(
            of_which_repository,
        ));
        if !the_answer.did_it_work() {
            return Err(format!(
                "the filings already on file could not be read, so this run cannot tell a new \
                 finding from a repeated one: {}",
                the_answer.the_words()
            ));
        }
        the_filings_in(the_answer.the_words())
    }

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
    if let Some(the_repository) = what_it_is_asked_to_do.the_repository_it_lists() {
        return the_arguments_for_a_listing(the_repository);
    }
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

/// What `gh issue list` would be run with, **and every filing on file and not a page of them.**
///
/// **`--state all` and not `open`,** because the point is not to duplicate what somebody is working
/// on **but what somebody already answered** — and filing the fourth copy of a closed observation is
/// the same duplication with a longer fuse.
fn the_arguments_for_a_listing(whose_filings: &Repository) -> Vec<String> {
    vec![
        "issue".to_string(),
        "list".to_string(),
        "--repo".to_string(),
        format!("{}/{}", whose_filings.owner, whose_filings.name),
        "--state".to_string(),
        "all".to_string(),
        "--limit".to_string(),
        "200".to_string(),
        "--json".to_string(),
        "number,title".to_string(),
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

/// Every filing GitHub listed, **and a refusal when GitHub said something else.**
///
/// **Two fields per filing and not more.** A number and a title are what lets a model judge
/// whether it is the same finding; **a body is somebody else's prose and this system's prompt has
/// no business carrying it anywhere.**
fn the_filings_in(
    what_gh_wrote: &str,
) -> Result<aisdlc_run::the_filings_already_on_file::TheFilingsAlreadyOnFile, String> {
    let every_one = the_list_in(what_gh_wrote)?;
    let mut all_of_them =
        aisdlc_run::the_filings_already_on_file::TheFilingsAlreadyOnFile::none_yet();
    for one in &every_one {
        all_of_them.and_this_one_was_too(the_filing_in(one));
    }
    Ok(all_of_them)
}

/// GitHub's own answer, **parsed by the JSON library and not by a hand.**
fn the_list_in(what_gh_wrote: &str) -> Result<Vec<serde_json::Value>, String> {
    let what_it_came_back_as: serde_json::Value =
        serde_json::from_str(what_gh_wrote).map_err(|_| {
            format!("GitHub's answer about the filings on file was not JSON: {what_gh_wrote}")
        })?;
    what_it_came_back_as
        .as_array()
        .cloned()
        .ok_or_else(|| String::from("GitHub's answer about the filings on file was not a list"))
}

/// One filing GitHub listed, **read out of its own two fields and never out of a hand.**
fn the_filing_in(
    one: &serde_json::Value,
) -> aisdlc_run::the_filings_already_on_file::AFilingAlreadyOnFile {
    let number = one
        .get("number")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or_default();
    let title = one
        .get("title")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    aisdlc_run::the_filings_already_on_file::AFilingAlreadyOnFile::that_is_open(number, title)
}
