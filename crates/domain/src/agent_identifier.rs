//! An agent, named.
//!
//! **Absence is `None` and never a string that says "absent".** The state machine decides
//! whether an agent is present by asking whether there is one, so a work item in
//! `IN_PROGRESS_BY_AGENT` holding a blank name would pass the biconditional while nobody was
//! working on it.

use std::fmt;

/// The one word that means "no agent" without being an agent.
///
/// **Refused as written and not by its letters.** `NONE` is a name, because the glossary
/// reserves one word and a reader who refused both spellings would be refusing a name
/// somebody may legitimately choose — with a rule the glossary does not carry. A word is a
/// word; `none-of-the-above` and `nonesuch` identify people.
const THE_WORD_THAT_MEANS_ABSENT: &str = "none";

/// The name an agent answers to, and never a bare string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AgentIdentifier(pub String);

/// A string that only means "absent", so it is not an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAgentThatIsNotAnIdentity(pub String);

impl TryFrom<&str> for AgentIdentifier {
    type Error = AnAgentThatIsNotAnIdentity;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.trim().is_empty() || name == THE_WORD_THAT_MEANS_ABSENT {
            return Err(AnAgentThatIsNotAnIdentity(name.to_string()));
        }
        Ok(AgentIdentifier(name.to_string()))
    }
}

impl fmt::Display for AgentIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Display for AnAgentThatIsNotAnIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} does not identify an agent", self.0)
    }
}

impl std::error::Error for AnAgentThatIsNotAnIdentity {}
