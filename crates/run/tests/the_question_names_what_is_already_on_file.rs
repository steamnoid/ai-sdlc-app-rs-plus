//! The question a creative agent is asked carries the filings already on file.
//!
//! **An agent cannot avoid restating a filing it was never shown.** The creative run listed the
//! repository and said nothing about the tracker, **and five real runs of it filed one SQLite
//! observation three times** — because on the evidence in front of it, every run was finding it for
//! the first time.
//!
//! **The reading happens before the question and not while answering it**, because a run that asked
//! first and read afterwards has already paid for an answer **that cannot have known**.

use aisdlc_domain::repository::Repository;
use aisdlc_run::an_agent_is_asked_what_it_would_file::the_question_an_agent_is_asked;
use aisdlc_run::the_filings_already_on_file::{AFilingAlreadyOnFile, TheFilingsAlreadyOnFile};

fn the_repository() -> Repository {
    Repository {
        owner: String::from("steamnoid"),
        name: String::from("wellbeing-tracker-public"),
    }
}

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

#[test]
fn the_question_names_every_filing_already_on_file() {
    let the_question = the_question_an_agent_is_asked(
        &the_repository(),
        &["src/db.ts".to_string()],
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
        &the_repository(),
        &["src/db.ts".to_string()],
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
        &the_repository(),
        &["src/db.ts".to_string()],
        &the_filings(),
    );

    assert!(
        the_question.contains("it_restates"),
        "**because naming the filings is not enough — a model asked about existing work and given \
         nowhere to say an offer repeats one will file it**, and the field name is the whole of what \
         lets it: {the_question}"
    );
}

#[test]
fn the_model_is_asked_what_is_on_file_because_the_run_read_it_first() {
    use aisdlc_run::a_filing_is_read::AFilingIsReadFrom;
    use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
    use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
    use aisdlc_run::an_issue_read::AnIssueRead;
    use aisdlc_run::an_agent_is_asked_what_it_would_file::an_agent_is_asked_what_it_would_file;
        use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
    use aisdlc_run::what_an_agent_offered::WhatAnAgentOffered;
    use std::cell::Cell;
    use std::rc::Rc;

    /// What a tracker was asked, **and the one fact this test needs: that it was asked at all.**
    #[derive(Clone, Default)]
    struct ATrackerWhoseFilingsAreRemembered {
        was_it_read: Rc<Cell<bool>>,
    }

    impl AFilingIsReadFrom for ATrackerWhoseFilingsAreRemembered {
        fn the_filing_at(
            &mut self,
            _where_it_answers: &aisdlc_domain::commissioned_request::AWhereItWasFiled,
        ) -> Result<AnIssueRead, String> {
            unreachable!("this run asks what is on file and reads none of it back")
        }

        fn the_filings_already_on_file_of(
            &mut self,
            _of_which_repository: &Repository,
        ) -> Result<TheFilingsAlreadyOnFile, String> {
            self.was_it_read.set(true);
            Ok(the_filings())
        }
    }

    /// A model that remembers the body it was handed, **which is the only way a test can see a
    /// question.**
    struct AModelRememberingItsQuestion {
        what_it_was_asked: Rc<Cell<String>>,
    }

    impl AProgramWeRun for AModelRememberingItsQuestion {
        fn run(&self, what_it_was_told_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
            self.what_it_was_asked.set(what_it_was_told_to_do.the_body_it_carries().to_string());
            WhatTheProgramSaid::the_words(
                "{\"offers\":[],\"what_it_found\":\"a\",\"what_it_could_not_settle\":\"b\",\"what_it_expects_next\":\"c\"}",
            )
        }
    }

    let what_the_model_was_asked = Rc::new(Cell::new(String::new()));
    let was_the_tracker_read = Rc::new(Cell::new(false));
    let mut a_tracker = ATrackerWhoseFilingsAreRemembered {
        was_it_read: was_the_tracker_read.clone(),
    };
    let the_model = AModelRememberingItsQuestion {
        what_it_was_asked: what_the_model_was_asked.clone(),
    };
    let where_the_model_is = WhereAModelIs {
        the_endpoint: String::from("http://localhost:1/v1/chat/completions"),
        which_model: String::from("a-model"),
    };

    let offered = an_agent_is_asked_what_it_would_file(
        &the_model,
        &mut a_tracker,
        &the_repository(),
        &["src/db.ts".to_string()],
        &where_the_model_is,
    );

    assert!(
        offered.is_ok() && was_the_tracker_read.get(),
        "**because the reading is the whole of this and a question naming nothing read is a question \\
         about a tracker this system never opened** — the third argument is a hardcoded `none_yet()` \\
         today, which is this system telling a model that thirty issues are not there"
    );
    assert!(
        what_the_model_was_asked.get().contains("#12"),
        "**because the model's question is the only place a filing on file can reach a model**, and \
         the run reading it proves nothing if the question drops it: {}",
        what_the_model_was_asked.get()
    );
}
