//! The desk starts a run, **and refuses before it copies anything when it cannot be configured.**
//!
//! **Cycle 131 read what a person typed and stopped there.** Everything this repository has is
//! still a library — **and a library with a front door and no run behind it is a door onto a room
//! with nothing in it.**
//!
//! **A desk that starts a run it could not configure has started nothing, and worse has copied a
//! repository.** So the collaborators are built first, every one of them is something a deployment
//! configured rather than something this system chose, **and a refusal here happens before the first
//! question reaches a model.**
//!
//! **There is no model in this file.** `WhereAModelIs` refuses an unset endpoint, a base rather than
//! an address, or a model nobody deployed — **and a desk that invented one would be a lie with a
//! position**, which is exactly what happened the first time the sibling ran.

use aisdlc_desk::the_run_is_started::{
    WhatThisDeskWasGiven, WhyThisRunCouldNotStart, the_run_is_started_with,
};

#[test]
fn a_desk_whose_model_cannot_be_reached_says_what_it_said_and_claims_no_page() {
    let mut the_run = the_collaborators_a_deployment_configured();

    let refused = the_run_is_started_with(&mut the_run);

    let why_not = match &refused {
        Err(why_not) => why_not.to_string(),
        Ok(said) => format!("a desk that reached nothing said it left a page: {said}"),
    };
    assert!(
        why_not.contains("the run did not finish"),
        "**because a desk that reported a page for a run that never got past its first agent is a \
         record that does not agree with the disk** — the same defect as the eighth gate being the \
         seventh decision again. It said: {why_not}"
    );
    assert!(
        !why_not.contains("left a page"),
        "**because the desk's whole output is where a person goes to read what happened** — and a \
         page claimed for a run that made none sends a reader to a file that is not there. It said: \
         {why_not}"
    );
}

#[test]
fn a_desk_with_no_model_configured_refuses_before_it_copies_anything() {
    let mut the_run = the_collaborators_a_deployment_configured();
    the_run.the_endpoint = None;

    let refused = the_run_is_started_with(&mut the_run);

    assert!(
        matches!(
            refused,
            Err(WhyThisRunCouldNotStart::NoModelWasConfigured { .. })
        ),
        "**because a desk that invented a model is a lie with a position** — and the first real run \\
         fell back to a local address nobody had read and lost the time to a connection refused. It \\
         refused: {refused:?}"
    );
}

#[test]
fn a_terminal_named_after_an_agent_is_refused_before_the_artifact_is_read() {
    let mut the_run = the_collaborators_a_deployment_configured();
    the_run.who_answers = String::from("aisdlc-pr");

    let refused = the_run_is_started_with(&mut the_run);

    assert!(
        matches!(
            refused,
            Err(WhyThisRunCouldNotStart::TheTerminalIsAMachine { .. })
        ),
        "**because a decider named in a machine's namespace cannot produce a ruling even to refuse \\
         with** — so a terminal named after an agent spends somebody's attention on a document a \\
         machine was going to be refused for. It refused: {refused:?}"
    );
}

/// Every collaborator a deployment configured, **and each one is a fact somebody typed** — the model
/// address, the model, the credential and the name at the terminal.
fn the_collaborators_a_deployment_configured() -> WhatThisDeskWasGiven {
    WhatThisDeskWasGiven {
        what_a_person_was_asked_for: what_a_person_was_asked_for(),
        the_endpoint: Some(String::from("https://a-model.example/v1/chat/completions")),
        which_model: Some(String::from("a-model-a-deployment-deployed")),
        the_key: Some(String::from("a-key-not-a-real-one")),
        who_answers: String::from("paligakrzychu"),
        which_organisation: Some(
            aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons::Openai,
        ),
    }
}

/// What a person typed at this desk, **and it is the same three arguments the front door refuses.**
fn what_a_person_was_asked_for() -> aisdlc_desk::what_a_run_was_asked_for::WhatARunWasAskedFor {
    aisdlc_desk::what_a_run_was_asked_for::what_a_run_was_asked_for(&[
        String::from("--repository"),
        String::from("steamnoid/wellbeing-tracker-public"),
        String::from("--projects-root"),
        where_this_test_keeps_its_projects(),
        String::from("--do"),
        String::from("untrack the sqlite sidecar files"),
    ])
    .expect("three arguments were given and all three can mean something")
}

/// Where this test keeps its projects, **and it is a directory of this process's own** because a
/// desk that wrote into somebody's home would be the defect this test exists beside.
fn where_this_test_keeps_its_projects() -> String {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-the-desk-starts-a-run-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&where_it_is);
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is.display().to_string()
}
