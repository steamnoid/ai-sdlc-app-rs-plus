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
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

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
///
/// **The instruction and the listing are both here and the instruction is not the check** — a run
/// that only asked nicely and never compared would rely on a model to be honest about which files it
/// invented, **and the whole grounding rule would be a sentence.** §0.4: quality rules live in
/// assertions, not in prompt text.
///
/// **The three keys are named as identifiers and not described in prose, and a real run is why.** The
/// first real run answered with no judgements at all, **because the question described them in words
/// and never wrote their names.** The second answered with all four and called the paths
/// `paths_it_concluded_from`, **because the question said "the paths it concluded from" and the model
/// named the field after the sentence.** A field named in prose is a field the model names, **and
/// describing a shape is not the same as stating it.**
pub fn the_question_an_agent_is_asked(
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    what_is_already_on_file: &crate::the_filings_already_on_file::TheFilingsAlreadyOnFile,
) -> String {
    let what_is_on_file = if what_is_already_on_file.how_many() == 0 {
        String::from("no issues are open on this repository")
    } else {
        what_is_already_on_file.as_lines_for_the_question()
    };
    format!(
        "The keys of your answer are these, written exactly, at the top level: offers, \
         what_it_found, what_it_could_not_settle, what_it_expects_next.\n\
         offers is an array. Each key of an offer is these three, written exactly: title, \
         what_it_asks_for, paths.\n\
         These issues are already on file, and an offer that repeats one of them has nothing to add: \
{what_is_on_file}\n\
         If an offer restates one of them, name its number in a key of that offer written exactly: \
it_restates. If it restates nothing, it_restates is null. An offer naming a filing that is not on \
file is refused, so name a number you were shown or null.\n\
         The three judgements are keys of your answer and not text inside any offer. what_it_found \
         is what you found, what_it_could_not_settle is what you could not settle, and \
         what_it_expects_next is what you expect to happen next. None of the three is ever empty.\n\
         You are looking at {owner}/{name}. Name work worth filing as issues.\n\
         You may only cite files from this listing:\n{listing}\n\
         An offer with no file from the listing is refused, and an empty offers array is a valid \
         answer. Do not name who decided; that is not yours to say.",
        owner = of_which_repository.owner,
        name = of_which_repository.name,
        listing = what_the_run_saw.join("\n"),
        what_is_on_file = what_is_on_file,
    )
}

/// The body a creative agent is given, **built by the JSON library and not by a hand.**
///
/// **The library and not a format string**, because a body that leaves this system is a wire format
/// and a wire format typed by hand is one broken by anything the hand did not think of — **and the
/// far end answering `400` with zero bytes is a refusal that reaches us as an empty string.**
pub fn the_body_a_creative_agent_is_given(
    the_question: &str,
    where_the_model_is: &WhereAModelIs,
) -> String {
    let the_asked = serde_json::json!({
        "model": where_the_model_is.which_model,
        "temperature": 0,
        "messages": [{"role": "user", "content": the_question}],
    });
    the_asked.to_string()
}

/// What a model said it would file, **read out of the envelope and held to the shape it was given.**
///
/// **Every way of failing here is a refusal and never an offer**, because that is the direction the
/// failure has to point: a broken answer read as an offer files an issue nobody read.
pub fn what_a_model_offered(
    what_the_endpoint_said: &str,
) -> Result<Vec<WhatAModelSaidItWouldFile>, AnAnswerThatIsNotAnOffer> {
    let the_content = the_words_a_model_said(what_the_endpoint_said)?;
    let what_it_came_back_as: serde_json::Value = serde_json::from_str(&the_content)
        .map_err(|_| AnAnswerThatIsNotAnOffer::because(&the_content))?;
    the_offers_in(&what_it_came_back_as, &the_content)
}

/// The three judgements a model gave about its own reading, **refused when one is blank.**
///
/// **The same refusal the constructor gives**, reached through a field — and a field is readable from
/// anywhere, **so a caller holding a `WhatAnAgentJudged` has to be able to ask what the constructor
/// asks.**
pub fn the_three_judgements_a_model_gave(
    what_the_endpoint_said: &str,
) -> Result<WhatAnAgentJudged, AnAnswerThatIsNotAnOffer> {
    let the_content = the_words_a_model_said(what_the_endpoint_said)?;
    let what_it_came_back_as: serde_json::Value = serde_json::from_str(&the_content)
        .map_err(|_| AnAnswerThatIsNotAnOffer::because(&the_content))?;
    the_three_in(&what_it_came_back_as, &the_content)
}

/// A creative agent asked what work is worth filing, **offering nothing for now.**
pub fn an_agent_is_asked_what_it_would_file(
    the_model: &impl AProgramWeRun,
    what_is_already_on_file: &mut impl crate::a_filing_is_read::AFilingIsReadFrom,
    of_which_repository: &Repository,
    what_the_run_saw: &[String],
    where_the_model_is: &WhereAModelIs,
) -> Result<WhatAnAgentOffered, ACreativeAgentThatCouldNotOffer> {
    let what_is_on_file = what_is_already_on_file
        .the_filings_already_on_file_of(of_which_repository)
        .map_err(ACreativeAgentThatCouldNotOffer)?;
    let the_question =
        the_question_an_agent_is_asked(of_which_repository, what_the_run_saw, &what_is_on_file);
    let the_body = the_body_a_creative_agent_is_given(&the_question, where_the_model_is);
    let what_the_model_said =
        what_the_model_answered_about_what_it_would_file(the_model, &the_body, where_the_model_is);
    let offered = what_a_model_offered(&what_the_model_said)
        .map_err(|a_fault| ACreativeAgentThatCouldNotOffer(a_fault.what_it_said().to_string()))?;
    let judged = the_three_judgements_a_model_gave(&what_the_model_said)
        .map_err(|a_fault| ACreativeAgentThatCouldNotOffer(a_fault.what_it_said().to_string()))?;
    WhatAnAgentOffered::the_offers_a_model_offered(
        of_which_repository,
        what_the_run_saw,
        offered,
        judged,
    )
    .map_err(|not_offered| ACreativeAgentThatCouldNotOffer(not_offered.0))
}

/// What the model said when asked what it would file, **and the question is not repeated here.**
fn what_the_model_answered_about_what_it_would_file(
    the_model: &impl AProgramWeRun,
    the_body: &str,
    where_the_model_is: &WhereAModelIs,
) -> String {
    let what_it_said = the_model.run(&WhatWeTellAProgramToDo::ask_a_model_what_it_would_file(
        &where_the_model_is.the_endpoint,
        &where_the_model_is.which_model,
        the_body,
    ));
    what_it_said.the_words().to_string()
}

/// The words inside the endpoint's envelope, **read through the one door #102 and #109 were burned at.**
pub fn the_words_a_model_said(
    what_the_endpoint_said: &str,
) -> Result<String, AnAnswerThatIsNotAnOffer> {
    the_words_inside_the_endpoints_envelope(what_the_endpoint_said).map_err(|nobody_can_read| {
        AnAnswerThatIsNotAnOffer::because(nobody_can_read.what_it_said())
    })
}

/// The offers inside what a model said, **or the refusal quoting what came back.**
fn the_offers_in(
    what_it_came_back_as: &serde_json::Value,
    what_it_said: &str,
) -> Result<Vec<WhatAModelSaidItWouldFile>, AnAnswerThatIsNotAnOffer> {
    let Some(the_offers) = what_it_came_back_as
        .get("offers")
        .and_then(|offers| offers.as_array())
    else {
        return Err(AnAnswerThatIsNotAnOffer::because(what_it_said));
    };
    let mut what_was_offered = Vec::new();
    for one_offer in the_offers {
        what_was_offered.push(one_offer_the_model_wrote(one_offer, what_it_said)?);
    }
    Ok(what_was_offered)
}

/// The three judgements inside what a model said, **or the refusal quoting what came back.**
fn the_three_in(
    what_it_came_back_as: &serde_json::Value,
    what_it_said: &str,
) -> Result<WhatAnAgentJudged, AnAnswerThatIsNotAnOffer> {
    WhatAnAgentJudged::what_an_agent_said(
        &the_text_in(what_it_came_back_as, "what_it_found", what_it_said)?,
        &the_text_in(
            what_it_came_back_as,
            "what_it_could_not_settle",
            what_it_said,
        )?,
        &the_text_in(what_it_came_back_as, "what_it_expects_next", what_it_said)?,
    )
    .map_err(|nobody_made| AnAnswerThatIsNotAnOffer::because(&nobody_made.0))
}

/// One offer the model wrote, **refused when a field of it is missing or is not text.**
fn one_offer_the_model_wrote(
    one_offer: &serde_json::Value,
    what_it_said: &str,
) -> Result<WhatAModelSaidItWouldFile, AnAnswerThatIsNotAnOffer> {
    let title = the_text_in(one_offer, "title", what_it_said)?;
    let what_it_asks_for = the_text_in(one_offer, "what_it_asks_for", what_it_said)?;
    let Some(paths) = one_offer.get("paths").and_then(|paths| paths.as_array()) else {
        return Err(AnAnswerThatIsNotAnOffer::because(what_it_said));
    };
    Ok(WhatAModelSaidItWouldFile {
        title,
        what_it_asks_for,
        paths: paths
            .iter()
            .map(|one_path| String::from(one_path.as_str().unwrap_or_default()))
            .collect(),
    })
}

/// One field of text in what a model wrote, **refused when it is missing or is not text.**
fn the_text_in(
    what_it_came_back_as: &serde_json::Value,
    which_field: &str,
    what_it_said: &str,
) -> Result<String, AnAnswerThatIsNotAnOffer> {
    what_it_came_back_as
        .get(which_field)
        .and_then(|said| said.as_str())
        .map(String::from)
        .ok_or_else(|| AnAnswerThatIsNotAnOffer::because(what_it_said))
}
