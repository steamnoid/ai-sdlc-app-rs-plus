//! A policy answers the gate, and it reads the artifact off disk to do it.
//!
//! **A policy that can only approve is a stamp.** `approved` has been a boolean since before
//! anything could answer, and refusing leaves no `Approval` at all — **so refusing is the half that
//! stops work and approving is the half that hands it on.** A policy with no way to refuse is a
//! rubber stamp with a model's name on it.
//!
//! **It is handed the artifact and not a description of it**, because a policy ruling on a summary
//! written for it **is not ruling on the artifact, it is ruling on somebody's description of it** —
//! and the description is a lossy thing produced by the very run being judged.
//!
//! **There is no special path.** The policy puts the same `ApprovalQuestion` a person is asked and
//! produces the same `ApprovalDecision`, and it asks for its ruling **through the same door** — so a
//! policy gets no shortcut past the version check and none past the agent check, **because the gate
//! cannot tell which side of `APersonToDecide` answered.**

use std::cell::RefCell;

use aisdlc_domain::approval::{ApprovalDecision, WhoDecided};

use crate::a_model_is_asked_how_it_rules::{
    AnAnswerThatIsNotARuling, WhereAModelIs, the_model_is_asked_how_it_rules_on,
};
use crate::a_program_we_run::AProgramWeRun;
use crate::who_answers_the_gate::{WhoAnswersTheGate, the_name_the_decision_is_recorded_under};
use crate::{APersonToDecide, ApprovalQuestion};

/// A policy that answers this run's gates, **and everything it needs and nothing more.**
pub struct APolicyThatAnswersTheGate<'a> {
    which_policy_it_is: WhoAnswersTheGate,
    where_the_model_is: WhereAModelIs,
    the_endpoint: &'a dyn AProgramWeRun,
    how_the_artifact_is_read: Box<dyn Fn() -> String>,
    /// Every question this policy put to a model, **and that is a first-class fact of a run rather
    /// than a test's convenience: "what did the machine actually see" is what a person needs when a
    /// policy approved something, and a record holding only the verdict could not answer it.**
    pub what_it_was_handed: RefCell<Vec<String>>,
}

impl<'a> APolicyThatAnswersTheGate<'a> {
    /// The policy under one name, **reading the artifact from disk and asking a real endpoint.**
    ///
    /// **The artifact is a closure and not an argument to `how_they_rule_on`,** because the file a
    /// person would open is a fact about this policy's deployment rather than about the question —
    /// **and a reader looking at the trait should not have to know it.**
    pub fn the_policy(
        the_endpoint: &'a dyn AProgramWeRun,
        where_the_model_is: WhereAModelIs,
        how_the_artifact_is_read: impl Fn() -> String + 'static,
        which_policy_it_is: WhoAnswersTheGate,
    ) -> Self {
        Self {
            which_policy_it_is,
            where_the_model_is,
            the_endpoint,
            how_the_artifact_is_read: Box::new(how_the_artifact_is_read),
            what_it_was_handed: RefCell::new(Vec::new()),
        }
    }

    /// The question this policy puts, **and it names the artifact and its version beside the
    /// artifact's own words.**
    ///
    /// **The version is in the question and not only in the record**, because a model ruling on
    /// version one while the record says version two is a decision about the wrong thing, **and the
    /// only place that can be caught is where the question is built.**
    fn the_question_it_puts(&self, a_question: &ApprovalQuestion) -> String {
        let the_artifact = (self.how_the_artifact_is_read)();
        format!(
            "About {} at version {}. The artifact as it stands:\n\n{the_artifact}",
            a_question.about_which_artifact(),
            a_question.about_which_version(),
        )
    }

    /// The ruling the model gave, **or the refusal that stands in for one.**
    ///
    /// **Both failure directions fall toward a refusal**, because a reader that failed the other
    /// way **would put a model at a human's gate on the strength of a bug** — and this repository
    /// has had a model answer a request body that was never valid JSON, three times running.
    fn the_ruling_out_of(
        &self,
        whatever_the_endpoint_answered: Result<ApprovalDecision, AnAnswerThatIsNotARuling>,
        a_question: &ApprovalQuestion,
    ) -> ApprovalDecision {
        let the_name = self.the_name_it_records_under();
        let the_refusal = ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            false,
            the_name,
            "the policy could not read a ruling out of what the endpoint answered",
        )
        .expect("a policy is not an agent and is never refused as one");
        whatever_the_endpoint_answered.unwrap_or(the_refusal)
    }

    /// The name this policy's decisions are recorded under, **and it is the one rule about names
    /// applied by the one function that has always applied it.**
    fn the_name_it_records_under(&self) -> WhoDecided {
        the_name_the_decision_is_recorded_under(&self.which_policy_it_is)
    }

    /// What the endpoint said, **asked through the one door a model's ruling comes through.**
    fn what_the_endpoint_answered(
        &self,
        a_question: &ApprovalQuestion,
    ) -> Result<ApprovalDecision, AnAnswerThatIsNotARuling> {
        let the_name = self.the_name_it_records_under();
        the_model_is_asked_how_it_rules_on(
            self.the_endpoint,
            a_question,
            &the_name,
            &self.where_the_model_is,
        )
    }
}

impl APersonToDecide for APolicyThatAnswersTheGate<'_> {
    /// The ruling, **and it is recorded under this policy's own name whatever the answer was.**
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let what_it_was_handed = self.the_question_it_puts(a_question);
        self.what_it_was_handed
            .borrow_mut()
            .push(what_it_was_handed);
        let whatever_the_endpoint_answered = self.what_the_endpoint_answered(a_question);
        self.the_ruling_out_of(whatever_the_endpoint_answered, a_question)
    }
}
