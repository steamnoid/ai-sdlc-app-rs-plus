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
use crate::the_words_inside_the_endpoints_envelope::the_words_inside_the_endpoints_envelope;
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
/// The ruling a model gave, out of the answer its endpoint wrapped it in.
///
/// **The envelope is the endpoint's and the ruling is not.** A model asked to say one word answers
/// with JSON around that word, and a reader that wants the whole answer to *be* the word decides
/// no model can ever rule.
///
/// **Every way of failing here is a refusal and never an approval**, because that is the direction
/// the failure has to point: a broken answer read as an approval puts a model at a human's gate on
/// the strength of a bug. The one word is still checked afterwards, so an envelope cannot approve
/// anything on its own.
pub fn the_ruling_a_model_gave(
    what_the_endpoint_said: &str,
) -> Result<WhatAModelRuled, AnAnswerThatIsNotARuling> {
    let the_content = the_words_inside_the_endpoints_envelope(what_the_endpoint_said).map_err(
        |nobody_can_read| AnAnswerThatIsNotARuling::because(nobody_can_read.what_it_said()),
    )?;
    the_ruling_in(&the_content)
}

/// What a model ruled and why, **and the two are separate because the verdict is one word and the
/// reason is a sentence.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAModelRuled {
    /// Whether it said yes.
    pub approved: bool,
    /// Why, **in the model's own words and never this system's paraphrase of them.**
    pub reason: String,
}

/// The one word on the first line, **and the reason on everything after it.**
///
/// **The verdict stays one word on a line of its own, and that is the whole of why a sentence on
/// the same line is still refused.** A reader that accepted the first word it found would be
/// accepting a model that opens a gate and then explains itself afterwards — **so the reason is a
/// second line, where it cannot be confused with the decision, and where its absence is visible.**
fn the_ruling_in(what_it_said: &str) -> Result<WhatAModelRuled, AnAnswerThatIsNotARuling> {
    let the_two_lines = what_it_said.trim().split_once('\n');
    let Some((the_verdict, the_reason)) = the_two_lines else {
        return Err(AnAnswerThatIsNotARuling::because(what_it_said));
    };
    let what_the_verdict_says = what_the_model_ruled(the_verdict)?;
    let the_reason = the_reason.trim();
    if the_reason.is_empty() {
        return Err(AnAnswerThatIsNotARuling::because(what_it_said));
    }
    Ok(WhatAModelRuled {
        approved: what_the_verdict_says,
        reason: the_reason.to_string(),
    })
}

/// The one word, **and nothing else on that line.**
fn what_the_model_ruled(the_verdict: &str) -> Result<bool, AnAnswerThatIsNotARuling> {
    match the_verdict.trim() {
        "APPROVED" => Ok(true),
        "REFUSED" => Ok(false),
        _ => Err(AnAnswerThatIsNotARuling::because(the_verdict)),
    }
}

/// Where a model is and which one, both deployment facts and neither a secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereAModelIs {
    /// The address the model answers at.
    pub the_endpoint: String,
    /// Which model answers there.
    pub which_model: String,
}

impl WhereAModelIs {
    /// Where a deployment put its model, or the variable it left unset.
    ///
    /// **A refusal and never a number this system chose.** The first real run fell back to
    /// `http://localhost:11434` because nobody had read `LLM_ENDPOINT`, and the message that came
    /// back was *Failed to connect to localhost port 11434* — a guess that costs the same time as a
    /// refusal and says less. **A fallback is a lie with a position**, and this is the place that
    /// used to have one.
    pub fn where_a_deployment_put_its_model(
        the_endpoint: Option<&str>,
        which_model: Option<&str>,
    ) -> Result<Self, String> {
        let Some(the_endpoint) = the_endpoint else {
            return Err(String::from(
                "LLM_ENDPOINT is not set, and this system will not choose an address for a model",
            ));
        };
        if !the_endpoint.ends_with("/chat/completions") {
            return Err(format!(
                "LLM_ENDPOINT is '{the_endpoint}', which is a base and not the address a model \
                 answers at, and a base this system completes is a second fact this system chose"
            ));
        }
        let Some(which_model) = which_model else {
            return Err(String::from(
                "LLM_MODEL is not set, and a model this system invented is one nobody deployed",
            ));
        };
        Ok(WhereAModelIs {
            the_endpoint: the_endpoint.to_string(),
            which_model: which_model.to_string(),
        })
    }
}

/// The body a model is given, built by the JSON library and not by a hand.
///
/// **The shape and not a word about who decided.** A model told to say one word and asked to say
/// whose decision it is has been asked two things, and the second one is a forgery waiting to be
/// refused — **so the body forbids it rather than the parser catching it.**
///
/// **The library and not a format string.** This body was once written by hand, with the question
/// put where a quoted string belongs and no quotes around it — **so every request this system ever
/// sent a model was invalid JSON**, the far end answered `400` with zero bytes, and the refusal that
/// reached us quoted an empty string. **A hand-rolled escape is a lossy channel in both directions**,
/// and the question is a work item's name and an artifact's name, which are strings people type.
pub fn the_body_a_model_is_given(the_question: &str, where_the_model_is: &WhereAModelIs) -> String {
    let the_asked = serde_json::json!({
        "model": where_the_model_is.which_model,
        "temperature": 0,
        "messages": [{"role": "user", "content": the_question}],
    });
    the_asked.to_string()
}

/// The model is asked how it rules, and its answer becomes a decision naming the model.
pub fn the_model_is_asked_how_it_rules_on(
    the_model: &impl AProgramWeRun,
    a_question: &ApprovalQuestion,
    who_ruled: &WhoDecided,
    where_the_model_is: &WhereAModelIs,
) -> Result<ApprovalDecision, AnAnswerThatIsNotARuling> {
    let the_work_item = a_question.about_which_work_item();
    let the_body = the_body_a_model_is_given(
        &the_question_a_model_is_asked(a_question, &the_work_item),
        where_the_model_is,
    );
    let what_it_said = the_model.run(&WhatWeTellAProgramToDo::ask_a_model(
        &where_the_model_is.the_endpoint,
        &where_the_model_is.which_model,
        &the_body,
    ));
    let the_ruling = the_ruling_a_model_gave(what_it_said.the_words())?;
    ApprovalDecision::the_ruling_about(
        the_work_item,
        a_question.about_which_artifact(),
        a_question.about_which_version(),
        the_ruling.approved,
        who_ruled.clone(),
        &the_ruling.reason,
    )
    .map_err(|a_why| AnAnswerThatIsNotARuling::because(&a_why.0))
}

/// A gate nobody switched a model on for, naming the flag that is off.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AMayNotRule {
    the_question: String,
}

/// The gate, asking a model only while the flag says it may.
///
/// **The flag is read before the model, and that order is the rule.** A gate that reaches the
/// model and then ignores the flag has already paid for a ruling nobody authorised — **and the
/// refusal is the honest answer until a person turns the flag**, rather than a decision this
/// system made on a human's behalf.
pub fn a_model_may_not_rule_without_a_flag(
    the_model: &impl AProgramWeRun,
    a_question: &ApprovalQuestion,
    who_ruled: &WhoDecided,
    where_the_model_is: &WhereAModelIs,
    may_it_rule: &crate::whether_a_model_rules_here::WhetherAModelRulesHere,
) -> Result<ApprovalDecision, AnAnswerThatIsNotARuling> {
    if !may_it_rule.may_be_asked() {
        return Err(the_refusal_for(a_question));
    }
    the_model_is_asked_how_it_rules_on(the_model, a_question, who_ruled, where_the_model_is)
}

/// The refusal for a gate the flag is off, built without nesting the call inside the return.
fn the_refusal_for(a_question: &ApprovalQuestion) -> AnAnswerThatIsNotARuling {
    let the_artifact = a_question.about_which_artifact().to_string();
    AnAnswerThatIsNotARuling::because(
        &AMayNotRule {
            the_question: the_artifact,
        }
        .why_it_may_not_rule(),
    )
}

impl AMayNotRule {
    /// Why the model was not asked, naming the artifact and the flag.
    ///
    /// **The flag and the artifact, and not a bare "no"** — a caller refusing a decision needs to
    /// know which gate refused and that the reason was a switch, so a human turning the flag on
    /// knows it was the thing that was missing.
    pub fn why_it_may_not_rule(&self) -> String {
        format!(
            "a model may not rule on {} here: the flag that lets one is off, and a gate answered by a model nobody switched on is a gate that ruled without anybody",
            self.the_question
        )
    }
}
