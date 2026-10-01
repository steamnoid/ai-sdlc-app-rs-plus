//! An offer that says it restates a filing never reaches GitHub.
//!
//! **The rule exists and nothing calls it.** #150 built the judgement and #151 put the filings in the
//! question, **and a run that files a restated offer anyway has both of them and does nothing with
//! either.**
//!
//! **The restated offer is dropped and the run goes on**, **because a model weighing four filings
//! and finding two of them already answered is a model doing its job** — and a run that refused the
//! whole answer over one offer would throw away three good ones.

use aisdlc_domain::repository::Repository;
use aisdlc_run::a_creative_agent_files_what_it_concluded::a_creative_agent_files_what_it_concluded;
use aisdlc_run::a_git_hub_we_write_to::{
    AGitHubWeWriteTo, WhatGitHubWasAskedFor, WhereGitHubAnswered,
};
use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::the_filings_already_on_file::{AFilingAlreadyOnFile, TheFilingsAlreadyOnFile};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

fn the_repository() -> Repository {
    Repository {
        owner: String::from("steamnoid"),
        name: String::from("ai-sdlc-app-rs-plus-selfaware"),
    }
}

fn the_listing() -> Vec<String> {
    vec![String::from("crates/run/src/lib.rs")]
}

fn where_the_model_is() -> WhereAModelIs {
    WhereAModelIs {
        the_endpoint: String::from("http://localhost:1/v1/chat/completions"),
        which_model: String::from("a-model"),
    }
}

/// What a model said, **with one offer that restates filing 12 and one that does not.**
fn a_model_offering_a_restatement_and_something_new() -> String {
    let its_content = r#"{"offers":[
        {"title":"the E2E database leaves artifacts behind","what_it_asks_for":"clean it up","paths":["crates/run/src/lib.rs"],"it_restates":12},
        {"title":"the router has no test for a refused artifact","what_it_asks_for":"write one","paths":["crates/run/src/lib.rs"],"it_restates":null}
    ],"what_it_found":"two things","what_it_could_not_settle":"one","what_it_expects_next":"a third"}"#;
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":{}}}}}]}}",
        serde_json::to_string(its_content).expect("a string is JSON")
    )
}

/// What a model said when every offer restates something, **and there is nothing left to file.**
fn a_model_offering_only_restatements() -> String {
    let its_content = r#"{"offers":[
        {"title":"the E2E database leaves artifacts behind","what_it_asks_for":"clean it up","paths":["crates/run/src/lib.rs"],"it_restates":12}
    ],"what_it_found":"one thing","what_it_could_not_settle":"one","what_it_expects_next":"nothing"}"#;
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":{}}}}}]}}",
        serde_json::to_string(its_content).expect("a string is JSON")
    )
}

/// A program that answers with whatever it was built holding.
struct AProgramThatAnswers(String);

impl AProgramWeRun for AProgramThatAnswers {
    fn run(&self, _what_it_was_told_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        WhatTheProgramSaid::of(true, &self.0)
    }
}

/// GitHub, counting what it was asked to create, **and nothing else.**
#[derive(Default)]
struct AGitHubCountingWhatItWasAskedFor {
    how_many_were_asked_for: usize,
}

impl AGitHubWeWriteTo for AGitHubCountingWhatItWasAskedFor {
    fn it_is_asked_to(
        &mut self,
        _what_it_is_asked_for: &WhatGitHubWasAskedFor,
    ) -> Result<WhereGitHubAnswered, String> {
        self.how_many_were_asked_for += 1;
        Ok(WhereGitHubAnswered::an_address_at(
            "https://github.com/steamnoid/ai-sdlc-app-rs-plus-selfaware/issues/99",
        ))
    }
}

/// A tracker holding the two filings, **and none of them read back.**
struct ATrackerHoldingTwoFilings;

impl aisdlc_run::a_filing_is_read::AFilingIsReadFrom for ATrackerHoldingTwoFilings {
    fn the_filing_at(
        &mut self,
        _where_it_answers: &aisdlc_domain::commissioned_request::AWhereItWasFiled,
    ) -> Result<aisdlc_run::an_issue_read::AnIssueRead, String> {
        Err(String::from("this test does not read a filing back"))
    }

    fn the_filings_already_on_file_of(
        &mut self,
        _of_which_repository: &Repository,
    ) -> Result<TheFilingsAlreadyOnFile, String> {
        let mut every_one = TheFilingsAlreadyOnFile::none_yet();
        every_one.and_this_one_was_too(AFilingAlreadyOnFile::that_is_open(
            12,
            "Gitignoring the E2E database artifacts is not the same as deleting them",
        ));
        every_one.and_this_one_was_too(AFilingAlreadyOnFile::that_is_open(
            14,
            "Stop shipping test code that no compiler reads",
        ));
        Ok(every_one)
    }
}

#[test]
fn an_offer_restating_a_filing_is_not_filed_and_the_new_one_beside_it_is() {
    let mut github = AGitHubCountingWhatItWasAskedFor::default();
    let mut tracker = ATrackerHoldingTwoFilings;

    let what_the_run_filed = a_creative_agent_files_what_it_concluded(
        &AProgramThatAnswers(a_model_offering_a_restatement_and_something_new()),
        &mut github,
        &mut tracker,
        &the_repository(),
        &the_listing(),
        &where_the_model_is(),
    );

    assert_eq!(
        github.how_many_were_asked_for, 1,
        "**because one offer of the two repeats filing 12 and the other does not** — and a run that \\
         filed both would put the fourth copy of the SQLite observation on the tracker, which is the \\
         thing #150 exists to stop. The run returned: {what_the_run_filed:?}"
    );
}

#[test]
fn a_model_restating_everything_files_nothing_and_says_so_rather_than_failing() {
    let mut github = AGitHubCountingWhatItWasAskedFor::default();
    let mut tracker = ATrackerHoldingTwoFilings;

    let what_the_run_filed = a_creative_agent_files_what_it_concluded(
        &AProgramThatAnswers(a_model_offering_only_restatements()),
        &mut github,
        &mut tracker,
        &the_repository(),
        &the_listing(),
        &where_the_model_is(),
    );

    let was_there_nothing_to_file = what_the_run_filed.map(|it| it.is_empty()).unwrap_or(false);
    assert!(
        github.how_many_were_asked_for == 0 && was_there_nothing_to_file,
        "**because a model that weighed two filings and restedated one has done the job correctly** — \
         and a run that reported an error here would tell its operator something is broken when the \
         right answer is that there was nothing new. GitHub was asked for {}.",
        github.how_many_were_asked_for
    );
}

#[test]
fn an_offer_naming_a_filing_that_is_not_on_file_is_refused_rather_than_filed() {
    let mut github = AGitHubCountingWhatItWasAskedFor::default();
    let mut tracker = ATrackerHoldingTwoFilings;
    let mut a_model_claiming_a_filing_nobody_has = AProgramThatAnswers(
        a_model_offering_a_restatement_and_something_new()
            .replace("\\\"it_restates\\\":12", "\\\"it_restates\\\":4242"),
    );

    let what_the_run_filed = a_creative_agent_files_what_it_concluded(
        &mut a_model_claiming_a_filing_nobody_has,
        &mut github,
        &mut tracker,
        &the_repository(),
        &the_listing(),
        &where_the_model_is(),
    );

    assert!(
        github.how_many_were_asked_for == 0,
        "**because a model naming a filing that is not on file is reporting one it never read** — and \\
         filing it would put a number nobody can open into a filing. GitHub was asked for {}.",
        github.how_many_were_asked_for
    );
    assert!(
        what_the_run_filed.is_err(),
        "**because a run that silently dropped it would leave a reader believing nothing was found** — \
         and the operator would never learn the model invented a filing"
    );
}
