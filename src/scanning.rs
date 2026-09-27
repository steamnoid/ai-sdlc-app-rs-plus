//! How a file is written, one stretch at a time.
//!
//! A gate that reads source has to answer two questions: is there a comment here, and on
//! which line. Both are answered by taking the file apart into stretches, each carrying
//! what is written in it and the line it starts on, and a reader of that list never walks
//! the file a second time to get either answer.
//!
//! The walking is written out by hand, one byte at a time, because the whole point is
//! that a reader can follow it and disagree with it. There is no pattern here: a regular
//! expression cannot tell `//` inside a string from `//` inside a comment, and a rule
//! enforced by a thing nobody can read is a rule nobody can correct.
//!
//! Two mistakes are opposites and each hides the other. A file whose only comment is
//! inside a string read as clean loses a finding. A file with no comment at all read as
//! full of them teaches its reader to stop believing the gate, and then the first real
//! mistake costs nothing to make. `tests/the_scanner_tells_a_comment_from_a_string.rs`
//! asserts both on the same fixtures.

/// What is written at one place in a file.
///
/// There are five answers and no sixth, because a file is made of code, of what its
/// reader is told, of the text a value holds, of one character held by a value, and of
/// the space between them. A sixth would be a thing the language has and this list has
/// not heard of, and a reader could not tell whether it was one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsWrittenThere {
    /// What the file says to the compiler: the instructions a name is bound by, an
    /// operator, a semicolon, anything that is not one of the four below.
    Code,
    /// What the file says to its reader: a line comment, a block comment, and the two
    /// documentation forms, which are line comments that begin with a third slash or with
    /// an exclamation mark after the first two.
    AComment,
    /// The text a value holds, whatever opened it: a plain string, a raw string with any
    /// number of hashes, a byte string, or a byte string that is raw.
    AString,
    /// One character between apostrophes, whether it is written plainly or as an escape,
    /// and the byte that is one character.
    ACharacter,
    /// The white space between two stretches, which carries every line ending in the
    /// file. It is an answer rather than an absence because the line a stretch starts on
    /// is counted across it, and throwing it away would throw the counting away.
    TheSpaceBetween,
}

/// One stretch of a file: what is written in it, its text exactly as the file has it, and
/// the line it starts on.
///
/// Lines are counted from 1, and a stretch that starts on line 4 says 4 whatever
/// happened on the three lines above it. The line is a fact about the file rather than
/// about this reading, so it does not move because the reading took another path to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStretchOfTheFile {
    kind: WhatIsWrittenThere,
    text: String,
    line_it_starts_on: usize,
}

impl AStretchOfTheFile {
    /// A stretch carrying what is written in it, its text, and the line it begins on.
    ///
    /// Private because a stretch is made by reading a file and by nothing else: a caller
    /// that built one by hand would be asserting a classification nothing checked, which
    /// is the one thing a stretch exists to be.
    fn of(kind: WhatIsWrittenThere, text: &str, line_it_starts_on: usize) -> Self {
        Self {
            kind,
            text: text.to_string(),
            line_it_starts_on,
        }
    }

    /// What is written in this stretch, which is the only question a gate asks about it:
    /// a comment here, and nowhere else in the file, is a comment in the file.
    pub fn kind(&self) -> WhatIsWrittenThere {
        self.kind
    }

    /// The text exactly as the file has it, so a gate can quote the line it objects to
    /// rather than describe it and be wrong.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The line this stretch starts on, counted from 1.
    pub fn line_it_starts_on(&self) -> usize {
        self.line_it_starts_on
    }
}

/// A file taken apart into stretches, in the order they were written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowTheFileIsWritten {
    stretches: Vec<AStretchOfTheFile>,
}

impl HowTheFileIsWritten {
    /// Take `source` apart into stretches, in the order the file has them.
    ///
    /// Every byte of the file ends up in exactly one stretch and the stretches read back
    /// as the file itself, because a gate that quotes a stretch has to be quoting the
    /// file. The reading moves forward on every step, so this ends at the last byte
    /// whatever the file holds — including a quote or a comment that is never closed.
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
    /// This is the question a gate asks before it believes anything else about the file,
    /// and it is asked of the stretches rather than of the text: a file whose comment is
    /// inside a string has none, and a scan for two slashes says it has one.
    pub fn is_there_a_comment_anywhere(&self) -> bool {
        self.stretches
            .iter()
            .any(|stretch| stretch.kind() == WhatIsWrittenThere::AComment)
    }

    /// The line each comment in the file starts on, in the order they were written.
    ///
    /// A comment is named by the line it starts on rather than the line it ends on,
    /// because that is the line a reader points at.
    pub fn the_lines_a_comment_starts_on(&self) -> Vec<usize> {
        self.stretches
            .iter()
            .filter(|stretch| stretch.kind() == WhatIsWrittenThere::AComment)
            .map(AStretchOfTheFile::line_it_starts_on)
            .collect()
    }
}

/// The quote that opened a string, and the two things a reader has to know to find the
/// one that closes it.
///
/// Three plain fields and nothing that owns anything, so it is copied rather than passed
/// about by reference: a reader that is only going to look at it should not have to
/// reason about who owns it.
#[derive(Debug, Clone, Copy)]
struct AQuoteThatOpens {
    /// The byte the string's own text begins at: past the `r`, the hashes, the `b` and
    /// the quote itself.
    where_the_text_begins: usize,
    /// Whether this quote is raw, which is the only thing that decides whether a backslash
    /// in it quotes anything.
    is_raw: bool,
    /// How many hashes close it. Zero for a quote that is not raw, and possibly zero for
    /// one that is: `r"a"` and `"a"` are told apart by `is_raw` and not by this.
    how_many_hashes_close_it: usize,
}

impl AQuoteThatOpens {
    /// A quote that is closed by the next unescaped quote.
    fn plain(where_the_text_begins: usize) -> Self {
        Self {
            where_the_text_begins,
            is_raw: false,
            how_many_hashes_close_it: 0,
        }
    }

    /// A quote that is closed by a quote followed by as many hashes as opened it.
    fn raw(where_the_text_begins: usize, how_many_hashes_close_it: usize) -> Self {
        Self {
            where_the_text_begins,
            is_raw: true,
            how_many_hashes_close_it,
        }
    }
}

/// Where the reading of a file has got to: the byte it is at, and the line that byte is
/// on.
///
/// The two are kept together because they are the same walk. Every step forward is either
/// one byte or a run of bytes whose line endings are counted, and nothing else may move
/// either, so the line a stretch is given is the line the file has it on rather than a
/// number this reading arrived at by its own route.
struct WhereTheReadingHasGotTo<'a> {
    source: &'a str,
    at: usize,
    line: usize,
}

impl<'a> WhereTheReadingHasGotTo<'a> {
    /// A reading at the first byte of `source`, on line 1, because lines are counted
    /// from 1 and a file that says otherwise is off by one everywhere.
    fn at_the_first_byte_of(source: &'a str) -> Self {
        Self {
            source,
            at: 0,
            line: 1,
        }
    }

    /// Whether every byte of the file has been read.
    fn is_at_the_end(&self) -> bool {
        self.at >= self.source.len()
    }

    /// Whether the byte at `at` is the one named. A byte past the end of the file is not
    /// any byte, which is what answers a question asked at the end of a file.
    fn is_the_byte_at(&self, at: usize, byte: u8) -> bool {
        self.source.as_bytes().get(at) == Some(&byte)
    }

    /// How many bytes the character at `at` occupies, which is one for nearly everything
    /// a file is made of and more for a letter outside ASCII. A character literal such as
    /// a quoted arrow is more than one byte, and a reading that stepped one byte at a time
    /// through it would walk past the closing apostrophe and go on looking for one.
    fn how_wide_the_character_at(&self, at: usize) -> usize {
        self.source
            .get(at..)
            .and_then(|rest| rest.chars().next())
            .map_or(1, char::len_utf8)
    }

    /// The text from `where_it_began` to where the reading now is, counting the lines it
    /// crossed.
    ///
    /// Counting them here rather than at each line ending is what makes a backslash and an
    /// n inside a string not a line: a line ending is a byte that was really in the text
    /// that was read, and an escape is two characters and quotes the next one.
    fn the_text_from(&mut self, where_it_began: usize) -> &'a str {
        let how_far = self.at.min(self.source.len());
        let text = &self.source[where_it_began..how_far];

        self.line += text.bytes().filter(|byte| *byte == b'\n').count();
        text
    }

    /// The next stretch of the file, and the line it starts on.
    fn the_next_stretch(&mut self) -> AStretchOfTheFile {
        let line_it_starts_on = self.line;
        let (text, kind) = self.the_text_and_kind_of_the_next_stretch();

        AStretchOfTheFile::of(kind, text, line_it_starts_on)
    }

    /// Read the next stretch, which is the first of the five kinds that begins here, and
    /// the order is the order they are recognised in. Whatever none of them claims is
    /// code, and it is last because code is the one kind that begins everywhere else.
    fn the_text_and_kind_of_the_next_stretch(&mut self) -> (&'a str, WhatIsWrittenThere) {
        let where_it_began = self.at;

        if self.the_space_between_starts_here() {
            return (
                self.read_the_space_between(),
                WhatIsWrittenThere::TheSpaceBetween,
            );
        }
        if self.a_comment_starts_here() {
            return (self.read_the_comment(), WhatIsWrittenThere::AComment);
        }
        if let Some(opening) = self.the_quote_that_opens_a_string() {
            return (self.read_the_string(opening), WhatIsWrittenThere::AString);
        }
        if self.a_character_starts_here() {
            return (self.read_the_character(), WhatIsWrittenThere::ACharacter);
        }
        (self.read_the_code(where_it_began), WhatIsWrittenThere::Code)
    }

    /// Whether the space between two stretches begins here. In source that is a space, a
    /// tab, a carriage return or a line feed, and a byte past the end of the file is none
    /// of them, so this is false at the end without a question being asked.
    fn the_space_between_starts_here(&self) -> bool {
        matches!(
            self.source.as_bytes().get(self.at),
            Some(b' ' | b'\t' | b'\r' | b'\n')
        )
    }

    /// Whether a comment begins here, which is a slash and then a slash or a star. The
    /// two documentation forms begin with the same two characters and are the same kind of
    /// thing, so they are not looked for separately.
    fn a_comment_starts_here(&self) -> bool {
        self.a_line_comment_starts_here() || self.a_block_comment_starts_here()
    }

    /// Whether a line comment begins here.
    fn a_line_comment_starts_here(&self) -> bool {
        self.is_the_byte_at(self.at, b'/') && self.is_the_byte_at(self.at + 1, b'/')
    }

    /// Whether a block comment begins here.
    fn a_block_comment_starts_here(&self) -> bool {
        self.is_the_byte_at(self.at, b'/') && self.is_the_byte_at(self.at + 1, b'*')
    }

    /// Whether a block comment ends at the reading's own position.
    fn a_block_comment_ends_here(&self) -> bool {
        self.is_the_byte_at(self.at, b'*') && self.is_the_byte_at(self.at + 1, b'/')
    }

    /// Whether one of the three kinds that is neither code nor the space between begins at
    /// the reading's own position.
    ///
    /// The code reading and the order the kinds are recognised in read this one list, so
    /// that a kind one of them named and the other did not would be read as code and
    /// swallowed. A string swallowed is a file with no comments in it, and nothing else
    /// in the reading would notice.
    fn a_classified_thing_begins_here(&self) -> bool {
        self.a_comment_starts_here()
            || self.the_quote_that_opens_a_string().is_some()
            || self.a_character_starts_here()
    }

    /// Whether a line ends at the reading's own position. A line ending belongs to no
    /// line's code, and it is what makes one stretch's line differ from its neighbour's.
    fn a_line_ending_starts_here(&self) -> bool {
        self.is_the_byte_at(self.at, b'\n')
    }

    /// Whether the byte at `at` is white space that is not a line ending: a space, a tab,
    /// or a carriage return. Those three are what stands between two things on one line,
    /// and the run of them is a stretch of its own rather than part of either thing.
    fn is_the_white_space_at(&self, at: usize) -> bool {
        matches!(self.source.as_bytes().get(at), Some(b' ' | b'\t' | b'\r'))
    }

    /// How many hashes stand at `at`, counted while they are hashes. A raw quote is closed
    /// by as many of as opened it, so this is how the two ends are compared.
    fn how_many_hashes_are_at(&self, at: usize) -> usize {
        let mut counted = 0;

        while self.is_the_byte_at(at + counted, b'#') {
            counted += 1;
        }
        counted
    }

    /// The quote that opens a string here, and nothing else: a quote, a `b` and a quote,
    /// or a raw quote with any number of hashes and with or without that `b`.
    fn the_quote_that_opens_a_string(&self) -> Option<AQuoteThatOpens> {
        if self.is_the_byte_at(self.at, b'"') {
            return Some(AQuoteThatOpens::plain(self.at + 1));
        }
        if self.is_the_byte_at(self.at, b'b') && self.is_the_byte_at(self.at + 1, b'"') {
            return Some(AQuoteThatOpens::plain(self.at + 2));
        }
        self.the_raw_quote_that_opens_here()
    }

    /// The raw quote that opens here: an `r`, any number of hashes, and a quote. A `b` in
    /// front of the `r` is a prefix rather than a word, so it is stepped over.
    ///
    /// The character after the hashes is part of the rule and not a detail of it, because
    /// an `r` and a hash in front of an apostrophe is a raw lifetime. This is the third
    /// case that looks like a string and is not one, and a reading that takes it for a
    /// raw string looks for a closing quote that is not there and reports the rest of the
    /// file as one long string — which is a file with no comments in it and no code
    /// either, and a gate that believes it stops reading.
    fn the_raw_quote_that_opens_here(&self) -> Option<AQuoteThatOpens> {
        let after_the_b = if self.is_the_byte_at(self.at, b'b') {
            self.at + 1
        } else {
            self.at
        };
        if !self.is_the_byte_at(after_the_b, b'r') {
            return None;
        }
        let after_the_r = after_the_b + 1;
        let the_hashes = self.how_many_hashes_are_at(after_the_r);
        if !self.is_the_byte_at(after_the_r + the_hashes, b'"') {
            return None;
        }
        Some(AQuoteThatOpens::raw(
            after_the_r + the_hashes + 1,
            the_hashes,
        ))
    }

    /// Whether a character begins here: an apostrophe and a `b`, or an apostrophe alone.
    ///
    /// It is never a lifetime. An apostrophe and a name is the compiler's, and the
    /// difference is the apostrophe that closes it, which a lifetime does not have until
    /// much later in the file or not at all. Reading a lifetime as a character is the
    /// mistake this whole test file is about, because a character that is never closed
    /// swallows every comment after it.
    fn a_character_starts_here(&self) -> bool {
        if self.is_the_byte_at(self.at, b'b') {
            return self.is_the_byte_at(self.at + 1, b'\'')
                && self.an_apostrophe_begins_a_character(self.at + 1);
        }
        self.is_the_byte_at(self.at, b'\'') && self.an_apostrophe_begins_a_character(self.at)
    }

    /// Whether the apostrophe at `at` is followed by one character and then an apostrophe,
    /// or by a backslash and an escape. Anything else after an apostrophe is a lifetime.
    fn an_apostrophe_begins_a_character(&self, at: usize) -> bool {
        if self.is_the_byte_at(at + 1, b'\\') {
            return true;
        }
        if at + 1 >= self.source.len() {
            return false;
        }
        self.is_the_byte_at(at + 1 + self.how_wide_the_character_at(at + 1), b'\'')
    }

    /// Read the space between two stretches: every space, tab, carriage return and line
    /// feed until something else begins.
    fn read_the_space_between(&mut self) -> &'a str {
        let where_it_began = self.at;

        while !self.is_at_the_end() && self.the_space_between_starts_here() {
            self.at += 1;
        }
        self.the_text_from(where_it_began)
    }

    /// Read a comment, which is either the rest of the line or everything to the close
    /// that matches the open.
    fn read_the_comment(&mut self) -> &'a str {
        if self.a_block_comment_starts_here() {
            return self.read_the_block_comment();
        }
        self.read_the_line_comment()
    }

    /// Read a line comment: everything up to the line ending, and not the line ending
    /// itself, so the line ending is the space between this comment and the next thing
    /// written.
    fn read_the_line_comment(&mut self) -> &'a str {
        let where_it_began = self.at;

        while !self.is_at_the_end() && !self.is_the_byte_at(self.at, b'\n') {
            self.at += 1;
        }
        self.the_text_from(where_it_began)
    }

    /// Read a block comment: everything to the close that matches the open, counting the
    /// opens inside it.
    ///
    /// Block comments nest, so the first close is not the end of the comment. A reading
    /// that stopped there would call the rest of the comment code, and then report any
    /// two slashes it found inside the comment as a comment of its own — a comment inside
    /// a comment, on a file that has one comment.
    fn read_the_block_comment(&mut self) -> &'a str {
        let where_it_began = self.at;
        self.at += 2;
        let mut how_many_opens_are_unclosed = 1;

        while !self.is_at_the_end() && how_many_opens_are_unclosed > 0 {
            if self.a_block_comment_starts_here() {
                how_many_opens_are_unclosed += 1;
                self.at += 2;
            } else if self.a_block_comment_ends_here() {
                how_many_opens_are_unclosed -= 1;
                self.at += 2;
            } else {
                self.at += 1;
            }
        }
        self.the_text_from(where_it_began)
    }

    /// Read a string, from whatever opened it to whatever closes it.
    fn read_the_string(&mut self, opening: AQuoteThatOpens) -> &'a str {
        let where_it_began = self.at;
        self.at = opening.where_the_text_begins;

        if opening.is_raw {
            self.read_on_to_the_hashes_that_close(&opening);
        } else {
            self.read_on_to_the_quote_that_closes(b'"');
        }
        self.the_text_from(where_it_began)
    }

    /// Read on until the quote and the hashes that close this raw string, and past them.
    /// Nothing in a raw string is an escape, so a backslash quotes nothing here and a
    /// quote in the middle of the string is a character of it.
    fn read_on_to_the_hashes_that_close(&mut self, opening: &AQuoteThatOpens) {
        while !self.is_at_the_end() {
            if self.the_quote_and_the_hashes_close_here(opening) {
                self.at += 1 + opening.how_many_hashes_close_it;
                break;
            }
            self.at += 1;
        }
    }

    /// Whether the quote at the reading's own position, and the hashes after it, are the
    /// ones that close this raw string. A quote followed by fewer hashes is a character
    /// of the string, and one followed by more opens a longer raw string.
    fn the_quote_and_the_hashes_close_here(&self, opening: &AQuoteThatOpens) -> bool {
        self.is_the_byte_at(self.at, b'"')
            && self.how_many_hashes_are_at(self.at + 1) == opening.how_many_hashes_close_it
    }

    /// Read on until the byte that closes this quote, and past it.
    ///
    /// A backslash quotes the byte after it, so an escaped quote does not close anything.
    /// A reading that forgot that would end a string at an escaped quote and read the rest
    /// of the line as code, including any two slashes it holds — a comment inside a
    /// string, on a file that has none.
    fn read_on_to_the_quote_that_closes(&mut self, what_closes_it: u8) {
        let mut an_escape_is_open = false;

        while !self.is_at_the_end() {
            let character = self.source.as_bytes()[self.at];
            self.at += 1;

            if an_escape_is_open {
                an_escape_is_open = false;
                continue;
            }
            if character == b'\\' {
                an_escape_is_open = true;
                continue;
            }
            if character == what_closes_it {
                break;
            }
        }
    }

    /// Read a character: the apostrophe, the one character it holds, and the apostrophe
    /// that closes it.
    ///
    /// The escape and the plain character are read apart because they are different
    /// lengths and neither is one byte: a backslash quotes whatever follows it, and a
    /// letter outside ASCII is more than one byte. Both are read by what closes them
    /// rather than by counting bytes, which is what keeps a quoted arrow from swallowing
    /// the rest of the file.
    fn read_the_character(&mut self) -> &'a str {
        let where_it_began = self.at;

        if self.is_the_byte_at(self.at, b'b') {
            self.at += 1;
        }
        self.at += 1;

        if self.is_the_byte_at(self.at, b'\\') {
            self.read_on_to_the_quote_that_closes(b'\'');
        } else {
            self.at += self.how_wide_the_character_at(self.at) + 1;
        }
        self.the_text_from(where_it_began)
    }

    /// Read code: the run of bytes holding no line ending and none of the three kinds that
    /// is not code, less the white space at the end of it.
    ///
    /// The space between two words stays inside the code, because it is part of the
    /// instruction the compiler reads. The run of white space that ends the run is left
    /// behind: that run is the space between this code and whatever is written next, and
    /// it is a stretch of its own. Reading `let x = 1;` as four words and three spaces
    /// would make the gate reassemble every line before it could quote one.
    ///
    /// The one byte taken when the run is empty is the promise that the reading always
    /// moves. Nothing above it can leave the reading standing still, because this is
    /// reached only when none of the three kinds begins here, and a reading that stood
    /// still would never end — and this reads every file of every repository it is
    /// pointed at.
    fn read_the_code(&mut self, where_it_began: usize) -> &'a str {
        while !self.is_at_the_end()
            && !self.a_line_ending_starts_here()
            && !self.a_classified_thing_begins_here()
        {
            self.at += 1;
        }
        if self.at > where_it_began {
            self.step_back_over_the_white_space_at_the_end();
        }
        if self.at == where_it_began {
            self.at += 1;
        }
        self.the_text_from(where_it_began)
    }

    /// Step back over the white space at the end of what was read, leaving that run for
    /// the stretch that follows it. A line ending is not stepped over: it is where one
    /// line ends and the next begins, and a stretch that crossed it would be given one
    /// line for text standing on two.
    fn step_back_over_the_white_space_at_the_end(&mut self) {
        while self.at > 0 && self.is_the_white_space_at(self.at - 1) {
            self.at -= 1;
        }
    }
}
