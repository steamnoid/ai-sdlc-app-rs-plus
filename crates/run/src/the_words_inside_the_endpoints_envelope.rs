//! The words inside the envelope an endpoint wraps every answer in.
//!
//! **A model asked to say one word answers with JSON around that word**, and a reader that wants the
//! whole answer to *be* the word decides no model can ever rule — **so the envelope is unwrapped here
//! and the content inside it is what a caller is held to.**
//!
//! **One function and not one per caller, because this is where #102 and #109 were burned.** A model
//! that answers `APPROVED, decided_by: Alice` is talking, and a reader that took the first word would
//! take the yes and drop the forgery in the same step. **Two readers of the same envelope are two
//! places for one of them to be wrong about where the model's words begin.**

/// An answer that was not wrapped in an envelope, quoting what came back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAnswerNobodyCanRead {
    what_it_said: String,
}

impl AnAnswerNobodyCanRead {
    /// The refusal, with the answer quoted back.
    pub fn because(what_it_said: &str) -> Self {
        Self {
            what_it_said: what_it_said.to_string(),
        }
    }

    /// What the endpoint said, exactly.
    pub fn what_it_said(&self) -> &str {
        &self.what_it_said
    }
}

/// The model's own words, inside the envelope the endpoint put them in.
///
/// **Quoted and not summarised on every failure**, because a caller deciding whether to retry needs to
/// know what came back — *Failed to connect to localhost port 11434* is what the first real run got,
/// **and a reader that paraphrased it would have thrown away the only clue there was.**
pub fn the_words_inside_the_endpoints_envelope(
    what_the_endpoint_said: &str,
) -> Result<String, AnAnswerNobodyCanRead> {
    let what_it_came_back_as: serde_json::Value = serde_json::from_str(what_the_endpoint_said)
        .map_err(|_| AnAnswerNobodyCanRead::because(what_the_endpoint_said))?;
    let the_content = what_it_came_back_as
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|the_first| the_first.get("message"))
        .and_then(|the_message| the_message.get("content"))
        .and_then(|the_content| the_content.as_str())
        .map(str::to_string)
        .ok_or_else(|| AnAnswerNobodyCanRead::because(what_the_endpoint_said))?;
    Ok(the_content)
}
