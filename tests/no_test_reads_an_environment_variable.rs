//! The gate for RULE 0.4: no test in this repository reads the environment at run time.
//!
//! `cargo test` has to stay green with no network, no API key, no Docker and no model, and
//! that is the one rule in this repository that decides whether a green run means anything.
//! A suite that needs a token does not fail. It is not run, or it is run by the one person
//! who has the token, and the tick is the same either way — so a suite in that state
//! reports the same thing a suite in the right state reports, and the difference between
//! them is invisible to everybody who is not already looking.
//!
//! **So the rule is held on the tests themselves rather than trusted.** Nothing in this
//! workspace may read a credential, and nothing may read the environment to find one.
//!
//! ## `env!` is not a read of the environment, and the difference is this whole gate
//!
//! **`env!` and `option_env!` are macros the compiler expands before the program starts.**
//! `CARGO_MANIFEST_DIR` is set by cargo for the crate being compiled, it is written into
//! the binary, and no credential has ever been in it. A gate that refused the word `env`
//! would refuse the only way a test can find the checkout, and four gates in this
//! repository use it to do exactly that.
//!
//! The test for the two is therefore not *does this line say `env`* but **does this code
//! reach the environment while the program is running**. `env!` is a macro and
//! `env::var` is a path into a module, and a stretch of code cannot tell them apart by
//! spelling — it can only tell them apart by asking whether what is written opens a
//! module. A refused call is written `env::var(`, with two colons between the words, and
//! nothing else this gate looks for carries them.
//!
//! One fact about a call makes the reading smaller than it looks. **The argument of a
//! refused call is a string, so the stretch of code ends at the opening bracket.**
//! `let token = env::var("GITHUB_TOKEN");` is read as code, then a string, then code, and
//! the stretch holding the call is `let token = env::var`. The reading therefore never has
//! to open a string to know that a read is there, and a file that writes the words
//! `env::var` inside a string is a file that holds the very text of a refused call and is
//! not refused — which is the only way a gate can hold a fixture and stay right about it.
//!
//! ## What a refusal says
//!
//! The file, the line, and the whole line as the file wrote it.
//!
//! **The whole line and not the stretch, because the stretch is not the line.** The
//! argument is the part of the line a reader most needs to see, and a stretch cut at the
//! opening bracket is a refusal that leaves the credential's name off it.
//!
//! ## The holes, named rather than closed
//!
//! Three, and each is a decision rather than an oversight.
//!
//! - **An import the reading does not follow.** `use std::env as the_environment;` followed
//!   by `the_environment::var("X")` reaches the environment and is not caught, because the
//!   reading joins a call to a `use` line by the word `env` standing on its own. The join
//!   is worth more than the alias is likely to be: the alias is rarer, and a reading that
//!   followed every `use` would have to understand what each one renamed.
//! - **A call this gate does not name.** `std::env::current_dir()` is a run-time read of
//!   the environment, it is written in this repository today, and it is not refused. The
//!   list is the six names below and not "anything in the `env` module", because a list of
//!   every read would be a list to keep in step with a standard library. **This gate
//!   therefore does not currently prove what its own module documentation says it proves,
//!   and the file that holds the call is named in the report of this work rather than left
//!   to be found later.**
//! - **A name inside a raw identifier.** `r#var("X")` is a call and is not a call, and a
//!   reading that did not understand raw identifiers would read the raw form as the plain
//!   one. No gate here can reach it and none pretends to.

use std::fmt::Write as _;
use std::path::Path;

use aisdlc_gates::scanning::{self, AStretchOfTheFile, HowTheFileIsWritten, WhatIsWrittenThere};

/// The six calls that reach the environment while the program is running, and no others.
///
/// **Each is refused whatever reaches it**, and the two ways a call is written are told
/// apart from the code around it rather than from this list: written through the path
/// `env::var(`, or written bare in a file that has imported the environment.
///
/// A `env!` and an `option_env!` are not here and are not missed. They are expanded by the
/// compiler before the program runs, and `CARGO_MANIFEST_DIR` — the one a test needs to
/// find the checkout — is set by cargo and written into the binary, so nothing reaches the
/// environment when such a line is executed. A gate that refused the word `env` rather
/// than the reach would refuse the only way four of this repository's gates can find
/// where they are running.
const THE_CALLS_THAT_REACH_THE_ENVIRONMENT: [&str; 6] =
    ["var", "vars", "var_os", "set_var", "remove_var", "temp_dir"];

/// The two colons and the module between them, which is what tells a call written through
/// a path from a call written bare and from a macro that happens to end in an exclamation
/// mark.
const THE_PATH_INTO_THE_ENVIRONMENT: &str = "env::";

/// One run-time read of the environment, with the line it stands on and the line as the
/// file wrote it.
///
/// Both are held because a refusal needs both and neither is enough. The line is what a
/// reader opens the file at, and the text is what they are looking for when they get
/// there; a refusal carrying the line alone makes the reader search for it.
#[derive(Debug)]
struct ARuntimeReadOfTheEnvironment {
    /// The line the call is written on, counted from 1.
    the_line_it_is_written_on: usize,
    /// The whole line, exactly as the file has it.
    the_line_as_it_is_written: String,
}

/// Every run-time read of the environment in `source`, in the order the file wrote them.
///
/// **Whether the environment is imported is asked once for the whole file, and that is
/// what joins two lines into one finding.** A call written bare says nothing about what
/// it is a call into; the `use` line above it says it, and the two are written apart
/// every time. A reading that asked each stretch on its own would answer neither, and
/// `a_bare_call_with_no_import_is_accepted` is the test that says so.
fn the_runtime_reads_in(source: &str) -> Vec<ARuntimeReadOfTheEnvironment> {
    let reading = HowTheFileIsWritten::of(source);
    let every_stretch = reading.stretches();
    let the_environment_is_imported = does_the_file_import_the_environment(every_stretch);

    (0..every_stretch.len())
        .filter_map(|at| {
            the_read_opening_at(source, every_stretch, at, the_environment_is_imported)
        })
        .collect()
}

/// The line `on_line` of `source`, exactly as the file wrote it.
///
/// **This is a refusal's own line rather than the stretch that carried the call, and the
/// difference is the argument.** A call's argument is a string, so the stretch of code
/// stops at the opening bracket and the credential's name is not in it. An empty string
/// rather than a panic, because the number came from the reading of this same file and a
/// refusal that stops the gate before it has reported anything is worse than one missing
/// its text.
fn the_line_at(source: &str, on_line: usize) -> String {
    source
        .lines()
        .nth(on_line.saturating_sub(1))
        .unwrap_or_default()
        .to_string()
}

/// Whether the file brings the environment into scope, and so whether a bare call in it
/// can be a call into the environment at all.
fn does_the_file_import_the_environment(every_stretch: &[AStretchOfTheFile]) -> bool {
    every_stretch
        .iter()
        .any(this_stretch_imports_the_environment)
}

/// Whether this one stretch is a `use` bringing in the module named `env`.
///
/// **The name has to stand on its own, because that is what tells one module from
/// another.** `use std::env;` is the environment and `use crate::environment;` is a
/// module of this repository's own, and a reading that looked for the letters `env`
/// anywhere in the line would take the second for the first.
fn this_stretch_imports_the_environment(stretch: &AStretchOfTheFile) -> bool {
    if stretch.kind() != WhatIsWrittenThere::Code {
        return false;
    }
    let written = stretch.text().trim_start();
    if !written.starts_with("use ") {
        return false;
    }
    written
        .trim_end_matches(';')
        .split("::")
        .any(|part| part.trim() == "env")
}

/// The read this stretch carries, and nothing at all when it carries none.
fn the_read_opening_at(
    source: &str,
    every_stretch: &[AStretchOfTheFile],
    at: usize,
    the_environment_is_imported: bool,
) -> Option<ARuntimeReadOfTheEnvironment> {
    let stretch = &every_stretch[at];
    if stretch.kind() != WhatIsWrittenThere::Code {
        return None;
    }
    if !this_code_reaches_the_environment(stretch.text(), the_environment_is_imported) {
        return None;
    }
    Some(ARuntimeReadOfTheEnvironment {
        the_line_it_is_written_on: stretch.line_it_starts_on(),
        the_line_as_it_is_written: the_line_at(source, stretch.line_it_starts_on()),
    })
}

/// Whether this line of code reaches the environment, either through the path or through an
/// import standing elsewhere in the same file.
fn this_code_reaches_the_environment(code: &str, the_environment_is_imported: bool) -> bool {
    THE_CALLS_THAT_REACH_THE_ENVIRONMENT
        .iter()
        .any(|call| is_there_a_call_to(code, call, THE_PATH_INTO_THE_ENVIRONMENT))
        || (the_environment_is_imported
            && THE_CALLS_THAT_REACH_THE_ENVIRONMENT
                .iter()
                .any(|call| is_there_a_call_to(code, call, "")))
}

/// Whether `code` calls `call`, with `written_in_front_of_it` in front of it and the bracket
/// that opens its arguments behind it.
///
/// The bracket is asked for as well as the name, and it is what tells `var` from
/// `variant` and `remove_var` from `var`. An empty `written_in_front_of_it` is a bare call.
fn is_there_a_call_to(code: &str, call: &str, written_in_front_of_it: &str) -> bool {
    code.contains(&format!("{written_in_front_of_it}{call}("))
}

/// How many stretches of code `source` holds, which is the count that says the reading
/// reached the code at all.
fn how_many_stretches_of_code_are_in(source: &str) -> usize {
    HowTheFileIsWritten::of(source)
        .stretches()
        .iter()
        .filter(|stretch| stretch.kind() == WhatIsWrittenThere::Code)
        .count()
}

/// How many `env!` and `option_env!` this workspace writes in its own code, which is the
/// count that says the reading reached the calls it must leave alone.
///
/// `option_env!` holds the letters `env!` and is counted by the same search, and that is
/// the right answer: both are expanded by the compiler and neither reaches the environment
/// when the program runs, so a reading that could not tell them apart would be a reading
/// refusing one and not the other for a reason it had not been given.
fn how_many_build_time_environment_macros_the_workspace_holds() -> usize {
    scanning::every_rust_file_in_the_workspace()
        .iter()
        .map(|path| {
            HowTheFileIsWritten::of(&scanning::the_text_at(path))
                .stretches()
                .iter()
                .filter(|stretch| {
                    stretch.kind() == WhatIsWrittenThere::Code && stretch.text().contains("env!")
                })
                .count()
        })
        .sum()
}

/// What one file's refusals say: the file, the line, and the whole line as the file wrote
/// it.
fn what_a_file_would_have_to_answer_for(the_file: &Path, source: &str) -> String {
    let mut said = String::new();
    for one_refused in the_runtime_reads_in(source) {
        writeln!(
            said,
            "{}:{}: {}",
            the_file.display(),
            one_refused.the_line_it_is_written_on,
            one_refused.the_line_as_it_is_written
        )
        .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

/// Everything this repository has to answer for, one line per refusal, over every file the
/// walk found.
fn what_the_workspace_would_have_to_answer_for() -> String {
    let mut said = String::new();
    for path in scanning::every_rust_file_in_the_workspace() {
        let source = scanning::the_text_at(&path);
        said.push_str(&what_a_file_would_have_to_answer_for(&path, &source));
    }
    said
}

/// The one read this gate refuses in `source`, and a sentence naming the rule when there is
/// not exactly one.
///
/// A test that indexed into a list the reading had left empty would fail with a number
/// rather than with the rule it is here to protect, and a test that passed over two reads
/// would have proved the one thing this gate is about and not the other.
fn the_one_read_refused_in(source: &str, because: &str) -> ARuntimeReadOfTheEnvironment {
    let refused = the_runtime_reads_in(source);

    assert_eq!(
        refused.len(),
        1,
        "{because}, and this is what it refused: {refused:?}"
    );
    refused.into_iter().next().expect("the count above is one")
}

/// Assert that the reading of `source` reached its code and refused nothing in it, and say
/// which of the two a failure was.
///
/// The two questions are asked together because either alone is satisfied by a broken
/// reading: a reading that had stopped before the code refuses nothing, and a reading that
/// called every call a read would refuse this repository's own gates.
fn the_reading_reached_the_code_and_refused_nothing(source: &str, because: &str) {
    let stretches_saw = how_many_stretches_of_code_are_in(source);
    let refused = the_runtime_reads_in(source);

    assert!(
        stretches_saw > 0,
        "{because}, and the reading saw {stretches_saw} stretches of code in it"
    );
    assert!(refused.is_empty(), "{because}, and it refused: {refused:?}");
}

#[test]
fn a_var_of_a_github_token_through_the_full_path_is_refused() {
    let source = r#"fn a_test() {
    let token = std::env::var("GITHUB_TOKEN");
}
"#;
    let refused = the_one_read_refused_in(source, "this is the plain case");

    assert_eq!(
        refused.the_line_it_is_written_on, 2,
        "the call is the second line of the file"
    );
    assert!(
        refused.the_line_as_it_is_written == r#"    let token = std::env::var("GITHUB_TOKEN");"#,
        "and the whole line is quoted, with the argument the stretch of code had cut off"
    );
}

#[test]
fn a_var_of_an_api_key_through_the_short_path_is_refused() {
    let source = r#"let key = env::var("OPENAI_API_KEY");
"#;
    let refused = the_one_read_refused_in(
        source,
        "`std::` is a convenience and not the rule, and a gate that needed it would be a gate about one spelling",
    );

    assert!(
        refused.the_line_as_it_is_written.contains("env::var("),
        "the two colons are what tell a path from a macro, and they are in the quote: {}",
        refused.the_line_as_it_is_written
    );
}

#[test]
fn a_vars_that_lists_the_whole_environment_is_refused() {
    let source = "let every_variable = std::env::vars();\n";

    the_one_read_refused_in(
        source,
        "reading every variable at once is the same reach and answers a question no test may ask",
    );
}

#[test]
fn a_set_var_that_writes_the_environment_is_refused() {
    let source = r#"env::set_var("AISDLC_PROVIDER", "openai");
"#;

    the_one_read_refused_in(
        source,
        "writing into the environment is the other direction of the same boundary, and a suite that writes one changes what the next test in the same process reads",
    );
}

#[test]
fn a_var_os_that_reads_a_path_is_refused() {
    let source = r#"let where = env::var_os("HOME");
"#;

    the_one_read_refused_in(
        source,
        "the only difference from `var` is the return type, and a path out of the environment is a path out of the environment",
    );
}

#[test]
fn a_remove_var_that_erases_the_environment_is_refused() {
    let source = r#"env::remove_var("OPENAI_API_KEY");
"#;

    the_one_read_refused_in(
        source,
        "erasing a variable is not a way of not reading it, and a test that did this would change the answer of the test beside it",
    );
}

#[test]
fn a_temp_dir_that_reads_where_to_write_is_refused() {
    let source = "let where = std::env::temp_dir();\n";

    the_one_read_refused_in(
        source,
        "this is the one refused read that is about no credential at all: the rule is reaching the environment while the program runs, and a temporary directory is named by a variable on this machine",
    );
}

#[test]
fn a_bare_call_after_the_import_of_the_environment_is_refused() {
    let source = r#"use std::env;

fn a_test() {
    let token = var("GITHUB_TOKEN");
}
"#;
    let refused = the_one_read_refused_in(
        source,
        "this is how it is written when it is written at all: the import on one line and the call on another",
    );

    assert_eq!(
        refused.the_line_it_is_written_on, 4,
        "and the refusal names the line of the call rather than the line of the import, because the call is what reached the environment"
    );
}

#[test]
fn the_import_of_the_environment_alone_is_accepted() {
    let source = "use std::env;\nfn a_test() {}\n";

    the_reading_reached_the_code_and_refused_nothing(
        source,
        "an import brings a name into scope and reaches nothing, and a gate that refused the import would refuse the line every other reading of the pair depends on",
    );
}

#[test]
fn a_bare_call_with_no_import_is_accepted() {
    let source = "fn a_test() { let variant = 1; }\n";

    the_reading_reached_the_code_and_refused_nothing(
        source,
        "this is the other half of the pair: a bare call with nothing bringing the environment into scope is a name this reading has no standing to judge, and it is the half a gate looking at one line at a time would get wrong",
    );
}

#[test]
fn an_env_of_the_manifest_directory_is_accepted() {
    let source = r#"fn a_test() {
    let here = env!("CARGO_MANIFEST_DIR");
}
"#;

    the_reading_reached_the_code_and_refused_nothing(
        source,
        "a macro the compiler expands before the program runs, a variable cargo sets for the crate being compiled, and the only way a test can find the checkout: a gate that refused this would refuse four of this repository's own gates",
    );
}

#[test]
fn an_option_env_of_the_home_directory_is_accepted() {
    let source = r#"fn a_test() {
    let where = option_env!("HOME");
}
"#;

    the_reading_reached_the_code_and_refused_nothing(
        source,
        "the same expansion as `env!` with a different answer when the variable is not set, and neither of the two asks the running program anything",
    );
}

#[test]
fn the_arguments_of_the_program_are_accepted() {
    let source = "let the_arguments = std::env::args();\n";

    the_reading_reached_the_code_and_refused_nothing(
        source,
        "a program's arguments are handed to it rather than taken out of the machine it runs on, and a name in the list is not a name this gate judges",
    );
}

#[test]
fn the_whole_of_a_refused_call_written_inside_a_string_is_accepted() {
    let source = r#"let written = "std::env::var(";
fn a_test() {}
"#;

    the_reading_reached_the_code_and_refused_nothing(
        source,
        "the exact text of a refused call, held by a value: a string is a value and not an instruction, and a gate that refused this file could not hold a fixture and would be wrong about itself on its first run",
    );
}

#[test]
fn a_document_naming_the_read_is_accepted() {
    let source = "\
//! why a test may not read `std::env::var` at run time
fn a_test() {}
";

    the_reading_reached_the_code_and_refused_nothing(
        source,
        "this is the rule writing itself down, and a reading that could not tell its own documentation from its own code would refuse this file too",
    );
}

/// The real gate, and the only assertion here that reads the repository rather than a
/// fixture. Every one above is a demonstration; this one is the rule, and it is the one
/// worth keeping if the rest were deleted.
#[test]
fn no_test_reads_an_environment_variable() {
    let said = what_the_workspace_would_have_to_answer_for();

    assert!(
        said.is_empty(),
        "a suite that reads a credential is a suite that is not run for everybody, and it reports the same tick either way. A run-time read of the environment, by file, by line and by line:\n{said}"
    );
}

/// **A gate that read no files and agreed with all of them would be green and would be a
/// lie.** So the walk is asked what it found before anything is judged: a file under each
/// of the three roots, and the build-time macros the reading has to leave alone.
#[test]
fn the_walk_read_a_file_under_every_root_and_reached_the_macros_it_leaves_alone() {
    let found = scanning::every_rust_file_in_the_workspace();
    for root in ["src", "crates", "tests"] {
        let under = scanning::the_root_of_the_checkout().join(root);
        assert!(
            found.iter().any(|path| path.starts_with(&under)),
            "the walk read no .rs file under {root}/, so it agreed with a directory it never opened"
        );
    }
    let macros_saw = how_many_build_time_environment_macros_the_workspace_holds();
    assert!(
        macros_saw > 0,
        "the walk reached no `env!` in any of them, and a reading that stopped before one would agree with all of this repository exactly as completely as one that had read nothing"
    );
}
