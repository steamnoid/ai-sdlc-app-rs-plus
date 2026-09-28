//! An agent, named, and the strings that are refused as a name for one.
//!
//! **This is the strict one of the two names, and the reason is the state machine**
//! rather than a taste about what an identifier ought to look like. The state machine
//! decides whether an agent is present by asking whether there is one, so the only thing
//! this type owes it is that a name it did hold really is an agent. That is the sentence
//! that makes the refusal of `none` load-bearing, and it is written out in full on
//! `named` below rather than here, because a reader who does not have it will read the
//! check as pedantry.
//!
//! Nothing here may reach outside this crate. There is no parsing of a file, no reading
//! of a clock and no name of a model, and the crate holding this file has no dependencies
//! at all — which is the rule every other boundary in the repository rests on.

use std::convert::TryFrom;
use std::fmt;

/// A string that only means absent, and is not the name of an agent.
///
/// **A refusal and not an `Option`, and it carries the string that was looked for.** A
/// caller asking "is this an agent" has the same question whichever way the answer came
/// out, and an `Option` would make it choose which of two shapes to hold before it knows
/// anything. The string matters for the same reason a model is corrected better when it
/// is handed its own spelling back — and for the stronger reason that this crate has two
/// refusals, so a caller told "no" has to learn which of them it was.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAgentThatIsNotAnIdentity {
    what_was_looked_for: String,
}

impl AnAgentThatIsNotAnIdentity {
    /// The refusal for a string that names no agent.
    pub fn for_a_name(name: &str) -> AnAgentThatIsNotAnIdentity {
        AnAgentThatIsNotAnIdentity {
            what_was_looked_for: name.to_string(),
        }
    }

    /// The string that was looked for, and named no agent.
    ///
    /// **Exactly as it was written, and that is the whole of what this promises.** The
    /// strings this type refuses are blanks and one word, so a caller that sent three
    /// spaces gets three spaces back, and a caller correcting itself is told which
    /// spelling of its own answer the system refused rather than a tidied one.
    pub fn what_was_looked_for(&self) -> &str {
        &self.what_was_looked_for
    }
}

impl fmt::Display for AnAgentThatIsNotAnIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} only means that no agent is here",
            self.what_was_looked_for
        )
    }
}

impl std::error::Error for AnAgentThatIsNotAnIdentity {}

/// The one word that means "no agent" and is not the name of one.
///
/// **A constant and not a string written twice, because the constructor and its
/// documentation have to be able to disagree about it in exactly one place.** The
/// alternative — spelling the word out at the point of comparison and describing it in
/// the `///` above — is two spellings of one refusal, and a reader who changes the
/// comparison would not be told that the sentence above it is now a lie.
const THE_WORD_THAT_ONLY_MEANS_ABSENT: &str = "none";

/// An agent, named. The identifier a work item is held by, and the answer to whether
/// there is an agent at all.
///
/// **The same six derives the work item's name has, and for the same reason.** A name is
/// a key: `Eq` and `Hash` because this is what a work item's stage and agent, a
/// checkpoint and a run's record are looked up by, and `Ord` because a record that
/// reorders its agents between two reads is a record nobody can diff. `Copy` is absent
/// because a name owns its text, and a copy would be two handles onto one string rather
/// than two agents. The two types differ in what they refuse, not in what they are once
/// they are a name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentIdentifier(String);

impl AgentIdentifier {
    /// The agent by this name, or the refusal quoting the string back.
    ///
    /// **A blank, and the one word that only means absent, are refused. Nothing else.**
    /// The state machine decides whether an agent is present by asking whether there is
    /// one, so a work item in `IN_PROGRESS_BY_AGENT` holding a blank agent would pass the
    /// biconditional while nobody was working on it. That is the whole reason this check
    /// exists, and the reason it is here rather than left to a caller.
    ///
    /// **`"none"` is the word, exactly, and the case is not tidied.** What is refused is
    /// the value a machine writes to mean "no agent", not a word that happens to look
    /// like one: `None`, `NONE` and ` none ` are agents, because an agent is identified by
    /// the exact string it is registered under and lowercasing or trimming one would
    /// answer about an agent nobody registered. The work item's name refuses a blank and
    /// nothing else, and that is the difference between the two types rather than an
    /// oversight in either.
    ///
    /// The name is kept exactly as it was written. Nothing is trimmed and nothing is
    /// lowercased, so a name a person typed and a name a run reads back are the same name.
    pub fn named(name: &str) -> Result<AgentIdentifier, AnAgentThatIsNotAnIdentity> {
        if name.trim().is_empty() || name == THE_WORD_THAT_ONLY_MEANS_ABSENT {
            return Err(AnAgentThatIsNotAnIdentity::for_a_name(name));
        }
        Ok(AgentIdentifier(name.to_string()))
    }
}

impl TryFrom<&str> for AgentIdentifier {
    type Error = AnAgentThatIsNotAnIdentity;

    /// The agent by this name, or the refusal quoting the string back.
    ///
    /// **`named` is the one implementation, and this is the door that calls it.** A second
    /// reading of what counts as absent would be two answers to one question the day the
    /// two disagreed, and the two are not a detail: `" none "` is an agent on one reading
    /// and not an agent on the other.
    fn try_from(name: &str) -> Result<AgentIdentifier, AnAgentThatIsNotAnIdentity> {
        AgentIdentifier::named(name)
    }
}

impl fmt::Display for AgentIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_written = &self.0;
        write!(formatter, "{as_written}")
    }
}
