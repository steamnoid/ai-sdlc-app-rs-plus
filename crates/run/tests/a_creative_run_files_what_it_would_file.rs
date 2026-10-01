//! A creative run from a deployment's flag to the works it will take up.
//!
//! **The flag is read before the model and before GitHub**, because a model asked costs money and a
//! filing made lands in a real repository — **and a deployment that did not ask must get neither a
//! question nor an issue.**
//!
//! **The filings are read back before they become work.** A work item built from what the run
//! remembers writing carries text GitHub never had, **and the two part company the moment a person
//! edits one word.**

use aisdlc_run::a_creative_run_files_what_it_would_file::a_creative_run_files_what_it_would_file;
use aisdlc_run::a_git_hub_we_write_to::{
    AGitHubWeWriteTo, WhatGitHubWasAskedFor, WhereGitHubAnswered,
};
use aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers;

/// The repository offers are filed in.
fn the_repository_offered_in() -> aisdlc_domain::repository::Repository {
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

/// The files the run really showed the model.
fn the_listing() -> Vec<String> {
    vec!["crates/run/src/what_an_agent_offered.rs".to_string()]
}

/// One offer and the three judgements, **which is what the model was asked for.**
fn one_offer_the_model_would_write() -> String {
    let its_content = r#"{"offers":[{"title":"the listing: why nothing compares a path","what_it_asks_for":"compare every cited path with the listing","paths":["crates/run/src/what_an_agent_offered.rs"]}],"what_it_found":"nothing compares the paths","what_it_could_not_settle":"whether the listing is truncated","what_it_expects_next":"add the comparison"}"#;
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":{}}}}}]}}",
        serde_json::to_string(its_content).expect("a string is JSON")
    )
}

/// What GitHub answered for one filing at one address, **with a title the model did not write.**
///
/// **A different title on purpose**, because the assertion this file rests on is that the run
/// preferred GitHub's copy of the words to its own memory of them.
fn what_github_answered_at(its_address: &str, its_number: i64) -> String {
    format!(
        "{{\"number\":{its_number},\"title\":\"Stop E2E runs leaving WAL state\",\"body\":\
         \"the ask\",\"html_url\":\"{its_address}\",\"state\":\"open\"}}"
    )
}

/// **What GitHub holds, shared by two programs** — because `gh issue create` and `gh api` are two
/// programs against one place, **and a run that filed and reads back goes to the same place.**
#[derive(Clone, Default)]
struct WhatGitHubHolds(std::rc::Rc<std::cell::RefCell<String>>);

/// Somewhere that files and hands back a real address, **which is `gh issue create`.**
struct AGitHubThatFiles {
    what_it_was_asked: Vec<WhatGitHubWasAskedFor>,
    what_github_holds: WhatGitHubHolds,
}

/// Somewhere that reads a filing back, **which is `gh api` and a different program.**
struct AGitHubThatReadsBack {
    what_github_holds: WhatGitHubHolds,
}

impl aisdlc_run::a_filing_is_read::AFilingIsReadFrom for AGitHubThatReadsBack {
    fn the_filing_at(
        &mut self,
        _where_it_answers: &aisdlc_domain::commissioned_request::AWhereItWasFiled,
    ) -> Result<aisdlc_run::an_issue_read::AnIssueRead, String> {
        aisdlc_run::an_issue_read::AnIssueRead::the_issue_read(&self.what_github_holds.0.borrow())
            .map_err(|why_not| why_not.to_string())
    }
}

impl AGitHubWeWriteTo for AGitHubThatFiles {
    fn it_is_asked_to(
        &mut self,
        what_it_is_asked_for: &WhatGitHubWasAskedFor,
    ) -> Result<WhereGitHubAnswered, String> {
        self.what_it_was_asked.push(what_it_is_asked_for.clone());
        let its_number = 10 + i64::try_from(self.what_it_was_asked.len()).expect("a few");
        let where_it_went =
            format!("https://github.com/steamnoid/wellbeing-tracker-public/issues/{its_number}");
        *self.what_github_holds.0.borrow_mut() =
            what_github_answered_at(&where_it_went, its_number);
        Ok(WhereGitHubAnswered::an_address_at(&where_it_went))
    }
}

/// A program that answers with one offer **and remembers whether it was asked at all**, because the
/// claim that a run which was not asked does not ask is only checkable here.
struct AProgramThatWasMaybeAsked {
    what_it_said: String,
    was_it_asked: std::cell::RefCell<bool>,
}

impl aisdlc_run::a_program_we_run::AProgramWeRun for AProgramThatWasMaybeAsked {
    fn run(
        &self,
        _what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo,
    ) -> aisdlc_run::a_program_we_run::WhatTheProgramSaid {
        *self.was_it_asked.borrow_mut() = true;
        aisdlc_run::a_program_we_run::WhatTheProgramSaid::of(true, &self.what_it_said)
    }
}

/// A deployment, its model and its two programs against GitHub, **all four of them a test needs.**
struct ADeploymentAndItsGitHub {
    the_model: AProgramThatWasMaybeAsked,
    where_it_files: AGitHubThatFiles,
    where_it_reads_back: AGitHubThatReadsBack,
}

impl ADeploymentAndItsGitHub {
    /// A deployment whose model would offer one grounded piece of work, **and nothing else set up
    /// yet** — the flag travels with the question, not with the deployment.
    fn one_ready_to_be_asked() -> Self {
        let what_github_holds = WhatGitHubHolds::default();
        ADeploymentAndItsGitHub {
            the_model: AProgramThatWasMaybeAsked {
                what_it_said: one_offer_the_model_would_write(),
                was_it_asked: std::cell::RefCell::new(false),
            },
            where_it_files: AGitHubThatFiles {
                what_it_was_asked: Vec::new(),
                what_github_holds: what_github_holds.clone(),
            },
            where_it_reads_back: AGitHubThatReadsBack { what_github_holds },
        }
    }

    /// What this deployment does, **and whether its model was ever asked.**
    fn what_it_does(
        &mut self,
        flag: WhetherAnAgentOffers,
    ) -> Result<
        Vec<aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn>,
        aisdlc_run::a_creative_run_files_what_it_would_file::AFlagThatIsOff,
    > {
        a_creative_run_files_what_it_would_file(
            &self.the_model,
            &mut self.where_it_files,
            &mut self.where_it_reads_back,
            &the_repository_offered_in(),
            &the_listing(),
            &where_the_model_is(),
            flag,
        )
    }
}

#[test]
fn a_deployment_that_did_not_ask_files_nothing_and_asks_no_model() {
    let mut the_deployment = ADeploymentAndItsGitHub::one_ready_to_be_asked();

    let how_it_went = the_deployment.what_it_does(WhetherAnAgentOffers::No);

    assert!(
        how_it_went.is_err()
            && !*the_deployment.the_model.was_it_asked.borrow()
            && the_deployment.where_it_files.what_it_was_asked.is_empty(),
        "**because the flag is read before the model and before GitHub** — a model asked costs \
         money and a filing made lands in a real repository, **and a deployment that never asked must \
         get neither a question nor an issue**: {how_it_went:?}"
    );
}

#[test]
fn the_refusal_names_the_variable_that_was_not_set() {
    let mut the_deployment = ADeploymentAndItsGitHub::one_ready_to_be_asked();

    let the_refusal = the_deployment
        .what_it_does(WhetherAnAgentOffers::No)
        .expect_err("the flag is off and this refused");

    assert!(
        the_refusal.0.contains("AISDLC_AN_AGENT_OFFERS"),
        "**because a refusal that does not name the variable is a refusal the operator cannot act \
         on** — and this one exists to tell a person their deployment asked for no such agent: \
         {the_refusal:?}"
    );
}

#[test]
fn a_deployment_that_asked_gets_a_work_for_the_filing_it_made() {
    let mut the_deployment = ADeploymentAndItsGitHub::one_ready_to_be_asked();

    let what_the_run_will_work_on = the_deployment
        .what_it_does(WhetherAnAgentOffers::Yes)
        .expect("the flag is on, the model answered and the filing was made");

    assert_eq!(
        (
            what_the_run_will_work_on.len(),
            the_deployment.where_it_files.what_it_was_asked.len(),
            *the_deployment.the_model.was_it_asked.borrow()
        ),
        (1, 1, true),
        "**because one offer is one filing and one filing is one work** — and the model was asked \
         once, which is the fact this test is really about: {what_the_run_will_work_on:?}"
    );
}

#[test]
fn the_work_answers_the_filing_read_back_and_not_what_the_run_wrote() {
    let mut the_deployment = ADeploymentAndItsGitHub::one_ready_to_be_asked();

    let what_the_run_will_work_on = the_deployment
        .what_it_does(WhetherAnAgentOffers::Yes)
        .expect("the flag is on and the filing was made");

    let what_the_work_answers = what_the_run_will_work_on[0].what_it_answers();
    let where_it_was_filed = what_the_work_answers
        .where_it_was_filed()
        .expect("a filing was filed somewhere")
        .as_str();
    assert_eq!(
        (what_the_work_answers.what_was_asked(), where_it_was_filed),
        (
            "Stop E2E runs leaving WAL state",
            "https://github.com/steamnoid/wellbeing-tracker-public/issues/11"
        ),
        "**because the filing was read back** — and what came back is a *different title* from the \
         one the model wrote, **which is the whole point**: a run that remembered its own words would \
         have put the model's title in the work and the two would part company the moment a person \
         edited one word: {what_the_work_answers:?}"
    );
}
