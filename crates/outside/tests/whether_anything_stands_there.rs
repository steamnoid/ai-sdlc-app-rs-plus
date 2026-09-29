//! The real directory, asked whether anything stands there, on the real disk.
//!
//! **A double is not a gate, and this is the half it cannot do.** Ten gates have proved that a
//! rule which refuses can be shown refusing; this asks the real filesystem a real question about
//! real directories, because a double that always answers `false` and a double that always answers
//! `true` both pass every test the use case can write.
//!
//! **`exists()` is not the question, and that is the whole of it.** A run that made its own
//! destination directory and then found it empty has to be allowed to clone into it — a check on
//! the directory's own existence refuses a first run forever, and a run that can never start is a
//! run that never finds out what it is wrong about.

use aisdlc_outside::whether_anything_stands_there::WhetherAnythingStandsOnDisk;
use aisdlc_run::whether_anything_stands_there::WhetherAnythingStandsThere;

/// A directory under `target/`, which git already ignores and a listing already skips.
fn under_the_target_directory(named: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join(named)
}

/// A directory that exists and holds nothing, made on purpose and not by a run.
fn an_empty_directory(named: &str) -> std::path::PathBuf {
    let where_it_is = under_the_target_directory(named);
    let _ = std::fs::remove_dir_all(&where_it_is);
    std::fs::create_dir_all(&where_it_is).expect("a directory can be made under the target");
    where_it_is
}

#[test]
fn a_directory_nobody_has_ever_made_holds_nothing() {
    let the_disk = WhetherAnythingStandsOnDisk::as_it_is;
    let where_it_would_land = under_the_target_directory("a-place-nobody-has-made");

    let _ = std::fs::remove_dir_all(&where_it_would_land);
    assert!(
        !the_disk.does_anything_stand_at(&where_it_would_land),
        "**because a place that does not exist cannot hold anything** — and the answer a run needs first of all is the one that lets it begin"
    );
}

#[test]
fn a_directory_that_exists_and_holds_nothing_holds_nothing() {
    let the_disk = WhetherAnythingStandsOnDisk::as_it_is;
    let where_it_is = an_empty_directory("a-directory-holding-nothing");

    assert!(
        !the_disk.does_anything_stand_at(&where_it_is),
        "**and this is the case `exists()` gets wrong** — a run that made its own destination and then found it empty would be refused by a check on the directory's own existence, forever, and a run that can never start is a run that never learns what it is wrong about"
    );
    assert!(
        where_it_is.exists(),
        "**the directory really is there**, because a test that passes because the directory was never made has proved nothing about an empty one"
    );
}

#[test]
fn a_directory_holding_one_thing_holds_something() {
    let the_disk = WhetherAnythingStandsOnDisk::as_it_is;
    let where_it_is = an_empty_directory("a-directory-holding-one-thing");
    std::fs::write(where_it_is.join("a-file-in-it"), "written by a test")
        .expect("a file can be written");

    assert!(
        the_disk.does_anything_stand_at(&where_it_is),
        "**because a run resumes and the directory is derived** — this is the answer that has to stop it, and an answer of `false` here is an answer that lets a run write over its own earlier work"
    );
}

#[test]
fn a_clone_already_made_holds_something_even_though_it_is_a_directory() {
    let the_disk = WhetherAnythingStandsOnDisk::as_it_is;
    let where_it_is = an_empty_directory("a-clone-somebody-else-made");
    std::fs::create_dir_all(where_it_is.join(".git"))
        .expect("a clone is a directory with a .git in it");

    assert!(
        the_disk.does_anything_stand_at(&where_it_is),
        "**and a directory inside a directory is still something** — a check that asked only about files would let a run clone into the middle of a repository somebody else is holding"
    );
}
