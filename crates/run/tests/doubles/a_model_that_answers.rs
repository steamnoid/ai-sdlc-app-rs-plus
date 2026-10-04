use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;

/// A model that answers everything, **and counts how often it was asked.**
///
/// **It records nothing else, and that is because the four tests that use it do not need it** — a
/// double asked for more than its own test uses is reported dead in every binary that has it.
pub struct AModelAnswering {
    how_often: std::cell::RefCell<usize>,
}

impl AModelAnswering {
    /// A model that answers everything, **and a function because four tests build the same double
    /// and a struct literal in each of them is four places to change when it grows a field.**
    pub fn answering_everything() -> Self {
        Self {
            how_often: std::cell::RefCell::new(0),
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
