//! A model asked how it rules on one exact artifact, and how its answer becomes a ruling.
//!
//! **The name in the record comes from our own configuration and never from the answer.** A model
//! that writes `decided_by: Alice` has written a sentence about a person, and taking it would put
//! a person's name on a decision no person made — **a forged audit trail looks exactly like a
//! real one at the moment somebody needs to tell them apart.**
//!
//! **The fixed form is `APPROVED` or `REFUSED` and nothing else.** A free-text ruling has no shape
//! to validate, and a contract a model can miss is not a contract; an answer in another shape is
//! a refusal rather than a guess at what was meant.

use aisdlc_domain::approval::{ApprovalDecision, WhoDecided};
use aisdlc_domain::work_item_id::WorkItemId;

use crate::ApprovalQuestion;
use crate::a_program_we_run::AProgramWeRun;
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// An answer that was not the fixed form, quoting what came back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAnswerThatIsNotARuling {
    what_it_said: String,
}

impl AnAnswerThatIsNotARuling {
    /// The refusal, with the answer quoted back.
    ///
    /// **Quoted and not summarised**, because a caller deciding whether to retry needs to know
    /// what the model actually said, and a paraphrase of it is one more layer to be wrong in.
    pub fn because(what_it_said: &str) -> Self {
        Self {
            what_it_said: what_it_said.to_string(),
        }
    }

    /// What the model said, exactly.
    pub fn what_it_said(&self) -> &str {
        &self.what_it_said
    }
}

/// The question put to a model, built here so a model cannot ask itself another one.
pub fn the_question_a_model_is_asked(
    a_question: &ApprovalQuestion,
    the_work_item: &WorkItemId,
) -> String {
    format!(
        "You are ruling at a human approval gate. Work item {the_work_item}, artifact {}, version {}. Answer with exactly one word: APPROVED or REFUSED. Do not name who decided; that is not yours to say.",
        a_question.about_which_artifact(),
        a_question.about_which_version()
    )
}

/// What a model said, read as a ruling, and refused when it is anything else.
///
/// **The whole answer and not its first word.** A model that answers `APPROVED, decided_by: Alice`
/// is talking, and a reader that took the first word would take the yes and drop the forgery in
/// the same step — **so the two are refused together, because they arrived together.**
fn the_ruling_in(what_it_said: &str) -> Result<bool, AnAnswerThatIsNotARuling> {
    match what_it_said.trim() {
        "APPROVED" => Ok(true),
        "REFUSED" => Ok(false),
        _ => Err(AnAnswerThatIsNotARuling::because(what_it_said)),
    }
}

/// The model is asked how it rules, and its answer becomes a decision naming the model.
pub fn the_model_is_asked_how_it_rules_on(
    the_model: &impl AProgramWeRun,
    a_question: &ApprovalQuestion,
    who_ruled: &WhoDecided,
) -> Result<ApprovalDecision, AnAnswerThatIsNotARuling> {
    let the_work_item = a_question.about_which_work_item();
    let what_it_said = the_model.run(&WhatWeTellAProgramToDo::ask_a_model_how_it_rules(
        the_question_a_model_is_asked(a_question, &the_work_item),
    ));
    let approved = the_ruling_in(what_it_said.the_words())?;
    ApprovalDecision::the_ruling_about(
        the_work_item,
        a_question.about_which_artifact(),
        a_question.about_which_version(),
        approved,
        who_ruled.clone(),
    )
    .map_err(|a_why| AnAnswerThatIsNotARuling::because(&a_why.0))
}
