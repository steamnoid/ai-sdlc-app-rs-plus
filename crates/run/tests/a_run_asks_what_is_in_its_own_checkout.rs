//! A run that has made a checkout asks what is in it — and only ever about that checkout.
//!
//! **A run makes a checkout and nothing has ever read one.** Every project this repository has
//! asked about was this repository's own working tree, so two facts that must agree were never
//! asked to agree: the checkout knows where it landed, and the project knew nothing at all.
//!
//! **The refusal is the whole of it.** A caller can hand a checkout and a project rooted
//! somewhere else, and a run that lists the project without asking whether they are the same
//! place hands an agent a listing of a directory it never cloned — **and the agent cannot tell,
//! because a listing is a listing.**

use std::path::{Path, PathBuf};

use aisdlc_domain::a_clone::ACloneOnDisk;
use aisdlc_run::ThePathInsideTheProject;
use aisdlc_run::a_run_asks_what_is_in_its_own_checkout::the_run_asks_what_is_in_its_own_checkout;
use aisdlc_run::the_project_on_disk::TheProjectOnDisk;

/// A checkout at one directory, which is what a run that cloned something holds.
fn a_checkout_at(where_it_lands: &str) -> ACloneOnDisk {
    ACloneOnDisk::the_clone_at(
        PathBuf::from(where_it_lands),
        aisdlc_domain::a_clone::TheCommit::try_from("e1c2d3").expect("a commit"),
        aisdlc_domain::a_clone::TheRef::try_from("HEAD").expect("a ref"),
    )
}

/// A project rooted somewhere, which may or may not be the checkout's own place.
struct AProjectAt {
    where_it_lives: PathBuf,
    what_it_holds: Vec<String>,
}

impl TheProjectOnDisk for AProjectAt {
    /// Where it lives, and a project has a place.
    fn where_it_lives(&self) -> &Path {
        &self.where_it_lives
    }

    /// What is in it, as project-relative names.
    fn what_is_in_it(&mut self) -> Vec<String> {
        self.what_it_holds.clone()
    }

    /// The text of a file, and nothing when it holds none.
    fn the_text_of(&mut self, _a_path: &ThePathInsideTheProject) -> Option<String> {
        None
    }
}

fn a_project_rooted_at(where_it_lives: &str) -> AProjectAt {
    AProjectAt {
        where_it_lives: PathBuf::from(where_it_lives),
        what_it_holds: vec!["README.md".to_string()],
    }
}

#[test]
fn a_run_asks_about_the_checkout_it_made() {
    let mut the_project = a_project_rooted_at("/projects/a/run/repository");

    let what_is_in_it = the_run_asks_what_is_in_its_own_checkout(
        &a_checkout_at("/projects/a/run/repository"),
        &mut the_project,
    );

    assert_eq!(
        what_is_in_it
            .expect("a project rooted where the run cloned is the run's own")
            .len(),
        1,
        "**because a run that made a checkout and then listed a different directory has answered about a repository it never cloned** — and the listing is a listing, so the agent cannot tell"
    );
}

#[test]
fn a_project_rooted_somewhere_else_is_refused_rather_than_listed() {
    let mut a_project_somewhere_else = a_project_rooted_at("/projects/a/different/run/repository");

    let what_happened = the_run_asks_what_is_in_its_own_checkout(
        &a_checkout_at("/projects/a/run/repository"),
        &mut a_project_somewhere_else,
    );

    assert!(
        what_happened.is_err(),
        "**because two places that must agree and do not are a silent wrong answer, and this one is silent in the worst way** — every file name would be a real file of some project, and the agent would read the wrong repository's README believing it read its own"
    );
}

#[test]
fn the_reason_a_project_was_refused_names_both_places() {
    let mut a_project_somewhere_else = a_project_rooted_at("/projects/a/different/run/repository");

    let why_not = the_run_asks_what_is_in_its_own_checkout(
        &a_checkout_at("/projects/a/run/repository"),
        &mut a_project_somewhere_else,
    )
    .expect_err("two different places is a refusal")
    .because();

    assert!(
        why_not.contains("/projects/a/run/repository") && why_not.contains("different"),
        "**because a refusal naming only one of the two places leaves the reader to guess which was wrong** — and both are facts this system holds, so a refusal quoting neither is a refusal about nothing"
    );
}
