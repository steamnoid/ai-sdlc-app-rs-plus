//! Reading a deployment's two facts out of what the process was told, by name.
//!
//! **A variable nobody set is a refusal that says which one, and never a default.** A reader that
//! invents a value for a key it did not find is a reader that reports a deployment it never
//! described — and a run that believes it may reach a model because a string was fabricated is a
//! run that finds out at the far end.
//!
//! **The names are in the source and not in the caller's hands,** because a caller that could name
//! a variable could read a different one, and then what a deployment is would be an opinion.

use aisdlc_outside::how_this_deployment_is_set_up::how_this_deployment_is_set_up;
use aisdlc_outside::the_environment::TheEnvironment;

/// What a process is told, in the form the tests can write down.
fn an_environment_saying(one_pair: (&str, &str), another_pair: (&str, &str)) -> TheEnvironment {
    TheEnvironment::of(vec![
        (one_pair.0, one_pair.1),
        (another_pair.0, another_pair.1),
    ])
}

#[test]
fn a_variable_nobody_set_is_a_refusal_and_not_a_value() {
    let an_empty_environment = TheEnvironment::of(vec![]);

    let what_it_held = an_empty_environment.the_variable_a_deployment_set("OPENAI_API_KEY");

    assert_eq!(
        what_it_held, None,
        "**because a reader that fills in a value for a key it did not find describes a deployment that does not exist** — and nothing here may stand in for a credential, because a credential that is an empty string still travels to a far end and comes back refused in words this system did not write"
    );
}

#[test]
fn the_flag_and_the_key_come_from_the_two_named_variables() {
    let the_environment = an_environment_saying(
        ("AISDLC_A_MODEL_RULES_HERE", "yes"),
        ("OPENAI_API_KEY", "sk-a-key"),
    );

    let how_it_is_set_up = how_this_deployment_is_set_up(&the_environment);

    assert!(
        how_it_is_set_up.a_model_may_not_rule()
            == aisdlc_run::whether_a_model_rules_here::AMayNotRule::Yes
            && how_it_is_set_up.a_credential().is_some(),
        "**because these two are one decision and not two features** — a model asked for and not reachable is a run that cannot answer anybody, so both halves have to arrive from the names this source writes down"
    );
}

#[test]
fn the_two_names_are_the_ones_this_system_uses() {
    let the_environment = an_environment_saying(
        ("AISDLC_A_MODEL_RULES_HERE", "yes"),
        ("OPENAI_API_KEY", "sk-a-key"),
    );

    let how_it_is_set_up = how_this_deployment_is_set_up(&the_environment);
    let with_no_key = how_this_deployment_is_set_up(&TheEnvironment::of(vec![(
        "AISDLC_A_MODEL_RULES_HERE",
        "yes",
    )]));

    assert!(
        how_it_is_set_up.a_credential().is_some() && with_no_key.a_credential().is_none(),
        "**and the two names are named here rather than handed in** — a caller that could name a variable could read a different one, and what a deployment is would be that caller's opinion"
    );
}
