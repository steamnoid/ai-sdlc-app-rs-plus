use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;

/// A model that answers everything, **and records what it was asked for in the order it was asked.**
pub struct AModelAnswering {
    every_artifact_it_was_asked_for: std::cell::RefCell<Vec<String>>,
}

impl AModelAnswering {
    /// A model that answers everything, **and a function because four tests build the same double
    /// and a struct literal in each of them is four places to change when it grows a field.**
    pub fn answering_everything() -> Self {
        Self {
            every_artifact_it_was_asked_for: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// Every artifact this model was asked for, **in order** — a run that asked seven agents and
    /// this says which, **and a test that cannot count the questions cannot prove how many agents
    /// there were.**
    pub fn what_it_was_asked_for(&self) -> Vec<String> {
        self.every_artifact_it_was_asked_for.borrow().clone()
    }
}

impl AReasoningModel for AModelAnswering {
    fn what_it_says_when_asked_for(
        &mut self,
        which_artifact: &WhichArtifact,
        _who_reasoned: WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        self.every_artifact_it_was_asked_for
            .borrow_mut()
            .push(which_artifact.0.clone());
        Ok(String::from("an artifact, of the length a real one is"))
    }
}
