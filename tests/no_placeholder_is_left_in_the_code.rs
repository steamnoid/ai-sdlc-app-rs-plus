//! The gate for RULE 1.4: no placeholder is left in the code.
//!
//! `todo!`, `unimplemented!`, `unreachable!`, `panic!("not implemented")`, and the words a
//! person types when they mean one of those. None of them is written in this repository,
//! and **there is no exception and no allowlist**.
//!
//! **The reason is not tidiness: an unfinished cycle is already visible in the history,
//! where the RED that was never made GREEN is.** A placeholder in a line of code is the
//! same unfinished thing given a home in the one place the history cannot reach, and it
//! is read by whoever opens the file next rather than by whoever reads the log. An
//! allowlist would not soften that — it would turn the unfinished thing into a thing with
//! an address, and the next person to need one would add an entry rather than a cycle.
//!
//! ## The three places a refused word can be, and the one that is this gate's
//!
//! A refused word reaches a `.rs` file three ways. It is a **comment**, and
//! `the_code_carries_no_comments` refuses every comment that is not documentation, so the
//! two-slash form is already gone before this gate is asked anything. It is a **string**,
//! and a string is text a value holds: a fixture, a message shown to a person, an example
//! written into a line of documentation. It is the **message of a `panic!`**, and a
//! `panic!` is code whose text is an argument rather than a stretch of its own.
//!
//! **The line this gate draws is between those last two: a refused word inside a string
//! literal is accepted, and a refused word in a panic message is refused.** A `panic!` is
//! the thing a run does when it cannot go on, so a message saying the work has not been
//! done is that run announcing an unfinished cycle from inside code that looks finished —
//! the one place a placeholder can hide from both the comment gate and the history. A
//! string that is not a message is a value, and a value holding the word is somebody
//! talking about a placeholder rather than leaving one. `opens_a_panic` carries the
//! reasoning and the cost, and there is a fixture on each side of the line.
//!
//! ## Why this is not a search for the letters
//!
//! A substring search for `todo` refuses **this file on its first run**, because the test
//! below is named `a_todo_macro_is_refused` and a gate that is wrong about the file that
//! defines it is a gate whose next finding is not read. It is wrong in the other
//! direction too, and there it is silent: this repository writes its names as whole
//! sentences — `the_run_is_reconstructable_from_its_own_record`,
//! `a_work_item_is_born_queued_and_names_who_asked` — so `todo_count` and
//! `the_todo_of_the_run` are names in its own style and not placeholders, and a search
//! that cannot tell them apart is a search that will be argued with.
//!
//! **A token is decided by the characters around it**, and a name is made of the same
//! characters as a word. `is_part_of_a_word` is what answers that, and the underscores on
//! either side of the word in a name are part of the name, which is the whole difference
//! between `todo!()` and `todo_count`.
//!
//! ## What a refusal says
//!
//! The file, the line, and the whole line as the file wrote it — including the parts of
//! it that are not the thing being refused, because a reader opens the file at the line
//! and reads the rest of it themselves.

use std::fmt::Write as _;
use std::path::Path;

use aisdlc_gates::scanning::{self, AStretchOfTheFile, HowTheFileIsWritten, WhatIsWrittenThere};

/// Every way this repository's code says the work has not been done, in the order Rust
/// writes them: the three macros a body stands a placeholder in, the message a `panic!`
/// is given, and then the words.
///
/// **It is one list and not three, because the rule is one rule.** A refused spelling is
/// read against a stretch of code, and read again against a string when that string is a
/// panic message. The eleven are here rather than written into a pattern because a
/// pattern is a rule nobody can read when it stops matching, and because a second list
/// held beside this one is a list that will disagree with it.
///
/// `not implemented` is the eleventh and the only entry Rust cannot spell in code at all,
/// which is why one list costs nothing to read in both places: the two words can stand in
/// a message and in nothing else, so the entry is spent on the message and idle on the
/// code. A list of eleven is not a list with a hole in it, and it is not padded — every
/// one of the eleven is proven to be refused by
/// `every_spelling_in_the_list_is_refused_in_a_panic_message`.
const THE_PLACEHOLDERS_WRITTEN_IN_RUST: [&str; 11] = [
    "todo!",
    "unimplemented!",
    "unreachable!",
    "not implemented",
    "TODO",
    "FIXME",
    "XXX",
    "HACK",
    "unimplemented",
    "todo",
    "to be written",
];

/// One refused placeholder: the line it stands on, and the whole of that line as the file
/// wrote it.
///
/// The two are held together because a refusal needs both and neither is enough. The line
/// is what a reader opens the file at and the text is what they are looking for when they
/// get there, and a refusal carrying the line alone sends them to search for it.
#[derive(Debug)]
struct APlaceholder {
    /// The line the placeholder stands on, counted from 1.
    the_line_it_stands_on: usize,
    /// The whole line, with the indentation and with whatever else the file wrote on it.
    the_line_as_written: String,
}

/// Whether this character could be part of a name, which is what decides whether a word
/// between two of them is a word or part of a name.
///
/// **A letter, a digit and an underscore, and nothing else.** A keyword is a name, a path
/// segment is a name, and `todo_count` is one name holding two words of it — so a match
/// that ignored these three characters would refuse this repository for naming things.
fn is_part_of_a_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// How many whole words the reading found in `written`, counted rather than judged.
///
/// **This is the count that says the reading split something at all.** A gate that
/// refused nothing on a file it never read is green and agrees with everything, so the
/// number of words a reading found is asked of the same text the refusals are read out of
/// — and a reading that returned zero for every file in the workspace would be caught by
/// the walk rather than by a fixture.
fn how_many_whole_words_are_in(written: &str) -> usize {
    let mut counted = 0;
    let mut inside_a_word = false;
    for character in written.chars() {
        let part_of_a_word = is_part_of_a_word(character);
        if part_of_a_word && !inside_a_word {
            counted += 1;
        }
        inside_a_word = part_of_a_word;
    }
    counted
}

/// How many whole words the code of `source` holds, which is the count the walk is judged
/// by rather than the number of refusals.
fn how_many_whole_words_the_code_of(source: &str) -> usize {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .filter(|stretch| stretch.kind() == WhatIsWrittenThere::Code)
        .map(|stretch| how_many_whole_words_are_in(stretch.text()))
        .sum()
}

/// How many whole words this repository's own code holds, over every file the walk found.
fn how_many_whole_words_the_workspace_holds() -> usize {
    scanning::every_rust_file_in_the_workspace()
        .iter()
        .map(|path| how_many_whole_words_the_code_of(&scanning::the_text_at(path)))
        .sum()
}

/// Every string in `source`, which is the fact that puts two fixtures carrying the same
/// refused word on different sides of the line this gate draws.
fn the_strings_in(source: &str) -> Vec<AStretchOfTheFile> {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .filter(|stretch| stretch.kind() == WhatIsWrittenThere::AString)
        .cloned()
        .collect()
}

/// The macro a run reaches for when it cannot go on, and the only word that turns a string
/// from a value into a message.
const THE_MACRO_THAT_PANICS: &str = "panic!";

/// Every placeholder in `source`, in the order the file wrote them.
///
/// **The reading is the scanner's and not a search, because the refused words in this file
/// are in string literals and a text search would refuse the file that defines the rule.**
/// A `Code` stretch is the code of exactly one line, and a string is a stretch of its own,
/// so this asks of every stretch what kind of thing it is before it looks at its letters.
/// One refusal is one stretch and not one line, because a line holding two placeholders is
/// two refusals and reporting only the first would make the second something a reader
/// believes was not found.
fn the_placeholders_in(source: &str) -> Vec<APlaceholder> {
    let reading = HowTheFileIsWritten::of(source);
    let every_stretch = reading.stretches();
    let mut found: Vec<APlaceholder> = Vec::new();
    for at in 0..every_stretch.len() {
        let Some(here) = the_placeholder_written_at(source, every_stretch, at) else {
            continue;
        };
        found.push(here);
    }
    found
}

/// The placeholder this stretch carries, and nothing at all when it carries none.
///
/// Both questions are asked before the letters are looked at: whether this gate reads this
/// stretch at all, and then whether the letters in it are a placeholder. The order is the
/// reason a fixture below is accepted — the kind of the stretch settles it, and the text
/// is only reached once the kind has said yes.
fn the_placeholder_written_at(
    source: &str,
    every_stretch: &[AStretchOfTheFile],
    at: usize,
) -> Option<APlaceholder> {
    let stretch = &every_stretch[at];
    if !is_judged_by_this_gate(every_stretch, at) {
        return None;
    }
    if !holds_a_placeholder(stretch.text()) {
        return None;
    }
    let line = stretch.line_it_starts_on();

    Some(APlaceholder {
        the_line_it_stands_on: line,
        the_line_as_written: the_line_as_written(source, line),
    })
}

/// Whether this gate reads the stretch at `at`, which is a stretch of code and a string
/// that is the message of a `panic!`, and nothing else.
///
/// **This is where the judgement this file exists to write down is made, and it is one
/// line because the other three places are somebody else's gate.** A comment is refused by
/// `the_code_carries_no_comments`, which refuses every comment that is not documentation,
/// and this gate refuses neither form: a `//` line is already gone, and a `///` line is a
/// sentence a reader chose to write — including in this very file, which names all eleven
/// spellings in its own documentation and would be refusing itself if it read them. A
/// string that is not a message is a value, and a value holding the word is an example, a
/// fixture or a message shown to a person. A panic message is none of those: it is what
/// the run says at the moment it gives up, so it is the one place an unfinished cycle can
/// speak from inside code that looks finished.
///
/// **What the choice costs, written down rather than discovered.** A message is read only
/// where it is the first thing after the opening, so `panic!("a {}", "TODO")` is not read,
/// and `expect("TODO")` is not read at all because `expect` is a method and not a
/// `panic!`. Both are holes rather than a direction this gate is safe in, and both are
/// named here because a hole a reader does not know about is a hole somebody comes to
/// rely on. Neither is a word a placeholder can hide in on its way to the history.
fn is_judged_by_this_gate(every_stretch: &[AStretchOfTheFile], at: usize) -> bool {
    let kind = every_stretch[at].kind();
    if kind == WhatIsWrittenThere::Code {
        return true;
    }
    kind == WhatIsWrittenThere::AString && is_the_message_of_a_panic(every_stretch, at)
}

/// Whether the string at `at` is the message of a `panic!`, decided by what stands
/// immediately before it.
///
/// The nearest written thing before it is asked for rather than the nearest line, because
/// `rustfmt` writes a message with two arguments over four lines and a reader looking for
/// the opening should not have to guess which of the three it was. Space between the
/// opening and the message is space, so the two are found the same way.
fn is_the_message_of_a_panic(every_stretch: &[AStretchOfTheFile], at: usize) -> bool {
    let written_before = every_stretch[..at]
        .iter()
        .rev()
        .find(|stretch| stretch.kind() != WhatIsWrittenThere::TheSpaceBetween);
    let Some(last_written) = written_before else {
        return false;
    };

    last_written.kind() == WhatIsWrittenThere::Code
        && is_the_opening_of_a_panic(last_written.text())
}

/// Whether `code` ends in the opening of a `panic!`, in either of the two brackets Rust
/// takes and with or without a space between the bang and the bracket.
///
/// The bracket is taken off before the word is looked for, so the two spellings are one
/// question rather than two endings somebody has to remember to extend. It is the only
/// opening a panic can have: a `panic!` is a macro, and a macro is called with a bracket
/// or with a brace and by nothing else.
fn is_the_opening_of_a_panic(code: &str) -> bool {
    let written = code.trim_end();
    let without_the_bracket = written.strip_suffix(['(', '{']).unwrap_or(written);

    without_the_bracket
        .trim_end()
        .ends_with(THE_MACRO_THAT_PANICS)
}

/// Whether `written` carries a refused spelling anywhere in it, which is the whole of what
/// the text of a stretch is asked.
fn holds_a_placeholder(written: &str) -> bool {
    THE_PLACEHOLDERS_WRITTEN_IN_RUST
        .iter()
        .any(|spelling| stands_as_a_whole_token(written, spelling))
}

/// Whether `spelling` stands in `written` as a whole token, decided by the characters
/// either side of it.
///
/// **A search for the letters is the wrong tool, and it is wrong in both directions at
/// once.** It refuses this file on its first run, because the test named
/// `a_todo_macro_is_refused` holds the word between two underscores; and it is silent
/// about `todo_count` and `the_todo_of_the_run`, which are names in the style this
/// repository writes every name in and are not placeholders. A gate that is right about
/// the file that defines it and wrong about a name in the house style is a gate whose
/// findings get argued with.
///
/// Every occurrence is looked at rather than only the first, because a name that holds a
/// refused word once and a `TODO` once is two findings and the first one is not the
/// answer. Searching resumes after the match rather than after the character beside it,
/// which costs the one overlap a word could have with itself and buys a slice that is
/// always on a character boundary.
fn stands_as_a_whole_token(written: &str, spelling: &str) -> bool {
    let mut from = 0;
    while let Some(at) = written[from..].find(spelling) {
        let begins_here = from + at;
        if stands_on_its_own(written, begins_here, spelling.len()) {
            return true;
        }
        from = begins_here + spelling.len();
    }
    false
}

/// Whether the spelling beginning at `begins_here` and running for `how_many_characters`
/// has nothing that could be part of a name standing on either side of it.
///
/// A phrase is held to the same rule as a word, so `to be written` is refused on its `t`
/// and its `n` rather than on a word in the middle of it — which is what makes one list
/// able to carry a phrase and a macro name without a second way of matching either.
fn stands_on_its_own(written: &str, begins_here: usize, how_many_characters: usize) -> bool {
    let before = &written[..begins_here];
    let after = &written[begins_here + how_many_characters..];

    !before.chars().next_back().is_some_and(is_part_of_a_word)
        && !after.chars().next().is_some_and(is_part_of_a_word)
}

/// The whole of `line` as the file wrote it, which is what a refusal quotes.
///
/// **The number and the words are read from two different things on purpose.** The line
/// comes from the stretch, which counted the line endings the reading crossed; the words
/// come from the file itself. A gate that rebuilt the line out of stretches would have to
/// put the parts back together, and a string holding a newline is one place where
/// re-joining and counting can come to disagree about which line a refusal belongs to.
fn the_line_as_written(source: &str, line: usize) -> String {
    source.lines().nth(line - 1).unwrap_or_default().to_string()
}

/// What one file's refusals say, one line each: the file, the line, and the whole line as
/// the file wrote it.
///
/// Nothing at all when there is nothing to refuse, and that is the whole report rather
/// than a line saying everything is fine — the count of what a gate refused is what a
/// reader needs, and a gate that says nothing on a breach and nothing on a pass cannot be
/// told apart from a gate that read nothing at all.
fn what_a_refused_placeholder_says(the_file: &Path, refused: &[APlaceholder]) -> String {
    if refused.is_empty() {
        return String::new();
    }
    let mut said = String::new();
    for one_refused in refused {
        writeln!(
            said,
            "{}:{}: {}",
            the_file.display(),
            one_refused.the_line_it_stands_on,
            one_refused.the_line_as_written
        )
        .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

/// Everything this repository has to answer for: one line per placeholder, over every file
/// the walk found.
///
/// The walk reads `.rs` files and does not widen, and the reason is worth reading before
/// somebody widens it. `README.md` and `AGENTS.md` are prose, and prose legitimately says
/// that a rule is not built yet — a gate that read them would refuse this repository's
/// own documentation for documenting its rules. A `Cargo.toml` carries no placeholder
/// either, and a lint configured in a manifest is read by clippy rather than by a person.
fn what_the_workspace_would_have_to_answer_for() -> String {
    let mut said = String::new();
    for path in scanning::every_rust_file_in_the_workspace() {
        let refused = the_placeholders_in(&scanning::the_text_at(&path));
        said.push_str(&what_a_refused_placeholder_says(&path, &refused));
    }
    said
}

/// The one placeholder this gate refuses in `source`, and a sentence naming the rule when
/// there is not exactly one.
///
/// A test that indexed into a list the reading had left empty would fail with a number
/// rather than with the rule it is here to protect, and a test that passed over two
/// refusals would have proved the one thing this gate is about and not the other.
fn the_one_placeholder_refused_in(source: &str, because: &str) -> APlaceholder {
    let refused = the_placeholders_in(source);

    assert_eq!(
        refused.len(),
        1,
        "{because}, and this is what it refused: {refused:?}"
    );
    refused.into_iter().next().expect("the count above is one")
}

#[test]
fn a_todo_macro_is_refused() {
    let source = "fn a_function_nobody_wrote() {\n    todo!();\n}\n";
    let refused = the_one_placeholder_refused_in(source, "this is the plain case");

    assert_eq!(
        refused.the_line_it_stands_on, 2,
        "the macro is on the second line of the file and nowhere else in it"
    );
    assert_eq!(
        refused.the_line_as_written, "    todo!();",
        "and the whole line is quoted, with the indentation the file wrote on it"
    );
}

#[test]
fn an_unimplemented_macro_is_refused() {
    let source = "fn a_function_nobody_wrote() {\n    unimplemented!();\n}\n";
    let refused = the_one_placeholder_refused_in(
        source,
        "a macro saying the work was not done is the same unfinished thing as one saying it is still to be done",
    );

    assert_eq!(
        refused.the_line_as_written, "    unimplemented!();",
        "the bang is part of the macro's name and is quoted with it"
    );
}

#[test]
fn an_unreachable_macro_is_refused() {
    let source = "\
match a_stage {
    Stage::DONE => finish(),
    _ => unreachable!(\"every other stage was handled above\"),
}
";
    let refused = the_one_placeholder_refused_in(
        source,
        "a macro claiming this arm cannot be reached is a claim a reader cannot check from the line",
    );

    assert_eq!(
        refused.the_line_it_stands_on, 3,
        "and it is the line the macro stands on that is named, because that is where a reader goes"
    );
}

#[test]
fn a_refused_word_in_capitals_in_code_is_refused() {
    let source = "const TODO: &str = \"finish this\";\n";
    let refused = the_one_placeholder_refused_in(
        source,
        "a word a person types in capitals stands in code on its own, and there is no allowlist",
    );

    assert_eq!(
        refused.the_line_it_stands_on, 1,
        "the constant is the only line of the file"
    );
    assert_eq!(
        refused.the_line_as_written, "const TODO: &str = \"finish this\";",
        "and the whole line is quoted, the string standing on it and all"
    );
}

#[test]
fn every_spelling_in_the_list_is_refused_in_a_panic_message() {
    for spelling in THE_PLACEHOLDERS_WRITTEN_IN_RUST {
        let source = format!("panic!(\"{spelling}\")");

        assert!(
            !the_placeholders_in(&source).is_empty(),
            "{spelling} is in the list of refused spellings and a line carrying it as a whole token was not refused, so the list and the reading have stopped agreeing"
        );
    }
}

#[test]
fn a_refused_word_in_a_string_that_is_not_a_panic_is_accepted() {
    let source = "let note = \"TODO: an example a reader may copy\";\nfn a_function() {}\n";

    assert_eq!(
        the_strings_in(source).len(),
        1,
        "the fixture is a whole line holding one string, and a reading that reached no string would agree with it for the wrong reason"
    );
    assert!(
        the_placeholders_in(source).is_empty(),
        "a string is text a value holds, and the function on the next line is what says the reading reached the code"
    );
}

#[test]
fn a_panic_message_without_a_refused_word_is_accepted() {
    let agreed = "fn a_reading() -> usize {\n    panic!(\"the reading stood still\")\n}\n";
    let refused = "fn a_reading() -> usize {\n    panic!(\"TODO: the reading stood still\")\n}\n";

    assert!(
        the_placeholders_in(agreed).is_empty(),
        "a message that says what went wrong is not a placeholder, and the refused word is what makes the other one one"
    );
    assert_eq!(
        the_strings_in(agreed).len(),
        1,
        "the message was read as a string, so the acceptance above is not a reading that never looked at one"
    );
    assert_eq!(
        the_placeholders_in(refused).len(),
        1,
        "the two lines differ by one word, and if that word changed nothing then this gate is not reading panic messages at all"
    );
}

#[test]
fn a_panic_message_written_with_a_brace_is_refused() {
    let source = "fn a_reading() {\n    panic! { \"not implemented\" }\n}\n";

    assert_eq!(
        the_placeholders_in(source).len(),
        1,
        "a bracket is a bracket, and a gate that read one spelling of a panic would leave the other for the next person"
    );
}

#[test]
fn a_panic_message_written_on_the_next_line_is_refused() {
    let source = "fn a_reading() {\n    None => panic!(\n        \"not implemented\",\n    ),\n}\n";

    assert_eq!(
        the_placeholders_in(source).len(),
        1,
        "the space between the opening and the message is space, and the message is still the message"
    );
}

#[test]
fn a_refused_word_that_is_part_of_a_name_is_accepted() {
    let source = "fn the_todo_of_the_run() {}\nlet todo_count = 3;\n";

    assert!(
        the_placeholders_in(source).is_empty(),
        "a word inside a name is part of a name, and this repository writes its names as whole sentences"
    );
    assert!(
        how_many_whole_words_the_code_of(source) > 4,
        "the reading reached {} whole words, and a reading that split none would agree with any name at all",
        how_many_whole_words_the_code_of(source)
    );
}

#[test]
fn a_lowercase_xxx_is_accepted() {
    let source = "let xxx = 1;\n";

    assert!(
        the_placeholders_in(source).is_empty(),
        "the list writes this one in capitals, and a list that matched either case would be a list of substrings"
    );
}

#[test]
fn a_refused_word_written_in_documentation_is_accepted() {
    let source = "//! TODO: this gate is being written\nfn a_function() {}\n";

    assert!(
        HowTheFileIsWritten::of(source).is_there_a_comment_anywhere(),
        "the refused word stands in a comment, and a reading that reached no comment would agree with it for the wrong reason"
    );
    assert!(
        the_placeholders_in(source).is_empty(),
        "a documentation line is a sentence a reader chose to write, and the two-slash form of the same word is the comment gate's to refuse"
    );
}

#[test]
fn a_file_with_no_placeholder_is_accepted() {
    let source = "fn nothing_to_say() {}\n";

    assert!(
        the_placeholders_in(source).is_empty(),
        "the fixture is a whole file with nothing in it this gate objects to"
    );
}

#[test]
fn a_refusal_names_the_file_the_line_and_the_whole_line() {
    let the_file = Path::new("crates/domain/tests/the_stage_is_a_closed_set_of_six.rs");
    let source = "fn a_function() {\n    todo!();\n}\n";
    let refused = the_one_placeholder_refused_in(
        source,
        "the same two lines, read by the report instead of by the walk",
    );

    let said = what_a_refused_placeholder_says(the_file, &[refused]);

    assert!(
        said.starts_with("crates/domain/tests/the_stage_is_a_closed_set_of_six.rs:2:     todo!();"),
        "a refusal a reader cannot open is a refusal nobody can act on, and this carries the file, the line and the line as written: {said}"
    );
}

#[test]
fn the_list_of_refused_spellings_is_not_a_list_nobody_wrote() {
    assert!(
        THE_PLACEHOLDERS_WRITTEN_IN_RUST.len() > 8,
        "the list holds {} spellings, and a list that short is a list it did not read",
        THE_PLACEHOLDERS_WRITTEN_IN_RUST.len()
    );
}

/// The real gate, and the only assertion here that reads the repository rather than a
/// fixture. Every one above is a demonstration; this one is the rule, and it is the one
/// worth keeping if the rest were deleted.
#[test]
fn no_placeholder_is_left_in_the_code() {
    let said = what_the_workspace_would_have_to_answer_for();

    assert!(
        said.is_empty(),
        "an unfinished cycle is visible in the history, and a placeholder is that cycle given a home in a line of code — by file, by line, and by the line as it is written:\n{said}"
    );
}

/// **A gate that read no files and agreed with all of them would be green and would be a
/// lie**, and there is no way to tell that apart from a clean repository by looking at what
/// it refused. So the walk is asked what it found before anything is judged: more than a
/// handful of files, a file under each of the three roots, and whole words read out of the
/// code of the files it opened.
///
/// The words are the load-bearing half. **There is no placeholder in this repository, and
/// that is the rule** — so the honest claim is not that the reading found one to refuse,
/// which a repair that stopped reading would also be able to say, but that it reached the
/// code of every file, split it into words, and agreed with all of them. A gate that can
/// only be seen to work when it fails is a gate that has never been seen to work.
#[test]
fn the_walk_read_a_file_under_every_root_and_judged_its_words() {
    let found = scanning::every_rust_file_in_the_workspace();

    assert!(
        found.len() > 5,
        "the walk opened {} files, and a walk that reached nothing agrees with everything",
        found.len()
    );
    for root in ["src", "crates", "tests"] {
        let under = scanning::the_root_of_the_checkout().join(root);
        assert!(
            found.iter().any(|path| path.starts_with(&under)),
            "the walk read no .rs file under {root}/, so it agreed with a directory it never opened"
        );
    }
    assert!(
        how_many_whole_words_the_workspace_holds() > 0,
        "the walk reached no whole word in any of them, and this repository is written in words, so a reading that had stopped splitting would agree with all of it"
    );
}
