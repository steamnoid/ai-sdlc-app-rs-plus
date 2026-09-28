//! The four organisations that reason, and the refusal for a fifth.
//!
//! **`WhichOrganisationReasons` is a closed set of four and distinct from `AReasoningModel`.**
//! The model is the thing that reasons; this is the company it works for. **A run that asks a
//! model and cannot say who answered has an answer nobody can hold anybody to** — and the
//! glossary has named this set since before the port existed, so the port shipped without it.
//!
//! **A fifth organisation is refused rather than admitted.** The specification lists four, and
//! a set that grows is a set somebody has to keep in step with a list of providers; the whole
//! point of the closed set is that it is closed.

use aisdlc_domain::which_organisation_reasons::{
    EVERY_ORGANISATION_THAT_REASONS, WhichOrganisationReasons,
};

#[test]
fn the_glossaries_four_are_the_four_the_code_writes() {
    let written: Vec<&str> = EVERY_ORGANISATION_THAT_REASONS
        .iter()
        .map(|one| one.as_str())
        .collect();

    assert_eq!(
        written,
        vec!["openai", "ollama", "openai_compatible", "openai_responses"],
        "the four the specification lists, in the order it lists them"
    );
}

#[test]
fn an_organisation_reasons_under_the_name_the_specification_gives() {
    let the_organisation =
        WhichOrganisationReasons::try_from("openai_compatible").expect("that is one of the four");

    assert_eq!(
        the_organisation.as_str(),
        "openai_compatible",
        "and reads back exactly what it was given, because a reader who cannot see which spelling a provider uses cannot compare two answers from two of them"
    );
}

#[test]
fn an_organisation_that_is_not_one_of_the_four_is_refused() {
    let refusal = WhichOrganisationReasons::try_from("anthropic");

    assert!(
        refusal.is_err(),
        "**a closed set that grows is a set somebody has to keep in step with a list of providers** — and the refusal names what was asked for, so the reader knows which one was refused rather than that something was"
    );
}

#[test]
fn a_blank_is_refused_before_the_question_of_which_four() {
    assert!(
        WhichOrganisationReasons::try_from("  ").is_err(),
        "because a blank is not one of the four and is not a blank name for one of them either"
    );
}
