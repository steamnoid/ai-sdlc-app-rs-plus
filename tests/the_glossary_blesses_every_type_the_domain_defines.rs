//! The gate for RULE 2.2: a type the system declares is a term the glossary blesses.
//!
//! A concept that is not in `docs/domain-glossary.md` does not get used yet, and a term in
//! the code that is not in it is a violation. **This gate enforces the second direction.**
//! The first stays a reviewer's job, because the glossary deliberately names things nobody
//! has built yet — `TheClock`, `ARecordOfARun` and `AnUntrustedTestSuite` — and a gate
//! that complained about those would have to be edited before anything could be written.
//!
//! ## What is a type here, and what is not
//!
//! A `pub` `struct`, `enum`, `trait` or `type`, read out of the stretches a source file is
//! made of and out of nothing else. **A comment is not code and a string is not code** —
//! a `///`, a `//!`, a `/* */` and a string all say what the file holds rather than what the
//! compiler is given — so a declaration written in one of them is a name a reader is being
//! *told* about, and this is not the gate that judges it:
//! `a_name_is_never_generic_vocabulary` already refuses a banned name found in a comment,
//! which is the opposite direction, and a file that discusses a type must not be read as
//! declaring one.
//!
//! ## The known state of the repository, which is not a passing rule
//!
//! **The system declares no type at all today.** All three crates under `crates/` hold
//! module documentation and nothing else, so the real assertion at the bottom of this file
//! has nothing to judge and passes for the wrong reason. That is left exactly as it is on
//! purpose, and neither of the two available repairs is taken: weakening the gate so that
//! the absence reads as agreement is the mistake this repository exists to not make, and
//! writing a type to make the gate non-vacuous is a cycle that belongs to the domain and
//! not to a gate. What is here instead is the evidence: the walk is asked what it found
//! before anything is judged, and the fixture
//! `a_type_the_glossary_does_not_name_is_reported_by_name` proves the judgement bites by
//! handing the checker a type the glossary has never heard of. **Delete this file and the
//! real assertion is worth nothing**, because it would be agreeing with whatever the walk
//! happened to reach.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use aisdlc_gates::scanning::{
    self, HowTheFileIsWritten, WhatIsWrittenThere, the_root_of_the_checkout, the_text_at,
};

/// The four keywords a type is declared with, and no others.
///
/// **A function is not in this list, and leaving it out is the second-most-expensive
/// mistake available in this file.** An earlier version of this rule asked the glossary to
/// bless `as_str` and `applies_to` as well, produced forty-three demands, and turned the
/// glossary into a list of every verb in the codebase and this check into a rubber stamp:
/// once every name the document held was a name the code could have, the gate could not
/// refuse anything, so it would have cost a cycle per edit to the glossary and reported
/// nothing at all.
///
/// A `mod` is not in the list either, for a different reason: `scanning` is a module, which
/// is a namespace a thing is written in rather than a thing.
const THE_TYPE_KEYWORDS: [&str; 4] = ["struct", "enum", "trait", "type"];

/// The heading of the section that is a list of prohibitions rather than a list of names.
/// It is removed before any name is read, and a full paragraph of this file says why.
const THE_HEADING_OF_THE_SECTION_OF_PROHIBITIONS: &str = "## Banned vocabulary";

/// The heading every section of the document opens with, which is how the end of the
/// section above is found.
const THE_HEADING_OF_ANY_SECTION: &str = "## ";

/// The directory the system itself lives in, which is what this gate reads.
const THE_DIRECTORY_OF_THE_SYSTEM: &str = "crates";

/// How many names the parse must have found before its count means anything.
///
/// Forty is well under what the document holds today and well over what a parse that read
/// nothing would hold, which is the whole purpose of the number: a guard that compared
/// nothing and agreed with anything would be green, and would be a lie.
const THE_FEWEST_NAMES_THE_PARSE_MUST_HAVE_FOUND: usize = 40;

/// A type the glossary does not bless, and the file it was declared in.
///
/// A pair rather than a bare string, because a failure that cannot name the file to open
/// is a failure nobody can act on.
struct ATypeTheGlossaryDoesNotBless {
    /// The file the type was declared in.
    file: PathBuf,
    /// The type as it was written there.
    type_name: String,
}

/// Letters and digits, lowercased, and nothing else.
///
/// The comparison this gate makes is a comparison of shapes rather than of spellings, so
/// `WorkItem`, `work_item` and `workItem` are one name to it, and the gate says so in
/// every failure it reports.
fn normalised(text: &str) -> String {
    text.chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// The document with the section of prohibitions taken out, which is the only form the
/// blessed names are read from.
///
/// **That removal is not optional, and this paragraph is here so that nobody can find it
/// optional later.** A prohibition read as a term is exactly backwards, and the first
/// version of this very guard proved it: the banned section explains its own rule in
/// backticks, the explanation blessed the example, and the guard passed the very name it
/// existed to catch. A guard whose own documentation can disable it is one careless edit to
/// the document away from being off.
fn the_document_with_the_banned_section_removed(glossary: &str) -> String {
    let mut reading_the_prohibitions = false;
    let mut remaining = String::new();
    for line in glossary.lines() {
        if line.starts_with(THE_HEADING_OF_THE_SECTION_OF_PROHIBITIONS) {
            reading_the_prohibitions = true;
            continue;
        }
        if line.starts_with(THE_HEADING_OF_ANY_SECTION) {
            reading_the_prohibitions = false;
        }
        if !reading_the_prohibitions {
            remaining.push_str(line);
            remaining.push('\n');
        }
    }
    remaining
}

/// Every bolded phrase, read between the first `**` and the second.
///
/// One bolded phrase is one entry rather than the words inside it, so a sentence the
/// document bolds is one thing it chose to say and not a name it declares.
fn bolded_entries(text: &str) -> Vec<String> {
    text.replace("**", "\u{1}")
        .split('\u{1}')
        .skip(1)
        .step_by(2)
        .map(String::from)
        .collect()
}

/// Every code span, which is where the glossary writes a name it blesses.
///
/// A fenced block is not a code span, and the only fence in the document today is inside
/// the section that is removed above, so nothing is stripped here. If a future edit adds a
/// fenced block, what it holds will be read as a name, and that is named here rather than
/// fixed by a line that would change nothing until then.
fn code_spans(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .map(String::from)
        .collect()
}

/// Every name the glossary blesses, in the order the document says them, normalised.
///
/// A name is allowed when it is one of these, whatever its last word is. This is the same
/// reading `a_name_is_never_generic_vocabulary` makes of the same document, and a second
/// reading would be two answers to one question.
///
/// **A cell in a column headed `Not` is a prohibition, and a prohibition is not a name.**
/// The document says so in its own "How this file is read" table, and the reading decides
/// by the table's header row. Without that decision this list contained `Task`, `Ticket`,
/// `Job`, `Entity`, `Status`, `State`, `move_to` and `advance` — the words the glossary
/// bans — so a synonym and a blessed name were the same set of words, and a synonym gate
/// reading the same document had nothing left to refuse. A prohibition read as a term is
/// exactly backwards, and this repository has now made that mistake twice in two shapes.
fn the_blessed_names() -> Vec<String> {
    let glossary = scanning::the_glossary_text();
    let without_the_prohibitions = the_document_with_the_banned_section_removed(&glossary);
    let without_the_refusals = the_document_with_the_not_columns_blanked(&without_the_prohibitions);
    let mut entries = bolded_entries(&without_the_refusals);
    entries.extend(code_spans(&without_the_refusals));
    entries.iter().map(|entry| normalised(entry)).collect()
}

/// The document with every table cell in a column headed `Not` emptied.
///
/// The decision is by the header row, so it does not care which tables carry the column,
/// which position it holds, or whether a table has one at all. A table the reading has not
/// seen a header for is left exactly as it is, because inventing a rule for a shape it has
/// not met is how a gate starts guessing.
fn the_document_with_the_not_columns_blanked(document: &str) -> String {
    let mut out = String::new();
    let mut not_column: Option<usize> = None;
    let mut row_in_this_table = 0;
    for line in document.lines() {
        out.push_str(&the_line_as_it_should_be_read(
            line,
            &mut not_column,
            &mut row_in_this_table,
        ));
        out.push('\n');
    }
    out
}

/// One line, read the way the rest of the reading reads it: a header names the column, a
/// separator is a separator, and anything after that has the refusal taken out of it.
fn the_line_as_it_should_be_read(
    line: &str,
    not_column: &mut Option<usize>,
    row_in_this_table: &mut usize,
) -> String {
    if !is_a_table_row(line) {
        *not_column = None;
        *row_in_this_table = 0;
        return line.to_string();
    }
    if *row_in_this_table == 0 {
        *not_column = the_column_a_table_refuses_in(line);
    }
    let read = if *row_in_this_table < 2 {
        line.to_string()
    } else {
        with_the_refusal_emptied(line, *not_column)
    };
    *row_in_this_table += 1;
    read
}

/// The position of a column headed `Not`, if the row is a header with one.
fn the_column_a_table_refuses_in(line: &str) -> Option<usize> {
    line.split('|')
        .map(str::trim)
        .position(|cell| cell.eq_ignore_ascii_case("not"))
}

/// A row of a table, as opposed to prose that happens to contain a vertical bar.
fn is_a_table_row(line: &str) -> bool {
    line.starts_with('|') && line.ends_with('|') && line.len() > 1
}

/// The row with a refusal taken out of it, and every other cell left exactly as written.
///
/// Splitting and joining is the whole implementation, and it keeps the padding of the
/// surviving cells: a reader comparing a refused cell against a blessed one must not be
/// able to tell them apart by a space.
fn with_the_refusal_emptied(line: &str, column: Option<usize>) -> String {
    let Some(column) = column else {
        return line.to_string();
    };
    let mut cells: Vec<String> = line.split('|').map(str::to_string).collect();
    if column < cells.len() {
        cells[column] = String::new();
    }
    cells.join("|")
}

/// Every type declared in `source`, read out of the stretches that are code.
///
/// A `Code` stretch holds one line of code and no line ending, so one stretch is one line
/// and is read as one. Nothing else in the file is read at all, which is what makes a
/// declaration written in documentation, in a block comment or in a string a name the
/// reader is told about rather than a name the file declares.
fn the_types_declared_in(source: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for stretch in HowTheFileIsWritten::of(source).stretches() {
        if stretch.kind() != WhatIsWrittenThere::Code {
            continue;
        }
        names.extend(the_types_declared_on_one_line(stretch.text()));
    }
    names
}

/// The types this one line of code declares.
///
/// The `pub` is looked for at every word rather than assumed to be the first, because
/// `#[derive(Clone)] pub struct WorkItemId(String);` is one line declaring one type, and
/// a gate that misses a declaration because of where a formatter put an attribute is a
/// gate that agrees with a formatting choice.
fn the_types_declared_on_one_line(code: &str) -> Vec<String> {
    let words = the_words_of_one_line(code);
    let mut names: Vec<String> = Vec::new();
    for (at, word) in words.iter().enumerate() {
        if word.as_str() == "pub" {
            collect_the_type_declared_after(&words, at + 1, &mut names);
        }
    }
    names
}

/// The type the declaration beginning at `at` introduces, added to `names`.
///
/// The three steps are read in order and the first two are optional: a parenthesised
/// visibility, a `const`, and then one of `THE_TYPE_KEYWORDS`. **Nothing is looked for
/// *somewhere ahead***, because `pub use domain::type_name;` holds the letters of a keyword
/// and declares no type at all — and a keyword is a whole word, which is why the words are
/// read first and the name is read out of them rather than out of the text.
fn collect_the_type_declared_after(words: &[String], at: usize, names: &mut Vec<String>) {
    let mut at = at;
    if words.get(at).is_some_and(|word| word.starts_with('(')) {
        at += 1;
    }
    if words.get(at).is_some_and(|word| word.as_str() == "const") {
        at += 1;
    }
    if let Some(name) = the_name_a_type_keyword_opens(words, at) {
        names.push(name);
    }
}

/// The name the keyword at `at` opens, or nothing when the word there opens no type.
///
/// The name is the run of letters and digits after the keyword, so `WorkItem<T>` and
/// `Stage;` are each the one name rather than the name and its punctuation: a unit struct
/// is `Stage` and not `Stage;`.
fn the_name_a_type_keyword_opens(words: &[String], at: usize) -> Option<String> {
    let keyword = words.get(at)?;
    if !THE_TYPE_KEYWORDS.contains(&keyword.as_str()) {
        return None;
    }
    let named: String = words
        .get(at + 1)?
        .chars()
        .take_while(|character| character.is_alphanumeric() || *character == '_')
        .collect();
    (!named.is_empty()).then_some(named)
}

/// The words one line of code is written in, with a parenthesised group held as one word.
///
/// `pub(crate)` is one step of a declaration and `pub(crate` is not, and the difference is
/// a bracket — which is why this cannot be `split_whitespace`, and why a bracket is also
/// where one word ends and the next begins. `#[derive(Clone)]` therefore reads as two words
/// and not as one, and the keyword after the attribute is still a word of its own.
fn the_words_of_one_line(code: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut how_many_brackets_are_open: usize = 0;
    for character in code.chars() {
        let closing = usize::from(character == ')');
        how_many_brackets_are_open = how_many_brackets_are_open.saturating_sub(closing);
        if how_many_brackets_are_open == 0 && begins_a_word(character) {
            words.push(std::mem::take(&mut current));
        }
        if !character.is_whitespace() {
            current.push(character);
        }
        how_many_brackets_are_open += usize::from(character == '(');
    }
    words.push(current);
    words.retain(|word| !word.is_empty());
    words
}

/// Whether this character begins a word rather than continuing the one before it.
///
/// White space begins a word and is not part of either, and so does the bracket that opens
/// a parenthesised group, because that group is one step of a declaration and the thing
/// before it is a word of its own. A closing bracket is neither: it belongs to the group it
/// closes, and the white space after it is what begins the next word.
fn begins_a_word(character: char) -> bool {
    character.is_whitespace() || character == '('
}

/// The types in one file the glossary does not bless, each with the file it came from.
///
/// The file is a parameter rather than something read from a path, because a fixture is not
/// on disk, and the only evidence this gate has that the judgement bites is a fixture.
fn the_types_the_glossary_does_not_bless_in(
    file: &Path,
    source: &str,
    blessed: &[String],
) -> Vec<ATypeTheGlossaryDoesNotBless> {
    the_types_declared_in(source)
        .into_iter()
        .filter(|type_name| !blessed.contains(&normalised(type_name)))
        .map(|type_name| ATypeTheGlossaryDoesNotBless {
            file: file.to_path_buf(),
            type_name,
        })
        .collect()
}

/// Every type the system declares that the glossary does not bless.
///
/// The document is read once here and handed down, because a walk that read six hundred
/// lines of it once per type would be a walk nobody would wait for.
fn the_types_the_system_does_not_bless() -> Vec<ATypeTheGlossaryDoesNotBless> {
    let blessed = the_blessed_names();
    let mut refused: Vec<ATypeTheGlossaryDoesNotBless> = Vec::new();
    for file in scanning::every_rust_file_of_the_system() {
        let source = the_text_at(&file);
        refused.extend(the_types_the_glossary_does_not_bless_in(
            &file, &source, &blessed,
        ));
    }
    refused
}

/// What the refusals say, one line each: the file, then the type.
///
/// Nothing at all when there is nothing to refuse, and that emptiness is the whole of what
/// a clean system gets to hear. It is the same emptiness a walk that read nothing would
/// produce, which is why the walk is asked what it found before this report is believed.
fn what_a_refusal_says(refused: &[ATypeTheGlossaryDoesNotBless]) -> String {
    let mut said = String::new();
    for one in refused {
        writeln!(said, "{}: {}", one.file.display(), one.type_name)
            .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

#[test]
fn the_glossary_offers_more_than_forty_blessed_names() {
    let blessed = the_blessed_names();
    assert!(
        blessed.len() > THE_FEWEST_NAMES_THE_PARSE_MUST_HAVE_FOUND,
        "the parse found {} names, and a list that short is a list it did not read",
        blessed.len()
    );
}

#[test]
fn the_glossary_blesses_the_five_types_the_system_starts_with() {
    let blessed = the_blessed_names();
    for named in ["WorkItem", "Stage", "Role", "Approval", "Repository"] {
        assert!(
            blessed.contains(&normalised(named)),
            "{named} is named in the glossary, and a gate that cannot see it is judging the wrong document"
        );
    }
}

#[test]
fn a_word_only_the_banned_section_names_is_not_blessed() {
    let blessed = the_blessed_names();
    for prohibited in ["Dto", "Payload", "Whatever"] {
        assert!(
            !blessed.contains(&normalised(prohibited)),
            "{prohibited} is named only inside the section of prohibitions, and a prohibition read as a term is exactly backwards"
        );
    }
}

#[test]
fn a_type_declared_in_code_is_found() {
    let source = "pub struct WorkItem {\n    stage: Stage,\n}\n";
    assert_eq!(
        the_types_declared_in(source),
        vec![String::from("WorkItem")],
        "a `pub struct` is the plain case, and a walk that found no declaration at all is a walk that reads nothing"
    );
}

#[test]
fn a_type_declared_inside_a_parenthesised_visibility_is_found() {
    let source = "pub(crate) enum Stage {\n    Idle,\n}\n";
    assert_eq!(
        the_types_declared_in(source),
        vec![String::from("Stage")],
        "the visibility is one step of a declaration and not a reason to read past the end of it"
    );
}

#[test]
fn a_const_between_the_visibility_and_the_keyword_is_stepped_over() {
    let source = "pub(crate) const type TheLongestPathComponent = usize;\n";
    assert_eq!(
        the_types_declared_in(source),
        vec![String::from("TheLongestPathComponent")],
        "the `const` is an optional step, and no line of Rust in this repository writes one, so this fixture is text rather than code that compiles"
    );
}

#[test]
fn a_type_declared_beside_an_attribute_is_found() {
    let source = "#[derive(Clone)] pub struct WorkItemId(String);\n";
    assert_eq!(
        the_types_declared_in(source),
        vec![String::from("WorkItemId")],
        "the keyword is looked for at every word and not assumed to be the first, so where a formatter put an attribute cannot change what this gate finds"
    );
}

#[test]
fn a_function_is_not_a_type_this_gate_judges() {
    assert!(
        the_types_declared_in("pub fn as_str(&self) -> &str;\n").is_empty(),
        "a function is not a type, and asking the glossary to bless one is what turned it into a list of every verb in the codebase"
    );
}

#[test]
fn a_module_is_not_a_type() {
    assert!(
        the_types_declared_in("pub mod scanning;\n").is_empty(),
        "a module is a namespace a thing is written in and not a thing, so `scanning` is not a type this gate judges"
    );
}

#[test]
fn a_constant_is_not_a_type() {
    let source = "pub const THE_LONGEST_A_COMPONENT_MAY_BE: usize = 255;\n";
    assert!(
        the_types_declared_in(source).is_empty(),
        "a constant has a name and no type of its own, and the `const` is stepped over rather than taken for a keyword"
    );
}

#[test]
fn a_declaration_without_a_pub_is_not_judged() {
    assert!(
        the_types_declared_in("struct Whatever;\n").is_empty(),
        "a private declaration is the implementation's own business, and the rule is about what a reader of the crate can name"
    );
}

#[test]
fn a_type_discussed_but_not_declared_is_not_judged() {
    let source = "\
/// The earlier shape was a `pub struct Whatever`, and this is what replaced it.
const EXAMPLE: &str = \"pub struct Whatever;\";
/* and `pub struct Whatever;` written here is not a declaration either */
pub struct AStageThatNobodyNamed;
";
    assert_eq!(
        the_types_declared_in(source),
        vec![String::from("AStageThatNobodyNamed")],
        "a name in documentation, in a block comment or in a string is a name the reader is told about, and this file discusses the ones this gate refuses constantly"
    );
}

#[test]
fn a_type_the_glossary_does_not_name_is_reported_by_name() {
    let source = "pub struct NothingTheGlossaryHasNamed;\n";
    let refused = the_types_the_glossary_does_not_bless_in(
        Path::new("a fixture that is not on disk"),
        source,
        &the_blessed_names(),
    );

    assert_eq!(
        what_a_refusal_says(&refused),
        "a fixture that is not on disk: NothingTheGlossaryHasNamed\n",
        "the whole report, because a gate that refuses without naming the file and the type is a gate nobody can act on, and because a report that named neither would pass this assertion"
    );
}

#[test]
fn a_type_the_glossary_does_name_is_not_reported() {
    let source = "pub struct Repository(String, String);\n";
    let refused = the_types_the_glossary_does_not_bless_in(
        Path::new("a fixture that is not on disk"),
        source,
        &the_blessed_names(),
    );

    assert!(
        refused.is_empty(),
        "a guard is exercised on what it must leave alone as well as on what it must catch, and a refusal here would be the gate refusing the one word the document blesses"
    );
}

#[test]
fn the_walk_read_the_crates_of_the_system() {
    let found = scanning::every_rust_file_of_the_system();
    assert!(
        !found.is_empty(),
        "the walk opened no .rs file under {}, so it agreed with a directory it never read",
        the_root_of_the_checkout()
            .join(THE_DIRECTORY_OF_THE_SYSTEM)
            .display()
    );
}

/// The real assertion, and the one that is worth keeping if every fixture above were
/// deleted.
///
/// **It judges nothing today, and that is a known state rather than a passing rule.** The
/// three crates under `crates/` declare no type at all, so this passes because there is
/// nothing to refuse and not because anything was checked. The two tests above it are what
/// make it worth reading: one says the walk reached the files, and one says the judgement
/// bites. Until the domain lands, this is a rule waiting for something to rule about.
#[test]
fn the_glossary_blesses_every_type_the_domain_defines() {
    let said = what_a_refusal_says(&the_types_the_system_does_not_bless());
    assert!(
        said.is_empty(),
        "these types are not terms the glossary blesses, by file and by name:\n{said}"
    );
}

/// Every word the glossary refuses, as the `Not` columns spell them.
///
/// A table rather than a list inside a test, because a table is a `const` and a `const`
/// has no body — and because a list of nineteen names written one per line is 28 counted
/// lines, which is the rule this repository refuses and which caught this very test when
/// the names were first written inline.
const EVERY_WORD_A_NOT_COLUMN_REFUSES: [&str; 19] = [
    "Task",
    "Ticket",
    "Job",
    "Entity",
    "Status",
    "State",
    "Phase",
    "Step",
    "Owner",
    "Assignee",
    "Team",
    "move_to",
    "set_stage",
    "advance",
    "change_state",
    "handle",
    "check",
    "verify",
    "assert_valid",
];

#[test]
fn a_word_in_a_not_column_is_a_refusal_and_not_a_blessed_name() {
    let blessed = the_blessed_names();
    for refused in EVERY_WORD_A_NOT_COLUMN_REFUSES {
        assert!(
            !blessed.contains(&normalised(refused)),
            "{refused} is written in a Not column, which is a prohibition, and a prohibition read as a term is a synonym gate with nothing left to refuse"
        );
    }
}

#[test]
fn a_word_in_an_ordinary_column_is_still_a_blessed_name() {
    let blessed = the_blessed_names();
    for name in ["WorkItem", "Stage", "Role", "Approval", "Repository"] {
        assert!(
            blessed.contains(&normalised(name)),
            "{name} is in the ordinary text of the document and the reader emptied a column it should not have emptied"
        );
    }
}

#[test]
fn a_prose_mention_of_a_refused_word_is_still_blessed() {
    let blessed = the_blessed_names();
    assert!(
        !blessed.contains(&normalised("WrongWithCase")),
        "a name that appears nowhere in the document is not in it, and this test is about the reader not inventing one"
    );
    assert!(
        blessed.contains(&normalised("AnArtifactVersion")),
        "a bolded entry in prose is blessed, because the reader empties cells and not words"
    );
}
