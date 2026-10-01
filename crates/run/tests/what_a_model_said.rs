//! What a run says while it is doing it, one line at a time.
//!
//! **Every line carries the elapsed time, because a label alone says nothing while the thing it
//! labels is still happening.** `planning` says a call is outstanding; `planned in 41s` says the model
//! answered — **and a reader watching a run cannot tell the two apart without it.**
//!
//! **The size of a question and never its text.** A 400-character prompt and a 40 000-character one are
//! two different lengths of wait, **and the number tells them apart without putting a repository's
//! contents into somebody's terminal.**
//!
//! **A refusal names the artifact and the field, and the model is an argument.** The model is the same
//! on every call, **so the model cannot stand in for which artifact was refused** — and a line naming
//! the wrong model is worse than no line, **because the question the line exists to answer is which
//! model was that.**

use aisdlc_run::what_a_model_said::{AWhomARunTellsWhatItIsDoing, HowLongItTook, WhatAModelSaid};

/// Somewhere a run's lines go, **and it keeps them because a run that overwrites answers *where is
/// it now* and loses the rest.**
struct SomewhereThatKeepsEveryLine {
    what_it_was_told: Vec<String>,
}

impl AWhomARunTellsWhatItIsDoing for SomewhereThatKeepsEveryLine {
    fn it_is_told(&mut self, what_the_run_said: &WhatAModelSaid) {
        self.what_it_was_told.push(what_the_run_said.as_a_line());
    }
}

/// The artifact this call is about.
fn the_artifact() -> aisdlc_domain::approval::WhichArtifact {
    aisdlc_domain::approval::WhichArtifact::try_from("DiscoveryReport").expect("a name")
}

/// Which model answered, **told to it and never carried by this module.**
fn the_model() -> String {
    String::from("space-bunny-free")
}

#[test]
fn a_call_still_outstanding_says_so_and_one_that_answered_says_how_long_it_took() {
    let mut somewhere = SomewhereThatKeepsEveryLine {
        what_it_was_told: Vec::new(),
    };
    let outstanding = WhatAModelSaid::a_call_is_outstanding(the_artifact(), 40_000, the_model());

    somewhere.it_is_told(&outstanding);
    somewhere.it_is_told(&outstanding.answered_after(HowLongItTook(41_032)));

    assert_eq!(
        somewhere.what_it_was_told,
        vec![
            String::from("planning DiscoveryReport for space-bunny-free — 40000 characters asked"),
            String::from("planned DiscoveryReport in 41.0s for space-bunny-free"),
        ],
        "**because a label alone says nothing while the thing it labels is still happening** — and a \
         reader watching a run cannot tell *waiting* from *answered* without it: {:?}",
        somewhere.what_it_was_told
    );
}

#[test]
fn a_line_carries_the_size_of_a_question_and_never_a_word_of_it() {
    let a_question_nobody_should_see = "the private text of somebody's repository ".repeat(40);

    let the_line = WhatAModelSaid::a_call_is_outstanding(
        the_artifact(),
        a_question_nobody_should_see.len(),
        the_model(),
    )
    .as_a_line();

    assert!(
        !the_line.is_empty()
            && the_line.contains(&a_question_nobody_should_see.len().to_string())
            && !the_line.contains("private"),
        "**because a line is written where anybody can read it, and a repository's contents in \
         somebody's terminal is a leak the run did not ask to make** — the size alone tells a 400 \
         from a 40 000, which is the only thing the number is for. **And an empty line passes this \
         by saying nothing, which is why the line has to carry the size as well:** {the_line:?}"
    );
}

#[test]
fn a_refusal_names_the_artifact_and_the_field_and_not_only_the_model() {
    let refused = WhatAModelSaid::a_call_is_outstanding(the_artifact(), 400, the_model())
        .refused_because_the_field("offers[0].paths was a sentence and not a list");

    assert!(
        refused.as_a_line().contains("offers[0].paths")
            && refused.as_a_line().contains("DiscoveryReport"),
        "**because the model is the same on every call, so the model cannot stand in for which \
         artifact was refused or which field was wrong** — and a line naming only the model answers \
         none of the questions it exists to answer: {}",
        refused.as_a_line()
    );
}

#[test]
fn the_model_a_line_names_is_the_one_it_was_given_and_not_one_this_module_carries() {
    let one_run = WhatAModelSaid::a_call_is_outstanding(the_artifact(), 400, the_model());
    let another_run =
        WhatAModelSaid::a_call_is_outstanding(the_artifact(), 400, String::from("a-local-model"));

    assert!(
        one_run.as_a_line().contains("space-bunny-free")
            && another_run.as_a_line().contains("a-local-model"),
        "**because the model is an argument and not a constant, and a line naming the wrong model is \
         worse than no line at all** — this is how a comparison between two models is read later: {} \
         and {}",
        one_run.as_a_line(),
        another_run.as_a_line()
    );
}

#[test]
fn a_run_that_overwrites_its_lines_answers_where_it_is_now_and_loses_the_rest() {
    let mut somewhere = SomewhereThatKeepsEveryLine {
        what_it_was_told: Vec::new(),
    };
    let first = WhatAModelSaid::a_call_is_outstanding(the_artifact(), 400, the_model());

    somewhere.it_is_told(&first);
    somewhere.it_is_told(&first);

    assert_eq!(
        somewhere.what_it_was_told.len(),
        2,
        "**because a run's lines are appended and never overwritten** — and a run that overwrites \
         answers *where is it now* and loses the rest, which is the whole history a reader of a stuck \
         run needs: {:?}",
        somewhere.what_it_was_told
    );
}
