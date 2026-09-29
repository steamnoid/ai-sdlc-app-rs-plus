//! The flag, read from this deployment's own environment, and what an absent one means.
//!
//! **A flag that is absent is off, and not an error.** A deployment that has never heard of the
//! flag must still run, and a run that cannot start because a switch is missing is a run that
//! cannot start at all — **and the failure would look like a misconfiguration rather than a
//! default.**
//!
//! **The name of the variable is ours and the value is not.** A deployment reads one name in one
//! place, and everything else this system does not depend on what a shell happens to hold.

use aisdlc_outside::whether_a_model_rules_here_is_read::the_flag_as_this_deployment_sets_it;
use aisdlc_run::whether_a_model_rules_here::WhetherAModelRulesHere;

#[test]
fn a_deployment_that_never_heard_of_the_flag_gets_no() {
    let the_flag = the_flag_as_this_deployment_sets_it(None);

    assert_eq!(
        the_flag,
        WhetherAModelRulesHere::No,
        "**because a missing flag is off and not an error** — a deployment that has never heard of it must still run, and the default has to be the one where a human rules"
    );
}

#[test]
fn a_deployment_that_asked_for_a_model_gets_yes() {
    let the_flag = the_flag_as_this_deployment_sets_it(Some("yes"));

    assert_eq!(
        the_flag,
        WhetherAModelRulesHere::Yes,
        "**and a deployment that asked for one gets it, in the words it used** — the variable holds a word a person typed, and reading a person's word is what this is for"
    );
}

#[test]
fn a_flag_somebody_misspelled_is_off_rather_than_guessed_at() {
    for what_they_typed in ["YES", "y", "true", "1", "on", "enabled", ""] {
        let the_flag = the_flag_as_this_deployment_sets_it(Some(what_they_typed));

        assert_eq!(
            the_flag,
            WhetherAModelRulesHere::No,
            "**because a word this system does not know is a word it will not act on** — and guessing that `true` means yes would put a model at a gate on a deployment whose operator never asked for one. A flag that reads only what it was told is off by every spelling nobody promised: {what_they_typed:?}"
        );
    }
}
