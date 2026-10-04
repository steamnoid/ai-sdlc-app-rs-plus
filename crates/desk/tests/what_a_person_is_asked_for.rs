//! What a person is asked for, **and the refusal when what they said cannot mean anything.**
//!
//! **This is the only place that decides what a person has to type**, and the sibling says the same
//! about its own front door — **so the claim is a fact about the system and not about this file, and
//! it is a fact that has to be tested.** A second file that decided what to type would be a second
//! answer to one question.
//!
//! **Every refusal names the argument.** A person who typed `owner/name` where `owner` was wanted
//! cannot find the mistake by reading four sentences about a run, **and a front door that explains
//! itself at length is a front door nobody reads.**
//!
//! **A run that was not told where it may write is not started anywhere on a person's disk**, so
//! the place is an argument and not a default — **and a run that was not told what to work on has
//! nothing to ask an agent about.**

use aisdlc_desk::what_a_run_was_asked_for::{
    WhatWasAskedForThatCannotMeanAnything, what_a_run_was_asked_for,
};

#[test]
fn a_repository_a_place_to_write_and_a_thing_to_do_is_asked_for() {
    let asked_for = what_a_run_was_asked_for(&[
        String::from("--repository"),
        String::from("steamnoid/wellbeing-tracker-public"),
        String::from("--projects-root"),
        String::from("/tmp/projects"),
        String::from("--do"),
        String::from("untrack the sqlite sidecar files"),
    ]);

    assert_eq!(
        asked_for
            .as_ref()
            .map(|asked| asked.which_repository().owner.clone()),
        Ok(String::from("steamnoid")),
        "**because this is the only place that decides what a person has to type** — and a run that \\
         read its repository anywhere else would be a second answer to that question. It asked for \\
         {asked_for:?}"
    );
    assert_eq!(
        asked_for
            .as_ref()
            .map(|asked| asked.where_it_may_write().to_path_buf()),
        Ok(std::path::PathBuf::from("/tmp/projects")),
        "**because a run that was not told where it may write is a run that writes wherever it \\
         happened to be started** — and that is somebody's home directory. It asked for {asked_for:?}"
    );
}

#[test]
fn what_the_person_typed_reaches_the_run_in_their_own_words() {
    let asked_for = what_a_run_was_asked_for(&[
        String::from("--repository"),
        String::from("steamnoid/wellbeing-tracker-public"),
        String::from("--projects-root"),
        String::from("/tmp/projects"),
        String::from("--do"),
        String::from("untrack the sqlite sidecar files"),
    ]);

    let what_they_typed = asked_for
        .as_ref()
        .map(|asked| asked.what_it_should_do().clone());
    let what_they_typed =
        what_they_typed.map(|request| request.what_was_said_besides().to_string());
    assert_eq!(
        what_they_typed,
        Ok(String::from(
            "change what is described in untrack the sqlite sidecar files"
        )),
        "**because the request is what somebody asked for in the words they used** — and a run that \\
         paraphrased it was answering a different question than the one that was put. It asked for \\
         {what_they_typed:?}"
    );
}

#[test]
fn a_repository_that_is_not_an_owner_and_a_name_is_refused_by_naming_the_argument() {
    let refused = what_a_run_was_asked_for(&[
        String::from("--repository"),
        String::from("wellbeing-tracker-public"),
        String::from("--projects-root"),
        String::from("/tmp/projects"),
        String::from("--do"),
        String::from("something"),
    ]);

    assert_eq!(
        refused,
        Err(WhatWasAskedForThatCannotMeanAnything::NotARepository {
            what_was_said: String::from("wellbeing-tracker-public"),
        }),
        "**because a front door that explains itself at length is a front door nobody reads** — the \\
         refusal names the argument and says what shape it wants. It refused: {refused:?}"
    );
}

#[test]
fn nothing_at_all_names_every_argument_a_run_needs() {
    let refused = what_a_run_was_asked_for(&[]);

    assert!(
        matches!(
            refused,
            Err(WhatWasAskedForThatCannotMeanAnything::NothingWasAskedFor { .. })
        ),
        "**because a person who typed one argument and was told that one was missing would be right \\
         about it and still not know what else to type** — so the refusal carries all three. It \\
         refused: {refused:?}"
    );
    let why_not = match refused {
        Err(why_not) => why_not.to_string(),
        Ok(asked_for) => format!("a run asked for nothing was started: {asked_for:?}"),
    };
    for one_flag in ["--repository", "--projects-root", "--do"] {
        assert!(
            why_not.contains(one_flag),
            "**because a refusal that names one of three missing arguments is a refusal the person \\
             has to run the command a second time to learn the rest of** — and `{why_not}` does not \\
             name {one_flag}"
        );
    }
}

#[test]
fn a_run_with_nowhere_to_write_is_refused_before_it_starts() {
    let refused = what_a_run_was_asked_for(&[
        String::from("--repository"),
        String::from("steamnoid/wellbeing-tracker-public"),
        String::from("--do"),
        String::from("something"),
    ]);

    assert!(
        matches!(
            refused,
            Err(WhatWasAskedForThatCannotMeanAnything::NowhereToWrite { .. })
        ),
        "**because a run that was not told where it may write is a run that writes wherever it \\
         happened to be started** — and this desk is the only place that can stop that, because \\
         nothing downstream is told either. It refused: {refused:?}"
    );
}

#[test]
fn a_run_with_nothing_to_do_is_refused_because_it_has_nothing_to_ask_about() {
    let refused = what_a_run_was_asked_for(&[
        String::from("--repository"),
        String::from("steamnoid/wellbeing-tracker-public"),
        String::from("--projects-root"),
        String::from("/tmp/projects"),
    ]);

    assert!(
        matches!(
            refused,
            Err(WhatWasAskedForThatCannotMeanAnything::NothingToDo { .. })
        ),
        "**because a run that does not know what to do has nothing to ask an agent about** — and \\
         seven agents asked about nothing would produce seven documents nobody commissioned. It \\
         refused: {refused:?}"
    );
}
