//! A model asked what work is worth filing, and what it said when it answered.
//!
//! **The shell for now.** It asks nothing and reads nothing — the tests are what demand both.

use aisdlc_domain::repository::Repository;

use crate::what_an_agent_offered::WhatAModelSaidItWouldFile;

/// An answer that was not offers, quoting what came back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAnswerThatIsNotAnOffer {
    what_it_said: String,
}

impl AnAnswerThatIsNotAnOffer {
    /// The refusal, with the answer quoted back.
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

/// The question put to a creative agent, **empty for now.**
pub fn the_question_an_agent_is_asked(
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
) -> String {
    let _ = (of_which_repository, what_the_run_saw);
    String::new()
}

/// What a model said it would file, **read as nothing for now.**
pub fn what_a_model_offered(
    what_the_endpoint_said: &str,
) -> Result<Vec<WhatAModelSaidItWouldFile>, AnAnswerThatIsNotAnOffer> {
    let _ = what_the_endpoint_said;
    Ok(Vec::new())
}
