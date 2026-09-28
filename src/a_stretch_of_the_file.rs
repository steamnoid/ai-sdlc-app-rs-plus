//! One stretch of a source file: what is written there, and where it starts.
//!
//! **Separate from the walk that makes stretches**, because the two have grown apart: the
//! walk reads bytes and this is what it finds, and keeping them in one file put that file
//! over the ceiling the gate in this very repository enforces — which the gate reported the
//! moment the addition happened, on the file being edited.
use std::fmt;

/// The five kinds a stretch of a `.rs` file can be.
///
/// **A closed set**, because every gate rests on the claim that a span of characters in a
/// source file is one of these five things and nothing else. A sixth would be a change to
/// what a gate may assume, so it is a change here and nowhere else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsWrittenThere {
    /// An instruction the compiler reads.
    Code,
    /// Something a person wrote for a person to read, `///` and `//!` included.
    AComment,
    /// A quoted run of characters, of any of its six spellings.
    AString,
    /// A quote, one character or one escape, and a quote.
    ACharacter,
    /// The space between, which includes the line ending.
    TheSpaceBetween,
}

impl fmt::Display for WhatIsWrittenThere {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let written = match self {
            WhatIsWrittenThere::Code => "code",
            WhatIsWrittenThere::AComment => "a comment",
            WhatIsWrittenThere::AString => "a string",
            WhatIsWrittenThere::ACharacter => "a character",
            WhatIsWrittenThere::TheSpaceBetween => "the space between",
        };
        write!(formatter, "{written}")
    }
}

/// A run of characters in a source file that are all the same kind of thing.
///
/// It carries the line it starts on, because a gate that says "there is a comment somewhere"
/// is a gate nobody can act on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStretchOfTheFile {
    pub(crate) kind: WhatIsWrittenThere,
    pub(crate) text: String,
    pub(crate) line_it_starts_on: usize,
}

impl AStretchOfTheFile {
    /// What this stretch is.
    pub fn kind(&self) -> WhatIsWrittenThere {
        self.kind
    }

    /// The characters, exactly as the file wrote them.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The line the stretch starts on, counted from one.
    pub fn line_it_starts_on(&self) -> usize {
        self.line_it_starts_on
    }

    /// The lines of this stretch that are a comment rather than documentation, numbered as
    /// the file numbers them.
    ///
    /// **A line and not a stretch, because the walk merges a run of comment lines into one
    /// stretch.** Three `//!` lines are one comment, and so are a `//!` and a `//` on the next
    /// line — so "is this stretch documentation" is a question about a region that can hold
    /// both answers, and the only honest question is about a line.
    ///
    /// Empty for anything that is not a comment, so a caller may ask every stretch.
    pub fn the_lines_that_only_comment(&self) -> Vec<usize> {
        if self.kind != WhatIsWrittenThere::AComment {
            return Vec::new();
        }
        let mut found = Vec::new();
        for (offset, line) in self.text.lines().enumerate() {
            if !a_line_documents_rather_than_comments(line) {
                found.push(self.line_it_starts_on + offset);
            }
        }
        found
    }
}

/// Whether one line of a comment stretch says why the code is as it is.
///
/// **Documentation is `///` or `//!`.** A comment that says neither is the thing this
/// repository refuses, and a rule that counted a run of comment lines as one thing could not
/// tell a file with one stray `//` from a file written entirely in `//`.
fn a_line_documents_rather_than_comments(a_line: &str) -> bool {
    let trimmed = a_line.trim_start();
    trimmed.starts_with("///") || trimmed.starts_with("//!")
}
