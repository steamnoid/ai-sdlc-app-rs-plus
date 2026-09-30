//! A deployment's two facts, read together in the one place that reaches the environment.
//!
//! **The flag and the key are one decision and are read once.** Two readers are two places to be
//! consistent, and what they must agree about is that a model asked for and not reachable is a run
//! that cannot answer anybody — **so the pair is refused together, naming both**, rather than
//! failing later at the far end in words this system did not write.

use aisdlc_run::a_credential::ACredential;
use aisdlc_run::how_this_deployment_is_set_up::HowThisDeploymentIsSetUp;

use crate::the_environment::TheEnvironment;
use crate::whether_a_model_rules_here_is_read::the_flag_as_this_deployment_sets_it;
use crate::whether_an_agent_offers_is_read::the_flag_as_this_deployment_sets_it as the_flag_letting_an_agent_offer;

/// How this deployment is set up, from the two variables this system reads and no others.
///
/// **The names are written down here and not taken as arguments,** because a caller that could name
/// a variable could read a different one — and what a deployment *is* would then be that caller's
/// opinion rather than a fact about the process. `std::env` is not reached here.
pub fn how_this_deployment_is_set_up(
    what_the_process_was_told: &TheEnvironment,
) -> HowThisDeploymentIsSetUp {
    let may_a_model_rule = the_flag_as_this_deployment_sets_it(
        what_the_process_was_told.the_variable_a_deployment_set("AISDLC_A_MODEL_RULES_HERE"),
    );
    let the_credential = what_the_process_was_told
        .the_variable_a_deployment_set("OPENAI_API_KEY")
        .map(ACredential::the_key_a_deployment_supplied);
    let may_an_agent_offer = the_flag_letting_an_agent_offer(
        what_the_process_was_told.the_variable_a_deployment_set("AISDLC_AN_AGENT_OFFERS"),
    );
    HowThisDeploymentIsSetUp::every_fact_a_deployment_was_told(
        may_a_model_rule,
        None,
        may_an_agent_offer,
    )
    .with_a_credential_or_a_refusal(the_credential)
}
