//! The model a deployment configured, asked over a program, and its answer read out of what
//! the far end wrote.
//!
//! **The one implementation of `AReasoningModel` outside a test file, and the reason the port
//! was open rather than declared.** While its only implementor was a double, the port a real
//! answer would arrive through had never been open at all, and the paths that do ask a model
//! went around it — which is the shape `README.md` records for `AGitHubWeWriteTo`, where seven
//! cycles of the creative path were proved against a place that accepts anything.
//!
//! **The envelope is unwrapped here and not by a caller.** A model asked to say one word
//! answers with JSON around that word, and a caller handed the whole answer decides no model
//! can ever rule — **so the adapter knows an envelope exists and the use case does not.**

use aisdlc_domain::approval::WhichArtifact;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;

use aisdlc_run::AReasoningModel;
use aisdlc_run::a_model_is_asked_how_it_rules::{WhereAModelIs, the_body_a_model_is_given};
use aisdlc_run::a_program_we_run::AProgramWeRun;
use aisdlc_run::the_words_inside_the_endpoints_envelope::{
    AnAnswerNobodyCanRead, the_words_inside_the_endpoints_envelope,
};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A model at the address a deployment named, asked through a program this system runs.
///
/// **Generic over the program and not concrete, because the composition root monomorphises and
/// no `dyn` belongs in the domain or the run.** A struct holding `TheProgramAsItRuns` would make
/// this the one type in the crate that cannot be built without a host, **and the program is the
/// port the tests use instead of a network.**
#[derive(Debug)]
pub struct TheModelAsItIsAsked<TheProgram> {
    where_the_model_is: WhereAModelIs,
    the_program: TheProgram,
}

impl<TheProgram> TheModelAsItIsAsked<TheProgram> {
    /// The model a deployment put at an address, asked through this program.
    ///
    /// **Both facts travel together and neither is guessed here.** The address and the model
    /// name are what a deployment configured and can say out loud; a model this system chose
    /// is one nobody deployed, and the first real run fell back to an address nobody had read.
    pub fn the_model_where_a_deployment_put_its_model(
        where_the_model_is: WhereAModelIs,
        the_program: TheProgram,
    ) -> Self {
        Self {
            where_the_model_is,
            the_program,
        }
    }
}

impl<TheProgram> AReasoningModel for TheModelAsItIsAsked<TheProgram>
where
    TheProgram: AProgramWeRun,
{
    /// The model's own words, out of the envelope the far end wrapped them in.
    ///
    /// **The organisation is not in the body and that is recorded rather than assumed.** All
    /// four organisations are reached at the same address an OpenAI-compatible endpoint answers
    /// at, so nothing about the request changes — **and a field carried because a signature
    /// carries it is a field somebody will read as meaning something.**
    fn what_it_says_when_asked_for(
        &mut self,
        _which_artifact: &WhichArtifact,
        _which_organisation: WhichOrganisationReasons,
        the_question: &str,
    ) -> Result<String, AnAnswerNobodyCanRead> {
        let the_body = the_body_a_model_is_given(the_question, &self.where_the_model_is);
        let what_the_endpoint_wrote = self
            .the_program
            .run(&the_question_to_put(&self.where_the_model_is, &the_body));
        the_words_inside_the_endpoints_envelope(what_the_endpoint_wrote.the_words())
    }
}

/// The request this adapter makes, and **no credential: a request never carries one.**
fn the_question_to_put(
    where_the_model_is: &WhereAModelIs,
    the_body: &str,
) -> WhatWeTellAProgramToDo {
    WhatWeTellAProgramToDo::ask_a_model(
        &where_the_model_is.the_endpoint,
        &where_the_model_is.which_model,
        the_body,
    )
}
