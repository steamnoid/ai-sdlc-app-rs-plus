//! A person who rules at every gate, and refuses at at most one.
//!
//! **One double per file, and not one file of doubles**, because `mod` compiles a copy into every
//! test binary that names it — **so a file holding four doubles is compiled four times and the three
//! a given binary does not use are reported dead**, and the only ways to stop that are an
//! `#[allow(dead_code)]`, which a gate in this repository refuses, or a copy.

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhoDecided};
use aisdlc_run::{APersonToDecide, ApprovalQuestion};
/// A person who approves everything up to one gate, **and refuses at that one and nowhere else.**
pub struct APersonWhoRules {
    refuses_at_the: Option<usize>,
    how_often: RefCell<usize>,
    what_each_gate_was_asked_about: RefCell<Vec<(String, u32)>>,
}

impl APersonWhoRules {
    /// A person who says yes at every gate, **and counts how often they were asked** — because a
    /// test that cannot count the questions cannot prove how many gates there were.
    pub fn approving_everything() -> Self {
        Self {
            refuses_at_the: None,
            how_often: RefCell::new(0),
            what_each_gate_was_asked_about: RefCell::new(Vec::new()),
        }
    }

    /// A person who says no at one gate and yes at every other, **and the gate is counted from one**
    /// so a reader can point at the third of seven without counting.
    pub fn refusing_at_the(nth: usize) -> Self {
        Self {
            refuses_at_the: Some(nth),
            how_often: RefCell::new(0),
            what_each_gate_was_asked_about: RefCell::new(Vec::new()),
        }
    }

    /// How many questions this person was asked, **and it is the whole of what two of the five tests
    /// are about** — a delivery that asked nobody and a run that asked eight both look identical from
    /// outside.
    pub fn how_often(&self) -> usize {
        *self.how_often.borrow()
    }

    /// What each gate was asked about **and at which version**, in the order the gates were reached —
    /// and a count alone cannot say which gate was the last one, so a run that asked eight questions
    /// about seven artifacts reads the same as a run that asked seven.
    pub fn what_each_gate_was_asked_about(&self) -> Vec<(String, u32)> {
        self.what_each_gate_was_asked_about.borrow().clone()
    }
}

impl APersonToDecide for APersonWhoRules {
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let nth = self.how_often.borrow().saturating_add(1);
        *self.how_often.borrow_mut() = nth;
        let the_artifact = a_question.about_which_artifact().0.clone();
        let the_version = a_question.about_which_version().0;
        self.what_each_gate_was_asked_about
            .borrow_mut()
            .push((the_artifact, the_version));
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
