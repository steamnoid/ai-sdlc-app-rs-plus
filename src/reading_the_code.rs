//! Every reading of the repository's source that a gate needs, in one place.
//!
//! **Two gates read the checkout and two copies are two lists waiting to disagree**, and this
//! repository had seven copies of one reader and eight of another before the gate that
//! measures this was built. Every function here takes what it reads as an argument where it
//! can, so a caller reads a file once and passes the text on.

use std::path::PathBuf;

use crate::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
use crate::reading_the_checkout;
use crate::scanning::HowTheFileIsWritten;

/// Every `.rs` file in this checkout, in the order a reader would meet them.
///
/// **Skipping `target` and `.git`,** because a build leaves behind copies of the source and a
/// gate that read them would be measuring a build rather than the repository.
pub fn every_source_file() -> Vec<PathBuf> {
    let mut found = Vec::new();
    collect_rust_files_under(
        &reading_the_checkout::the_root_of_the_checkout(),
        &mut found,
    );
    found.sort();
    found
}

/// The `.rs` files under one directory, and the ones under its subdirectories.
fn collect_rust_files_under(directory: &std::path::Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "target" || name == ".git" {
            continue;
        }
        if path.is_dir() {
            collect_rust_files_under(&path, found);
        } else if path.extension().is_some_and(|it| it == "rs") {
            found.push(path);
        }
    }
}

/// Every line of the code that carries code on it, as `(the line it is on, the line)`.
///
/// **Joined, because a line can arrive in pieces.** A string in the middle of a line ends the
/// code stretch and starts another on the same line — `format!("{} …", x)` is two pieces — and
/// a reader that treated each stretch as a line counted the line twice and its braces as two
/// levels. **Five readers in this repository made that mistake before this one existed.**
pub fn the_lines_of_code(the_code: &str) -> Vec<(usize, String)> {
    let mut found: Vec<(usize, String)> = Vec::new();
    for stretch in HowTheFileIsWritten::of(the_code).stretches() {
        append_the_lines_of(stretch, &mut found);
    }
    found.retain(|(_, one_line)| !one_line.trim().is_empty());
    found
}

/// Add one stretch's lines, numbered as the file numbers them, joining a line cut in two.
fn append_the_lines_of(a_stretch: &AStretchOfTheFile, found: &mut Vec<(usize, String)>) {
    let is_code = a_stretch.kind() == WhatIsWrittenThere::Code;
    if !is_code {
        return;
    }
    for (offset, one_line) in a_stretch.text().split('\n').enumerate() {
        let the_line = a_stretch.line_it_starts_on() + offset;
        let Some(last) = found.last_mut() else {
            found.push((the_line, one_line.to_string()));
            continue;
        };
        if last.0 == the_line {
            last.1.push_str(one_line);
            continue;
        }
        found.push((the_line, one_line.to_string()));
    }
}

/// The text of every source file, as `(its path, its text)`.
pub fn the_text_of_every_source_file() -> Vec<(PathBuf, String)> {
    every_source_file()
        .into_iter()
        .map(|a_file| {
            let the_text = reading_the_checkout::the_text_at(&a_file);
            (a_file, the_text)
        })
        .collect()
}
