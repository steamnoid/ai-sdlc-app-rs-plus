//! A creative agent filing what it concluded, and getting each address back.
//!
//! **The seam and the thing it stands for have to agree, because the first real run in this
//! repository was refused by an arity at the last step** after seven gates and six models — and
//! nobody had compared them until twenty-four minutes in.
//!
//! **A port that returns nothing cannot hand back an address**, and a filing whose address was
//! thrown away is a filing in a real repository that nobody can find — **so `it_is_asked_to` answers
//! with what GitHub answered, for a comment as much as for a filing.**

use aisdlc_run::a_creative_agent_files_what_it_concluded::a_creative_agent_files_what_it_concluded;
use aisdlc_run::a_git_hub_we_write_to::{
    AGitHubWeWriteTo, WhatGitHubWasAskedFor, WhereGitHubAnswered,
};
use aisdlc_run::what_an_agent_offered::AWorkAnAgentOffered;

/// The repository a filing is filed in.
fn the_repository_it_is_filed_in() -> aisdlc_domain::repository::Repository {
    aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
        .expect("two halves")
}

/// Where a deployment put its model.
fn where_the_model_is() -> aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs {
    aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs::where_a_deployment_put_its_model(
        Some("https://opencode.ai/zen/go/v1/chat/completions"),
        Some("space-bunny-free"),
    )
    .expect("a full address and a model")
}

/// One offer grounded in one file of the repository.
fn an_offer_about_one_file() -> AWorkAnAgentOffered {
    AWorkAnAgentOffered {
        title: "the listing: why nothing compares a path".to_string(),
        what_it_asks_for: "compare every cited path with the listing".to_string(),
        what_it_concluded_from: vec![aisdlc_run::what_discovery_concluded::EvidenceFile {
            path: "crates/run/src/what_an_agent_offered.rs".to_string(),
            link: "https://github.com/steamnoid/wellbeing-tracker-public/blob/main/\
crates/run/src/what_an_agent_offered.rs"
                .to_string(),
        }],
        it_restates: None,
    }
}

/// The files a run really did show the model.
fn the_listing() -> Vec<String> {
    vec!["crates/run/src/what_an_agent_offered.rs".to_string()]
}

/// The address a real `gh issue create` prints, and one that cannot exist.
fn the_address_github_would_print() -> String {
    String::from("https://github.com/steamnoid/wellbeing-tracker-public/issues/11")
}

/// What an endpoint said, wrapping one grounded offer and the three judgements.
fn a_program_answered_with_one_offer() -> String {
    let its_content = r#"{"offers":[{"title":"the listing: why nothing compares a path","what_it_asks_for":"compare every cited path with the listing","paths":["crates/run/src/what_an_agent_offered.rs"]}],"what_it_found":"nothing compares the paths","what_it_could_not_settle":"whether the listing is truncated","what_it_expects_next":"add the comparison"}"#;
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":{}}}}}]}}",
        serde_json::to_string(its_content).expect("a string is JSON")
    )
}

/// A run that records what GitHub was asked and hands back one address.
struct ARunRememberingWhatGitHubSaid {
    what_it_was_asked: Vec<WhatGitHubWasAskedFor>,
    the_answer_it_hands_back: String,
}

impl ARunRememberingWhatGitHubSaid {
    /// A run that answers with the address a real `gh issue create` prints.
    fn one_that_answers() -> Self {
        ARunRememberingWhatGitHubSaid {
            what_it_was_asked: Vec::new(),
            the_answer_it_hands_back: the_address_github_would_print(),
        }
    }
}

impl AGitHubWeWriteTo for ARunRememberingWhatGitHubSaid {
    fn it_is_asked_to(
        &mut self,
        what_it_is_asked_for: &WhatGitHubWasAskedFor,
    ) -> Result<WhereGitHubAnswered, String> {
        self.what_it_was_asked.push(what_it_is_asked_for.clone());
        Ok(WhereGitHubAnswered::an_address_at(
            &self.the_answer_it_hands_back,
        ))
    }
}

/// What an endpoint said, wrapping no offers at all.
fn a_program_answered_with_no_offer() -> String {
    let its_content = r#"{"offers":[],"what_it_found":"nothing worth filing","what_it_could_not_settle":"whether the listing is truncated","what_it_expects_next":"tell a human there was nothing"}"#;
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":{}}}}}]}}",
        serde_json::to_string(its_content).expect("a string is JSON")
    )
}

/// A tracker with nothing on file, **and the run asks it before it asks the model.**
struct ATrackerWithNothingOnFile;

impl aisdlc_run::a_filing_is_read::AFilingIsReadFrom for ATrackerWithNothingOnFile {
    fn the_filing_at(
        &mut self,
        _where_it_answers: &aisdlc_domain::commissioned_request::AWhereItWasFiled,
    ) -> Result<aisdlc_run::an_issue_read::AnIssueRead, String> {
        Err(String::from(
            "this run files issues and reads none of them back, so asking is a bug and not a step",
        ))
    }

    fn the_filings_already_on_file_of(
        &mut self,
        _of_which_repository: &aisdlc_domain::repository::Repository,
    ) -> Result<aisdlc_run::the_filings_already_on_file::TheFilingsAlreadyOnFile, String> {
        Ok(aisdlc_run::the_filings_already_on_file::TheFilingsAlreadyOnFile::none_yet())
    }
}

/// A program that answers with a fixed answer.
struct AProgramThatAnswers(String);

impl aisdlc_run::a_program_we_run::AProgramWeRun for AProgramThatAnswers {
    fn run(
        &self,
        _what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo,
    ) -> aisdlc_run::a_program_we_run::WhatTheProgramSaid {
        aisdlc_run::a_program_we_run::WhatTheProgramSaid::of(true, &self.0)
    }
}

#[test]
fn the_port_hands_back_the_address_github_answered() {
    let mut a_run = ARunRememberingWhatGitHubSaid::one_that_answers();

    let what_github_answered = a_run
        .it_is_asked_to(
            &aisdlc_run::an_offer_becomes_a_filing::the_filing_an_offer_becomes(
                &the_repository_it_is_filed_in(),
                &an_offer_about_one_file(),
            ),
        )
        .expect("GitHub answered");

    assert_eq!(
        what_github_answered.the_address().as_str(),
        the_address_github_would_print(),
        "**because a port that returns nothing cannot hand back an address** — and a filing whose \
         address was thrown away is a filing in a real repository that nobody can find: \
         {what_github_answered:?}"
    );
}

#[test]
fn the_whole_creative_run_files_one_filing_and_returns_its_address() {
    let mut somewhere = ARunRememberingWhatGitHubSaid::one_that_answers();
    let the_model = AProgramThatAnswers(a_program_answered_with_one_offer());

    let mut a_tracker = ATrackerWithNothingOnFile;
    let what_it_filed = a_creative_agent_files_what_it_concluded(
        &the_model,
        &mut somewhere,
        &mut a_tracker,
        &the_repository_it_is_filed_in(),
        &the_listing(),
        &where_the_model_is(),
    )
    .expect("the run filed what the model concluded and knows where");

    assert_eq!(
        what_it_filed.len(),
        1,
        "**because the whole chain is one function** — checkout listing, question, body, answer, \
         grounding, filing, and the address back: {what_it_filed:?}"
    );
    assert_eq!(
        what_it_filed[0].as_str(),
        the_address_github_would_print(),
        "**because the address GitHub composed is the only address** — one this system assembled \
         from a repository and a number resolved to a page that did not exist: {what_it_filed:?}"
    );
}

#[test]
fn a_run_that_is_asked_to_file_nothing_files_nothing() {
    let mut somewhere = ARunRememberingWhatGitHubSaid::one_that_answers();
    let the_model = AProgramThatAnswers(a_program_answered_with_no_offer());

    let mut a_tracker = ATrackerWithNothingOnFile;
    let what_it_filed = a_creative_agent_files_what_it_concluded(
        &the_model,
        &mut somewhere,
        &mut a_tracker,
        &the_repository_it_is_filed_in(),
        &the_listing(),
        &where_the_model_is(),
    )
    .expect("a model that found nothing has told the truth");

    assert!(
        what_it_filed.is_empty() && somewhere.what_it_was_asked.is_empty(),
        "**because a creative agent that files nothing has delivered nothing but has not broken** — \
         the fault is a filing naming a file that is not there, and this committed it zero times: \
         {what_it_filed:?}"
    );
}
