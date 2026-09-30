//! The flag and the key a deployment supplied, read as one fact about one decision.
//!
//! **Two readers are two places to be consistent, and the thing they must agree about is one
//! decision.** A deployment that switched the flag on and supplied no key has asked for a model
//! and has no way to reach one — **so the pair is refused together, naming both**, rather than
//! failing later at the far end in words this system did not write.
//!
//! **This type is a thing runs print, log and put in a commit message**, and it holds a credential
//! behind a value that prints as `a credential` and has no `Debug`. **A value that reaches a
//! reader through a summary was never protected to begin with.**

use std::fmt;

use crate::a_credential::{ACredential, ACredentialThatIsNothing};
use crate::whether_a_model_rules_here::WhetherAModelRulesHere;
use crate::whether_an_agent_offers::WhetherAnAgentOffers;

/// One deployment's two facts, and neither of them readable.
#[derive(Clone, PartialEq, Eq)]
pub struct HowThisDeploymentIsSetUp {
    may_a_model_rule: WhetherAModelRulesHere,
    the_credential: Option<ACredential>,
    asked_and_cannot_reach: Option<String>,
}

/// A setup that asked for a model it cannot reach, naming both halves of the one decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASetupThatAsksForAModelItCannotReach {
    what_they_asked_for: String,
}

impl ASetupThatAsksForAModelItCannotReach {
    /// Why this deployment cannot answer anybody, naming the word it typed and the missing half.
    pub fn why_not(&self) -> String {
        format!(
            "this deployment typed {:?} to ask for a model, and no credential is behind it — a model asked for and not reachable is a run that cannot answer anybody, and the far end's refusal would be in words this system did not write",
            self.what_they_asked_for
        )
    }
}

impl fmt::Display for ASetupThatAsksForAModelItCannotReach {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.why_not())
    }
}

impl HowThisDeploymentIsSetUp {
    /// The two facts of one deployment, and nothing more.
    ///
    /// **A door and not two public fields**, because a struct whose fields are private to the
    /// crate that declares it is one nobody outside can build — and this is the fourth time in this
    /// repository a port's own type has turned out to be unbuildable from the crate that reads the
    /// world for it.
    pub fn the_two_facts_of_one_deployment(
        may_a_model_rule: WhetherAModelRulesHere,
        the_credential: Option<ACredential>,
    ) -> Self {
        Self {
            may_a_model_rule,
            the_credential,
            asked_and_cannot_reach: None,
        }
    }

    /// Whether an agent may offer work, as a value and not as a lookup.
    ///
    /// **Always no, for now** — the shape is here and the decision is not read yet.
    pub fn may_an_agent_offer(&self) -> WhetherAnAgentOffers {
        WhetherAnAgentOffers::No
    }

    /// Whether a model may rule here, as a value and not as a lookup.
    pub fn may_a_model_rule(&self) -> WhetherAModelRulesHere {
        self.may_a_model_rule
    }

    /// This setup with the credential a deployment supplied, or with the refusal that names both
    /// halves of a deployment that asked for a model it cannot reach.
    ///
    /// **A refused credential and a refused setup are the same refusal.** A deployment that typed
    /// `yes` and supplied nothing has not supplied *a* credential — it has asked a question whose
    /// answer is this sentence, and the sentence names both halves so neither half gets fixed on
    /// its own.
    #[must_use]
    pub fn with_a_credential_or_a_refusal(
        self,
        the_credential: Option<Result<ACredential, ACredentialThatIsNothing>>,
    ) -> Self {
        if self.may_a_model_rule.may_be_asked() && the_credential.is_none() {
            return self.asking_for_a_model_it_cannot_reach("yes");
        }
        let the_credential = the_credential.and_then(Result::ok);
        Self {
            may_a_model_rule: self.may_a_model_rule,
            the_credential,
            asked_and_cannot_reach: self.asked_and_cannot_reach,
        }
    }

    /// This setup, remembering that a model was asked for and there is nothing behind it.
    fn asking_for_a_model_it_cannot_reach(self, what_they_asked_for: &str) -> Self {
        Self {
            may_a_model_rule: self.may_a_model_rule,
            the_credential: None,
            asked_and_cannot_reach: Some(what_they_asked_for.to_string()),
        }
    }

    /// The credential, for a program to carry, and never for a request.
    pub fn the_credential(&self) -> Option<&ACredential> {
        self.the_credential.as_ref()
    }

    /// Why this setup cannot answer anybody, and nothing when it can.
    ///
    /// **Both halves in the sentence**, because a caller reading only *it is off* goes and sets
    /// the flag again, and a caller reading only *there is no key* goes and sets a key on a
    /// deployment that never asked for a model.
    pub fn why_it_cannot_answer_anybody(&self) -> Option<ASetupThatAsksForAModelItCannotReach> {
        self.asked_and_cannot_reach
            .as_ref()
            .map(|what_they_asked_for| ASetupThatAsksForAModelItCannotReach {
                what_they_asked_for: what_they_asked_for.clone(),
            })
    }
}

impl fmt::Display for HowThisDeploymentIsSetUp {
    /// What this deployment can do, and never what its credential is.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.asked_and_cannot_reach.is_some() {
            return formatter.write_str("a deployment that asked for a model and cannot reach one");
        }
        if self.may_a_model_rule.may_be_asked() {
            return formatter.write_str("a deployment a model may rule at, holding a credential");
        }
        formatter.write_str("a deployment nobody has asked anything of")
    }
}
