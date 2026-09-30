//! What a process was told before it started, which is a value and never a reading.
//!
//! **One place reaches the process, and it is this one.** Everywhere else in this system takes
//! what a process was told as a value it was handed — **because a function that reads its own
//! configuration has moved a decision out of a type and into a line of code**, and a reader that
//! goes looking cannot be handed a test.
//!
//! **A variable nobody set is `None`, and never a default.** A reader that invents a value for a
//! key it did not find describes a deployment that does not exist.

use std::collections::BTreeMap;

/// The six variables this system reads, and the only six.
///
/// **Four and not two, because the two that were missing are how a model gets asked at all** — and
/// the first real run had no way to read them, so a test filled the gap with a number and `curl`
/// answered *Failed to connect to localhost port 11434*.
const THE_FLAG_THAT_LETS_A_MODEL_RULE: &str = "AISDLC_A_MODEL_RULES_HERE";
const THE_KEY_A_MODEL_ANSWERS_WITH: &str = "OPENAI_API_KEY";
const WHERE_A_MODEL_ANSWERS: &str = "LLM_ENDPOINT";
const WHICH_MODEL_ANSWERS_THERE: &str = "LLM_MODEL";
const WHO_SIGNS_A_COMMENT: &str = "AISDLC_WHO_SIGNS_A_COMMENT";
const WHAT_A_MODEL_IS_ASKED_BESIDES: &str = "LLM_HEADERS";

/// What a process was told, kept whole so that a name can be asked for by name.
#[derive(Clone, PartialEq, Eq)]
pub struct TheEnvironment(BTreeMap<String, String>);

impl TheEnvironment {
    /// What the process running this was told, and the only call in this system that asks.
    ///
    /// **`std::env` appears here and nowhere else,** so a reader that reaches for it is a reader
    /// that has decided its own configuration, and the one place a test can point at is the one
    /// place it happens.
    pub fn reading_what_the_process_was_told() -> Self {
        let mut what_it_held = BTreeMap::new();
        for a_name in [
            THE_FLAG_THAT_LETS_A_MODEL_RULE,
            THE_KEY_A_MODEL_ANSWERS_WITH,
            WHERE_A_MODEL_ANSWERS,
            WHICH_MODEL_ANSWERS_THERE,
            WHO_SIGNS_A_COMMENT,
            WHAT_A_MODEL_IS_ASKED_BESIDES,
        ] {
            if let Ok(what_it_said) = std::env::var(a_name) {
                what_it_held.insert(a_name.to_string(), what_it_said);
            }
        }
        TheEnvironment(what_it_held)
    }

    /// What a process was told, written down — for the tests, and for a caller that already knows.
    pub fn of(what_it_held: Vec<(String, String)>) -> Self {
        TheEnvironment(what_it_held.into_iter().collect())
    }

    /// What one named variable said, and `None` when it said nothing at all.
    pub fn the_variable_a_deployment_set(&self, its_name: &str) -> Option<&str> {
        self.0.get(its_name).map(String::as_str)
    }
}
