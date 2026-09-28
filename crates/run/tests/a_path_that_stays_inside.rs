//! A path that resolves outside the project is refused, whatever it spells.
//!
//! **The rule decides by the resolved location and not by the spelling**, because `..` is a
//! way out however it is written and a symlink is a way out however innocent it looks.
//!
//! **Resolution returns a different spelling from the one it was given**, so a caller that
//! compares strings without resolving calls a correct read a wrong one. That is the assertion
//! this file is about, and it is why the return type carries the resolved path rather than the
//! one that was written.
//!
//! **Read against the real checkout rather than a temporary directory.** The gate that refuses
//! a run-time read of the environment refuses `temp_dir` too, and it is right: the project this
//! rule exists for is the one under test.

use aisdlc_run::a_path_in_the_project::a_path_in_the_project;
use aisdlc_run::the_path_inside_the_project::the_path_inside_the_project;
use aisdlc_run::the_project_this_run_is_in;

fn the_root() -> std::path::PathBuf {
    the_project_this_run_is_in()
}

#[test]
fn an_ordinary_path_stays_inside() {
    let a_path =
        a_path_in_the_project("crates/run/src/lib.rs").expect("an ordinary path is a path");

    let inside =
        the_path_inside_the_project(&the_root(), &a_path).expect("that path is inside the project");

    assert!(
        inside.as_str().ends_with("crates/run/src/lib.rs"),
        "and it is the resolved spelling, which for an ordinary path looks the same"
    );
}

#[test]
fn a_path_that_spell_its_way_out_is_refused() {
    let a_path = a_path_in_the_project("crates/run/../../etc/passwd").expect("the shape is a path");

    let refusal = the_path_inside_the_project(&the_root(), &a_path);

    assert!(
        refusal.is_err(),
        "**`..` is a way out however it is written**, and a rule that reads the spelling accepts every one of them"
    );
}

#[test]
fn a_path_that_spell_its_way_back_in_stays_inside() {
    let a_path =
        a_path_in_the_project("crates/run/../run/src/lib.rs").expect("the shape is a path");

    let inside =
        the_path_inside_the_project(&the_root(), &a_path).expect("it resolves back inside");

    assert!(
        !inside.as_str().contains(".."),
        "**and the spelling that comes back is the resolved one**, because a caller comparing strings without resolving calls a correct read a wrong one"
    );
}

#[test]
fn a_path_that_does_not_exist_is_refused() {
    let a_path =
        a_path_in_the_project("crates/run/src/nothing_here.rs").expect("the shape is a path");

    let refusal = the_path_inside_the_project(&the_root(), &a_path);

    assert!(
        refusal.is_err(),
        "**resolution needs something to resolve**, and a path naming nothing is not inside the project so much as nowhere"
    );
}

#[test]
fn a_refusal_says_what_it_resolved_to() {
    let a_path = a_path_in_the_project("crates/run/../../etc/passwd").expect("the shape is a path");

    let refusal = the_path_inside_the_project(&the_root(), &a_path).expect_err("it is outside");

    assert!(
        !refusal.why_not().is_empty(),
        "because a refusal that does not say where it thought the path was leaves the caller to work out the answer again"
    );
}

#[test]
fn the_project_this_run_is_in_is_the_checkout_and_not_the_crate() {
    let the_project = the_project_this_run_is_in();

    assert!(
        the_project.join("crates").join("run").is_dir(),
        "**the workspace and not the crate** — the rule is about the project, and a crate is a directory inside it. The first version of this test asserted the opposite and the code was right: the name of the test said checkout and the assertion said `crates/run`."
    );
}
