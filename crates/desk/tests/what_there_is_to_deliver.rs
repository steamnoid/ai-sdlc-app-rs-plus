//! What there is to deliver, **built from what the desk was given and nothing else.**
//!
//! **Cycle 134 signs the delivery with the deployment's key.** The desk built what there is to
//! deliver beside what it was given, with a key from its own tests — **so a run a deployment
//! configured delivered with a key nobody deployed**, and a branch pushed that way authenticates
//! as nobody.

use aisdlc_desk::what_the_process_told_this_desk::what_the_process_told_this_desk;
use aisdlc_outside::the_environment::TheEnvironment;

#[test]
fn the_delivery_acts_with_the_key_a_deployment_supplied() {
    let given = the_desk_as_it_was_told(&the_environment_where_everything_is_set());
    let what_there_is = aisdlc_desk::the_run_is_started::what_there_is_to_deliver(&given)
        .expect("a deployment supplied a key, and a delivery without it authenticates as nobody");
    let the_key = aisdlc_run::a_credential::ACredential::the_key_a_deployment_supplied(
        "a-key-not-a-real-one",
    )
    .expect("the test key is a key");

    assert!(
        what_there_is.the_key() == &the_key,
        "**because a branch pushed with a key nobody deployed authenticates as nobody** — and a \
         delivery carrying the desk's own test key delivers somebody else's run."
    );
}

#[test]
fn no_key_supplied_is_no_delivery_rather_than_a_test_key() {
    let given = the_desk_as_it_was_told(&the_environment_where_no_key_is_supplied());
    let refused = aisdlc_desk::the_run_is_started::what_there_is_to_deliver(&given);

    assert!(
        matches!(
            refused,
            Err(aisdlc_desk::the_run_is_started::WhyThisRunCouldNotStart::NoKeyWasSupplied)
        ),
        "**because a delivery that cannot authenticate cannot be built** — and a test key in its \
         place is a signature nobody can answer for."
    );
}

/// The desk as it was told, **typed and set together or not at all.**
fn the_desk_as_it_was_told(
    the_environment: &TheEnvironment,
) -> aisdlc_desk::the_run_is_started::WhatThisDeskWasGiven {
    let told = what_the_process_told_this_desk(&what_a_person_typed(), the_environment);
    told.expect("three arguments were typed")
}

/// What a person typed, **and it is the same three arguments the front door refuses.**
fn what_a_person_typed() -> Vec<String> {
    vec![
        String::from("--repository"),
        String::from("steamnoid/wellbeing-tracker-public"),
        String::from("--projects-root"),
        where_this_test_keeps_its_projects(),
        String::from("--do"),
        String::from("untrack the sqlite sidecar files"),
    ]
}

/// Where this test keeps its projects, **and it is a directory of this process's own** because a
/// desk that wrote into somebody's home would be the defect this test exists beside.
fn where_this_test_keeps_its_projects() -> String {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-what-there-is-to-deliver-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&where_it_is);
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is.display().to_string()
}

/// The environment where everything is set, **as a value and never as a reading.**
fn the_environment_where_everything_is_set() -> TheEnvironment {
    the_environment_where_a_key_is(Some("a-key-not-a-real-one"))
}

/// The environment where no key is supplied, **and no delivery can authenticate.**
fn the_environment_where_no_key_is_supplied() -> TheEnvironment {
    the_environment_where_a_key_is(None)
}

/// The environment with the key it was given, **and nothing when it was given none.**
fn the_environment_where_a_key_is(the_key: Option<&str>) -> TheEnvironment {
    let mut what_it_held = vec![
        (
            String::from("LLM_ENDPOINT"),
            String::from("https://a-model.example/v1/chat/completions"),
        ),
        (
            String::from("LLM_MODEL"),
            String::from("a-model-a-deployment-deployed"),
        ),
        (
            String::from("AISDLC_WHO_DECIDES"),
            String::from("paligakrzychu"),
        ),
        (String::from("LLM_ORGANISATION"), String::from("ollama")),
    ];
    if let Some(the_key) = the_key {
        what_it_held.push((String::from("OPENAI_API_KEY"), String::from(the_key)));
    }
    TheEnvironment::of(what_it_held)
}
