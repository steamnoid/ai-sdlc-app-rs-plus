//! A deployment's two facts, read together in the one place that reaches the environment.
//!
//! **The flag and the key are one decision and are read once.** Two readers are two places to be
//! consistent, and what they must agree about is that a model asked for and not reachable is a run
//! that cannot answer anybody — **so the pair is refused together, naming both**, rather than
//! failing later at the far end in words this system did not write.

use aisdlc_run::a_credential::ACredential;
use aisdlc_run::how_this_deployment_is_set_up::HowThisDeploymentIsSetUp;

use crate::whether_a_model_rules_here_is_read::the_flag_as_this_deployment_sets_it;

/// How this deployment is set up, given what its environment held and no more.
pub fn how_this_deployment_is_set_up<'a>(
    what_the_environment_held: (Option<&'a str>, Option<&'a str>),
) -> HowThisDeploymentIsSetUp {
    let (what_they_typed, the_key) = what_the_environment_held;
    let may_a_model_rule = the_flag_as_this_deployment_sets_it(what_they_typed);
    let the_credential = the_key.map(ACredential::the_key_a_deployment_supplied);
    HowThisDeploymentIsSetUp::the_two_facts_of_one_deployment(may_a_model_rule, None)
        .with_a_credential_or_a_refusal(the_credential)
}
