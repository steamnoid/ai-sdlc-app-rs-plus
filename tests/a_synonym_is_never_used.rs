//! The gate for RULE 2.1: a synonym is never used.
//!
//! One concept has one name, everywhere. If `WorkItem` is the term then Task, Ticket,
//! Job and Entity are all wrong, and if `transition` is the verb then `move_to`,
//! `set_stage` and `advance` are all wrong. **The words that are wrong are read out of
//! `docs/domain-glossary.md` and not written here**, so the guard follows the document
//! instead of drifting from it — a list restated in Rust is a list that was correct on
//! the day it was written and says nothing about the document afterwards.
//!
//! ## The reading: the `Not` columns, and nothing else
//!
//! The document writes a refusal down in a cell in a column headed `Not`, and a refused
//! word appears **once, in such a cell, and nowhere else in backticks**. That is not a
//! convention this file invented: it is the document's own obligation, stated in its
//! "How this file is read" section, and it is what makes a gate possible at all. So the
//! refused set is every code span standing in a `Not` column, and the decision is by the
//! **header row** — which table carries the column and where in the table it stands are
//! both questions this gate does not ask.
//!
//! **A second reading was tried and the document refused it.** The block of
//! `## Banned vocabulary` holds another thirty-six refused words, and reading it as well
//! makes this gate a *second* gate — and it breaks a name the document blesses by shape:
//! `a_work_item_is_born_queued_and_names_who_asked` is made of the word `item`, and that
//! name is one this gate must leave alone. The gate that reads the block is
//! `a_name_is_never_generic_vocabulary`, which judges the last word of a name rather
//! than any word of it, and two gates judging the same document two different ways is
//! what this repository has already paid for twice.
//!
//! **What that costs, written down rather than discovered.** Four words the document
//! calls wrong are not in a `Not` column, so this gate cannot refuse them: Task, Ticket
//! and Job are named in prose, and Entity is named in the block of prohibitions. A
//! refused word this gate cannot refuse is a gap in the document and not in the reading —
//! the document has to write it in a `Not` column — and
//! `a_word_no_not_column_mentions_cannot_be_refused` is the test that names the gap and
//! that watches it.
//!
//! ## What is read, and what is not
//!
//! Only the stretches of a file that are **code**, and only the names a `pub` declares. A
//! synonym inside a `///` is this repository talking about a name it refuses, and this
//! repository talks about `move_to` and `Task` and `Status` constantly and on purpose, so
//! a reading that took a documentation line for a declaration would refuse this repository
//! for discussing itself. A synonym inside a string is a fixture, and a fixture this gate
//! could read would be a fixture it judged.
//!
//! ## Why a name is judged by its words
//!
//! A synonym is a **word somebody reached for**, so the judgement is on the words a name
//! is made of and not on the whole spelling. `advance_the_work_item` reaches for
//! `advance` and is a perfectly good name otherwise, and a rule that only caught a name
//! spelled exactly as the document spells the synonym would be walked past by the first
//! person to add a prefix to one.
//!
//! And the document's blessing is asked **first**: `validate_state` is made of the word
//! `state`, `state` is refused by the stages column, and a gate that asked the words
//! before the blessing would refuse the one verb that asserts the invariants.

use std::path::{Path, PathBuf};

use aisdlc_gates::scanning::{self, HowTheFileIsWritten, WhatIsWrittenThere, the_text_at};

/// The seven keywords a name is declared with, and no others.
///
/// A `use` is not one of them, because a name brought in from elsewhere was declared
/// somewhere this gate does not read and a gate that judges another file's names is
/// judging the wrong thing. That is the same argument the two gates beside this one
/// make, and it is why the walking asks for a `pub`.
const THE_KEYWORDS_A_NAME_IS_DECLARED_WITH: [&str; 7] =
    ["struct", "enum", "trait", "type", "fn", "mod", "const"];

/// The header a table column carries when what it holds is refused rather than named.
///
/// The decision is by the header row and by nothing else, so which table carries the
/// column and where in the table it stands are both questions this gate does not ask.
const THE_COLUMN_THAT_REFUSES: &str = "Not";

/// How many refused words the parse must have found before its size means anything.
///
/// This is the number the document's three columns hold today, and a floor rather than
/// an exact count on purpose: the count a reader wants is the count the failure message
/// prints, and a gate that refused an edit to the glossary for adding a prohibition would
/// be a gate about the size of a table rather than about the words in it. What the floor
/// refuses is the opposite — a parse that found nothing, or nearly nothing, and a gate
/// that compared an empty set with an empty file and found them in agreement.
const THE_FEWEST_REFUSED_WORDS_THE_PARSE_MUST_HAVE_FOUND: usize = 15;

/// How many files the walk must have opened before it has earned the right to agree with
/// all of them. A gate that read nothing and said nothing is a green run that is a lie.
const THE_FEWEST_FILES_THE_WALK_MUST_HAVE_FOUND: usize = 5;

/// How many names the walk must have judged for the same reason, and one more that is
/// particular to this gate: a walk that opened the files and then read no declaration out
/// of them agrees with a workspace just as completely as one that opened nothing.
///
/// **It is a low number on purpose, and the count today is eleven.** All but one of them
/// is in the gates' own scanner, because `crates/` declares no type at all yet — the same
/// known state `the_glossary_blesses_every_type_the_domain_defines` writes down at
/// length. A floor set where the count happens to be would be a tripwire on a rename.
const THE_FEWEST_NAMES_THE_WALK_MUST_HAVE_JUDGED: usize = 8;

/// The names a refused word must refuse.
///
/// **A table of strings rather than a table of `const`s**, because a refused word written
/// in a string is a fixture: this gate reads code, and a fixture it could read would be a
/// fixture it judged.
const THE_NAMES_A_REFUSED_WORD_MUST_REFUSE: [&str; 18] = [
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
    "work_item_status",
    "work_item_state",
    "advance_the_work_item",
];

/// The names a refused word must leave alone.
///
/// Every one of them is a name the document blesses, and two of them — `validate_state`
/// and `state`-bearing names generally — are made of words the document *refuses*. They
/// are here because a gate is exercised on what it must leave alone as well as on what it
/// must catch, and a guard that has only ever been seen to refuse is not known to be able
/// to do anything else.
const THE_NAMES_A_REFUSED_WORD_MUST_ALLOW: [&str; 11] = [
    "WorkItem",
    "Stage",
    "Role",
    "Approval",
    "transition",
    "validate_state",
    "legal_transitions_from",
    "the_blessed_names",
    "a_work_item_is_born_queued_and_names_who_asked",
    "AnArtifactVersion",
    "the_root_of_the_checkout",
];

/// The words the document calls wrong and writes in no `Not` column, so that there is
/// nothing in them for this gate's reading to find.
///
/// They are here so that the gap the module documentation names is a table rather than a
/// paragraph, and so that a document which starts writing one of them in a `Not` column
/// is caught by the test that reads this table rather than by a reader noticing.
const THE_WORDS_NO_NOT_COLUMN_MENTIONS: [&str; 4] = ["Task", "Ticket", "Job", "Entity"];

/// A name the walk found, with the file it was declared in.
///
/// The two are held together because a refusal that cannot name the file to open is a
/// refusal nobody can act on, and because the count of these is what says the walk looked
/// at something.
struct ANameDeclared {
    /// The file the name was declared in.
    file: PathBuf,
    /// The name as it was written there.
    name: String,
}

/// Letters and digits, lowercased, and nothing else.
///
/// Every comparison this gate makes is a comparison of shapes rather than of spellings, so
/// `WorkItem`, `work_item` and `WORK_ITEM` are one name to it and it says so in every
/// failure it reports.
fn normalised(text: &str) -> String {
    text.chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// The words a name is made of: split on an underscore, and on a letter that follows a
/// lowercase one, which are the two breaks `work_item_status` and `WorkItemStatus`
/// between them both need.
fn words_of(name: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut previous: Option<char> = None;
    for character in name.chars() {
        if begins_a_word(character, previous) {
            words.push(normalised(&current));
            current.clear();
        }
        if character != '_' {
            current.push(character);
        }
        previous = Some(character);
    }
    words.push(normalised(&current));
    words.retain(|word| !word.is_empty());
    words
}

/// Whether this character opens a word rather than continuing the one before it. It is a
/// function of its own so that `words_of` reads as the two rules it enforces rather than
/// as a loop with a condition in it.
fn begins_a_word(character: char, previous: Option<char>) -> bool {
    character == '_' || (character.is_uppercase() && previous.is_some_and(char::is_lowercase))
}

/// The words the document refuses: every code span standing in a column headed `Not`, with
/// the ones said twice counted once so that a set can be compared with a reading of it.
fn the_refused_words() -> Vec<String> {
    let words = the_words_written_in_a_not_column(&scanning::the_glossary_text());
    let mut every: Vec<String> = words.iter().map(|word| normalised(word)).collect();
    every.sort();
    every.dedup();
    every
}

/// Every code span in a cell standing in a column headed `Not`, in the order the document
/// writes them.
///
/// A line that is not a row of a table ends the table it was in, and a table with no such
/// column is read for nothing: inventing a rule for a shape the reading has not met is
/// how a gate starts guessing.
fn the_words_written_in_a_not_column(document: &str) -> Vec<String> {
    let mut refused: Vec<String> = Vec::new();
    let mut column: Option<usize> = None;
    let mut row_in_this_table = 0;
    for line in document.lines() {
        let Some(cells) = the_cells_of_a_row(line) else {
            column = None;
            row_in_this_table = 0;
            continue;
        };
        if row_in_this_table == 0 {
            column = the_column_a_table_refuses_in(&cells);
        }
        if let Some(cell) = column.and_then(|at| cells.get(at)) {
            refused.extend(code_spans(cell));
        }
        row_in_this_table += 1;
    }
    refused
}

/// Where the column headed `Not` stands in a header row, or nothing at all when the row is
/// a header that does not carry one.
fn the_column_a_table_refuses_in(header: &[&str]) -> Option<usize> {
    header
        .iter()
        .position(|cell| *cell == THE_COLUMN_THAT_REFUSES)
}

/// The cells of one row of a table, or nothing at all when the line is not one.
///
/// Prose that happens to hold a vertical bar is not a row, and a row is a line that opens
/// with a bar and closes with one and has something written between them.
fn the_cells_of_a_row(line: &str) -> Option<Vec<&str>> {
    if !(line.starts_with('|') && line.ends_with('|') && line.len() > 1) {
        return None;
    }
    Some(line.split('|').map(str::trim).collect())
}

/// Every name the document blesses, with the refused words taken out first.
///
/// **Taking the refused words out is not optional, and the reason is the document's own.**
/// A cell in a `Not` column is written in code spans, so a reading that took every code
/// span as a blessed name put `Status`, `State` and `move_to` in the blessed set, and a
/// synonym and a blessed name were the same set of words — which leaves a gate that
/// refuses synonyms with nothing at all to refuse. The sibling gate empties the cells
/// before it reads; this one subtracts the words, and **the two agree because of the
/// obligation the document carries**: a refused word appears once, in a `Not` column, and
/// nowhere else in backticks. A code span outside one of those columns that holds a
/// refused word is a cell this reading cannot tell from one, and taking the word out is
/// the same taking-out either way.
fn the_blessed_names(refused: &[String]) -> Vec<String> {
    let document = scanning::the_glossary_text();
    let mut entries = bolded_entries(&document);
    entries.extend(code_spans(&document));
    let mut every: Vec<String> = entries.iter().map(|entry| normalised(entry)).collect();
    every.retain(|name| !refused.contains(name));
    every
}

/// Every bolded phrase, read between the first `**` and the second, so that a sentence
/// the document bolds is one thing it chose to say and not a name it declares.
fn bolded_entries(text: &str) -> Vec<String> {
    text.replace("**", "\u{1}")
        .split('\u{1}')
        .skip(1)
        .step_by(2)
        .map(String::from)
        .collect()
}

/// Every code span, which is where the glossary writes a name it blesses and where a
/// column headed `Not` writes a word it refuses.
fn code_spans(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .map(String::from)
        .collect()
}

/// The refused word this name reaches for, and nothing at all when the document blesses
/// the name or when the name reaches for no refused word.
///
/// **The judgement is on the words a name is made of and not on the whole spelling**,
/// because a synonym is a word somebody reached for: `advance_the_work_item` reaches for
/// `advance` and is a perfectly good name otherwise, and a rule that only caught a name
/// spelled exactly as the document spells the synonym would be walked past by the first
/// person to put a prefix in front of one.
///
/// The blessing is asked first and it is asked separately for the same reason:
/// `validate_state` is made of the refused word `state`, and a gate that asked the words
/// first would refuse the one verb that asserts the invariants of a work item's state.
fn the_refused_word_in(name: &str, refused: &[String], blessed: &[String]) -> Option<String> {
    if blessed.contains(&normalised(name)) {
        return None;
    }
    let whole = normalised(name);
    if refused.contains(&whole) {
        return Some(whole);
    }
    words_of(name)
        .into_iter()
        .find(|word| refused.contains(word))
}

/// The names this source declares to whatever may read the crate, read out of the
/// stretches that are code and out of nothing else.
///
/// A `Code` stretch holds one line of code and no line ending, so one stretch is one line
/// and is read as one. Nothing else in the file is read at all, which is what makes a
/// declaration written in documentation, in a block comment or in a string a name the
/// reader is told about rather than a name the file declares.
///
/// **The cost of asking for a `pub`, which is what a private name costs.** A name only
/// the inside of a file can see is not judged here, and `src/scanning.rs` holds one today
/// that reaches for a refused word: `step_back_over_the_white_space_at_the_end` reaches
/// for `step`, and `step` is refused by the stages column because three different words
/// have been used for the same fact across two implementations and one of the three is
/// this one. The reading cannot tell that name from a synonym, the file is
/// not this gate's to change, and a gate that reported it on every run would be reported
/// and ignored. That is a reviewer's finding and this file records it rather than
/// pretending the reading is sharper than it is.
fn the_public_names_declared_in(source: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for stretch in HowTheFileIsWritten::of(source).stretches() {
        if stretch.kind() != WhatIsWrittenThere::Code {
            continue;
        }
        names.extend(the_public_names_declared_on_one_line(stretch.text()));
    }
    names
}

/// The names one line of code declares, which is every name a `pub` on the line declares.
///
/// The `pub` is looked for at every word rather than assumed to be the first, because
/// `#[derive(Clone)] pub struct WorkItemId(String);` is one line declaring one type, and
/// a gate that misses a declaration because of where a formatter put an attribute is a
/// gate that agrees with a formatting choice.
fn the_public_names_declared_on_one_line(code: &str) -> Vec<String> {
    let words = the_words_of_one_line(code);
    let mut names: Vec<String> = Vec::new();
    for (at, word) in words.iter().enumerate() {
        if word.as_str() == "pub" {
            let keyword = the_place_the_keyword_begins_in(&words, at + 1);
            names.extend(the_name_the_keyword_after_declares(&words, keyword));
        }
    }
    names
}

/// The place the keyword begins in, which is one word on when a visibility is written in
/// brackets: `pub(crate)` is one step of a declaration and `pub(crate` is not, and the
/// difference is a bracket.
///
/// A `const` is not stepped over, because `pub const THE_ROOT` declares a constant and
/// `const` is the keyword that declares it rather than a modifier in front of one.
fn the_place_the_keyword_begins_in(words: &[String], at: usize) -> usize {
    if words.get(at).is_some_and(|word| word.starts_with('(')) {
        return at + 1;
    }
    at
}

/// The name the keyword standing at `at` declares, or nothing when the word there opens
/// no declaration or is followed by no name.
///
/// Nothing is looked for *somewhere ahead*, because `pub use domain::WorkItem;` holds the
/// letters of a keyword and declares no name at all — and a keyword is a whole word, which
/// is why the words are read first and the name is read out of them rather than out of the
/// text.
fn the_name_the_keyword_after_declares(words: &[String], at: usize) -> Option<String> {
    let keyword = words.get(at)?;
    if !THE_KEYWORDS_A_NAME_IS_DECLARED_WITH.contains(&keyword.as_str()) {
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
/// `pub(crate)` is one step of a declaration and `pub(crate` is not, and the difference
/// is a bracket — which is why this cannot be `split_whitespace`, and why a bracket is
/// also where one word ends and the next begins.
fn the_words_of_one_line(code: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut how_many_brackets_are_open: usize = 0;
    for character in code.chars() {
        let closing = usize::from(character == ')');
        how_many_brackets_are_open = how_many_brackets_are_open.saturating_sub(closing);
        if how_many_brackets_are_open == 0 && begins_a_step(character) {
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
/// White space begins a word and is part of neither, and so does the bracket that opens a
/// parenthesised group, because that group is one step of a declaration and the word
/// before it is a word of its own.
fn begins_a_step(character: char) -> bool {
    character.is_whitespace() || character == '('
}

/// Every name declared in the workspace, each with the file it was declared in.
///
/// The files are handed in rather than walked again so that a test can ask what the walk
/// found and what the walk judged from one reading of the disk and not two.
fn every_name_in_the_workspace(files: &[PathBuf]) -> Vec<ANameDeclared> {
    let mut every: Vec<ANameDeclared> = Vec::new();
    for file in files {
        every.extend(the_names_in(file));
    }
    every
}

/// One file's worth, with the file each name came from. The text comes from the shared
/// reader, so a file that cannot be read stops the walk by saying so rather than passing
/// it in silence.
fn the_names_in(file: &Path) -> Vec<ANameDeclared> {
    the_public_names_declared_in(&the_text_at(file))
        .into_iter()
        .map(|name| ANameDeclared {
            file: file.to_path_buf(),
            name,
        })
        .collect()
}

/// The refusals among the names the walk found, one line each, naming the file, the name
/// and the refused word inside it.
///
/// All three, because a refusal naming only the name leaves a reader hunting for which
/// file to open and a refusal naming only the file leaves a reader hunting for what to
/// change.
fn the_refusals_in(names: &[ANameDeclared], refused: &[String], blessed: &[String]) -> Vec<String> {
    let mut said: Vec<String> = Vec::new();
    for found in names {
        let Some(word) = the_refused_word_in(&found.name, refused, blessed) else {
            continue;
        };
        said.push(format!(
            "{}: {} reaches for {}",
            found.file.display(),
            found.name,
            word
        ));
    }
    said
}

/// What the walk reached, asked before anything the walk judged is believed.
///
/// **A gate that read nothing and agreed with all of it is green**, and a green run of
/// this file is exactly what a reader would be told is a proof. Two counts and not one,
/// because a walk that opened the files and then read no declaration out of them agrees
/// with a workspace just as completely as one that opened nothing.
fn assert_the_walk_reached_something(files: &[PathBuf], names: &[ANameDeclared]) {
    assert!(
        files.len() > THE_FEWEST_FILES_THE_WALK_MUST_HAVE_FOUND,
        "the walk opened {} files, and a walk that reached nothing agrees with everything",
        files.len()
    );
    assert!(
        names.len() > THE_FEWEST_NAMES_THE_WALK_MUST_HAVE_JUDGED,
        "the walk judged {} names out of {} files, and a walk that reached the files and read no declaration out of them is not a walk",
        names.len(),
        files.len()
    );
}

#[test]
fn the_refused_words_are_not_an_empty_set() {
    let refused = the_refused_words();
    assert!(
        refused.len() >= THE_FEWEST_REFUSED_WORDS_THE_PARSE_MUST_HAVE_FOUND,
        "the parse found {} words, and a set that short is a set it did not read",
        refused.len()
    );
    for word in ["move_to", "status", "advance", "handle", "check"] {
        assert!(
            refused.contains(&normalised(word)),
            "{word} is written in a Not column, and a refused set that did not read the column would agree with every name in the workspace"
        );
    }
}

#[test]
fn the_refused_words_hold_every_word_a_not_column_writes() {
    let document = scanning::the_glossary_text();
    let refused = the_refused_words();
    for word in the_words_written_in_a_not_column(&document) {
        assert!(
            refused.contains(&normalised(&word)),
            "{word} is written in a Not column, which is a prohibition, and a prohibition the parse did not read is one this gate cannot refuse"
        );
    }
}

#[test]
fn the_refused_set_and_the_not_columns_agree_in_size() {
    let in_a_column = the_words_written_in_a_not_column(&scanning::the_glossary_text());
    let mut read_separately: Vec<String> =
        in_a_column.iter().map(|word| normalised(word)).collect();
    read_separately.sort();
    read_separately.dedup();
    assert_eq!(
        the_refused_words().len(),
        read_separately.len(),
        "the reading is the Not columns and nothing else: a refused set holding a word no Not column holds is a second reading of this document, and two readings are what this repository has already paid for twice"
    );
}

#[test]
fn a_name_built_out_of_a_refused_word_is_refused() {
    let refused = the_refused_words();
    let blessed = the_blessed_names(&refused);
    for name in THE_NAMES_A_REFUSED_WORD_MUST_REFUSE {
        assert!(
            the_refused_word_in(name, &refused, &blessed).is_some(),
            "{name} reaches for a word the glossary refuses, and a name is refused for the word it reaches for"
        );
    }
}

#[test]
fn a_name_the_document_blesses_is_not_a_synonym() {
    let refused = the_refused_words();
    let blessed = the_blessed_names(&refused);
    for name in THE_NAMES_A_REFUSED_WORD_MUST_ALLOW {
        assert!(
            the_refused_word_in(name, &refused, &blessed).is_none(),
            "{name} is a name the glossary blesses, and a name it blesses is not a synonym however that name is made of its words"
        );
    }
}

#[test]
fn a_name_the_document_blesses_is_asked_before_the_words_it_is_made_of() {
    let refused = the_refused_words();
    let blessed = the_blessed_names(&refused);
    assert!(
        refused.contains(&normalised("state")),
        "state is written in the stages column, and the precedence below is only worth anything if there is a refused word for it to precede"
    );
    assert!(
        blessed.contains(&normalised("validate_state")),
        "validate_state is named in the glossary's own table of the two verbs, and a gate that cannot see it is judging the wrong document"
    );
    assert!(
        the_refused_word_in("validate_state", &refused, &blessed).is_none(),
        "validate_state is made of the refused word `state` and is still the name of the one verb that asserts the invariants, so the blessing is asked first"
    );
}

#[test]
fn a_word_no_not_column_mentions_cannot_be_refused() {
    let refused = the_refused_words();
    for word in THE_WORDS_NO_NOT_COLUMN_MENTIONS {
        assert!(
            !refused.contains(&normalised(word)),
            "{word} is refused in the refused set, so the document has written it in a Not column and the gap this file names is no longer the whole of it: take {word} out of this table and into the table of names that must be refused"
        );
    }
}

#[test]
fn a_public_declaration_of_every_kind_is_found() {
    let source = "\
pub struct WorkItem;
pub enum Stage;
pub trait Role;
pub type Approval = u8;
pub fn transition();
pub mod scanning;
pub const THE_ROOT: u8 = 0;
";
    assert_eq!(
        the_public_names_declared_in(source),
        vec![
            String::from("WorkItem"),
            String::from("Stage"),
            String::from("Role"),
            String::from("Approval"),
            String::from("transition"),
            String::from("scanning"),
            String::from("THE_ROOT"),
        ],
        "a walk that found no declaration at all is a walk that agrees with whatever it reached"
    );
}

#[test]
fn a_visibility_written_in_brackets_is_a_step_and_not_a_name() {
    assert_eq!(
        the_public_names_declared_on_one_line("pub(crate) enum Stage;"),
        vec![String::from("Stage")],
        "`pub(crate)` is one step of a declaration, and a reading that reported the name `crate` would put a name in this gate's report that no file declares"
    );
}

#[test]
fn a_use_declaration_is_left_alone() {
    assert!(
        the_public_names_declared_in("pub use crate::domain::WorkItem;\n").is_empty(),
        "a name brought in from elsewhere was declared where this gate does not read, and a gate that judges another file's names is judging the wrong thing"
    );
}

#[test]
fn a_synonym_written_in_documentation_is_not_a_name_the_file_declares() {
    let source = "\
/// The earlier name was `work_item_status` and this is what replaced it.
pub struct WorkItem;
";
    assert_eq!(
        the_public_names_declared_in(source),
        vec![String::from("WorkItem")],
        "a name in a line of documentation is a name somebody is talking about, and this repository's own documents name the refused ones constantly and on purpose"
    );
}

#[test]
fn a_synonym_written_in_a_string_is_not_a_name_the_file_declares() {
    let source = "pub const NOTE: &str = \"pub struct work_item_status;\";\n";
    assert_eq!(
        the_public_names_declared_in(source),
        vec![String::from("NOTE")],
        "a name in a string is a fixture, and a fixture this gate could read would be a fixture it judged"
    );
}

/// The real assertion, and the one worth keeping if every fixture above were deleted.
#[test]
fn no_name_in_the_workspace_reaches_for_a_refused_word() {
    let refused = the_refused_words();
    let blessed = the_blessed_names(&refused);
    let files = scanning::every_rust_file_in_the_workspace();
    let every = every_name_in_the_workspace(&files);
    assert_the_walk_reached_something(&files, &every);
    let refusals = the_refusals_in(&every, &refused, &blessed);
    assert!(
        refusals.is_empty(),
        "one concept has one name, and these reach for another:\n{}",
        refusals.join("\n")
    );
}
