//! A source file, taken apart into the stretches it is made of.
//!
//! A gate that reads a `.rs` file has to know where the code stops and the words begin, and
//! a text search cannot know that: `let address = "https://github.com";` holds two slashes
//! and has no comment in it, and three fixtures in this repository's own tests hold them
//! inside strings on purpose.
//!
//! **Six gates will call this walk, so it is one walk in the library and not six in the
//! gates.** A private helper in a test binary is reported dead the moment the test that does
//! not use it is compiled, and the only ways to stop that are `#[allow(dead_code)]`, which a
//! gate in this repository refuses, or a copy.

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
    kind: WhatIsWrittenThere,
    text: String,
    line_it_starts_on: usize,
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
}

/// A source file, taken apart into the stretches it is made of, in the order it has them.
///
/// **Every byte of the file ends up in exactly one stretch**, because a gate that quotes a
/// stretch has to be quoting the file. The reading moves forward on every step, so it ends at
/// the last byte whatever the file holds — including a quote or a comment that is never
/// closed, which is a fact about a file that does not compile and not one this walk decides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowTheFileIsWritten {
    stretches: Vec<AStretchOfTheFile>,
}

impl HowTheFileIsWritten {
    /// Take `source` apart, in the order the file has them.
    pub fn of(source: &str) -> HowTheFileIsWritten {
        let mut reading = WhereTheReadingHasGotTo::at_the_first_byte_of(source);
        let mut stretches = Vec::new();
        while !reading.is_at_the_end() {
            stretches.push(reading.the_next_stretch());
        }
        HowTheFileIsWritten { stretches }
    }

    /// Every stretch, in the order the file has them.
    pub fn stretches(&self) -> &[AStretchOfTheFile] {
        &self.stretches
    }

    /// Whether the file has a comment anywhere in it.
    ///
    /// This is the question a gate asks before it believes anything else about the file, and
    /// it is asked of the stretches rather than of the text: a file whose slashes are inside
    /// a string has no comment, and a search for two slashes says it has one.
    pub fn is_there_a_comment_anywhere(&self) -> bool {
        self.stretches
            .iter()
            .any(|stretch| stretch.kind() == WhatIsWrittenThere::AComment)
    }

    /// The line each comment in the file starts on, in the order they were written.
    pub fn the_lines_a_comment_starts_on(&self) -> Vec<usize> {
        self.stretches
            .iter()
            .filter(|stretch| stretch.kind() == WhatIsWrittenThere::AComment)
            .map(AStretchOfTheFile::line_it_starts_on)
            .collect()
    }
}

/// How far along a source file the reading has come.
///
/// **A type and not four arguments**, because a walk that passes a position and a line and a
/// collection to every step is a walk whose steps cannot be read one at a time.
struct WhereTheReadingHasGotTo<'a> {
    the_file: &'a str,
    the_bytes: &'a [u8],
    at: usize,
    line: usize,
}

impl<'a> WhereTheReadingHasGotTo<'a> {
    /// A reading that has not read anything, standing on the first byte of the file.
    fn at_the_first_byte_of(the_file: &'a str) -> WhereTheReadingHasGotTo<'a> {
        WhereTheReadingHasGotTo {
            the_file,
            the_bytes: the_file.as_bytes(),
            at: 0,
            line: 1,
        }
    }

    /// Whether there is anything left to read.
    fn is_at_the_end(&self) -> bool {
        self.at >= self.the_bytes.len()
    }

    /// Read the next stretch, and leave the reading standing after it.
    fn the_next_stretch(&mut self) -> AStretchOfTheFile {
        let kind = self.what_starts_here();
        let from = self.at;
        let line = self.line;
        self.read_past_one_of(kind);
        AStretchOfTheFile {
            kind,
            text: self.the_file[from..self.at].to_string(),
            line_it_starts_on: line,
        }
    }

    /// The kind of the thing standing here, which is how the reading knows what to skip.
    fn what_starts_here(&self) -> WhatIsWrittenThere {
        if self.at_a_line_comment() {
            return WhatIsWrittenThere::AComment;
        }
        if self.at_a_block_comment() {
            return WhatIsWrittenThere::AComment;
        }
        if the_string_opening_at(self.the_bytes, self.at).is_some() {
            return WhatIsWrittenThere::AString;
        }
        if the_character_opening_at(self.the_bytes, self.at).is_some() {
            return WhatIsWrittenThere::ACharacter;
        }
        if self.at_white_space() {
            return WhatIsWrittenThere::TheSpaceBetween;
        }
        WhatIsWrittenThere::Code
    }

    /// Whether the byte standing here is white space, which is the whole of what separates a
    /// stretch of space from a stretch of code — a test for the byte *before* would have said
    /// a file starting with an instruction begins in the space between it and nothing.
    fn at_white_space(&self) -> bool {
        self.the_bytes
            .get(self.at)
            .is_some_and(u8::is_ascii_whitespace)
    }

    fn at_a_line_comment(&self) -> bool {
        self.the_bytes[self.at..].starts_with(b"//")
    }

    fn at_a_block_comment(&self) -> bool {
        self.the_bytes[self.at..].starts_with(b"/*")
    }

    /// Read past one stretch of the kind named, and count the lines it crossed.
    fn read_past_one_of(&mut self, kind: WhatIsWrittenThere) {
        match kind {
            WhatIsWrittenThere::AComment => self.read_past_a_comment(),
            WhatIsWrittenThere::AString => self.read_past_a_string(),
            WhatIsWrittenThere::ACharacter => self.read_past_a_character(),
            WhatIsWrittenThere::Code => self.read_past_a_line_of_code(),
            WhatIsWrittenThere::TheSpaceBetween => self.read_past_white_space(),
        }
    }

    /// A line comment ends where its line does, and a block comment ends at the first close
    /// that is not the close of a comment opened inside it.
    fn read_past_a_comment(&mut self) {
        if self.at_a_block_comment() {
            self.read_past_a_block_comment();
            return;
        }
        self.read_past_a_line_ending();
    }

    fn read_past_a_block_comment(&mut self) {
        let mut how_deep = 0usize;
        while self.at < self.the_bytes.len() {
            if self.the_bytes[self.at..].starts_with(b"/*") {
                how_deep += 1;
                self.take(2);
            } else if self.the_bytes[self.at..].starts_with(b"*/") {
                how_deep -= 1;
                self.take(2);
                if how_deep == 0 {
                    return;
                }
            } else {
                self.take(1);
            }
        }
    }

    /// Walk to the end of the line, taking the line ending with it so the next stretch is
    /// told which line it starts on.
    fn read_past_a_line_ending(&mut self) {
        while !self.is_at_the_end() && !self.at_a_line_ending() {
            self.take(1);
        }
        if !self.is_at_the_end() {
            self.take(1);
        }
    }

    fn at_a_line_ending(&self) -> bool {
        self.the_bytes.get(self.at) == Some(&b'\n')
    }

    /// Code ends at a line ending, at a comment, at a string or at a character, whichever
    /// comes first — so a line holding an address and a value is three stretches and not one.
    ///
    /// **The line ending itself is left to the space between.** A stretch of code is the code
    /// of one line, and a line that carried its own ending could not be told from one that did
    /// not, which is the difference between a line and a line and a half.
    fn read_past_a_line_of_code(&mut self) {
        while !self.is_at_the_end() {
            if self.at_a_line_ending() || self.something_else_starts_here() {
                return;
            }
            self.take(1);
        }
    }

    /// Whether the reading has to stop because something that is not code begins here.
    fn something_else_starts_here(&self) -> bool {
        self.at_a_line_comment()
            || self.at_a_block_comment()
            || the_string_opening_at(self.the_bytes, self.at).is_some()
            || the_character_opening_at(self.the_bytes, self.at).is_some()
    }

    /// Space ends where the next non-space begins, and a blank line is space.
    fn read_past_white_space(&mut self) {
        while self.at_white_space() {
            self.take(1);
        }
    }

    /// A string ends at its closing quote, and a raw string's closing quote is followed by
    /// as many hashes as opened it.
    fn read_past_a_string(&mut self) {
        let the_opening = the_string_opening_at(self.the_bytes, self.at).unwrap_or(1);
        let the_hashes = self.counting_the_hashes_of_an_opening(the_opening);
        self.take(the_opening);
        let closing = format!("\"{}", "#".repeat(the_hashes));
        loop {
            if self.is_at_the_end() {
                return;
            }
            if the_hashes == 0 && self.the_bytes[self.at] == b'\\' {
                self.take(2);
                continue;
            }
            if self.the_bytes[self.at..].starts_with(closing.as_bytes()) {
                self.take(closing.len());
                return;
            }
            self.take(1);
        }
    }

    /// How many hashes opened a string, which is the number that have to close it.
    fn counting_the_hashes_of_an_opening(&self, the_opening: usize) -> usize {
        how_many_of(&self.the_bytes[self.at + 1..self.at + the_opening], b'#')
    }

    /// A character ends at the quote that closes it, and a lifetime has none.
    fn read_past_a_character(&mut self) {
        let the_length = the_character_opening_at(self.the_bytes, self.at).unwrap_or(1);
        self.take(the_length);
    }

    /// Take so many bytes, and count the line endings among them.
    ///
    /// **The one place a line is counted.** An earlier draft of this walk counted the ending
    /// in the reader *and* counted it again here, and every line after the first came out one
    /// too high — a walk that is off by a constant still looks right at the top of a file,
    /// which is why the fixture that caught it is three blank lines and one comment rather
    /// than a comment on line two.
    fn take(&mut self, how_many: usize) {
        let the_last = self.at + how_many;
        let the_took = &self.the_bytes[self.at..the_last.min(self.the_bytes.len())];
        self.line += how_many_of(the_took, b'\n');
        self.at = the_last;
    }
}

/// How many times `wanted` stands in `the_bytes`.
///
/// **A loop and not `iter().filter().count()`**, because the lint that would replace it wants
/// a crate added, and a walk whose whole purpose is to hold no dependencies is the last place
/// in this repository to add one. The counting is four lines and the counting is not the hard
/// part.
fn how_many_of(the_bytes: &[u8], wanted: u8) -> usize {
    let mut how_many = 0;
    for one in the_bytes {
        if *one == wanted {
            how_many += 1;
        }
    }
    how_many
}

/// How many bytes open a string standing at `at`, or nothing if no string opens there.
///
/// **Six spellings and the set is closed**: `"`, `r"`, `r#"`, `b"`, `c"`, `br"`, `br#"` and
/// `cr#"`. A missing one is not a missing feature but a hole in the claim this walk makes,
/// because a `c"…"` read as code and then a string makes the halves of one literal into two
/// stretches, and a gate counting comments then reads a line twice.
fn the_string_opening_at(the_bytes: &[u8], at: usize) -> Option<usize> {
    let first = *the_bytes.get(at)?;
    if first == b'"' {
        return Some(1);
    }
    let (after_the_letter, is_raw) = after_the_first_letter_of_a_string(the_bytes, at, first);
    let where_the_quote_is = the_hashes_standing_at(the_bytes, after_the_letter);
    if the_bytes.get(where_the_quote_is) != Some(&b'"') {
        return None;
    }
    if !is_raw && where_the_quote_is != after_the_letter {
        return None;
    }
    Some(where_the_quote_is + 1 - at)
}

/// Where a string's opening stands after its first letter, and whether that letter made it
/// a raw string.
///
/// **`r` is the raw marker whether it stands alone or after a `b` or a `c`.** A `b` or a `c`
/// on its own opens a byte or a C string and takes no hashes — `b#"` is not a thing, and a
/// reader that accepted it would open a string that ends in a way the language never ends
/// one.
fn after_the_first_letter_of_a_string(the_bytes: &[u8], at: usize, first: u8) -> (usize, bool) {
    if !matches!(first, b'r' | b'b' | b'c') {
        return (at, false);
    }
    let after_the_letter = at + 1;
    if the_bytes.get(after_the_letter) == Some(&b'r') {
        return (after_the_letter + 1, true);
    }
    (after_the_letter, first == b'r')
}

/// Where the hashes of a raw string's opening stop, which is where its quote stands.
fn the_hashes_standing_at(the_bytes: &[u8], from: usize) -> usize {
    let mut where_to = from;
    while the_bytes.get(where_to) == Some(&b'#') {
        where_to += 1;
    }
    where_to
}

/// How many bytes open a character standing at `at`, or nothing if no character opens there.
///
/// **The difference from a lifetime is the closing apostrophe**, and getting it backwards
/// swallows the rest of the file: `let binding = 'a;` is a binding, and a walk that treats the
/// quote as opening a character never finds its close and reports the remainder of the file
/// as one literal. So a character is a quote, then either a backslash or a character, then a
/// quote — and a raw lifetime is a third case that is neither.
fn the_character_opening_at(the_bytes: &[u8], at: usize) -> Option<usize> {
    if the_bytes.get(at) != Some(&b'\'') {
        return None;
    }
    if the_bytes.get(at + 1) == Some(&b'\\') {
        return the_quote_closing_an_escape_from(the_bytes, at + 2).map(|end| end + 1 - at);
    }
    let the_character = the_character_standing_at(the_bytes, at + 1)?;
    let after = at + 1 + the_character.len_utf8();
    if the_bytes.get(after) == Some(&b'\'') {
        return Some(after + 1 - at);
    }
    None
}

/// The quote that closes an escape, which is the first one the escape does not itself hold.
fn the_quote_closing_an_escape_from(the_bytes: &[u8], from: usize) -> Option<usize> {
    let mut where_to = from;
    while let Some(one) = the_bytes.get(where_to) {
        if *one == b'\'' {
            return Some(where_to);
        }
        where_to += 1;
    }
    None
}

/// The character standing at `at`, as a character and not as a byte.
///
/// A letter outside ASCII is more than one byte, and a walk stepping one byte at a time
/// would walk past the closing apostrophe of `'→'` and swallow the rest of the file.
fn the_character_standing_at(the_bytes: &[u8], at: usize) -> Option<char> {
    let the_rest = std::str::from_utf8(the_bytes.get(at..)?).ok()?;
    the_rest.chars().next()
}
