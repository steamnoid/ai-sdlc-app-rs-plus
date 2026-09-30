//! A creative agent asked over a program, and what the run does with its answer.
//!
//! **The shell for now.** Every function here has the shape it will keep and none of the behaviour —
//! **the tests are what demand any of it**, because a shell that accepts the absence cannot prove it.

use aisdlc_domain::repository::Repository;

use crate::a_model_is_asked_how_it_rules::WhereAModelIs;
use crate::a_program_we_run::AProgramWeRun;
use crate::the_words_inside_the_endpoints_envelope::the_words_inside_the_endpoints_envelope;
use crate::what_an_agent_judged::WhatAnAgentJudged;
use crate::what_an_agent_offered::{WhatAModelSaidItWouldFile, WhatAnAgentOffered};

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

/// A creative agent that could not produce an offer this run is willing to file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACreativeAgentThatCouldNotOffer(pub String);

/// The question put to a creative agent, **carrying every name it is allowed to cite.**
pub fn the_question_an_agent_is_asked(
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
) -> String {
    let _ = (of_which_repository, what_the_run_saw);
    String::new()
}

/// The body a creative agent is given, **empty for now.**
pub fn the_body_a_creative_agent_is_given(
    the_question: &str,
    where_the_model_is: &WhereAModelIs,
) -> String {
    let _ = (the_question, where_the_model_is);
    String::new()
}

/// What a model said it would file, **read as nothing for now.**
pub fn what_a_model_offered(
    what_the_endpoint_said: &str,
) -> Result<Vec<WhatAModelSaidItWouldFile>, AnAnswerThatIsNotAnOffer> {
    let _ = what_the_endpoint_said;
    Ok(Vec::new())
}

/// What a model said about its own reading, **refused for now.**
pub fn the_three_judgements_a_model_gave(
    what_the_endpoint_said: &str,
) -> Result<WhatAnAgentJudged, AnAnswerThatIsNotAnOffer> {
    let _ = what_the_endpoint_said;
    Err(AnAnswerThatIsNotAnOffer::because(what_the_endpoint_said))
}

/// A creative agent asked what work is worth filing, **offering nothing for now.**
pub fn an_agent_is_asked_what_it_would_file(
    the_model: &impl AProgramWeRun,
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    where_the_model_is: &WhereAModelIs,
) -> Result<WhatAnAgentOffered, ACreativeAgentThatCouldNotOffer> {
    let _ = (
        the_model,
        of_which_repository,
        what_the_run_saw,
        where_the_model_is,
    );
    Ok(WhatAnAgentOffered {
        offers: Vec::new(),
        judged: WhatAnAgentJudged::what_an_agent_said("nothing", "nothing", "nothing")
            .expect("three said"),
    })
}

/// The words inside the endpoint's envelope, **re-read here so the shell has one reader left.**
pub fn the_words_a_model_said(
    what_the_endpoint_said: &str,
) -> Result<String, AnAnswerThatIsNotAnOffer> {
    the_words_inside_the_endpoints_envelope(what_the_endpoint_said).map_err(|nobody_can_read| {
        AnAnswerThatIsNotAnOffer::because(nobody_can_read.what_it_said())
    })
}
