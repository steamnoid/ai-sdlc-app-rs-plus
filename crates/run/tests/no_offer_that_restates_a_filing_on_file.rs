//! An offer that restates a filing already on file, and is not filed again.
//!
//! **A run that files what is already filed is the definition of a run nobody asked for twice.** Five
//! runs of the real tier filed the same SQLite observation three times, **because the run read the
//! repository and never read the issue tracker** — and the tracker is where a person already said the
//! same thing.
//!
//! **The run reads the filings already on file and puts them in the question**, because an agent
//! cannot avoid restating a filing it was never shown.
//!
//! **The model says which filings it weighed and which its offer restates**, and **the run holds it to
//! that.** The judgement of what counts as a duplicate is a model's; **the rule about what happens
//! afterwards is not.**

use aisdlc_run::what_a_model_said_would_restate::WhatAModelSaidWouldRestate;

/// A filing already on file, **which is a number and a title and nothing else.**
fn a_filing_already_on_file() -> aisdlc_run::the_filings_already_on_file::AFilingAlreadyOnFile {
    aisdlc_run::the_filings_already_on_file::AFilingAlreadyOnFile::that_is_open(
        15,
        "Make the E2E SQLite database hermetic and remove WAL/SHM artifacts",
    )
}

#[test]
fn an_offer_restating_a_filing_on_file_names_it_and_is_not_filed() {
    let what_it_would_restate = WhatAModelSaidWouldRestate::the_filing_already_on_file(15);

    let how_it_went = what_it_would_restate.how_the_run_answers(a_filing_already_on_file());

    assert!(
        how_it_went.is_not_filed_because_it_restates(),
        "**because an offer that restates a filing already on file has nothing to add** — and filing \\
         it again is a second page asking a person the question they were already asked, **in the \\
         place they already answered it:** {how_it_went:?}"
    );
}

#[test]
fn an_offer_restating_a_filing_that_is_not_on_file_is_refused_rather_than_filed() {
    let what_it_would_restate = WhatAModelSaidWouldRestate::the_filing_already_on_file(99);

    let how_it_went = what_it_would_restate.how_the_run_answers(a_filing_already_on_file());

    assert!(
        how_it_went.is_refused_because_it_named_a_filing_that_is_not_on_file(),
        "**because a duplicate of a filing that does not exist is a model reporting a file it never \\
         read** — and filing it would put a number nobody can open into a filing, which is the same \\
         forged address `CommissionedIssue` used to compose: {how_it_went:?}"
    );
}

#[test]
fn an_offer_restating_nothing_is_filed_and_that_is_the_usual_case() {
    let what_it_would_restate = WhatAModelSaidWouldRestate::nothing_already_on_file();

    let how_it_went = what_it_would_restate.how_the_run_answers(a_filing_already_on_file());

    assert!(
        how_it_went.is_filed(),
        "**because almost every real finding is new** — and a rule that refused offers mentioning any \
         filing would stop the run filing anything at all, which is a run that analyses and proposes \\
         nothing: {how_it_went:?}"
    );
}
