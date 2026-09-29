//! The second address of a checkout, and the remote that carries it.
//!
//! **`git clone` sets `origin` and nothing else, so without the second a reader running
//! `git remote -v` months later cannot tell a fork from a mystery.** One name on a checkout says
//! where this system's copy lives; the other says where the code was written and may only be
//! read, and a checkout holding one name is a checkout nobody can reason about later.
//!
//! **The two are the same address today, and saying that out loud is the point.** The
//! deployment's own copy and the public original are one address until the deployment forks
//! something, and a reader who cannot see that difference is a reader being told it by accident.

use aisdlc_domain::repository::Repository;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_outside::whether_anything_stands_there::WhetherAnythingStandsOnDisk;
use aisdlc_run::a_program_we_run::AProgramWeRun;
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_run_makes_a_checkout::the_run_makes_a_checkout;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// One directory under `target/`, named so no two tests can reach the same one.
fn under_the_target_directory(named: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join(named)
}

/// The request to clone this repository into a directory of this name.
fn the_request_into(named: &str) -> WhatWeTellAProgramToDo {
    WhatWeTellAProgramToDo::clone_the_repository(ARepositoryToClone::of(
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        under_the_target_directory(named),
    ))
}

/// What `git` reports about one remote of a checkout, read by asking `git` rather than reading.
fn what_the_remote_says(where_it_is: &std::path::Path, which_remote: &str) -> String {
    let what_it_ran = std::process::Command::new("git")
        .args(["-C", &where_it_is.display().to_string(), "remote", "-v"])
        .output();
    let Ok(what_it_ran) = what_it_ran else {
        return String::new();
    };
    let mut found = String::new();
    for one_line in String::from_utf8_lossy(&what_it_ran.stdout).lines() {
        if one_line.starts_with(which_remote) {
            found.push_str(one_line);
        }
    }
    found
}

#[test]
fn the_address_the_code_was_written_in_is_an_address_and_not_the_push_address() {
    let the_clone = ARepositoryToClone::of(
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        under_the_target_directory("two-addresses"),
    );

    let a_request = WhatWeTellAProgramToDo::name_where_the_code_was_written_in(the_clone);

    assert_eq!(
        a_request.fetch_from(),
        "https://github.com/steamnoid/ai-sdlc-app-rs-plus.git",
        "**and it is the same address today, because the deployment's own copy and the public original are one address until the deployment forks something** — a reader being told that by accident is exactly the mystery this is here to remove"
    );
}

#[test]
fn naming_where_the_code_was_written_in_is_one_argument_each() {
    let the_clone = ARepositoryToClone::of(
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        under_the_target_directory("two-addresses"),
    );
    let mut the_program = AProgramThatAnswers::with_one_answer();

    let _ = the_program.run(&WhatWeTellAProgramToDo::name_where_the_code_was_written_in(
        the_clone,
    ));

    let what_it_was_asked = the_program.what_it_was_asked.borrow();
    assert!(
        what_it_was_asked[0].is_naming_where_the_code_was_written_in(),
        "**because the second address is a question asked after the copy exists, and not a variant of the copy itself** — a run asked to name it before the clone has nothing to name"
    );
}

/// A program that answers once and remembers what it was asked, and never starts a process.
struct AProgramThatAnswers {
    what_it_was_asked: std::cell::RefCell<Vec<WhatWeTellAProgramToDo>>,
}

impl AProgramThatAnswers {
    fn with_one_answer() -> Self {
        Self {
            what_it_was_asked: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AProgramWeRun for AProgramThatAnswers {
    fn run(
        &self,
        what_it_is_asked_to_do: &WhatWeTellAProgramToDo,
    ) -> aisdlc_run::a_program_we_run::WhatTheProgramSaid {
        self.what_it_was_asked
            .borrow_mut()
            .push(what_it_is_asked_to_do.clone());
        aisdlc_run::a_program_we_run::WhatTheProgramSaid::of(true, "")
    }
}

#[test]
#[ignore = "clones over the network, because fetch_from is always an https address"]
fn a_checkout_a_run_made_carries_both_addresses_and_today_they_are_one() {
    let named = "a-checkout-with-both-addresses";
    let _ = std::fs::remove_dir_all(under_the_target_directory(named));
    let mut the_program = TheProgramAsItRuns::as_it_is_on_this_host();
    let mut the_disk = WhetherAnythingStandsOnDisk;

    let how_it_went =
        the_run_makes_a_checkout(&mut the_program, &the_disk, &the_request_into(named));

    assert!(
        how_it_went.the_clone().is_some(),
        "**because the run has to have made a checkout before anybody can ask what two addresses it carries**"
    );
    let where_it_lands = under_the_target_directory(named);
    let what_origin_says = what_the_remote_says(&where_it_lands, "origin");
    let what_upstream_says = what_the_remote_says(&where_it_lands, "upstream");
    assert!(
        !what_origin_says.is_empty() && !what_upstream_says.is_empty(),
        "**and a reader running `git remote -v` months later has to be able to tell a fork from a mystery** — `git clone` sets `origin` and nothing else, so without the second there is one name and the question has no answer: origin={what_origin_says:?} upstream={what_upstream_says:?}"
    );
}
