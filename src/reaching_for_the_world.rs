//! The five modules of the standard library that can touch the world, and where a file reaches
//! for one.
//!
//! **One list, read by every gate that draws a boundary at a reach.** A gate carrying its own
//! copy of these five names is a gate whose copy is a version behind, and the one that is behind
//! is the one nobody re-reads.
//!
//! **`temp_dir` is here and it carries no credential at all**, because a gate asked to refuse a
//! path lookup has to refuse where it is going to look as well, or a crate can be handed the
//! answer on a plate.

use crate::a_stretch_of_the_file::WhatIsWrittenThere;
use crate::scanning::HowTheFileIsWritten;

/// The five modules of the standard library that can touch the world.
pub const THE_MODULES_THAT_REACH_THE_WORLD: [&str; 5] = ["fs", "process", "time", "net", "thread"];

/// The path each reach stands at, and not the last segment on its own.
pub const THE_PATH_EACH_REACH_STANDS_AT: &str = "std::";

/// What one line of code reaches for, and nothing when it reaches for none.
///
/// **A line and not a file, because a reader has to be able to act on a finding** — a refusal
/// saying "somewhere in this file" is a refusal saying nothing.
pub fn what_reaches_the_world_says(one_line: &str, the_line_number: usize) -> Vec<String> {
    let mut found = Vec::new();
    for a_module in THE_MODULES_THAT_REACH_THE_WORLD {
        let the_path = format!("{THE_PATH_EACH_REACH_STANDS_AT}{a_module}");
        if one_line.contains(&the_path) {
            found.push(format!("line {the_line_number} reaches for {the_path}"));
        }
    }
    found
}

/// What a whole file reaches for, reading its code and not its comments.
///
/// **The stretches, and not a search**, because a file that explains where the reach lives is
/// not a file that makes it — and a gate that read its own explanation would refuse the sentence
/// that keeps the rule from being undone.
pub fn what_reaches_the_world_in(a_file: &str) -> Vec<String> {
    let mut found = Vec::new();
    for stretch in HowTheFileIsWritten::of(a_file).stretches() {
        if stretch.kind() != WhatIsWrittenThere::Code {
            continue;
        }
        found.extend(what_reaches_the_world_says(
            stretch.text(),
            stretch.line_it_starts_on(),
        ));
    }
    found
}
