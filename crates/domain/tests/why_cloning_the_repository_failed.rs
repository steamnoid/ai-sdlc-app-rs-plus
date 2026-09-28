//! Why cloning a repository failed, in four ways, and the caller can tell them apart.
//!
//! **Two of the four are this system's own findings**, and both are about a value that was read
//! out of a command that failed: a directory that already holds something, and a repository
//! somebody created and has not committed to.
//!
//! **"No such repository" quotes git's own words inside ours**, because the caller has to tell
//! *no such repository* from *git is unhappy* from *git is not installed*. Three things that
//! feel alike and are not — and a refusal that collapses them makes the caller guess.
//!
//! **The fourth is a reading and not a quotation.** The glossary says "four ways" and names
//! three; the fourth is named inside the second one's description, as the thing a caller must
//! tell apart. I have read it that way and said so, because inventing a fourth the document
//! does not describe would be writing the specification backwards.

use aisdlc_domain::why_cloning_the_repository_failed::{
    WhichWayItFailed, why_cloning_the_repository_failed,
};

#[test]
fn there_are_four_ways_and_the_refusal_names_which() {
    let every_one = WhichWayItFailed::every_way();

    assert_eq!(
        every_one.len(),
        4,
        "**because the glossary says four**, and a refusal that names three is a refusal whose fourth case arrives wearing the third one's name"
    );
}

#[test]
fn no_such_repository_carries_gits_own_words() {
    let refusal = why_cloning_the_repository_failed(
        WhichWayItFailed::NoSuchRepository,
        "fatal: repository 'https://github.com/steamnoid/nope' not found",
    );

    assert!(
        refusal.what_git_said().contains("not found"),
        "**because a caller has to tell no such repository from git being unhappy**, and that telling needs git's own words rather than our paraphrase of them"
    );
}

#[test]
fn two_ways_that_feel_alike_are_not_the_same_way() {
    let not_there =
        why_cloning_the_repository_failed(WhichWayItFailed::NoSuchRepository, "not found");
    let unhappy = why_cloning_the_repository_failed(
        WhichWayItFailed::GitIsUnhappy,
        "could not resolve host: github.com",
    );

    assert_ne!(
        not_there.which_way(),
        unhappy.which_way(),
        "**because a caller has to tell them apart, and a refusal that collapses them leaves the caller guessing which of the two happened** — one is a wrong address and the other is a network, and the fixes are not the same"
    );
}

#[test]
fn a_repository_nobody_committed_to_is_not_a_repository_that_is_not_there() {
    let not_there =
        why_cloning_the_repository_failed(WhichWayItFailed::NoSuchRepository, "not found");
    let empty = why_cloning_the_repository_failed(
        WhichWayItFailed::NothingCommittedToItYet,
        "warning: You appear to have cloned an empty repository.",
    );

    assert_ne!(
        not_there.which_way(),
        empty.which_way(),
        "**because somebody created a repository and has not committed to it, and that is a real repository with nothing in it** — the same address, two different facts, and a value read out of a command that failed is that command's complaint wearing a field's name"
    );
}

#[test]
fn a_destination_that_already_holds_something_is_refused_on_principle() {
    let the_destination_holds_something = why_cloning_the_repository_failed(
        WhichWayItFailed::TheDestinationAlreadyHoldsSomething,
        "",
    );

    assert_eq!(
        the_destination_holds_something.which_way(),
        WhichWayItFailed::TheDestinationAlreadyHoldsSomething,
        "**because a run resumes and the directory is derived** — a refusal that arrives after the damage is not a refusal, so this one is a decision made before git was run at all"
    );
}

#[test]
fn the_first_two_ways_are_this_systems_own_findings_and_the_last_two_are_gits() {
    let the_ways = WhichWayItFailed::every_way();

    assert_eq!(
        the_ways,
        vec![
            WhichWayItFailed::TheDestinationAlreadyHoldsSomething,
            WhichWayItFailed::NoSuchRepository,
            WhichWayItFailed::GitIsUnhappy,
            WhichWayItFailed::NothingCommittedToItYet,
        ],
        "**because the glossary says the last two are this system's own findings and the first two are git's** — the order is the document's, and a reader who cannot tell which half is ours cannot tell which half to fix"
    );
}
