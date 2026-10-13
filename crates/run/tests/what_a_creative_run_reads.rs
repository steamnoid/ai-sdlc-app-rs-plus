//! What a creative run reads from its own checkout, **before any model is asked.**
//!
//! **Cycle 135 reads the copy before the question.** The checkout exists, the listing exists,
//! the filing exists — **and nothing joins the copy to the listing**, so a creative run that
//! filed issues reasoned about a directory somebody else filled, or about none at all.
//!
//! **The copy comes first and the listing is read off the copy.** A run that asked a model
//! about a repository before copying it reasons about a description, which is the defect the
//! checkout exists to prevent.

use aisdlc_run::a_program_we_run::WhatTheProgramSaid;
use aisdlc_run::what_a_creative_run_reads::{
    ACheckoutThatCouldNotBeRead, what_a_creative_run_reads_from_its_own_checkout,
};

/// The commit the copy stands on, as git wrote it.
const THE_COMMIT: &str = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8";

/// The ref the copy stands on.
const THE_REF: &str = "main";

/// The address the code was written in, which may only be read.
const THE_ADDRESS_THE_CODE_WAS_WRITTEN_IN: &str =
    "https://github.com/steamnoid/wellbeing-tracker-public";

#[test]
fn a_creative_run_reads_what_is_in_the_checkout_it_made() {
    let where_it_lands = the_destination_nothing_stands_at("reads-what-is-in-it");
    let mut the_project =
        AProjectHoldingAListing::holding(where_it_lands.clone(), vec![String::from("README.md")]);
    let mut the_program = AProgramThatCopies::copying_everything();

    let read = what_a_creative_run_reads_from_its_own_checkout(
        &mut the_program,
        &mut the_project,
        &the_request_to_clone_into(&where_it_lands),
    )
    .expect("nothing stood at the destination and git answered");

    assert_eq!(
        read.what_it_saw(),
        &[String::from("README.md")],
        "**because offers grounded in another directory's names file issues about files nobody \
         holds** — and the grounding check compares against what this run really saw."
    );
    assert_eq!(
        read.the_checkout().where_it_lands(),
        where_it_lands.as_path(),
        "**because a listing without its copy is a directory nobody can point at** — and the \
         works that follow read the repository through this checkout."
    );
}

#[test]
fn a_destination_that_holds_something_is_not_cloned_over() {
    let where_it_lands = the_destination_something_stands_at("holds-something");
    let mut the_project = AProjectHoldingAListing::holding(where_it_lands.clone(), vec![]);
    let mut the_program = AProgramThatCopies::copying_everything();

    let refused = what_a_creative_run_reads_from_its_own_checkout(
        &mut the_program,
        &mut the_project,
        &the_request_to_clone_into(&where_it_lands),
    );

    assert!(
        matches!(
            refused,
            Err(ACheckoutThatCouldNotBeRead::TheRepositoryWasNeverCopied { .. })
        ),
        "**because a run that cloned over a resumed run's copy would ask about a repository it \
         overwrote to learn something git would have told it anyway**"
    );
}

#[test]
fn a_repository_git_would_not_copy_is_no_checkout() {
    let where_it_lands = the_destination_nothing_stands_at("git-would-not-copy");
    let mut the_project = AProjectHoldingAListing::holding(where_it_lands.clone(), vec![]);
    let mut the_program = AProgramThatCopies::refusing_the_clone("could not read repository");

    let refused = what_a_creative_run_reads_from_its_own_checkout(
        &mut the_program,
        &mut the_project,
        &the_request_to_clone_into(&where_it_lands),
    );

    assert!(
        matches!(
            refused,
            Err(ACheckoutThatCouldNotBeRead::TheRepositoryWasNeverCopied { .. })
        ),
        "**because seven offers about a repository nobody holds are seven issues about nothing** \
         — and a person reading them cannot check a single path."
    );
}

#[test]
fn a_project_somewhere_else_is_not_the_run_own_checkout() {
    let where_it_lands = the_destination_nothing_stands_at("somewhere-else");
    let mut the_project = AProjectHoldingAListing::holding(
        the_destination_nothing_stands_at("not-the-clone"),
        vec![],
    );
    let mut the_program = AProgramThatCopies::copying_everything();

    let refused = what_a_creative_run_reads_from_its_own_checkout(
        &mut the_program,
        &mut the_project,
        &the_request_to_clone_into(&where_it_lands),
    );

    assert!(
        matches!(
            refused,
            Err(ACheckoutThatCouldNotBeRead::NotTheRunItsOwnCheckout { .. })
        ),
        "**because every name in that listing would be a real name of some other project** — and \
         an offer grounded in it files an issue about a file this run never saw."
    );
}

/// The request to clone into a directory, **for the creative run's own checkout.**
fn the_request_to_clone_into(
    where_it_lands: &std::path::Path,
) -> aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo {
    let the_clone_of = aisdlc_run::the_repository_to_clone_into::ARepositoryToClone::of(
        aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("CREATIVE-1").expect("a name"),
        where_it_lands.to_path_buf(),
    );
    aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo::clone_the_repository(
        the_clone_of,
    )
}

/// A destination nothing stands at, **of this test's own and no other.**
fn the_destination_nothing_stands_at(a_name: &str) -> std::path::PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-creative-read-{}-{:?}-{a_name}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&where_it_is);
    where_it_is
}

/// A destination something stands at, **because the directory itself is there.**
fn the_destination_something_stands_at(a_name: &str) -> std::path::PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-creative-read-{}-{:?}-{a_name}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&where_it_is);
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is
}

/// A project holding a listing at a place, **and answering about the real disk.**
struct AProjectHoldingAListing {
    where_it_lives: std::path::PathBuf,
    what_is_in_it: Vec<String>,
}

impl AProjectHoldingAListing {
    /// A project holding a listing at a place.
    fn holding(where_it_lives: std::path::PathBuf, what_is_in_it: Vec<String>) -> Self {
        Self {
            where_it_lives,
            what_is_in_it,
        }
    }
}

impl aisdlc_run::TheProjectOnDisk for AProjectHoldingAListing {
    fn where_it_lives(&self) -> &std::path::Path {
        self.where_it_lives.as_path()
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        self.what_is_in_it.clone()
    }

    fn the_text_of(&mut self, _a_path: &aisdlc_run::ThePathInsideTheProject) -> Option<String> {
        None
    }

    fn a_file_is_written(
        &mut self,
        a_path: &aisdlc_run::ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), aisdlc_run::the_project_on_disk::TheProjectRefusedToWrite> {
        Err(
            aisdlc_run::the_project_on_disk::TheProjectRefusedToWrite::there_was_no_room_for_it(
                a_path,
                what_it_says,
            ),
        )
    }

    fn does_anything_stand_at(&self, a_path: &std::path::Path) -> bool {
        a_path.exists()
    }
}

/// A program that copies, **and answers what the copy stands on.**
struct AProgramThatCopies {
    refuses_the_clone: Option<String>,
}

impl AProgramThatCopies {
    /// A program that copies everything it is asked to copy.
    fn copying_everything() -> Self {
        Self {
            refuses_the_clone: None,
        }
    }

    /// A program that refuses the clone with git's own words.
    fn refusing_the_clone(what_git_said: &str) -> Self {
        Self {
            refuses_the_clone: Some(String::from(what_git_said)),
        }
    }

    /// What the clone is answered with, **and the refusal comes first because a run that was
    /// refused has no copy to ask about.**
    fn what_the_clone_is_answered_with(&self) -> WhatTheProgramSaid {
        match &self.refuses_the_clone {
            Some(what_git_said) => WhatTheProgramSaid::of(false, what_git_said),
            None => WhatTheProgramSaid::of(true, ""),
        }
    }

    /// Where the code was written, which may only be read.
    fn where_the_code_was_written() -> WhatTheProgramSaid {
        WhatTheProgramSaid::of(true, THE_ADDRESS_THE_CODE_WAS_WRITTEN_IN)
    }
}

impl aisdlc_run::a_program_we_run::AProgramWeRun for AProgramThatCopies {
    fn run(
        &self,
        what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo,
    ) -> WhatTheProgramSaid {
        use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo as Asked;
        match what_it_is_asked_to_do {
            Asked::CloneTheRepository { .. } => self.what_the_clone_is_answered_with(),
            Asked::TheCommitItStandsOn { .. } => WhatTheProgramSaid::of(true, THE_COMMIT),
            Asked::TheRefItStandsOn { .. } => WhatTheProgramSaid::of(true, THE_REF),
            Asked::NameWhereTheCodeWasWrittenIn { .. } => Self::where_the_code_was_written(),
            _ => WhatTheProgramSaid::of(false, "not a question about a clone"),
        }
    }
}
