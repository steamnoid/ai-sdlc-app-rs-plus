//! The `git` behind a branch reaching a remote, **and the real `git` proving it.**
//!
//! **A branch is not delivered by a `Vec<String>`.** The arguments a push is run with are
//! where a refspec goes wrong, and `a_test_is_not_its_own_proof` refuses a file that only
//! describes a program's arguments — **which exists because a green test once asserted
//! `--author-name` on `gh issue comment`, which has no such flag, and it was the only proof
//! there was.**
//!
//! **A real `git` against a real remote on this machine, and nothing leaves it.** The remote
//! is a bare repository in a directory, named `origin` by the fixture, **so `git push origin`
//! and `git ls-remote --heads origin` are the same two commands the real delivery runs** — and
//! a refspec that is wrong on a local path is a refspec that is wrong against GitHub.
//!
//! **The fixture is built with `Command` and the two requests under test are not.** Setting a
//! repository up is not what this file is about; **the push and the reading are, and both of
//! them go through `AProgramWeRun` like every other request this system makes.**

use aisdlc_domain::repository::Repository;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_run::a_program_we_run::AProgramWeRun;
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// A work tree with one commit and a bare remote named `origin`, **all on this machine.**
struct ARepositoryOnThisMachine {
    where_the_work_tree_is: std::path::PathBuf,
    of_which_clone: ARepositoryToClone,
}

/// Clear one place before `git` is run in it, **and a lock file is a real thing a killed run
/// leaves behind.**
///
/// **A gate killed mid-suite leaves `git` locks in these fixtures, and the next run fails with
/// `could not lock config file` — a failure that names neither this test nor the thing that
/// caused it.** `git init` creates the lock itself when the directory is new, so removing the whole
/// place is what clears it; this function exists to say so, because the removal alone did not read
/// as deliberate.
fn the_place_is_clear_of_a_lock_left_behind(the_place: &std::path::Path) {
    let _ = std::fs::remove_dir_all(the_place);
}

impl ARepositoryOnThisMachine {
    /// Make one, **or say why it could not be made.**
    fn one_named(named: &str) -> Result<Self, String> {
        let the_place = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target")
            .join(named);
        the_place_is_clear_of_a_lock_left_behind(&the_place);
        let the_bare_remote = the_place.join("the-remote.git");
        let where_the_work_tree_is = the_place.join("repository");
        make_a_work_tree(&where_the_work_tree_is, &the_bare_remote)?;
        let of_which_clone = ARepositoryToClone::of(
            Repository::try_from("steamnoid/wellbeing-tracker-public").expect("two halves"),
            WorkItemId::try_from("WELLBEINGT-42").expect("a name is a name"),
            where_the_work_tree_is.clone(),
        );
        Ok(Self {
            where_the_work_tree_is,
            of_which_clone,
        })
    }
}

/// A program with no credential, **because a remote on this machine needs none.**
fn the_program() -> TheProgramAsItRuns {
    TheProgramAsItRuns::as_it_is_on_this_host(None)
}

/// The arguments one request would be run with, **which is what the first test reads.**
fn the_arguments_of(what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> Vec<String> {
    the_program().the_arguments_of(what_it_is_asked_to_do)
}

/// A work tree with one commit and a bare remote called `origin`, **built by `git` itself.**
///
/// **Three functions and not one doing all of it**, because a closure inside a function that
/// already calls `git` twice is a body nesting three deep for the sake of a nicer message.
fn make_a_work_tree(
    the_work_tree: &std::path::Path,
    the_bare_remote: &std::path::Path,
) -> Result<(), String> {
    let _ = std::fs::create_dir_all(the_work_tree);
    make_the_bare_remote(the_bare_remote, the_work_tree)?;
    make_the_work_tree(the_work_tree, the_bare_remote)
}

/// The remote, **which is a bare repository and not a working one.**
fn make_the_bare_remote(
    the_bare_remote: &std::path::Path,
    where_it_runs: &std::path::Path,
) -> Result<(), String> {
    let the_bare_place = the_bare_remote.to_str().expect("a path is a path");
    git(&["init", "--bare", "-q", the_bare_place], where_it_runs)
}

/// The work tree, **pointed at the remote under the name `git` already gave it.**
fn make_the_work_tree(
    the_work_tree: &std::path::Path,
    the_bare_remote: &std::path::Path,
) -> Result<(), String> {
    let the_bare_place = the_bare_remote.to_str().expect("a path is a path");
    git(&["init", "-q", "."], the_work_tree)?;
    git(&["remote", "add", "origin", the_bare_place], the_work_tree)?;
    name_the_one_who_may_commit_in(the_work_tree)?;
    git(
        &["commit", "-q", "--allow-empty", "-m", "what this run made"],
        the_work_tree,
    )
}

/// Who may commit in this repository, **and in this repository only.
///
/// **A local `git config` and never a global one, and this is a repair rather than a fixture
/// choice.** The commit above refuses with *"Author identity unknown"* on a machine that has no
/// `user.name` configured — which is every CI runner, and this developer, and nothing else. **The
/// first run of this file was green here and red on the runner, which is a test that proved the
/// machine and not the code.** Configuring the identity inside the repository the test made
/// touches nothing on the host and reads the same on every machine.
fn name_the_one_who_may_commit_in(the_work_tree: &std::path::Path) -> Result<(), String> {
    git(&["config", "user.name", "aisdlc"], the_work_tree)?;
    git(&["config", "user.email", "aisdlc@localhost"], the_work_tree)
}

/// `git` with the arguments it was given, **and the words it wrote if it would not run.**
fn git(the_arguments: &[&str], where_it_runs: &std::path::Path) -> Result<(), String> {
    let what_it_ran = std::process::Command::new("git")
        .args(the_arguments)
        .current_dir(where_it_runs)
        .output()
        .map_err(|could_not_start| format!("git could not be started: {could_not_start}"))?;
    if what_it_ran.status.success() {
        return Ok(());
    }
    Err(String::from_utf8_lossy(&what_it_ran.stderr)
        .trim()
        .to_string())
}

#[test]
fn the_remote_is_read_by_a_real_git_and_the_branch_is_one_argument() {
    let the_repository = ARepositoryOnThisMachine::one_named("git-asks-the-remote")
        .expect("git made a work tree and a bare remote on this machine");

    let what_the_remote_holds = the_program().run(&WhatWeTellAProgramToDo::read_the_remote(
        the_repository.of_which_clone.clone(),
        "aisdlc/WELLBEINGT-42",
    ));

    let the_arguments = the_arguments_of(&WhatWeTellAProgramToDo::read_the_remote(
        the_repository.of_which_clone.clone(),
        "aisdlc/WELLBEINGT-42",
    ));
    let the_branch_is_one_argument = the_arguments
        .iter()
        .filter(|one| one.contains("aisdlc/WELLBEINGT-42"))
        .count()
        == 1;
    assert!(
        what_the_remote_holds.did_it_work() && the_branch_is_one_argument,
        "**because a branch name is quoted into a command whose shape is fixed: nothing a caller \
         supplies reaches the program as an option** — the branch is one whole argument beside the \
         work tree and nothing else. git said: {} and the arguments were: {the_arguments:?}",
        what_the_remote_holds.the_words()
    );
}

#[test]
fn a_branch_is_pushed_by_a_real_git_and_lands_on_the_remote() {
    let the_repository =
        ARepositoryOnThisMachine::one_named("a-real-push-lands").expect("git made a repository");
    let the_program = the_program();

    let what_the_run_asked = the_program.run(&WhatWeTellAProgramToDo::push_the_branch(
        the_repository.of_which_clone.clone(),
        "aisdlc/WELLBEINGT-42",
    ));
    let what_the_remote_now_holds = the_program.run(&WhatWeTellAProgramToDo::read_the_remote(
        the_repository.of_which_clone.clone(),
        "aisdlc/WELLBEINGT-42",
    ));

    assert!(
        what_the_run_asked.did_it_work()
            && what_the_remote_now_holds
                .the_words()
                .contains("refs/heads/aisdlc/WELLBEINGT-42"),
        "**because a refspec that is right on paper and wrong to `git` delivers nothing**, and the \
         only proof is a real `git` accepting or refusing it. The push said: {} — and the remote \
         holds: {}",
        what_the_run_asked.the_words(),
        what_the_remote_now_holds.the_words()
    );
}

#[test]
fn the_branch_the_checkout_stands_on_is_never_what_goes_to_the_remote() {
    let the_repository =
        ARepositoryOnThisMachine::one_named("the-standing-ref-stays-put").expect("git made one");
    let the_program = the_program();

    let what_the_run_asked = the_program.run(&WhatWeTellAProgramToDo::push_the_branch(
        the_repository.of_which_clone.clone(),
        "aisdlc/WELLBEINGT-42",
    ));
    let what_the_remote_holds_for_main = the_program.run(&WhatWeTellAProgramToDo::read_the_remote(
        the_repository.of_which_clone.clone(),
        "main",
    ));

    let the_working_tree = the_repository.where_the_work_tree_is.clone();
    assert!(
        what_the_run_asked.did_it_work()
            && what_the_remote_holds_for_main.the_words().trim().is_empty(),
        "**because pushing the branch the checkout is standing on would move *that* branch on the \
         remote** — and the branch the checkout is standing on is somebody else's `main`. The push \
         said: {} and the remote holds for `main`: {} — the work tree was {}",
        what_the_run_asked.the_words(),
        what_the_remote_holds_for_main.the_words(),
        the_working_tree.display()
    );
}
