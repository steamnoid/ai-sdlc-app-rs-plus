//! The gate for RULE 2.1: a synonym is never used.
//!
//! One concept has one name, everywhere. If `WorkItem` is the term then Task, Ticket,
//! Job and Entity are all wrong, and if `transition` is the verb then `move_to`,
//! `set_stage` and `advance` are all wrong. **The words that are wrong are read out of
//! `docs/domain-glossary.md` and not written here**, so the guard follows the document
//! instead of drifting from it — a list restated in Rust is a list that was correct on
//! the day it was written and says nothing about the document afterwards.
//!
//! ## Where the document writes a refusal down, and where it does not
//!
//! The glossary writes a refusal in exactly two places a reader can find: **a cell in a
//! column headed `Not`**, and **the fenced block of `## Banned vocabulary`**. Its own
//! "How this file is read" table calls both of them *refused, and not blessed*, and this
//! gate reads both. That is the reading, and it is a reading of the document rather than
//! of this file.
//!
//! **It is not the whole of what the document refuses, and the gap is worth knowing
//! about.** Task, Ticket and Job are named in the glossary's prose — "Task, Ticket, Job
//! and Entity are all wrong" — and in no column and in no block, so there is nothing for
//! a reader to read and this gate cannot refuse them. The reading that would supply them
//! is a rule about the shape of one sentence in one list, and a second reading of one
//! document is what this file exists to stop. **The document has to write them down in
//! one of its two places before a gate can refuse them**, and
//! `a_synonym_the_document_writes_only_in_prose_cannot_be_refused` is the test that says
//! so and that watches the gap.
//!
//! ## What is read, and what is not
//!
//! Only the stretches of a file that are **code**. A synonym inside a `///` is this
//! repository talking about a name it refuses, and this repository talks about `move_to`
//! and `Task` and `Status` constantly and on purpose, so a reading that took a
//! documentation line for a declaration would refuse this repository for discussing
//! itself. A synonym inside a string is a fixture, and a fixture this gate could read
//! would be a fixture it judged.
//!
//! ## Why a name is judged by its words
//!
//! A synonym is a **word somebody reached for**, so the judgement is on the words a name
//! is made of and not on the whole spelling. `advance_the_work_item` reaches for
//! `advance` and is a perfectly good name otherwise, and a rule that only caught a name
//! spelled exactly as the document spells the synonym would be walked past by the first
//! person to add a prefix to one.
//!
//! And the document's blessing is asked **first**: `WorkItem` is made of the word `item`
//! and `item` is refused by the block of prohibitions, so a gate that asked the words
//! before the blessing would refuse the one name this whole repository is about.

use std::path::{Path, PathBuf};

use aisdlc_gates::scanning::{self, HowTheFileIsWritten, WhatIsWrittenThere, the_text_at};

/// The seven keywords a name is declared with, and no others.
///
/// A `pub` is not asked for, and the reason is the opposite of the reason the gate
/// beside this one gives for asking for one: a private name is a name a reader of this
/// repository still has to read, and a synonym is a synonym whoever is allowed to see it.
const THE_KEYWORDS_A_NAME_IS_DECLARED_WITH: [&str; 7] =
    ["struct", "enum", "trait", "type", "fn", "mod", "const"];

/// The heading of the section that is a list of prohibitions rather than a list of names.
const THE_HEADING_OF_THE_SECTION_OF_PROHIBITIONS: &str = "## Banned vocabulary";

/// The heading every section of the document opens with, which is also how the end of the
/// section above is found.
const THE_HEADING_OF_ANY_SECTION: &str = "## ";

/// The header a table column carries when what it holds is refused rather than named.
///
/// The decision is by the header row and by nothing else, so which table carries the
/// column and where in the table it stands are both questions this gate does not ask.
const THE_COLUMN_THAT_REFUSES: &str = "Not";

/// What opens and closes the block the prohibited words are written in, and the opening
/// carries a language after it that is not a second fence.
const THE_FENCE_AROUND_A_BLOCK: &str = "```";

/// How many refused words the parse must have found before its size means anything.
///
/// Sixteen is one more than the fifteen a `Not` column holds today, and it is here for
/// the same reason the sibling gates carry a floor: a set that short is a set the parse
/// did not read, and a gate that compared nothing would agree with everything.
const THE_FEWEST_REFUSED_WORDS_THE_PARSE_MUST_HAVE_FOUND: usize = 16;

/// How many files the walk must have opened before it has earned the right to agree with
/// all of them. A gate that read nothing and said nothing is a green run that is a lie.
const THE_FEWEST_FILES_THE_WALK_MUST_HAVE_FOUND: usize = 5;

/// How many names the walk must have judged for the same reason, and for the one more
/// that is particular to this gate: a walk that found the files and then read no
/// declaration out of them agrees with a workspace it never looked inside.
const THE_FEWEST_NAMES_THE_WALK_MUST_HAVE_JUDGED: usize = 100;

/// The names a refused word must refuse.
///
/// **A table of strings rather than a table of `const`s**, because a refused word written
/// in a string is a fixture: this gate reads code, and a fixture it could read would be
/// a fixture it judged. Task, Ticket and Job are not in this table — they are in
/// `THE_WORDS_WRITTEN_ONLY_IN_PROSE`, and the test that reads that one says why.
const THE_NAMES_A_REFUSED_WORD_MUST_REFUSE: [&str; 19] = [
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
    "work_item_status",
    "work_item_state",
    "advance_the_work_item",
];

/// The names a refused word must leave alone.
///
/// Every one of them is either a word the document blesses or a name built out of words
/// the document blesses, and two of them — `WorkItem` and `validate_state` — are made of
/// words the document *refuses*. They are here because a gate is exercised on what it
/// must leave alone as well as on what it must catch, and a guard that has only ever
/// been seen to refuse is not known to be able to do anything else.
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

/// The words the document calls wrong in prose and writes down in neither of its two
/// places. They are here so that the gap the module documentation names is a table rather
/// than a paragraph, and so that a document that starts writing them down is caught by
/// the test that reads this table.
const THE_WORDS_WRITTEN_ONLY_IN_PROSE: [&str; 3] = ["Task", "Ticket", "Job"];

/// A name the walk found, with the file it was declared in.
///
/// The two are held together because a refusal that cannot name the file to open is a
/// refusal nobody can act on, and because the count of these is what says the walk
/// looked at something.
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

/// The words the document refuses, read from the two places it writes a refusal where a
/// reader can find one, and from nowhere else.
///
/// The two readings are kept in one list rather than asked of the caller separately,
/// because a name is refused by the set and not by whichever of the two places the
/// question happened to be asked of.
fn the_refused_words() -> Vec<String> {
    Vec::new()
}

/// The words with the ones said twice counted once, so that two readings of the same
/// word are one refused word — and so that two sets of words can be compared by size,
/// which is the only way a test can say whether two readings of one document agree.
fn the_words_never_counted_twice(words: &[String]) -> Vec<String> {
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
            column = cells
                .iter()
                .position(|cell| *cell == THE_COLUMN_THAT_REFUSES);
        }
        if let Some(cell) = column.and_then(|at| cells.get(at)) {
            refused.extend(code_spans(cell));
        }
        row_in_this_table += 1;
    }
    refused
}

/// The cells of one row of a table, or nothing at all when the line is not one.
///
/// Prose that happens to hold a vertical bar is not a row, and a row is a line that
/// opens with a bar and closes with one and has something between them.
fn the_cells_of_a_row(line: &str) -> Option<Vec<&str>> {
    if !(line.starts_with('|') && line.ends_with('|') && line.len() > 1) {
        return None;
    }
    Some(line.split('|').map(str::trim).collect())
}

/// Every word written in the fenced block of the section that is a list of prohibitions.
///
/// The prose around the block is about the list rather than being the list, so only the
/// block counts: reading the prose as entries would make a word refused because the
/// document explains that the word is refused.
fn the_words_written_in_the_section_of_prohibitions(document: &str) -> Vec<String> {
    let section = the_section_named(THE_HEADING_OF_THE_SECTION_OF_PROHIBITIONS, document);
    the_block_fenced_within(&section)
        .split_whitespace()
        .map(String::from)
        .collect()
}

/// One section of the document, from its own heading to the next heading or to the end of
/// the document, whichever comes first.
fn the_section_named(heading: &str, document: &str) -> String {
    let mut inside = false;
    let mut section = String::new();
    for line in document.lines() {
        if line.starts_with(heading) {
            inside = true;
            continue;
        }
        if inside && line.starts_with(THE_HEADING_OF_ANY_SECTION) {
            break;
        }
        if inside {
            section.push_str(line);
            section.push('\n');
        }
    }
    section
}

/// The text inside the fenced block of a section, which is where a list of words is
/// written and the only part of a section that is the list.
fn the_block_fenced_within(section: &str) -> String {
    let mut inside = false;
    let mut block = String::new();
    for line in section.lines() {
        if line.trim_start().starts_with(THE_FENCE_AROUND_A_BLOCK) {
            inside = !inside;
            continue;
        }
        if inside {
            block.push_str(line);
            block.push(' ');
        }
    }
    block
}

/// Every name the document blesses: a bolded entry and a code span, with the refused
/// words taken out first.
///
/// **Taking the refused words out is not optional, and the reason is the document's own.**
/// A cell in a `Not` column is written in code spans, so a reading that took every code
/// span as a blessed name put `Status`, `State` and `move_to` in the blessed set, and a
/// synonym and a blessed name were the same set of words — which leaves a gate that
/// refuses synonyms with nothing at all to refuse. The sibling gate empties the cells
/// before it reads; this one subtracts the words, and the two agree because the document
/// carries the obligation that **a refused word appears once, in a `Not` column, and
/// nowhere else in backticks** — so a code span outside one of those columns that holds
/// a refused word is a cell this reading could not tell from it, and removing the word is
/// the same removal either way.
fn the_blessed_names(refused: &[String]) -> Vec<String> {
    let document = scanning::the_glossary_text();
    let mut entries = bolded_entries(&document);
    entries.extend(code_spans(&document));
    let mut every = the_words_never_counted_twice(&entries);
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
/// The blessing is asked first and it is asked separately for the same reason: `WorkItem`
/// is made of the word `item`, `item` is refused by the block of prohibitions, and a
/// gate that asked the words first would refuse the one name this repository is about.
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

/// The names `source` declares, read out of the stretches that are code and out of
/// nothing else.
///
/// A `Code` stretch holds one line of code and no line ending, so one stretch is one line
/// and is read as one. Nothing else in the file is read at all, which is what makes a
/// declaration written in documentation, in a block comment or in a string a name the
/// reader is told about rather than a name the file declares.
fn the_names_declared_in(source: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for stretch in HowTheFileIsWritten::of(source).stretches() {
        if stretch.kind() != WhatIsWrittenThere::Code {
            continue;
        }
        names.extend(the_names_declared_on_one_line(stretch.text()));
    }
    names
}

/// The names one line of code declares, found by asking every keyword of the seven
/// whether it is here rather than by asking at the first one.
///
/// Asking each keyword in turn is what makes `pub(crate) const fn` three declarations and
/// not one: a modifier standing between the keyword and the name is a step of a
/// declaration, and this gate has no bracket to keep track of because it never looks for
/// a `pub`.
fn the_names_declared_on_one_line(code: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for keyword in THE_KEYWORDS_A_NAME_IS_DECLARED_WITH {
        names.extend(the_names_the_keyword_declares(code, keyword));
    }
    names
}

/// The names `keyword` declares on this line, which is none at all when the keyword is
/// not here, is a part of a longer word, or is followed by a name that is itself a
/// keyword.
fn the_names_the_keyword_declares(code: &str, keyword: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut from = 0;
    while let Some(at) = code[from..].find(keyword) {
        let begins = from + at;
        let ends = begins + keyword.len();
        if is_a_whole_word(code, begins, ends) {
            names.extend(the_name_written_after(code, ends));
        }
        from = ends;
    }
    names
}

/// Whether the run of characters between two offsets is a word of its own, so that the
/// `fn` in `fns` and the `struct` in `structural` are not a declaration.
fn is_a_whole_word(code: &str, begins: usize, ends: usize) -> bool {
    let before = code[..begins].chars().next_back();
    let after = code[ends..].chars().next();
    !before.is_some_and(is_part_of_a_word) && !after.is_some_and(is_part_of_a_word)
}

/// Whether a character is part of a word rather than standing between two of them.
fn is_part_of_a_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// The name written after a declaration keyword, which is the run of letters, digits and
/// underscores once the space in front of it is stepped over.
///
/// A keyword standing there is not a name: `const fn` declares one function, and reading
/// the two keywords as a declaration named `fn` would put a name in this gate's report
/// that no file of this repository declares.
fn the_name_written_after(code: &str, after: usize) -> Option<String> {
    let name: String = code[after..]
        .trim_start()
        .chars()
        .take_while(|character| is_part_of_a_word(*character))
        .collect();
    if name.is_empty() || THE_KEYWORDS_A_NAME_IS_DECLARED_WITH.contains(&name.as_str()) {
        return None;
    }
    Some(name)
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
    the_names_declared_in(&the_text_at(file))
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

#[test]
fn the_refused_words_are_more_than_the_not_columns_alone() {
    let refused = the_refused_words();
    assert!(
        refused.len() > THE_FEWEST_REFUSED_WORDS_THE_PARSE_MUST_HAVE_FOUND,
        "the parse found {} words, and a set that short is a set it did not read",
        refused.len()
    );
}

#[test]
fn the_refused_words_hold_every_word_a_not_column_writes() {
    let document = scanning::the_glossary_text();
    let refused = the_refused_words();
    for word in the_words_written_in_a_not_column(&document) {
        assert!(
            refused.contains(&normalised(&word)),
            "{word} is written in a Not column, and a refusal the document writes down is one this gate must refuse"
        );
    }
}

#[test]
fn the_refused_words_hold_every_word_the_prohibitions_write() {
    let document = scanning::the_glossary_text();
    let refused = the_refused_words();
    for word in the_words_written_in_the_section_of_prohibitions(&document) {
        assert!(
            refused.contains(&normalised(&word)),
            "{word} is written in the block of prohibitions, and the document's own reading table calls that block refused and not blessed"
        );
    }
}

#[test]
fn the_reading_says_which_places_of_the_document_it_read() {
    let document = scanning::the_glossary_text();
    let in_a_column = the_words_written_in_a_not_column(&document);
    let in_the_prohibitions = the_words_written_in_the_section_of_prohibitions(&document);
    let mut both = in_a_column.clone();
    both.extend(in_the_prohibitions.iter().cloned());
    assert_eq!(
        the_refused_words().len(),
        the_words_never_counted_twice(&both).len(),
        "the refused set and the two places the document writes a refusal down hold a different number of words, so the parse and the document have drifted apart"
    );
    assert!(
        in_a_column.len() < the_refused_words().len(),
        "the reading is the union of the {} words a Not column holds and the {} words the block of prohibitions holds, and the two of them together are more than the column alone",
        in_a_column.len(),
        in_the_prohibitions.len()
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
            "{name} is a name the glossary blesses, and a name it blesses is not a synonym however the name is made of its words"
        );
    }
}

#[test]
fn a_name_the_document_blesses_is_asked_before_the_words_it_is_made_of() {
    let refused = the_refused_words();
    let blessed = the_blessed_names(&refused);
    assert!(
        refused.contains(&normalised("item")),
        "item is written in the block of prohibitions, and the precedence below is only worth anything if there is a refused word to precede"
    );
    assert!(
        blessed.contains(&normalised("WorkItem")),
        "WorkItem is bolded in the glossary, and a gate that cannot see it is judging the wrong document"
    );
    assert!(
        the_refused_word_in("WorkItem", &refused, &blessed).is_none(),
        "WorkItem is made of the refused word `item` and is still the one name for the thing, so the blessing is asked first"
    );
}

#[test]
fn a_synonym_the_document_writes_only_in_prose_cannot_be_refused() {
    let refused = the_refused_words();
    for word in THE_WORDS_WRITTEN_ONLY_IN_PROSE {
        assert!(
            !refused.contains(&normalised(word)),
            "{word} is in the document's refused set, so it is no longer the gap the module documentation names: take it out of THE_WORDS_WRITTEN_ONLY_IN_PROSE and into the table of names that must be refused"
        );
    }
}

#[test]
fn a_declaration_of_every_kind_is_found() {
    let source = "\
struct WorkItem;
enum Stage;
trait Role;
type Approval = u8;
fn transition();
mod scanning;
const THE_ROOT: u8 = 0;
";
    assert_eq!(
        the_names_declared_in(source),
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
fn a_modifier_between_the_keyword_and_the_name_is_a_step_and_not_a_name() {
    assert_eq!(
        the_names_declared_on_one_line("pub(crate) const fn a_lock() {}"),
        vec![String::from("a_lock")],
        "`const fn` declares one function, and a reading that reported the name `fn` would put a name in this gate's report that no file declares"
    );
}

#[test]
fn a_synonym_written_in_documentation_is_not_a_name_the_file_declares() {
    let source = "\
/// The earlier name was `work_item_status` and this is what replaced it.
pub struct WorkItem;
";
    assert_eq!(
        the_names_declared_in(source),
        vec![String::from("WorkItem")],
        "a name in a line of documentation is a name somebody is talking about, and this repository's own documents name the refused ones constantly and on purpose"
    );
}

#[test]
fn a_synonym_written_in_a_string_is_not_a_name_the_file_declares() {
    let source = "const NOTE: &str = \"pub struct work_item_status;\";\n";
    assert_eq!(
        the_names_declared_in(source),
        vec![String::from("NOTE")],
        "a name in a string is a fixture, and a fixture this gate could read would be a fixture it judged"
    );
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
