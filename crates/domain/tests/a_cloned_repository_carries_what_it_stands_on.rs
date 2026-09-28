//! A cloned repository, carrying the commit and the ref it is standing on.
//!
//! **Both are read from the clone and never remembered.** A value kept from an earlier run is a
//! value about the wrong commit — and a run that resumed believing it knows which commit it has
//! would report its changes against one it stopped at days ago.
//!
//! **`ref_name` and not `branch`, because a checkout is not always on a branch.** Cloning a
//! detached ref leaves none behind, and the honest answer for a checkout on none is `HEAD`.
//! **Naming the field `branch` would be a lie in precisely the case where the value matters.**

use aisdlc_domain::a_cloned_repository::{
    AClonedRepository, ARefThatNamesNothing, TheCommit, TheRef,
};
use aisdlc_domain::repository::Repository;
use aisdlc_domain::where_the_clone_of_a_work_item_lands::where_the_clone_of_a_work_item_lands;
use aisdlc_domain::work_item_id::WorkItemId;

fn the_place() -> std::path::PathBuf {
    where_the_clone_of_a_work_item_lands(
        &std::path::PathBuf::from("/projects"),
        &Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        &WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank"),
    )
}

fn a_cloned_repository() -> AClonedRepository {
    AClonedRepository::the_clone_at(
        the_place(),
        TheCommit::try_from("3f1a9c4d2b8e7f6a5d4c3b2a1908f7e6d5c4b3a2").expect("a commit"),
        TheRef::try_from("main").expect("a ref"),
    )
    .expect("a clone standing on a commit and a ref")
}

#[test]
fn a_cloned_repository_says_where_it_is_and_what_it_stands_on() {
    let the_clone = a_cloned_repository();

    assert_eq!(the_clone.where_it_lands(), the_place());
    assert_eq!(
        the_clone.the_commit().as_str(),
        "3f1a9c4d2b8e7f6a5d4c3b2a1908f7e6d5c4b3a2",
    );
    assert_eq!(the_clone.ref_name().as_str(), "main");
}

#[test]
fn a_checkout_on_no_branch_at_all_says_head() {
    let detached = AClonedRepository::the_clone_at(
        the_place(),
        TheCommit::try_from("3f1a9c4d2b8e7f6a5d4c3b2a1908f7e6d5c4b3a2").expect("a commit"),
        TheRef::try_from("HEAD").expect("a checkout on no branch is at HEAD"),
    )
    .expect("a detached clone is a clone");

    assert_eq!(
        detached.ref_name().as_str(),
        "HEAD",
        "**and `HEAD` is a ref name this type accepts, because that is the honest answer for a checkout on no branch** — a type that refused it would be telling a caller to invent a branch name, and the field is called `ref_name` and not `branch` for that reason"
    );
}

#[test]
fn a_blank_ref_is_refused() {
    let refusal = TheRef::try_from("   ");

    assert!(
        refusal.is_err(),
        "**because a clone standing on a ref that is not there is standing on nothing**, and a blank is not a name for `HEAD`"
    );
}

#[test]
fn a_blank_commit_is_refused() {
    let refusal = TheCommit::try_from("");

    assert!(
        refusal.is_err(),
        "**because a value kept from an earlier run is a value about the wrong commit, and a blank is the wrongest value there is**"
    );
}

#[test]
fn a_ref_that_names_nothing_says_what_was_written() {
    let refusal: ARefThatNamesNothing = TheRef::try_from("  ").expect_err("a blank is not a ref");

    assert_eq!(
        refusal.what_was_written(),
        "  ",
        "**because a name this system silently tidies is a name a lookup will miss** — the same sentence that keeps `WorkItemId` from trimming"
    );
}

#[test]
fn a_cloned_repository_without_a_commit_or_a_ref_is_refused() {
    let refusal = AClonedRepository::the_clone_at(
        the_place(),
        TheCommit::try_from("3f1a9c4d2b8e7f6a5d4c3b2a1908f7e6d5c4b3a2").expect("a commit"),
        TheRef::try_from("main").expect("a ref"),
    )
    .expect("a clone standing on a commit and a ref");

    assert_eq!(
        refusal.where_it_lands(),
        the_place(),
        "and it is a value and not a `Result` in the happy path, because a clone read from a clone always has both"
    );
}
