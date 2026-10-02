//! A model asked for the artifact a work item needs, and the answer before anything checked it.
//!
//! **The port, the answer, and the two refusals live together because they are one question.** A
//! caller holds an artifact it asked for, words a model said, and a reason there is neither — and
//! splitting them across two files would put the two refusals somewhere a reader looking for
//! either would not think to look.

use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::work_item::WorkItem;

use crate::the_project_on_disk::TheProjectOnDisk;
use crate::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;

/// A model that answers with a validated artifact.
///
/// **A `Result` and not a `String`, because "the model did not answer" is a fact about the world
/// and not about the question.** A port returning `String` has nowhere to put a failure, so every
/// unreachable model arrives as its own refusal and each caller reads it separately — **and the
/// two callers of this one are exactly the two that would have read it differently.**
pub trait AReasoningModel {
    /// What the model says when asked for one artifact, **or the answer that could not be read.**
    ///
    /// **The words and not the envelope, and that is the point of the type.** A model asked to
    /// say one word answers with JSON around that word, and a caller handed the whole answer
    /// decides no model can ever rule.
    fn what_it_says_when_asked_for(
        &mut self,
        which_artifact: &WhichArtifact,
        which_organisation: WhichOrganisationReasons,
        the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead>;
}

/// What a model said when it was asked for an artifact, before anything checked it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAnswerIsNotYetAnArtifact {
    which_artifact: WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
    the_question: String,
    what_the_model_said: String,
}

impl AnAnswerIsNotYetAnArtifact {
    /// Which artifact the model was asked for.
    pub fn which_artifact_was_asked_for(&self) -> WhichArtifact {
        self.which_artifact.clone()
    }

    /// The same answer, with what the project holds written into the question.
    ///
    /// **A copy and not a mutation**, because the answer came back from a call already spent and
    /// a model cannot be asked the same question twice for one listing.
    #[must_use]
    pub fn with_the_project_named_in_the_question(
        &self,
        what_is_in_it: &[String],
    ) -> AnAnswerIsNotYetAnArtifact {
        let mut the_answer = self.clone();
        the_answer.the_question = format!(
            "{}\n\nthe project holds:\n{}",
            self.the_question,
            what_is_in_it.join("\n")
        );
        the_answer
    }

    /// Which organisation's model answered.
    pub fn who_reasoned(&self) -> WhichOrganisationReasons {
        self.who_reasoned
    }

    /// The question the model was given.
    pub fn the_question_it_was_given(&self) -> String {
        self.the_question.clone()
    }

    /// The words the model said, unaltered.
    pub fn what_the_model_said(&self) -> String {
        self.what_the_model_said.clone()
    }
}

/// An agent asks a model for the artifact its work item needs.
///
/// **The question is the request somebody typed, and not a summary of it.** A model asked a
/// question nobody asked is a model answering a question of its own, and the request on a work
/// item is the only record that this piece of work is this person's work.
///
/// **The return type is an answer and not an artifact**, so a caller cannot get hold of
/// something with a name that could be mistaken for a plan until something has checked one. A
/// work item with no request cannot be asked about, so it is refused here rather than sent to a
/// model with nothing to answer.
///
/// **A model that could not be reached is the same refusal as a work item with no request, and
/// both name the artifact.** The caller asked one question and got no answer either way, and a
/// caller that had to tell them apart would have to know which half of the world went wrong.
pub fn the_agent_asks_for_its_artifact(
    the_model: &mut impl AReasoningModel,
    the_work_item: &WorkItem,
    which_artifact: &WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
) -> Result<AnAnswerIsNotYetAnArtifact, AnUnanswerableRequest> {
    let the_request = the_work_item.the_request_it_answers();
    let Some(requested) = the_request else {
        return Err(AnUnanswerableRequest::because(
            which_artifact,
            "the work item answers no request",
        ));
    };
    let what_the_model_said =
        the_model.what_it_says_when_asked_for(which_artifact, who_reasoned, &requested.title);
    let what_the_model_said = what_the_model_said
        .map_err(|nobody_can_read| the_model_did_not_answer(which_artifact, &nobody_can_read))?;
    Ok(AnAnswerIsNotYetAnArtifact {
        which_artifact: which_artifact.clone(),
        who_reasoned,
        the_question: requested.title,
        what_the_model_said,
    })
}

/// A model that could not be read, as the refusal this use case hands its caller.
///
/// **Quoted and not summarised, and the artifact is named beside it.** A run asks eight agents
/// for eight different artifacts, so "the model did not answer" without saying which one is a
/// question rather than a diagnosis — and the words that came back are the only clue a caller
/// has for deciding whether to retry.
fn the_model_did_not_answer(
    which_artifact: &WhichArtifact,
    nobody_can_read: &AnAnswerNobodyCanRead,
) -> AnUnanswerableRequest {
    AnUnanswerableRequest::because(
        which_artifact,
        &format!(
            "the model did not answer, and what came back was: {}",
            nobody_can_read.what_it_said()
        ),
    )
}

/// A work item whose request a model cannot be asked about, named with the artifact and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnUnanswerableRequest {
    which_artifact: WhichArtifact,
    why_not: String,
}

impl AnUnanswerableRequest {
    /// The artifact the model was about to be asked for.
    pub fn which_artifact_was_asked_for(&self) -> WhichArtifact {
        self.which_artifact.clone()
    }

    /// Why no question could be built.
    pub fn why_it_could_not_be_asked(&self) -> String {
        self.why_not.clone()
    }

    /// This refusal, for a caller that has the artifact and the reason.
    pub fn because(which_artifact: &WhichArtifact, why_not: &str) -> Self {
        Self {
            which_artifact: which_artifact.clone(),
            why_not: why_not.to_string(),
        }
    }
}

/// The agent asks a model about the project, and the question carries what the project holds.
///
/// **The question is the request and the listing**, because a model asked a question with no
/// repository in it is answering about a repository it has not seen. **The listing is read once
/// and the read has to be used**, or the port is opened, the answer is read, and it is thrown
/// away — which is the shape a call site takes when the thing it reads is not yet needed.
///
/// **A project holding nothing is refused before the call and not after.** A model asked about
/// a repository with nothing in it is answering about nothing, and a refusal that has already
/// spent a call has paid for it.
pub fn the_agent_asks_a_model_about_the_project(
    the_model: &mut impl AReasoningModel,
    the_project: &mut impl TheProjectOnDisk,
    the_work_item: &WorkItem,
    which_artifact: &WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
) -> Result<AnAnswerIsNotYetAnArtifact, AnUnanswerableRequest> {
    let what_is_in_it = the_project.what_is_in_it();
    if what_is_in_it.is_empty() {
        return Err(AnUnanswerableRequest::because(
            which_artifact,
            "the project holds nothing, and a model asked about a repository with nothing in it is answering about nothing",
        ));
    }
    the_agent_asks_a_question_naming(
        the_model,
        the_work_item,
        which_artifact,
        who_reasoned,
        &what_is_in_it,
    )
}

/// The same call, with what the project holds written into the question.
fn the_agent_asks_a_question_naming(
    the_model: &mut impl AReasoningModel,
    the_work_item: &WorkItem,
    which_artifact: &WhichArtifact,
    who_reasoned: WhichOrganisationReasons,
    what_is_in_it: &[String],
) -> Result<AnAnswerIsNotYetAnArtifact, AnUnanswerableRequest> {
    let the_answer =
        the_agent_asks_for_its_artifact(the_model, the_work_item, which_artifact, who_reasoned)?;
    Ok(the_answer.with_the_project_named_in_the_question(what_is_in_it))
}
