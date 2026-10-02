//! Who answers the gate, and the name a decision is recorded under.
//!
//! **Two ways a gate is answered and they must not be able to look the same.** A person rules in
//! their own name; **a policy answers in a namespace of its own, and the namespace is applied by
//! the thing that writes the record and never by the thing that reads the setting** — because that
//! is the only place it cannot be bypassed, **and a policy that could be recorded as a person would
//! forge the one thing this record exists to be.**

use std::fmt;

use aisdlc_domain::approval::WhoDecided;

/// The namespace a policy's decisions are recorded under, **and it is a namespace on purpose.**
///
/// **`autopilot:` shares no prefix with an agent's `aisdlc-`, and that is the whole reason.** A
/// policy answering in the agent namespace would be refused along with the agents it is defined in
/// contrast to, **and an autopilot that may approve every gate could then not rule at all.**
pub const THE_NAMESPACE_A_POLICY_IS_RECORDED_IN: &str = "autopilot:";

/// Who answers this run's gates, **and the two are told apart by the variant rather than by a
/// string a caller matches on.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhoAnswersTheGate {
    /// A person, **in the name they gave and with nothing added to it.**
    APersonNamed(WhoDecided),
    /// A policy, **and it is this system's own name for it and never a name the deployment chose
    /// whole.**
    ThisPolicyAnswering {
        /// The bare policy name a deployment set, **with no namespace on it.**
        which_policy: String,
    },
}

/// A policy nobody could name in an argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum APolicyNobodyCanQuote {
    /// The deployment named nothing at all after the flag, **and that is the shape a truncated
    /// value takes.**
    ThePolicySaysNothing,
}

impl fmt::Display for APolicyNobodyCanQuote {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "a policy was named with nothing behind it, and a claim that carries no claim cannot be \
             quoted in an argument — the fallback would be a run answering its own gate with a \
             policy nobody can name"
        )
    }
}

impl std::error::Error for APolicyNobodyCanQuote {}

/// Who answers the gate, **from what a deployment named.**
///
/// **Naming nothing gives a person and not a policy**, because **the setting nobody wrote must not
/// decide the gate** — a deployment that names no policy is the ordinary case, and the ordinary case
/// is a person at a terminal.
///
/// **A name that says nothing is refused rather than defaulted**, because a truncated value is the
/// shape this takes and `autopilot:` with nothing behind it is a claim carrying no claim.
pub fn the_policy_a_deployment_named(
    what_a_deployment_named: Option<&str>,
) -> Result<WhoAnswersTheGate, APolicyNobodyCanQuote> {
    let Some(what_was_named) = what_a_deployment_named else {
        return Ok(WhoAnswersTheGate::APersonNamed(a_person_to_ask()));
    };
    let the_policy = what_was_named.trim();
    if the_policy.is_empty() {
        return Err(APolicyNobodyCanQuote::ThePolicySaysNothing);
    }
    Ok(WhoAnswersTheGate::ThisPolicyAnswering {
        which_policy: the_policy.to_string(),
    })
}

/// The person a run asks when nobody was named, **and it is a name and not a blank** — a decision
/// recorded as `""` names nobody and looks like a bug.
fn a_person_to_ask() -> WhoDecided {
    WhoDecided::try_from("the person running this").expect("a name is not blank")
}

/// The name a decision is recorded under, **and this is the only place the namespace is applied.**
///
/// **A policy's name is prefixed here and nowhere else, and that is the whole design.** The
/// alternative was for the thing that reads the deployment's setting to prefix it, **and that would
/// be a second place to forget** — a caller that reached a record directly would get a policy's
/// decision under a bare name, **and a bare name is indistinguishable from a person's.** A
/// deployment whose policy name already carries the prefix gets it twice, **which is loud and not
/// subtle, and loud is the right outcome for a mistake in this direction.**
///
/// **A person is recorded verbatim and nothing is added to them**, because a person's name in a
/// record is the one thing that cannot be edited, **and adding a namespace would make them harder
/// to find in the very table meant to name them — and it would be a lie about what they are.**
pub fn the_name_the_decision_is_recorded_under(whoever_answers: &WhoAnswersTheGate) -> WhoDecided {
    match whoever_answers {
        WhoAnswersTheGate::APersonNamed(whoever) => whoever.clone(),
        WhoAnswersTheGate::ThisPolicyAnswering { which_policy } => WhoDecided(format!(
            "{THE_NAMESPACE_A_POLICY_IS_RECORDED_IN}{which_policy}"
        )),
    }
}
