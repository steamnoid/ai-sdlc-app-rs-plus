//! The two things every gate needs before it can look at anything.
//!
//! **Six gates read the checkout and six copies would be six names waiting to disagree.** An
//! earlier version of this repository had four copies of the walk, and the copies had
//! already begun to differ — one read three roots and another two, one panicked on a failed
//! read and another returned a `Result`.

/// The root of this checkout, found by walking up from where cargo compiled this file.
///
/// **Six gates read the checkout and six copies would be six names waiting to disagree**, so
/// it is here rather than in a test. It walks up rather than down because the gates' own
/// package sits at the root and the crates it judges sit below it, and a reader should not
/// have to know that in order to find a file.
pub fn the_root_of_the_checkout() -> std::path::PathBuf {
    let mut here = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !here.join("crates").is_dir() {
        assert!(
            here.pop(),
            "no crates/ directory above where this file was compiled"
        );
    }
    here
}

/// The text of the file at `path`, or a panic naming the file and what the read said.
///
/// **A file that cannot be read is a failure and not an empty answer.** A gate that turns a
/// missing file into a pass has stopped being a gate, and the quietest version of that is
/// `unwrap_or_default()`.
pub fn the_text_at(path: &std::path::Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|why| panic!("{} must be readable: {why}", path.display()))
}
