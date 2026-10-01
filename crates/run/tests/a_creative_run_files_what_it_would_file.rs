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

/// A program that answers with one offer **and remembers whether it was asked at all**, because
/// the claim that a run which was not asked does not ask is only checkable here.
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

/// What GitHub answered for one filing at one address.
fn what_github_answered_at(its_address: &str, its_number: i64) -> String {
    format!(
        "{{\"number\":{its_number},\"title\":\"Stop E2E runs leaving WAL state\",\"body\":\
         \"the ask\",\"html_url\":\"{its_address}\",\"state\":\"open\"}}"
    )
}

/// Somewhere that files and hands back a real address, **which is `gh issue create`.**
struct AGitHubThatFiles {
    what_it_was_asked: Vec<WhatGitHubWasAskedFor>,
    what_it_hands_back: String,
}

/// Somewhere that reads a filing back, **which is `gh api` and a different program.**
struct AGitHubThatReadsBack {
    what_it_hands_back: String,
}

impl aisdlc_run::a_filing_is_read::AFilingIsReadFrom for AGitHubThatReadsBack {
    fn the_filing_at(
        &mut self,
        _where_it_answers: &aisdlc_domain::commissioned_request::AWhereItWasFiled,
    ) -> Result<aisdlc_run::an_issue_read::AnIssueRead, String> {
        aisdlc_run::an_issue_read::AnIssueRead::the_issue_read(&self.what_it_hands_back)
            .map_err(|why_not| why_not.to_string())
    }
}

impl AGitHubWeWriteTo for AGitHubThatFiles {
    fn it_is_asked_to(
        &mut self,
        what_it_is_asked_for: &WhatGitHubWasAskedFor,
    ) -> Result<WhereGitHubAnswered, String> {
        self.what_it_was_asked.push(what_it_is_asked_for.clone());
        let its_number = 10 + self.what_it_was_asked.len() as i64;
        let its_number = 10 + self.what_it_was_asked.len() as i64;
        let where_it_went =
            format!("https://github.com/steamnoid/wellbeing-tracker-public/issues/{its_number}");
        self.what_it_hands_back = what_github_answered_at(&where_it_went, its_number);
        Ok(WhereGitHubAnswered::an_address_at(&where_it_went))
    }
}

#[test]
fn a_deployment_that_did_not_ask_files_nothing_and_asks_no_model() {
    let mut the_program = AProgramThatWasMaybeAsked {
        what_it_said: one_offer_the_model_would_write(),
        was_it_asked: std::cell::RefCell::new(false),
    };
    let mut somewhere = AGitHubThatFiles {
        what_it_was_asked: Vec::new(),
        what_it_hands_back: String::new(),
    };
    let mut somewhere_else = AGitHubThatReadsBack {
        what_it_hands_back: String::new(),
    };

    let how_it_went = a_creative_run_files_what_it_would_file(
        &the_program,
        &mut somewhere,
        &mut somewhere_else,
        &the_repository_offered_in(),
        &the_listing(),
        &where_the_model_is(),
        WhetherAnAgentOffers::No,
    );

    assert!(
        how_it_went.is_err()
            && !*the_program.was_it_asked.borrow()
            && somewhere.what_it_was_asked.is_empty(),
        "**because the flag is read before the model and before GitHub** — a model asked costs \
         money and a filing made lands in a real repository, **and a deployment that never asked must \
         get neither a question nor an issue**: {how_it_went:?}"
    );
}

#[test]
fn the_refusal_names_the_variable_that_was_not_set() {
    let mut the_program = AProgramThatWasMaybeAsked {
        what_it_said: one_offer_the_model_would_write(),
        was_it_asked: std::cell::RefCell::new(false),
    };
    let mut somewhere = AGitHubThatFiles {
        what_it_was_asked: Vec::new(),
        what_it_hands_back: String::new(),
    };
    let mut somewhere_else = AGitHubThatReadsBack {
        what_it_hands_back: String::new(),
    };

    let how_it_went = a_creative_run_files_what_it_would_file(
        &the_program,
        &mut somewhere,
        &mut somewhere_else,
        &the_repository_offered_in(),
        &the_listing(),
        &where_the_model_is(),
        WhetherAnAgentOffers::No,
    );

    let the_refusal = how_it_went.expect_err("the flag is off and this refused");
    assert!(
        the_refusal.0.contains("AISDLC_AN_AGENT_OFFERS"),
        "**because a refusal that does not name the variable is a refusal the operator cannot act \
         on** — and this one exists to tell a person their deployment asked for no such agent: \
         {the_refusal:?}"
    );
}

#[test]
fn a_deployment_that_asked_gets_a_work_for_the_filing_it_made() {
    let mut the_program = AProgramThatWasMaybeAsked {
        what_it_said: one_offer_the_model_would_write(),
        was_it_asked: std::cell::RefCell::new(false),
    };
    let mut somewhere = AGitHubThatFiles {
        what_it_was_asked: Vec::new(),
        what_it_hands_back: String::new(),
    };
    let mut somewhere_else = AGitHubThatReadsBack {
        what_it_hands_back: String::new(),
    };

    let what_the_run_will_work_on = a_creative_run_files_what_it_would_file(
        &the_program,
        &mut somewhere,
        &mut somewhere_else,
        &the_repository_offered_in(),
        &the_listing(),
        &where_the_model_is(),
        WhetherAnAgentOffers::Yes,
    )
    .expect("the flag is on, the model answered and the filing was made");

    assert_eq!(
        (
            what_the_run_will_work_on.len(),
            somewhere.what_it_was_asked.len(),
            *the_program.was_it_asked.borrow()
        ),
        (1, 1, true),
        "**because one offer is one filing and one filing is one work** — and the model was asked \
         once, which is the fact this test is really about: {what_the_run_will_work_on:?}"
    );
}

#[test]
fn the_work_answers_the_filing_read_back_and_not_what_the_run_wrote() {
    let mut the_program = AProgramThatWasMaybeAsked {
        what_it_said: one_offer_the_model_would_write(),
        was_it_asked: std::cell::RefCell::new(false),
    };
    let mut somewhere = AGitHubThatFiles {
        what_it_was_asked: Vec::new(),
        what_it_hands_back: String::new(),
    };
    let mut somewhere_else = AGitHubThatReadsBack {
        what_it_hands_back: String::new(),
    };

    let what_the_run_will_work_on = a_creative_run_files_what_it_would_file(
        &the_program,
        &mut somewhere,
        &mut somewhere_else,
        &the_repository_offered_in(),
        &the_listing(),
        &where_the_model_is(),
        WhetherAnAgentOffers::Yes,
    )
    .expect("the flag is on and the filing was made");

    let what_the_work_answers = what_the_run_will_work_on[0].what_it_answers();
    assert_eq!(
        (
            what_the_work_answers.what_was_asked(),
            what_the_work_answers
                .where_it_was_filed()
                .expect("a filing was filed somewhere")
                .as_str()
        ),
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
