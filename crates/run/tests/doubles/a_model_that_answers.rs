//! A model that answers whatever it is asked for.
//!
//! **One double per file, and not one file of doubles**, because `mod` compiles a copy into every
//! test binary that names it — **so a file holding four doubles is compiled four times and the three
//! a given binary does not use are reported dead**, and the only ways to stop that are an
//! `#[allow(dead_code)]`, which a gate in this repository refuses, or a copy.

use std::cell::RefCell;

use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_run::a_model_is_asked_for_an_artifact::AReasoningModel;
use aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead;
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
