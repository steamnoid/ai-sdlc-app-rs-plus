//! A program we run, asked for by what we want it to do, and never by a string.
//!
//! **This is the first thing in this repository that executes anything**, so the shape of the
//! question is the whole of its safety. A port whose arguments are strings is a port a caller
//! can smuggle a shell line through, and the glossary says a port's arguments are named types
//! and never a `String` and never a tuple.
//!
//! **A full clone and not a shallow one is a property of the type and not of a check.** The
//! request below has no variant that is shallow, so there is nothing to pass one and nothing to
//! forget: **the run does things with the history — a diff against a base, `git log` for how a
//! file got the way it is — and a shallow clone answers none of them.**

use aisdlc_run::a_program_we_run::WhichProgramWeRun;
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

fn the_clone_the_run_wants() -> ARepositoryToClone {
    ARepositoryToClone::of(
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        std::path::PathBuf::from("/projects/steamnoid/ai-sdlc-app-rs-plus/WELLBEINGT-5/repository"),
    )
}

#[test]
fn the_only_program_this_system_runs_is_named() {
    let every_one = WhichProgramWeRun::every_program();

    assert_eq!(
        every_one,
        vec![WhichProgramWeRun::Git],
        "**because a program we run is a closed set and not a name a caller supplies** — a port that runs whatever string it is given is a port that runs whatever reaches it"
    );
}

#[test]
fn what_a_program_is_asked_to_do_is_a_value_and_not_a_list_of_strings() {
    let the_clone = the_clone_the_run_wants();

    let what_it_is_asked_to_do = WhatWeTellAProgramToDo::clone_the_repository(the_clone);

    assert!(matches!(
        what_it_is_asked_to_do,
        WhatWeTellAProgramToDo::CloneTheRepository { .. }
    ));
}

#[test]
fn a_request_carries_what_was_asked_for_and_not_a_formatted_line() {
    let the_clone = the_clone_the_run_wants();

    let the_request = WhatWeTellAProgramToDo::clone_the_repository(the_clone);

    assert_eq!(
        the_request.fetch_from(),
        "https://github.com/steamnoid/ai-sdlc-app-rs-plus.git",
        "**and the address is a field on the request, not a line somebody formatted** — a formatted line is where a value read out of a model becomes part of a command"
    );
    assert_eq!(
        the_request.where_it_lands(),
        std::path::Path::new("/projects/steamnoid/ai-sdlc-app-rs-plus/WELLBEINGT-5/repository"),
    );
}

#[test]
fn there_is_no_shallow_clone_to_ask_for() {
    let the_clone = the_clone_the_run_wants();

    let the_request = WhatWeTellAProgramToDo::clone_the_repository(the_clone);

    assert!(
        the_request.is_the_whole_history(),
        "**the whole history and not a depth, because the type has no field for a depth** — the cheapest clone is a false economy paid at the first review, and a request that cannot express a shallow clone cannot be built into one by a later change"
    );
}
