//! Who does the reasoning: the company, not the model.
//!
//! **Distinct from `AReasoningModel`, which is the thing that reasons.** This is the company it
//! works for, and a run that asks a model and cannot say who answered has an answer nobody can
//! hold anybody to.
//!
//! **A closed set of four**, because the specification lists four and a set that grows is a set
//! somebody has to keep in step with a list of providers.

use std::error::Error;
use std::fmt;

/// Every organisation that reasons, in the order the specification lists them.
pub const EVERY_ORGANISATION_THAT_REASONS: [WhichOrganisationReasons; 4] = [
    WhichOrganisationReasons::Openai,
    WhichOrganisationReasons::Ollama,
    WhichOrganisationReasons::OpenaiCompatible,
    WhichOrganisationReasons::OpenaiResponses,
];

/// The company a reasoning model works for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WhichOrganisationReasons {
    /// `OpenAI`, the company's own name.
    Openai,
    /// A model running on the machine.
    Ollama,
    /// A gateway speaking OpenAI's own protocol.
    OpenaiCompatible,
    /// OpenAI's own responses protocol.
    OpenaiResponses,
}

impl WhichOrganisationReasons {
    /// The specification's own name for this organisation.
    ///
    /// **The capitals exist in exactly one place.** The variant is the reading of a name and
    /// the writing is what a reader sees, joined by one `Display` and one `TryFrom`.
    pub fn as_str(self) -> &'static str {
        match self {
            WhichOrganisationReasons::Openai => "openai",
            WhichOrganisationReasons::Ollama => "ollama",
            WhichOrganisationReasons::OpenaiCompatible => "openai_compatible",
            WhichOrganisationReasons::OpenaiResponses => "openai_responses",
        }
    }
}

impl fmt::Display for WhichOrganisationReasons {
    /// Write the organisation the way the specification writes it.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A name that is not one of the four, so it is not an organisation that reasons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnOrganisationThatDoesNotReason(pub String);

impl AnOrganisationThatDoesNotReason {
    /// Why this name is not an organisation.
    pub fn why_not(&self) -> String {
        let the_four: Vec<&str> = EVERY_ORGANISATION_THAT_REASONS
            .iter()
            .map(|one| one.as_str())
            .collect();
        format!(
            "{} is not one of the four that reason: {}",
            self.0,
            the_four.join(", ")
        )
    }
}

impl Error for AnOrganisationThatDoesNotReason {}

impl fmt::Display for AnOrganisationThatDoesNotReason {
    /// Write the refusal, naming what was asked for and what there is.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.why_not())
    }
}

impl TryFrom<&str> for WhichOrganisationReasons {
    type Error = AnOrganisationThatDoesNotReason;

    /// Read an organisation the way the specification writes it.
    ///
    /// **Refuses a blank and refuses a fifth, and says which four there are.** A refusal that
    /// only says "no" leaves the reader to guess which name was wrong, and there are exactly
    /// four right ones.
    fn try_from(a_name: &str) -> Result<Self, Self::Error> {
        let one = a_name.trim();
        for organisation in EVERY_ORGANISATION_THAT_REASONS {
            if organisation.as_str() == one {
                return Ok(organisation);
            }
        }
        Err(AnOrganisationThatDoesNotReason(a_name.to_string()))
    }
}
