//! The question a creative agent is asked carries the filings already on file.
//!
//! **An agent cannot avoid restating a filing it was never shown.** The creative run listed the
//! repository and said nothing about the tracker, **and five runs of it filed one SQLite observation
//! three times** — because on the evidence in front of it, every run was finding it for the first
//! time.
//!
//! **The reading happens before the question and not while answering it**, because a run that asked
//! first and read afterwards has already paid for an answer **that cannot have known**.

use aisdlc_domain::repository::Repository;
use aisdlc_run::a_filing_is_read::AFilingIsReadFrom;
use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::an_agent_is_asked_what_it_would_file::{
    an_agent_is_asked_what_it_would_file, the_question_an_agent_is_asked,
};
use aisdlc_run::an_issue_read::AnIssueRead;
use aisdlc_run::the_filings_already_on_file::{AFilingAlreadyOnFile, TheFilingsAlreadyOnFile};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// What a model answers when it concludes nothing is worth filing, **and that is a valid answer.**
const A_MODEL_CONCLUDING_NOTHING_IS_WORTH_FILING: &str = "{\"choices\":[{\"message\":{\"content\":\"{\\\"offers\\\":[],\\\"what_it_found\\\":\\\"a\\\",\\\"what_it_could_not_settle\\\":\\\"b\\\",\\\"what_it_expects_next\\\":\\\"c\\\"}\"}}]}";

fn the_repository_the_run_reads() -> Repository {
    Repository {
        owner: String::from("steamnoid"),
        name: String::from("ai-sdlc-app-rs-plus-selfaware"),
    }
}

/// Two filings on file, **and one about the very thing a third run filed three times.**
fn the_filings() -> TheFilingsAlreadyOnFile {
    let mut every_one = TheFilingsAlreadyOnFile::none_yet();
    every_one.and_this_one_was_too(AFilingAlreadyOnFile::that_is_open(
        12,
        "Gitignoring the E2E database artifacts is not the same as deleting them",
    ));
    every_one.and_this_one_was_too(AFilingAlreadyOnFile::that_is_open(
        14,
        "Stop shipping test code that no compiler reads",
    ));
    every_one
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

/// A tracker that remembers whether it was asked, **and holds two filings when it is.**
#[derive(Clone, Default)]
struct ATrackerWhoseFilingsAreRemembered {
    was_it_read: Rc<Cell<bool>>,
}

impl AFilingIsReadFrom for ATrackerWhoseFilingsAreRemembered {
    fn the_filing_at(
        &mut self,
        _where_it_answers: &aisdlc_domain::commissioned_request::AWhereItWasFiled,
    ) -> Result<AnIssueRead, String> {
        Err(String::from(
            "this run asks what is on file and reads no filing back, so asking is a bug and not a step",
        ))
    }

    fn the_filings_already_on_file_of(
        &mut self,
        _of_which_repository: &Repository,
    ) -> Result<TheFilingsAlreadyOnFile, String> {
        self.was_it_read.set(true);
        Ok(the_filings())
    }
}

/// A model that keeps the question it was handed, **which is the only way a test sees a question.**
struct AModelRememberingItsQuestion {
    what_it_was_asked: Rc<RefCell<String>>,
}

impl AProgramWeRun for AModelRememberingItsQuestion {
    fn run(&self, what_it_was_told_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        self.what_it_was_asked
            .replace(what_it_was_told_to_do.what_it_is_asked_about().to_string());
        WhatTheProgramSaid::of(true, A_MODEL_CONCLUDING_NOTHING_IS_WORTH_FILING)
    }
}

#[test]
fn the_question_names_every_filing_already_on_file() {
    let the_question = the_question_an_agent_is_asked(
        &the_repository_the_run_reads(),
        &the_listing(),
        &the_filings(),
    );

    assert!(
        the_question.contains("#12 Gitignoring the E2E database artifacts")
            && the_question.contains("#14 Stop shipping test code that no compiler reads"),
        "**because a model asked about a subset repeats the rest** — and the filings on file are the \
         only evidence that says an observation is not new: {the_question}"
    );
}

#[test]
fn a_repository_with_nothing_on_file_says_so_rather_than_showing_nothing() {
    let the_question = the_question_an_agent_is_asked(
        &the_repository_the_run_reads(),
        &the_listing(),
        &TheFilingsAlreadyOnFile::none_yet(),
    );

    assert!(
        the_question.contains("no issues are open"),
        "**because a question that shows an empty list reads as a repository with no issues**, and a \
         model told that files the fourth copy of an observation a person already filed: {the_question}"
    );
}

#[test]
fn a_question_naming_what_is_on_file_says_what_to_do_with_a_duplicate() {
    let the_question = the_question_an_agent_is_asked(
        &the_repository_the_run_reads(),
        &the_listing(),
        &the_filings(),
    );

    assert!(
        the_question.contains("it_restates"),
        "**because naming the filings is not enough — a model asked about existing work and given \
         nowhere to say an offer repeats one will file it**, and the field name is the whole of what \
         lets it: {the_question}"
    );
}

/// What one whole run did, **and the three facts no single assertion can see together.**
struct WhatTheWholeRunDid {
    was_the_tracker_read: bool,
    what_the_model_was_asked: String,
    why_the_run_refused: String,
}

/// One whole run, from a tracker holding two filings to whatever the model was asked.
///
/// **In one function because the three facts are only together in one place** — and a test that
/// read the tracker in one call and the question in another could pass on a run that never did both.
fn what_the_whole_run_did() -> WhatTheWholeRunDid {
    let what_the_model_was_asked = Rc::new(RefCell::new(String::new()));
    let was_the_tracker_read = Rc::new(Cell::new(false));
    let mut a_tracker = ATrackerWhoseFilingsAreRemembered {
        was_it_read: was_the_tracker_read.clone(),
    };
    let the_model = AModelRememberingItsQuestion {
        what_it_was_asked: what_the_model_was_asked.clone(),
    };
    let offered = an_agent_is_asked_what_it_would_file(
        &the_model,
        &mut a_tracker,
        &the_repository_the_run_reads(),
        &the_listing(),
        &where_the_model_is(),
    );
    WhatTheWholeRunDid {
        was_the_tracker_read: was_the_tracker_read.get(),
        what_the_model_was_asked: what_the_model_was_asked.borrow().clone(),
        why_the_run_refused: offered.err().map(|it| it.0).unwrap_or_default(),
    }
}

#[test]
fn the_model_is_asked_what_is_on_file_because_the_run_read_it_first() {
    let what_the_run_did = what_the_whole_run_did();

    assert!(
        what_the_run_did.was_the_tracker_read && what_the_run_did.why_the_run_refused.is_empty(),
        "**because the reading is the whole of this** — a question naming nothing read is a question \
         about a tracker this system never opened, and a hardcoded empty list would be this system \
         telling a model that thirty issues are not there. Tracker read: {}, refusal: {}",
        what_the_run_did.was_the_tracker_read,
        what_the_run_did.why_the_run_refused
    );
    assert!(
        what_the_run_did.what_the_model_was_asked.contains("#12"),
        "**because the model's question is the only place a filing on file can reach a model**, and \
         the run reading it proves nothing if the question drops it: {}",
        what_the_run_did.what_the_model_was_asked
    );
}
