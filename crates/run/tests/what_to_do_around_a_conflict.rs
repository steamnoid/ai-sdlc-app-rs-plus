//! Three conflicts, three closed sets of answers, and a model choosing among them.
//!
//! **Free text is not a decision.** A conflict is the one point in a delivery where a person is
//! genuinely wanted — two branches cannot be joined, a proposal restates a filed issue, two runs
//! are touching one file — **and each of those has a small number of sensible answers that are the
//! same every time.** A model asked to "write what to do" will write a paragraph, **and a paragraph
//! is something a run cannot check, cannot record, and cannot tell from one day to the next whether
//! it did the same thing.**
//!
//! So the model **chooses a name and this system holds the set of names.** An answer outside the set
//! is refused rather than guessed, **because guessing here would go on to a rebase, a push or a
//! filing on somebody's board that nobody asked for.**

use aisdlc_run::what_to_do_around_a_conflict::{
    AConflict, AConflictNobodyCouldRuleOn, the_options_for, the_question_about,
    what_to_do_around_a_conflict,
};

/// The three situations, **and they are three and not one.**
fn every_situation() -> Vec<AConflict> {
    vec![
        AConflict::APullRequestConflicts,
        AConflict::AProposalIsAlreadyFiled,
        AConflict::TwoRunsTouchOneFile,
    ]
}

#[test]
fn every_situation_offers_exactly_its_own_answers() {
    let every_answer_of_each = vec![
        the_options_for(AConflict::APullRequestConflicts),
        the_options_for(AConflict::AProposalIsAlreadyFiled),
        the_options_for(AConflict::TwoRunsTouchOneFile),
    ];

    let all_three_differ = every_answer_of_each[0] != every_answer_of_each[1]
        && every_answer_of_each[1] != every_answer_of_each[2]
        && every_answer_of_each[0] != every_answer_of_each[2];

    assert!(
        all_three_differ,
        "**because the three are not the same decision in different words, and merging them would \
         make each of them worse** — an option set spanning all three would offer *leave it and say \
         why* for a code conflict, where the only honest answer is to fix it or not merge. They \
         offered: {every_answer_of_each:?}"
    );
}

#[test]
fn a_named_answer_is_the_decision_and_a_sentence_is_not() {
    let what_the_model_should_write = "leave_the_pull_request_and_record_why";

    let the_decision = what_to_do_around_a_conflict(
        AConflict::APullRequestConflicts,
        what_the_model_should_write,
    );

    assert_eq!(
        what_to_do_around_a_conflict(
            AConflict::APullRequestConflicts,
            "I would rebase it and let CI try again",
        ),
        Err(the_refusal_for(
            AConflict::APullRequestConflicts,
            "I would rebase it and let CI try again"
        )),
        "**because the second half of this test is the whole of why the set is closed**: a model \
         that wrote a paragraph has made no decision a run can carry out, **and refusing it is what \
         stops a run treating it as one.** The named answer gave: {the_decision:?}"
    );
}

#[test]
fn a_refusal_says_what_to_send_instead_and_not_only_what_was_wrong() {
    let the_refusal = the_refusal_for(AConflict::TwoRunsTouchOneFile, "push it whenever");

    assert!(
        the_refusal
            .to_string()
            .contains("push_the_change, leave_the_change_unpushed"),
        "**because a model told *that is not one of the options* and not told what the options are \
         will send the same sentence again** — and this repository has been burned by exactly that, \
         on an invalid JSON body and three times running. The refusal has to carry the answer. It \
         said: {the_refusal}"
    );
}

#[test]
fn an_answer_named_for_another_situation_is_refused() {
    let the_refusal = the_refusal_for(AConflict::AProposalIsAlreadyFiled, "push_the_change");

    assert!(
        the_refusal.about == AConflict::AProposalIsAlreadyFiled,
        "**because every option name is a sentence about a different thing** — `push_the_change` is \
         a true answer to whether two runs touch one file and no answer at all to whether a \
         proposal restates a filing, **and accepting it would carry an answer into a decision it \
         was never written for.** It was refused: {the_refusal}"
    );
}

#[test]
fn the_three_decisions_are_told_apart_by_the_type_and_not_by_a_tag() {
    let every_decision = vec![
        what_to_do_around_a_conflict(
            AConflict::APullRequestConflicts,
            "rebase_the_branch_and_try_again",
        ),
        what_to_do_around_a_conflict(
            AConflict::AProposalIsAlreadyFiled,
            "file_it_naming_the_filed_one",
        ),
        what_to_do_around_a_conflict(AConflict::TwoRunsTouchOneFile, "leave_the_change_unpushed"),
    ];

    let every_one_is_distinct = every_decision[0] != every_decision[1]
        && every_decision[1] != every_decision[2]
        && every_decision[0] != every_decision[2];

    assert!(
        every_one_is_distinct,
        "**because a caller that could reach a variant some other way could put a run into a state \
         its own rules never described** — and three situations that compared equal by value would \
         make that possible. They are: {every_decision:?}"
    );
}

#[test]
fn half_the_decisions_reach_the_world_and_half_do_not() {
    let the_ones_that_do = vec![
        "rebase_the_branch_and_try_again",
        "file_it_naming_the_filed_one",
        "push_the_change",
    ];
    let the_ones_that_do_not = vec![
        "leave_the_pull_request_and_record_why",
        "do_not_file_it",
        "leave_the_change_unpushed",
    ];

    let every_writes_as_it_should = the_ones_that_do
        .iter()
        .all(|one| one_writes_to_the_world(one) == Some(true))
        && the_ones_that_do_not
            .iter()
            .all(|one| one_writes_to_the_world(one) == Some(false));

    assert!(
        every_writes_as_it_should,
        "**because filing an issue, pushing a branch and retrying a merge all reach somebody else, \
         and leaving a pull request, not filing and not pushing do not** — a caller that could not \
         ask would have to enumerate all six at every call site. Which write: {the_ones_that_do:?} \
         and which do not: {the_ones_that_do_not:?}"
    );
}

#[test]
fn the_question_names_every_answer_in_full_and_warns_against_another_situations() {
    let the_question = the_question_about(
        AConflict::APullRequestConflicts,
        "the branch and main both changed the same function",
    );

    let names_both_answers = the_question.contains("rebase_the_branch_and_try_again")
        && the_question.contains("leave_the_pull_request_and_record_why");

    assert!(
        names_both_answers,
        "**because the answers are in the prompt and not only in the shape, and that is this \
         system's own rule** — a model shown a fixed set of strings in a tool's shape answers with \
         something outside it, and a prompt that does not repeat the answers does not help: the \
         answer is refused, the correction is sent, and the same sentence comes back. The question \
         was: {the_question}"
    );
}

#[test]
fn every_situation_has_a_question_and_there_are_exactly_three() {
    assert_eq!(
        (
            every_situation().len(),
            the_options_for(AConflict::APullRequestConflicts).len()
        ),
        (3, 2),
        "**because two answers each is the point** — three situations with a closed pair each is a \
         set a model can choose from, and three situations with open answers is three paragraphs."
    );
}

/// The refusal for one situation and one wrong answer.
fn the_refusal_for(about: AConflict, what_the_model_wrote: &str) -> AConflictNobodyCouldRuleOn {
    match what_to_do_around_a_conflict(about, what_the_model_wrote) {
        Err(the_refusal) => the_refusal,
        Ok(a_decision) => {
            panic!("{what_the_model_wrote} was accepted and it gave {a_decision:?}")
        }
    }
}

/// Whether this answer reaches somebody else, **and it is read out of the decision rather than
/// looked up in the test.**
///
/// **Every situation is asked, and not only the first**, because an answer name belongs to exactly
/// one of the three and a helper that looked in one place would call two thirds of them `false` for
/// the wrong reason — **which is a test that would have passed against a broken system.**
fn one_writes_to_the_world(what_the_model_wrote: &str) -> Option<bool> {
    let mut what_it_said = None;
    for about in every_situation() {
        if let Ok(the_decision) = what_to_do_around_a_conflict(about, what_the_model_wrote) {
            what_it_said = Some(the_decision.does_it_reach_the_world());
        }
    }
    what_it_said
}
