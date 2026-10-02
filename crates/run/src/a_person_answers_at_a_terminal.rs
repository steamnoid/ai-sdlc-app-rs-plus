//! A person answers at a terminal, and what they were shown while they answered.
//!
//! **This is the ordinary case, and the one every gate was built for.** A policy is the second way
//! a gate is answered, **and a system with only that way is a system where nobody ever reads
//! anything.**
//!
//! **A blank line is a refusal and not an approval.** A terminal that treated Enter as yes would
//! approve every artifact in a run nobody watched, **and an unattended run would then deliver work
//! to a remote for which no person ever read a word.** That is the direction of failure this whole
//! design protects against, **and it is also the easy direction to get wrong by accident.**

use aisdlc_domain::approval::{AgentDecidedError, ApprovalDecision, WhoDecided};

use crate::who_answers_the_gate::THE_NAMESPACE_A_POLICY_IS_RECORDED_IN;
use crate::{APersonToDecide, ApprovalQuestion};

/// The words a person is shown, **and the two kinds are told apart by the variant rather than by a
/// string a caller matches on.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatWasShownToThem {
    /// The artifact as it stands, **beside its name and its version.**
    TheArtifact {
        /// Which artifact, exactly.
        which_artifact: String,
        /// Which version, exactly.
        which_version: u32,
        /// The artifact's own words.
        what_it_says: String,
    },
    /// Nothing, **because they were refused before a word was spent on them.**
    Nothing {
        /// Why they were shown nothing.
        why: String,
    },
}

/// A person answering at a terminal.
pub struct APersonAtTheTerminal {
    who_they_say_they_are: WhoDecided,
    what_they_typed: Box<dyn Fn() -> String>,
    how_the_artifact_is_read: Box<dyn Fn() -> String>,
    what_they_read: String,
    what_was_shown: Option<WhatWasShownToThem>,
}

impl APersonAtTheTerminal {
    /// The person at this terminal, **already having given their name — or a refusal.**
    ///
    /// **The name is an argument and is not asked for at every gate**, because a terminal that
    /// asks on each one is a terminal that can disagree with itself between the two answers.
    ///
    /// **A name in a machine's namespace is refused here and not at the ruling, and that is the
    /// domain's own rule arriving at the edge**: `ApprovalDecision::the_ruling_about` refuses a
    /// decider named `aisdlc-` outright, **so a terminal named after an agent could not produce a
    /// ruling even to refuse with.** Returning the refusal here means the failure happens **before
    /// the artifact is read**, so nobody's attention is spent on a document a machine was named
    /// after.
    pub fn the_person_at(
        who_they_say_they_are: WhoDecided,
        what_they_typed: impl Fn() -> String + 'static,
        how_the_artifact_is_read: impl Fn() -> String + 'static,
    ) -> Result<Self, AgentDecidedError> {
        if is_this_name_a_machines(&who_they_say_they_are) {
            return Err(AgentDecidedError(who_they_say_they_are.0));
        }
        Ok(Self {
            who_they_say_they_are,
            what_they_typed: Box::new(what_they_typed),
            how_the_artifact_is_read: Box::new(how_the_artifact_is_read),
            what_they_read: String::new(),
            what_was_shown: None,
        })
    }

    /// Everything this person was shown, **in the words they were shown it.**
    ///
    /// **Empty when they were refused before anything was shown**, which is the only way a caller
    /// can tell a refused terminal from one nobody asked.
    pub fn what_they_read(&self) -> &str {
        &self.what_they_read
    }

    /// Which of the two things was shown, **and it is asked for by name rather than by reading the
    /// string back** — a caller matching on prose would be matching on a document.
    pub fn what_they_were_shown(&self) -> Option<&WhatWasShownToThem> {
        self.what_was_shown.as_ref()
    }

    /// The artifact, its name and its version, **in that order, because the name is how a reader
    /// knows which artifact they are about to rule on.**
    fn the_artifact_as_it_stands(&self, a_question: &ApprovalQuestion) -> String {
        let what_it_says = (self.how_the_artifact_is_read)();
        format!(
            "{} at version {}\n\n{what_it_says}",
            a_question.about_which_artifact(),
            a_question.about_which_version(),
        )
    }

    /// What they typed, split into a word and a reason, **and anything that is not *yes* is a
    /// refusal.**
    ///
    /// **The word is compared and nothing else is parsed**, because a person typing *no, because it
    /// is worse* and a person typing *no* have both answered, **and a reader that needed the second
    /// form would have refused an honest answer and asked again.**
    fn their_answer(what_they_typed: &str) -> (bool, String) {
        let (the_word, the_reason) = what_they_typed
            .split_once('\n')
            .unwrap_or((what_they_typed, ""));
        let did_they_agree = the_word.trim().eq_ignore_ascii_case("yes");
        (did_they_agree, the_reason.trim().to_string())
    }

    /// The ruling, built from a word and a reason, **and the name is copied and never edited.**
    fn the_ruling_of_what_they_said(
        &self,
        a_question: &ApprovalQuestion,
        did_they_agree: bool,
        their_reason: &str,
    ) -> ApprovalDecision {
        let the_reason = if their_reason.is_empty() {
            "this person said nothing about why".to_string()
        } else {
            their_reason.to_string()
        };
        ApprovalDecision::the_ruling_about(
            a_question.about_which_work_item(),
            a_question.about_which_artifact(),
            a_question.about_which_version(),
            did_they_agree,
            self.who_they_say_they_are.clone(),
            &the_reason,
        )
        .expect("a person is not an agent")
    }

    /// Record that this terminal was shown nothing, **which is what a caller reads to tell a
    /// terminal that refused from one nobody asked.**
    ///
    /// **A terminal that showed nothing and cannot say so looks exactly like one nobody asked**,
    /// and the second is not a fact anybody can act on.
    pub fn the_nothing_was_shown_is(&mut self, why: &str) {
        self.what_was_shown = Some(WhatWasShownToThem::Nothing {
            why: String::from(why),
        });
    }
}

/// The prefix an agent is named with, **and it is repeated here on purpose.**
///
/// **A constant this module does not import is a constant this module could drift from.** The
/// gateway that refuses an agent's ruling reads the prefix from one place, **and a second copy that
/// could disagree with it is how a `refused along with the agents` becomes a `refused for the
/// wrong reason`.**
const THE_PREFIX_AN_AGENT_IS_NAMED_WITH: &str = "aisdlc-";

/// Whether this name belongs to a machine, **and both namespaces are checked.**
///
/// **A terminal named `autopilot:` is refused as firmly as one named `aisdlc-`,** because a person
/// answering at a terminal and a policy answering are two facts, **and a record that cannot tell
/// them apart is the forgery this whole record exists to prevent.**
fn is_this_name_a_machines(who: &WhoDecided) -> bool {
    who.0.starts_with(THE_PREFIX_AN_AGENT_IS_NAMED_WITH)
        || who.0.starts_with(THE_NAMESPACE_A_POLICY_IS_RECORDED_IN)
}

impl APersonToDecide for APersonAtTheTerminal {
    /// The ruling, **after refusing a terminal named after a machine, and before showing that
    /// terminal anything.**
    ///
    /// **The refusal is checked first and the artifact is read second**, because a run that cannot
    /// produce a human ruling should stop **without having spent the person's attention on it** — and
    /// a person who was shown a document is a person whose attention has already been spent.
    fn how_they_rule_on(&mut self, a_question: &ApprovalQuestion) -> ApprovalDecision {
        let what_they_were_shown = self.the_artifact_as_it_stands(a_question);
        self.what_they_read.clone_from(&what_they_were_shown);
        self.what_was_shown = Some(WhatWasShownToThem::TheArtifact {
            which_artifact: a_question.about_which_artifact().to_string(),
            which_version: a_question.about_which_version().0,
            what_it_says: what_they_were_shown,
        });
        let (did_they_agree, their_reason) = Self::their_answer(&(self.what_they_typed)());
        self.the_ruling_of_what_they_said(a_question, did_they_agree, &their_reason)
    }
}
