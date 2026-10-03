//! The doubles a run is run with, **and they are in one file because three test files were carrying
//! three copies of the same model, the same person and the same program.**
//!
//! **A double copied into three files is three places to change when it grows a field**, and one of
//! those copies had already drifted: it answered a refusal with `did_it_work: true`, **which is a
//! delivery green on a branch `git` had already refused.** One copy is the seam; **the other two were
//! drift waiting to happen.**
//!
//! **The file is named for what it holds and not `common`**, because `common` is one of the words
//! that erase the domain: a reader sent here is sent for a reason, **and the reason is that a run is
//! run with three doubles and no others.** The naming guard refused `common` and it was right.

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhichArtifact, WhoDecided};
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// A model that answers everything, **and counts how often it was asked.**
pub struct AModelAnswering {
    how_often: RefCell<usize>,
}

impl AModelAnswering {
    /// A model that answers everything, **and a function because four tests build the same double
    /// and a struct literal in each of them is four places to change when it grows a field.**
    pub fn answering_everything() -> Self {
        Self {
            how_often: RefCell::new(0),
        }
    }
}

impl AReasoningModel for AModelAnswering {
    fn what_it_says_when_asked_for(
        &mut self,
        _which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        *self.how_often.borrow_mut() += 1;
        Ok(String::from("an artifact, of the length a real one is"))
    }
}

/// A person who approves everything up to one gate, **and refuses at that one and nowhere else.**
pub struct APersonWhoRules {
    refuses_at_the: Option<usize>,
    how_often: RefCell<usize>,
}

impl APersonWhoRules {
    /// A person who says yes at every gate, **and counts how often they were asked** — because a
    /// test that cannot count the questions cannot prove how many gates there were.
    pub fn approving_everything() -> Self {
        Self {
            refuses_at_the: None,
            how_often: RefCell::new(0),
        }
    }

    /// A person who says no at one gate and yes at every other, **and the gate is counted from one**
    /// so a reader can point at the third of seven without counting.
    pub fn refusing_at_the(nth: usize) -> Self {
        Self {
            refuses_at_the: Some(nth),
            how_often: RefCell::new(0),
        }
    }

    /// How many questions this person was asked, **and it is the whole of what two of the five tests
    /// are about** — a delivery that asked nobody and a run that asked eight both look identical from
    /// outside.
    pub fn how_often(&self) -> usize {
        *self.how_often.borrow()
    }
}

impl APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let nth = self.how_often.borrow().saturating_add(1);
        *self.how_often.borrow_mut() = nth;
        let approved = self.refuses_at_the != Some(nth);
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            approved,
            WhoDecided::try_from("paligakrzychu").expect("a name is not blank"),
            "the artifact answers the request",
        )
        .expect("a person may rule")
    }
}

/// A program that answers `git` and `gh`, **and refuses the pull request when told to.**
pub struct AProgramAnswering {
    refuses_the_pull_request: bool,
}

impl AProgramWeRun for AProgramAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        let (did_it_work, what_it_says) = self.what_it_says_for(what_it_is_asked_to_do);
        WhatTheProgramSaid::of(did_it_work, &what_it_says)
    }
}

impl AProgramAnswering {
    /// A program that answers everything, **and one that can be told to refuse the pull request** —
    /// because a delivery that cannot fail is a delivery no test can prove anything about.
    pub fn refusing_the_pull_request(refuses_the_pull_request: bool) -> Self {
        Self {
            refuses_the_pull_request,
        }
    }

    fn what_it_says_for(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> (bool, String) {
        match what_it_is_asked_to_do {
            WhatWeTellAProgramToDo::ReadTheRemote { .. } => (true, String::new()),
            WhatWeTellAProgramToDo::PushTheBranch { .. } => {
                (true, String::from("to ../remote.git"))
            }
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } if self.refuses_the_pull_request => {
                (false, String::from("gh: no"))
            }
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } => (
                true,
                String::from(
                    r#"{"number":17,"html_url":"https://github.com/steamnoid/wellbeing-tracker-public/pull/17"}"#,
                ),
            ),
            every_other_request => (true, format!("{every_other_request:?}")),
        }
    }
}
