//! The flag that lets an agent offer work, read from what this deployment's environment held.
//!
//! **One word, and not a family of spellings.** A word this system does not know is a word it will
//! not act on, and guessing that `true` means yes would put an agent that files issues into a
//! repository whose operator never asked for one — **and that is the one consequence here a person
//! cannot undo by reading a log afterwards.**
//!
//! **`None` is off and is not an error.** A deployment that has never heard of the flag must still
//! run, and a run that could not start without a switch would be a run that does not run.

use aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers;

/// The one word that lets an agent offer work, and the only one.
const THE_WORD_THAT_OPENS_IT: &str = "yes";

/// The flag as this deployment set it, given what the environment held or nothing at all.
pub fn the_flag_as_this_deployment_sets_it(
    what_the_environment_holds: Option<&str>,
) -> WhetherAnAgentOffers {
    match what_the_environment_holds {
        Some(THE_WORD_THAT_OPENS_IT) => WhetherAnAgentOffers::Yes,
        _ => WhetherAnAgentOffers::No,
    }
}
