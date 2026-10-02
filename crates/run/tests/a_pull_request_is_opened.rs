//! A pull request, opened at the address GitHub composed, or the reason it was not.
//!
//! **A branch on a remote is not a delivery.** `a_branch_goes_to_a_remote.rs` proved that a
//! branch this run made really lands and that `main` stays put — **and nothing in this
//! repository has ever asked GitHub for a pull request.** A run that pushed a branch and
//! stopped has left something nobody can see.
//!
//! **A pull request and not a filing, and the two are the same verb at the same API.** A
//! `gh issue create` answers with an address a person can open; **an issue carrying a change
//! is not a reviewable thing**, and `an_issue_read.rs` already refuses a pull request handed
//! to it as if it were one.
//!
//! **The number and the address come from what GitHub wrote and from nowhere else.** A
//! delivery that recorded a number it composed would record a link to a page that does not
//! exist, **which is the exact defect `CommissionedIssue` had and why `AWhereItWasFiled` is
//! the only door an address comes through.**
//!
//! **Nothing of a model's reaches the title or the body.** The words a run delivers with are
//! the plan a person approved and the report a person approved; **a title composed by a model
//! is prose nobody checked, on the one page every reviewer of a change will read first.**

use aisdlc_domain::repository::Repository;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::a_pull_request_is_opened::{
    APullRequestToOpen, APullRequestWasNotOpened, AnOpenedPullRequest, a_pull_request_is_opened,
    the_request_a_run_makes_of_its_own_change,
};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A credential a deployment supplied, **and a real one is never written down in a test.**
fn a_credential() -> ACredential {
    ACredential::the_key_a_deployment_supplied("a-key-no-real-one-is-here").expect("a key")
}

/// The repository a change is offered to, **which is not the fork it was pushed from.**
fn the_repository_the_change_is_offered_to() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

/// The fork a run has a credential for, **which is where its own branch lives.**
fn the_fork_the_run_pushes_to() -> Repository {
    Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves")
}

/// The request a run makes, **with no model anywhere in it.**
fn the_request_this_run_makes() -> APullRequestToOpen {
    the_request_a_run_makes_of_its_own_change(
        the_repository_the_change_is_offered_to(),
        the_fork_the_run_pushes_to(),
        "aisdlc/WELLBEINGT-42",
        "main",
        "the E2E database leaves artifacts behind",
        "A file that was not there, and a test that says so.",
    )
    .expect("a title and a body were both given")
}

/// A program that answers with GitHub's own shape, **and remembers whether it was asked.**
struct AProgramGitHubAnswering {
    what_github_wrote: String,
    did_it_work: bool,
    what_it_was_asked: std::cell::RefCell<Vec<WhatWeTellAProgramToDo>>,
}

impl AProgramGitHubAnswering {
    /// A program GitHub answers with these words.
    fn answering(did_it_work: bool, what_github_wrote: &str) -> Self {
        Self {
            what_github_wrote: what_github_wrote.to_string(),
            did_it_work,
            what_it_was_asked: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// A pull request GitHub really opened, **with the number and the address it composed.**
    fn a_pull_request_was_opened() -> Self {
        Self::answering(
            true,
            r#"{"number":17,"html_url":"https://github.com/steamnoid/wellbeing-tracker-public/pull/17","head":{"ref":"aisdlc/WELLBEINGT-42"}}"#,
        )
    }
}

impl AProgramWeRun for AProgramGitHubAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.what_it_was_asked
            .borrow_mut()
            .push(what_it_is_asked_to_do.clone());
        WhatTheProgramSaid::of(self.did_it_work, &self.what_github_wrote)
    }
}

#[test]
fn a_pull_request_comes_back_with_the_number_and_the_address_github_composed() {
    let the_program = AProgramGitHubAnswering::a_pull_request_was_opened();

    let what_was_opened = a_pull_request_is_opened(
        &the_program,
        &the_request_this_run_makes(),
        Some(&a_credential()),
    );

    assert_eq!(
        what_was_opened,
        Ok(AnOpenedPullRequest {
            which_number: 17,
            where_it_answers: "https://github.com/steamnoid/wellbeing-tracker-public/pull/17"
                .to_string(),
        }),
        "**because a delivery that records a number it composed records a link to a page that \
         does not exist** — and that is the exact defect the filing address had before it became \
         the only door an address comes through. The number and the address are read out of what \
         GitHub wrote, in its own words"
    );
}

#[test]
fn a_credential_is_refused_before_anything_is_asked_of_github() {
    let the_program = AProgramGitHubAnswering::a_pull_request_was_opened();

    let the_refusal = a_pull_request_is_opened(&the_program, &the_request_this_run_makes(), None);

    assert_eq!(
        the_refusal,
        Err(APullRequestWasNotOpened::ThereWasNoCredential),
        "**because an empty credential is a request that goes out claiming to be somebody it is \
         not, and a host may answer it differently** — so it is refused here, before a word \
         reaches GitHub. The program was asked {} times",
        the_program.what_it_was_asked.borrow().len()
    );
}

#[test]
fn a_pull_request_github_refused_is_refused_with_what_github_said() {
    let what_github_said = r#"{"message":"Validation Failed","errors":[{"message":"A pull request already exists for steamnoid:aisdlc/WELLBEINGT-42."}]}"#;
    let the_program = AProgramGitHubAnswering::answering(false, what_github_said);

    let the_refusal = a_pull_request_is_opened(
        &the_program,
        &the_request_this_run_makes(),
        Some(&a_credential()),
    );

    assert_eq!(
        the_refusal,
        Err(APullRequestWasNotOpened::GitHubRefusedIt {
            what_github_said: what_github_said.to_string(),
        }),
        "**because `message` alone is half of what GitHub says and `errors` is the other half** — \
         and a reader told only \"Validation Failed\" cannot tell a branch somebody else is using \
         from a base that does not exist. Quoted whole, and not summarised"
    );
}

#[test]
fn an_answer_carrying_no_number_is_not_a_pull_request() {
    let the_program = AProgramGitHubAnswering::answering(
        true,
        r#"{"message":"A pull request already exists","documentation_url":"https://docs.github.com"}"#,
    );

    let the_refusal = a_pull_request_is_opened(
        &the_program,
        &the_request_this_run_makes(),
        Some(&a_credential()),
    );

    assert!(
        matches!(
            the_refusal,
            Err(APullRequestWasNotOpened::ThereWasNoNumberInIt { .. })
        ),
        "**because only a `2xx` is a pull request** — and a `2xx` that carries no number is not one \
         either. A run that read a number out of an error page and recorded it as a delivery would \
         be reporting a link to nothing: {the_refusal:?}"
    );
}

#[test]
fn the_head_is_the_fork_and_the_base_is_where_the_code_was_written() {
    let the_program = AProgramGitHubAnswering::a_pull_request_was_opened();

    let _ = a_pull_request_is_opened(
        &the_program,
        &the_request_this_run_makes(),
        Some(&a_credential()),
    );

    let what_was_asked = the_program.what_it_was_asked.borrow()[0].clone();
    let the_body = what_was_asked.what_it_is_asked_about().to_string();
    assert!(
        the_body.contains("\"head\"") && the_body.contains("\"base\":\"main\""),
        "**because a pull request whose head is the wrong repository is somebody else's branch, \
         and one whose base is the run's own fork asks GitHub to merge a fork into itself** — so \
         the head names the fork and the base names what is being changed. The body was: {the_body}"
    );
}
