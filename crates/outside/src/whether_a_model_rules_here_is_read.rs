//! The flag, read from this deployment's own environment, and what an absent one means.
//!
//! **A flag that is absent is off, and not an error.** A deployment that has never heard of the
//! flag must still run, and a run that cannot start because a switch is missing is a run that
//! cannot start at all — **and the failure would look like a misconfiguration rather than a
//! default.**
//!
//! **One word, and not a family of spellings.** A word this system does not know is a word it
//! will not act on, and guessing that `true` means yes would put a model at a gate on a deployment
//! whose operator never asked for one.

use aisdlc_run::whether_a_model_rules_here::WhetherAModelRulesHere;

/// The one word that opens the gate to a model, and the only one.
const THE_WORD_THAT_OPENS_IT: &str = "yes";

/// The flag as this deployment set it, given what the environment held or nothing at all.
///
/// **`None` is off and is not a failure.** The default has to be the one where a human rules, and
/// a run that refused to start without a switch would not be a run that fails safely — it would be
/// a run that does not run.
pub fn the_flag_as_this_deployment_sets_it(
    what_the_environment_holds: Option<&str>,
) -> WhetherAModelRulesHere {
    match what_the_environment_holds {
        Some(THE_WORD_THAT_OPENS_IT) => WhetherAModelRulesHere::Yes,
        _ => WhetherAModelRulesHere::No,
    }
}
