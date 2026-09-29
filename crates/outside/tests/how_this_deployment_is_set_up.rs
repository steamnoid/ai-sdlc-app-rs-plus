//! The flag and the key read together, because a model asked for and not reachable is a run that
//! cannot answer anybody.
//!
//! **Two readers are two places to be consistent, and the thing they must agree about is one
//! decision.** A deployment that switched the flag on and supplied no key has asked for a model and
//! has no way to reach one — **so the pair is read once and refused together**, naming both, rather
//! than failing later at the far end in words this system did not write.

use aisdlc_outside::how_this_deployment_is_set_up::how_this_deployment_is_set_up;

/// What a deployment's environment held: the flag, and whether a key was there.
fn an_environment_holding<'a>(
    what_they_typed: Option<&'a str>,
    the_key: Option<&'a str>,
) -> (Option<&'a str>, Option<&'a str>) {
    (what_they_typed, the_key)
}

#[test]
fn a_deployment_that_asked_for_nothing_gets_nothing_and_asks_nobody() {
    let the_setup = how_this_deployment_is_set_up(an_environment_holding(None, None));

    assert!(
        !the_setup.may_a_model_rule().may_be_asked(),
        "**because a deployment that has said nothing gets no model at a gate** — and the default is the one where a person rules"
    );
}

#[test]
fn a_deployment_that_asked_for_a_model_and_gave_it_one_is_set_up() {
    let the_setup =
        how_this_deployment_is_set_up(an_environment_holding(Some("yes"), Some("sk-a-key")));

    assert!(
        the_setup.may_a_model_rule().may_be_asked(),
        "**and one that said yes with a key behind it gets what it asked for** — the whole of the flag, with nothing more"
    );
}

#[test]
fn a_deployment_that_asked_for_a_model_and_gave_it_nothing_is_refused_naming_both() {
    let the_setup = how_this_deployment_is_set_up(an_environment_holding(Some("yes"), None));

    let how_it_went = the_setup.why_it_cannot_answer_anybody();

    let Some(the_refusal) = how_it_went else {
        panic!("a flag asking for a model has no key behind it, so it can answer nobody");
    };
    let why_not = the_refusal.why_not();
    assert!(
        why_not.contains("yes") && why_not.contains("credential"),
        "**and the refusal names both halves of the one decision** — a caller reading only \"it is off\" would go and set the flag again, and a caller reading only \"there is no key\" would go and set a key on a deployment that never asked for a model: {why_not:?}"
    );
}

#[test]
fn the_key_is_never_printed_by_the_setup_that_holds_it() {
    let the_setup =
        how_this_deployment_is_set_up(an_environment_holding(Some("yes"), Some("sk-a-key")));

    let what_a_reader_sees = format!("{the_setup}");

    assert!(
        !what_a_reader_sees.contains("sk-a-key"),
        "**because the setup is a thing runs print, log and put in a commit message** — and a value that reaches a reader through a summary is a value that was never protected to begin with. What a reader sees is: {what_a_reader_sees}"
    );
    assert!(
        what_a_reader_sees.contains("a model") || what_a_reader_sees.contains("nobody"),
        "**and the summary still says what this deployment can do**, because a summary that says nothing is the same as no summary: {what_a_reader_sees}"
    );
}
