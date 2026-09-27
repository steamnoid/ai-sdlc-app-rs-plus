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
use std::fs;
use std::path::{Path, PathBuf};

use aisdlc_gates::scanning::{HowTheFileIsWritten, WhatIsWrittenThere};

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

/// The document the names are read from, relative to the root of the checkout.
const THE_DOCUMENT_OF_THE_NAMES: &str = "docs/domain-glossary.md";

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

/// The root of this checkout, found by walking up from where cargo compiled this file.
///
/// `crates` is what is worth walking up to, because it is in the root of the system and
/// the document of the names is beside it.
fn the_root_of_the_checkout() -> PathBuf {
    let mut here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !here.join(THE_DIRECTORY_OF_THE_SYSTEM).is_dir() {
        assert!(
            here.pop(),
            "no crates/ directory above where this file was compiled"
        );
    }
    here
}

/// The text of the file at `path`, or a panic naming the file and what the read said.
///
/// A file that cannot be read is not a file that has nothing in it. It is a file nobody has
/// looked at, and agreeing with it silently is the one wrong answer available here that
/// leaves nothing to argue with, because every other refusal names a type in a file and a
/// file that was never opened is the one failure with nothing at all to say.
fn the_text_at(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(text) => text,
        Err(problem) => panic!("{} cannot be read as source: {problem}", path.display()),
    }
}

/// The glossary as it stands on disk, read on every call so that an edit to the document
/// is an edit to this gate, with nothing re-declared here to fall behind.
fn the_glossary_text() -> String {
    let path = the_root_of_the_checkout().join(THE_DOCUMENT_OF_THE_NAMES);
    the_text_at(&path)
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
/// **The list is far broader than the types the system declares, and a reader should know
/// that before trusting it.** The document blesses every bolded phrase and every code span,
/// so it blesses the words written in its own `Not` columns: `Task`, `Ticket`, `Job`,
/// `Entity`, `Status`, `State`, `move_to` and `advance` are all in this list. The one
/// prohibition that is removed is the banned section, because it is fenced; the `Not`
/// columns are in tables and are not. That is a gap in the document rather than in this
/// reading, it is the gap the glossary's own "How this file is read" table describes, and
/// it is named here rather than quietly closed by a reading the other gate does not share.
fn the_blessed_names() -> Vec<String> {
    let glossary = the_glossary_text();
    let without_the_prohibitions = the_document_with_the_banned_section_removed(&glossary);
    let mut entries = bolded_entries(&without_the_prohibitions);
    entries.extend(code_spans(&without_the_prohibitions));
    entries.iter().map(|entry| normalised(entry)).collect()
}

/// The `.rs` files of the system, which is `crates/` and nothing else.
///
/// **`aisdlc-gates` is not walked, and that is a decision rather than an oversight.** It is
/// a package of the workspace and deliberately not a part of the system, and this rule is
/// about the code of the system. Its own library is read for banned vocabulary by
/// `a_name_is_never_generic_vocabulary`, and its types are the subject of the section of
/// the glossary headed "How a file is read by the gates" — which is the section to extend
/// if this rule is ever meant to reach them.
fn every_rust_file_of_the_system() -> Vec<PathBuf> {
    let root = the_root_of_the_checkout();
    let mut found: Vec<PathBuf> = Vec::new();
    collect_rust_files_under(&root.join(THE_DIRECTORY_OF_THE_SYSTEM), &mut found);
    found
}

/// One directory's worth of the walk, and the files in it appended to the ones found.
///
/// A directory this gate cannot open is skipped rather than fatal, and the trade is paid
/// for from the other side: `the_walk_read_the_crates_of_the_system` asks the walk what it
/// found, so a root that stopped being walkable is a count of zero rather than an
/// agreement. **A gate that stopped reading judges nothing at all and says so by passing**,
/// which is the one failure this repository exists to make impossible.
fn collect_rust_files_under(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_under(&path, found);
        } else if path.extension().is_some_and(|ending| ending == "rs") {
            found.push(path);
        }
    }
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
    for file in every_rust_file_of_the_system() {
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
    let found = every_rust_file_of_the_system();
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
