//! Whether a deployment asked for an agent that offers work, read from what the process was told.
//!
//! **The default has to be the one where nothing files itself.** An agent that invents work and
//! files it as issues reaches a real repository, and **the operator of that repository never asked
//! for it** — so a flag that is absent means a human's filing is the only work this system has.

use aisdlc_outside::how_this_deployment_is_set_up::how_this_deployment_is_set_up;
use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers;

/// A deployment told only what this test says it was told.
fn a_deployment_told(what_it_said: Vec<(&str, &str)>) -> TheEnvironment {
    TheEnvironment::of(
        what_it_said
            .into_iter()
            .map(|(a_name, what_it_was)| (a_name.to_string(), what_it_was.to_string()))
            .collect(),
    )
}

#[test]
fn a_deployment_that_asks_for_an_agent_gets_one_that_offers() {
    let what_the_process_was_told = a_deployment_told(vec![("AISDLC_AN_AGENT_OFFERS", "yes")]);

    assert_eq!(
        how_this_deployment_is_set_up(&what_the_process_was_told).may_an_agent_offer(),
        WhetherAnAgentOffers::Yes,
        "**because the flag is the only thing that opens this** — and reading it is the whole of what \
         this deployment is asked for. A flag nothing reads is a comment in the environment"
    );
}

#[test]
fn a_deployment_that_says_nothing_is_not_asked_for_one() {
    let what_the_process_was_told = a_deployment_told(vec![("LLM_MODEL", "space-bunny-free")]);

    assert_eq!(
        how_this_deployment_is_set_up(&what_the_process_was_told).may_an_agent_offer(),
        WhetherAnAgentOffers::No,
        "**because an agent that files its own issues reaches a real repository** — and the operator \
         of that repository never asked for it, so the default is the one where a human's filing is \
         the only work this system has"
    );
}

#[test]
fn a_word_this_system_does_not_know_is_not_taken_as_yes() {
    let what_the_process_was_told = a_deployment_told(vec![("AISDLC_AN_AGENT_OFFERS", "true")]);

    assert_eq!(
        how_this_deployment_is_set_up(&what_the_process_was_told).may_an_agent_offer(),
        WhetherAnAgentOffers::No,
        "**because a word this system does not know is a word it will not act on** — and guessing \
         that `true` means yes would file invented issues in a repository whose operator asked for \
         none, which is the one consequence here that a person cannot undo by reading a log"
    );
}
