//! A model asked what work is worth filing, and what it said when it answered.
//!
//! **The listing goes into the question and the check is code.** A model asked to improve a project
//! will invent one that does not exist, **so the run hands it the names of the files there are** and
//! then checks the answer against that same list — **and a prompt forbidding something is not
//! verification**, which is why the refusal lives here and not in the sentence.
//!
//! **The envelope is the endpoint's and the offer is the model's.** A reader that demanded the whole
//! answer *be* the offer would decide no model can ever file anything — **so the envelope is unwrapped
//! and the content inside it is held to a shape.**

use aisdlc_domain::repository::Repository;

use crate::the_words_inside_the_endpoints_envelope::the_words_inside_the_endpoints_envelope;
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

/// What a model said it would file, **read out of the envelope and held to the shape it was given.**
///
/// **Every way of failing here is a refusal and never an offer**, because that is the direction the
/// failure has to point: a broken answer read as an offer files an issue nobody read.
pub fn what_a_model_offered(
    what_the_endpoint_said: &str,
) -> Result<Vec<WhatAModelSaidItWouldFile>, AnAnswerThatIsNotAnOffer> {
    let the_content = the_words_inside_the_endpoints_envelope(what_the_endpoint_said).map_err(
        |what_came_back| AnAnswerThatIsNotAnOffer::because(what_came_back.what_it_said()),
    )?;
    let the_offers: serde_json::Value = serde_json::from_str(&the_content)
        .map_err(|_| AnAnswerThatIsNotAnOffer::because(&the_content))?;
    the_offers_in(&the_offers, &the_content)
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
    one_offer: &serde_json::Value,
    which_field: &str,
    what_it_said: &str,
) -> Result<String, AnAnswerThatIsNotAnOffer> {
    one_offer
        .get(which_field)
        .and_then(|said| said.as_str())
        .map(String::from)
        .ok_or_else(|| AnAnswerThatIsNotAnOffer::because(what_it_said))
}
