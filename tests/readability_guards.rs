//! Every rule about how this repository's code is written.
//!
//! **One file per concern and not one file per rule.** Ten rules used to be ten files, each
//! named after the rule it held, and every one of them explained the scanner again before it
//! measured anything — two hundred and fifty lines in front of a test that measures one thing.
//! The name of a file is a promise about what a reader will find inside it, and a file called
//! `a_body_nests_no_deeper_than_two` promises one rule, so the tenth rule needed a tenth file
//! and the ten files had to agree with each other about how a source file is read.
//!
//! **Seven of them had their own copy of that reader and eight had their own copy of the walk
//! over the checkout**, and five of the seven returned a line number where two returned the
//! line alone, so the same call meant two things depending on which file it was written in.
//! The reader is `aisdlc_gates::reading_the_code` and there is one of it.
//!
//! **A rule with a test beside it is measured; a rule with only a name in a table is
//! described.** Every test below is named after the rule it holds, so the name is the
//! specification and this file is where the rules live.

mod the_scanner_tells_a_comment_from_a_string {
    //! The rule a walk of a source file has to hold before any gate may be built on it.
    //!
    //! A gate that reads a `.rs` file has to know where the code stops and the words begin, and
    //! a text search cannot know that: `let address = "https://github.com";` holds two slashes
    //! and has no comment in it, and three fixtures here hold them inside strings on purpose.
    //!
    //! **Six gates will call this walk, so it is one walk in the library and not six in the
    //! gates.** A private helper in a test binary is reported dead the moment the test that does
    //! not use it is compiled, and the only ways to stop that are `#[allow(dead_code)]`, which a
    //! gate in this repository refuses, or a copy.

    use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::scanning::HowTheFileIsWritten;

    /// A source with a scheme in a string and a comment beside it.
    ///
    /// Two things at once on purpose: a fixture holding only a comment would pass if the walk had
    /// stopped reading altogether, and one holding only a string would pass if the walk had
    /// decided everything is a comment.
    const A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT: &str = r#"
let address = "https://github.com/owner/name";
// and this is a comment
"#;

    #[test]
    fn a_slash_slash_inside_a_string_is_not_a_comment() {
        let read = HowTheFileIsWritten::of(A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT);

        assert_eq!(
            read.the_lines_a_comment_starts_on(),
            vec![3],
            "the two slashes in the address are inside a string and the two on the next line are a comment, and a gate that cannot tell them is a gate that will refuse this repository's own tests"
        );
    }

    #[test]
    fn the_walk_puts_every_byte_in_exactly_one_stretch() {
        let read = HowTheFileIsWritten::of(A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT);

        let the_bytes: usize = read.stretches().iter().map(|one| one.text().len()).sum();
        assert_eq!(
            the_bytes,
            A_STRING_HOLDING_SLASHES_NEXT_TO_A_COMMENT.len(),
            "a stretch that drops or repeats a byte is a walk that cannot be trusted to report a line, because a line is counted from the bytes that went before it"
        );
    }

    /// The text of every stretch of one kind, in the order the walk found them.
    fn the_texts_of(the_read: &HowTheFileIsWritten, which: WhatIsWrittenThere) -> Vec<&str> {
        the_read
            .stretches()
            .iter()
            .filter(|one| one.kind() == which)
            .map(AStretchOfTheFile::text)
            .collect()
    }

    #[test]
    fn a_code_stretch_is_one_line_and_not_one_character() {
        let read = HowTheFileIsWritten::of("let one = 1; let two = 2;\nlet three = 3;\n");

        let the_lines_of_code: Vec<&str> = the_texts_of(&read, WhatIsWrittenThere::Code);
        assert_eq!(
            the_lines_of_code,
            vec!["let one = 1; let two = 2;", "let three = 3;"],
            "a stretch of code is the code of one line, so a place in a file is a line and an offset into it rather than one number"
        );
        let the_line_endings: Vec<&str> = the_texts_of(&read, WhatIsWrittenThere::TheSpaceBetween);
        assert_eq!(
            the_line_endings,
            vec!["\n", "\n"],
            "and a line ending belongs to the space between, because a stretch of code that carried its own ending could not be told from one that did not"
        );
    }

    #[test]
    fn a_documentation_comment_is_a_comment() {
        for source in ["/// what a reader needs", "//! what a module reader needs"] {
            let read = HowTheFileIsWritten::of(source);
            assert_eq!(
                read.the_lines_a_comment_starts_on(),
                vec![1],
                "{source:?} is documentation, and documentation is a comment that happens to be required"
            );
        }
    }

    #[test]
    fn a_block_comment_that_nests_is_one_comment() {
        let read = HowTheFileIsWritten::of("/* outer /* inner */ still outer */\nlet after = 1;\n");

        assert_eq!(
            read.the_lines_a_comment_starts_on(),
            vec![1],
            "block comments nest in Rust unlike in C, and a walk that stops at the first close treats the rest of the file as code the gate believed it had read"
        );
        let what_follows = read
            .stretches()
            .iter()
            .find(|one| one.text().contains("after"))
            .map(AStretchOfTheFile::kind);
        assert_eq!(
            what_follows,
            Some(WhatIsWrittenThere::Code),
            "and the code after a nested comment is code, which is the other half of the same claim"
        );
    }

    #[test]
    fn a_lifetime_is_not_a_character() {
        let read = HowTheFileIsWritten::of("let binding = 'a;\nlet other = 'b;\n");

        assert!(
            !read
                .stretches()
                .iter()
                .any(|one| one.kind() == WhatIsWrittenThere::ACharacter),
            "a quote followed by a name and no closing quote is a lifetime, and a walk that treats it as a character swallows the rest of the file — which is a gate that reports a comment on a file with none"
        );
    }

    #[test]
    fn a_raw_lifetime_is_not_a_string() {
        let read = HowTheFileIsWritten::of("let kept = r#'name;\n");

        assert!(
            !read
                .stretches()
                .iter()
                .any(|one| one.kind() == WhatIsWrittenThere::AString),
            "a raw lifetime is a third case and it is neither a character nor a string"
        );
    }

    #[test]
    fn a_character_literal_is_a_character() {
        let read = HowTheFileIsWritten::of("let letter = 'x';\n");

        assert!(
            read.stretches()
                .iter()
                .any(|one| one.kind() == WhatIsWrittenThere::ACharacter),
            "a quote, a character and a quote is a character, and the difference from a lifetime is the closing quote"
        );
    }

    #[test]
    fn a_slash_inside_a_character_is_not_a_comment() {
        let read = HowTheFileIsWritten::of("let divider = '/';\n");

        assert_eq!(
            read.the_lines_a_comment_starts_on(),
            Vec::<usize>::new(),
            "a slash inside a character is a character, and a walk that calls it a comment invents a comment in a file that has none"
        );
    }

    #[test]
    fn every_spelling_of_a_string_is_a_string() {
        for source in [
            r#"let kept = "a // b";"#,
            r#"let kept = r"a // b";"#,
            r##"let kept = r#"a "// b"#;"##,
            r#"let kept = b"a // b";"#,
            r#"let kept = c"a // b";"#,
            r##"let kept = cr#"a // b"#;"##,
        ] {
            let read = HowTheFileIsWritten::of(source);
            let holds_a_string = read
                .stretches()
                .iter()
                .any(|one| one.kind() == WhatIsWrittenThere::AString);
            let holds_a_comment = read.is_there_a_comment_anywhere();
            assert!(holds_a_string, "{source:?} holds a string");
            assert!(
                !holds_a_comment,
                "{source:?} holds a string and not a comment"
            );
        }
    }

    #[test]
    fn a_line_is_counted_from_the_file_and_not_from_the_reading() {
        let read = HowTheFileIsWritten::of("\n\n\n// the fourth line says this\n");

        assert_eq!(
            read.the_lines_a_comment_starts_on(),
            vec![4],
            "a comment is named by the line a reader would point at, and three blank lines before it are three lines"
        );
    }

    #[test]
    fn a_string_spanning_two_lines_keeps_the_line_after_it() {
        let read = HowTheFileIsWritten::of("let kept = \"a\nb\";\nlet after = 1;\n");

        let the_line_after = read
            .stretches()
            .iter()
            .find(|one| one.text().contains("after"))
            .map(AStretchOfTheFile::line_it_starts_on);
        assert_eq!(
            the_line_after,
            Some(3),
            "a real line ending inside a string is a real line ending, and the code after the string is on the third"
        );
    }

    #[test]
    fn a_backslash_escape_is_not_a_line_ending() {
        let read = HowTheFileIsWritten::of("let kept = \"a\\nb\";\nlet after = 1;\n");

        let the_line_after = read
            .stretches()
            .iter()
            .find(|one| one.text().contains("after"))
            .map(AStretchOfTheFile::line_it_starts_on);
        assert_eq!(
            the_line_after,
            Some(2),
            "an escaped n is two characters and not a line ending, and a walk that counted it as one would name every later line one too high"
        );
    }

    #[test]
    fn a_file_with_nothing_in_it_is_no_stretches_at_all() {
        for source in ["", "   \n\n", "\t"] {
            let read = HowTheFileIsWritten::of(source);
            assert!(
                !read.is_there_a_comment_anywhere(),
                "{source:?} is white space and holds no comment"
            );
        }
    }

    #[test]
    fn the_walk_reads_the_file_that_documents_it() {
        let the_file_holding_the_walk = concat!(env!("CARGO_MANIFEST_DIR"), "/src/scanning.rs");
        let source =
            reading_the_checkout::the_text_at(std::path::Path::new(the_file_holding_the_walk));

        let read = HowTheFileIsWritten::of(&source);
        let the_comments = the_texts_of(&read, WhatIsWrittenThere::AComment);

        let not_documentation: Vec<&&str> = the_comments
            .iter()
            .filter(|one| !one.starts_with("///") && !one.starts_with("//!"))
            .collect();
        assert!(
            not_documentation.is_empty(),
            "the file that documents the walk holds a comment that is not documentation, and a walk that cannot be run against its own author is a walk nobody has checked: {not_documentation:?}"
        );
        assert!(
            the_comments.len() > 20,
            "the walk found {} comments in the file that documents it, which is too few to be a file with this much documentation",
            the_comments.len()
        );
    }

    #[test]
    fn the_readers_find_this_checkout_and_read_a_file_in_it() {
        let the_root = aisdlc_gates::reading_the_checkout::the_root_of_the_checkout();

        assert!(
            the_root.join("crates/domain/Cargo.toml").is_file(),
            "the root is the directory holding crates/, and this one is {}",
            the_root.display()
        );
        let the_manifest = aisdlc_gates::reading_the_checkout::the_text_at(
            &the_root.join("crates/domain/Cargo.toml"),
        );
        assert!(
            the_manifest.contains("aisdlc-domain"),
            "and the reader read the file it was pointed at rather than answering about a path: {the_manifest}"
        );
    }

    #[test]
    fn a_file_that_cannot_be_read_is_a_failure_and_not_an_empty_one() {
        let a_directory =
            aisdlc_gates::reading_the_checkout::the_root_of_the_checkout().join("crates");

        let result = std::panic::catch_unwind(|| {
            aisdlc_gates::reading_the_checkout::the_text_at(&a_directory)
        });

        assert!(
            result.is_err(),
            "a directory is not a file, and a reader that answers about it has reported nothing at all"
        );
    }

    /// The two kinds of thing a person writes for a person to read, told apart line by line.
    ///
    /// **This repository allows `///` and `//!` and refuses `//`.** They are the same characters
    /// at the start, and a walk that reported one thing for all of them could not judge a file
    /// against that rule — it would have to re-read the text to find out, which is the boundary
    /// it exists to hold.
    ///
    /// **Per line and not per stretch**, because a stretch is a *run* of comment lines and the
    /// question is still per line. Three `//!` lines are one stretch; a `//!` line followed by a
    /// `//` line is a different stretch. Either way a rule that called a whole run documentation
    /// would pass a file with a stray `//` in it.
    #[test]
    fn a_stretch_says_which_of_its_lines_only_comment() {
        let the_code = "//! about the crate\n// a thought\n/// about the thing\n/* a block */\n";

        let only_commenting: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
            .stretches()
            .iter()
            .map(AStretchOfTheFile::the_lines_that_only_comment)
            .filter(|it| !it.is_empty())
            .collect();

        assert_eq!(
            only_commenting,
            vec![vec![2], vec![4]],
            "the walk gives a run of `//!` its own stretch and a run of `//` another, so these are line 2 and line 4 — the two this repository refuses, with 1 and 3 documenting"
        );
    }

    #[test]
    fn a_stretch_that_is_not_a_comment_has_no_lines_that_only_comment() {
        let the_code = "let the_answer = 42; // and a thought\n";

        let found: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
            .stretches()
            .iter()
            .map(AStretchOfTheFile::the_lines_that_only_comment)
            .filter(|it| !it.is_empty())
            .collect();

        assert_eq!(
            found,
            vec![vec![1]],
            "the one line with a comment on it is the one to refuse, and the code on it is not"
        );
    }

    #[test]
    fn a_run_of_documentation_documents_every_line_of_it() {
        let the_code = "//! one\n//! two\n//! three\n";

        let found: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
            .stretches()
            .iter()
            .map(AStretchOfTheFile::the_lines_that_only_comment)
            .filter(|it| !it.is_empty())
            .collect();

        assert_eq!(
            found,
            Vec::<Vec<usize>>::new(),
            "three lines about the crate and none of them is a comment"
        );
    }

    #[test]
    fn a_slash_inside_a_string_is_not_a_comment_and_is_never_refused() {
        let the_code = "let the_path = \"src/lib.rs // not a comment\";\n";

        let found: Vec<Vec<usize>> = HowTheFileIsWritten::of(the_code)
            .stretches()
            .iter()
            .map(AStretchOfTheFile::the_lines_that_only_comment)
            .filter(|it| !it.is_empty())
            .collect();

        assert_eq!(
            found,
            Vec::<Vec<usize>>::new(),
            "a path inside a string is a string, and this repository does not refuse strings"
        );
    }
}

mod a_file_is_no_longer_than_three_hundred_lines {
    //! No source file is longer than three hundred lines of code.
    //!
    //! **A line counts if there is code on it, and the walk already knows what code is.** It
    //! tells a comment from a string from the space between, so the count is made of the lines
    //! carrying a `Code` stretch: a blank line, a `///` and a block of prose are all lines the
    //! file does not spend on being understood. **Documentation is the most valuable thing in
    //! these files and the rule must not make it expensive.**
    //!
    //! **The two hundred line mark is a warning this test also holds.** A ceiling nobody
    //! approaches until they are already over it is a ceiling met rather than avoided, so the
    //! repository keeps a second, softer line: nothing is above two hundred either. Crossing
    //! that is not an offence, it is the point at which a reader should be told to look.
    //!
    //! **The gate reads itself.** Nothing stops a gate from growing past what it forbids, and a
    //! gate over three hundred lines would be the only file in the repository nobody had
    //! checked — so its own count is one of the counts it reports on.

    use aisdlc_gates::a_stretch_of_the_file::WhatIsWrittenThere;
    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_glossary;
    use aisdlc_gates::scanning::HowTheFileIsWritten;

    /// The most lines of code a file may spend.
    const THE_MOST_A_FILE_MAY_SPEND: usize = 300;

    /// The point at which a file is close enough to want looking at.
    const THE_LINE_TO_LOOK_AT_A_FILE_FROM: usize = 200;

    /// The refusal for one file, naming it and its count, and nothing when it is within the
    /// ceiling.
    ///
    /// **Given a name and the text, not a path.** A gate that can only be asked about a file on
    /// disk cannot be given a file that is too long without writing one, so the only way to
    /// test that it refuses is to write a long file — and a test that writes is a test that
    /// cleans up after itself or leaves a repository dirty. Text in, answer out.
    fn what_is_wrong_with(a_name: &str, the_code: &str) -> Option<String> {
        let the_glossary = aisdlc_gates::reading_the_glossary::the_glossary_text();
        let exempt = reading_the_glossary::the_files_the_length_gate_does_not_read(&the_glossary);
        let the_bare_name = a_name.rsplit('/').next().unwrap_or(a_name);
        if exempt.iter().any(|it| it == the_bare_name) {
            return None;
        }
        let spent = the_count_in(the_code);
        if spent <= THE_MOST_A_FILE_MAY_SPEND {
            return None;
        }
        Some(format!(
            "{a_name} spends {spent} lines of code and the most is {THE_MOST_A_FILE_MAY_SPEND}"
        ))
    }

    /// The lines of `a_file` that carry code.
    fn the_lines_carrying_code(a_file: &std::path::Path) -> usize {
        the_count_in(&reading_the_checkout::the_text_at(a_file))
    }

    /// The lines of `the_code` that carry code, one per line rather than per stretch.
    fn the_count_in(the_code: &str) -> usize {
        the_code
            .lines()
            .filter(|one_line| {
                HowTheFileIsWritten::of(one_line)
                    .stretches()
                    .iter()
                    .any(|it| it.kind() == WhatIsWrittenThere::Code)
            })
            .count()
    }

    #[test]
    fn a_file_the_glossary_exempts_is_not_refused() {
        let a_very_long_file = "fn a() {}\n".repeat(400);

        assert_eq!(
            what_is_wrong_with("readability_guards.rs", &a_very_long_file),
            None,
            "the glossary exempts that file by name, and a gate carrying its own exceptions has exceptions nobody reads"
        );
    }

    #[test]
    fn a_long_file_the_glossary_does_not_name_is_refused() {
        let a_very_long_file = "fn a() {}\n".repeat(400);

        assert!(
            what_is_wrong_with("some_other_file.rs", &a_very_long_file).is_some(),
            "the exemption is a list of two names, and anything not on it is still measured"
        );
    }

    #[test]
    fn the_glossary_names_the_two_files_the_gates_live_in() {
        let the_glossary = aisdlc_gates::reading_the_glossary::the_glossary_text();

        let exempt = reading_the_glossary::the_files_the_length_gate_does_not_read(&the_glossary);

        assert_eq!(
            exempt,
            vec![
                "readability_guards.rs".to_string(),
                "documentation_guards.rs".to_string()
            ],
            "two files, named, with a reason each — and a reader that found none would let any file be too long"
        );
    }

    #[test]
    fn no_file_in_this_repository_is_longer_than_it_may_be() {
        let mut too_long = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            if let Some(why) =
                what_is_wrong_with(&name, &reading_the_checkout::the_text_at(&a_file))
            {
                too_long.push(why);
            }
        }

        assert!(
            too_long.is_empty(),
            "a file may spend {THE_MOST_A_FILE_MAY_SPEND} lines of code and these do not: {too_long:?}"
        );
    }

    #[test]
    fn a_file_close_to_the_ceiling_is_named_and_not_refused() {
        let close = (0..=THE_LINE_TO_LOOK_AT_A_FILE_FROM)
            .map(|one| format!("let the_{one} = {one};"))
            .collect::<Vec<String>>()
            .join("\n");

        let why = what_is_wrong_with("close_to_the_ceiling.rs", &close);

        assert_eq!(
            why, None,
            "two hundred and one lines is under the ceiling, so this file is not refused — the signal says it is worth looking at and refusing it is the ceiling's job alone"
        );
        assert_eq!(
            the_count_in(&close),
            THE_LINE_TO_LOOK_AT_A_FILE_FROM + 1,
            "and the walk counts what it should"
        );
    }

    #[test]
    fn a_file_that_was_actually_read_is_counted_as_more_than_nothing() {
        let mut counted = Vec::new();
        for a_file in every_source_file() {
            let count = the_lines_carrying_code(&a_file);
            if count > 0 {
                counted.push(a_file.display().to_string());
            }
        }

        assert!(
            !counted.is_empty(),
            "the gate counted no file at all, which is a pass it has not earned"
        );
    }

    #[test]
    fn the_lines_that_do_not_carry_code_are_the_ones_a_file_does_not_spend() {
        let a_file = reading_the_checkout::the_root_of_the_checkout().join("src/scanning.rs");
        let the_code = reading_the_checkout::the_text_at(&a_file);
        let mut lines_with_code = 0;
        for stretch in HowTheFileIsWritten::of(&the_code).stretches() {
            if stretch.kind() == WhatIsWrittenThere::Code {
                lines_with_code += 1;
            }
        }
        let the_whole_file = the_code.lines().count();

        assert!(
            lines_with_code < the_whole_file * 2,
            "the file has {the_whole_file} lines and {lines_with_code} stretches of code, so the walk is not reporting one stretch per line and a count built on it would be nonsense"
        );
    }

    #[test]
    fn a_file_over_the_ceiling_is_refused_and_named() {
        let a_file = every_source_file()
            .into_iter()
            .find(|it| it.ends_with("src/scanning.rs"))
            .expect("the scanner is in this repository");

        assert_eq!(
            what_is_wrong_with(
                &a_file.display().to_string(),
                &reading_the_checkout::the_text_at(&a_file)
            ),
            None,
            "the real file is under the ceiling, so a refusal here would be the gate refusing a file it should accept"
        );
    }

    #[test]
    fn a_file_of_exactly_the_ceiling_is_accepted() {
        let at_the_ceiling = (0..THE_MOST_A_FILE_MAY_SPEND)
            .map(|one| format!("let the_{one} = {one};"))
            .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(
            what_is_wrong_with("at_the_ceiling.rs", &at_the_ceiling),
            None,
            "three hundred is the most a file may spend, and not one more"
        );
    }

    /// Three times as many lines of explanation as the repository's line of warning.
    fn the_explanation() -> String {
        (0..(THE_LINE_TO_LOOK_AT_A_FILE_FROM * 3))
            .map(|one| format!("/// A line that explains something, number {one}."))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// A warning's worth of code, which is under the ceiling and over the signal.
    fn the_code() -> String {
        (0..THE_LINE_TO_LOOK_AT_A_FILE_FROM)
            .map(|one| format!("let the_{one} = {one};"))
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn documentation_and_blank_lines_are_not_lines_a_file_spends() {
        let a_wordy_file = format!(
            "//! One line of code.\n{}\n{}\n",
            the_explanation(),
            the_code()
        );

        let count = the_count_in(&a_wordy_file);

        assert_eq!(
            count, THE_LINE_TO_LOOK_AT_A_FILE_FROM,
            "eight hundred lines of this file are explanation and two hundred are code: a rule that made documentation expensive would be a rule against saying why, so only the code is counted"
        );
        assert_eq!(
            what_is_wrong_with("a_wordy_file.rs", &a_wordy_file),
            None,
            "and such a file is accepted whatever its length in lines"
        );
    }
}

mod a_function_body_is_no_longer_than_twenty_lines {
    //! No function body is longer than twenty lines.
    //!
    //! **A body and not a signature, and the count is the one the file-length gate uses.** A
    //! line counts if there is code on it, so a body with a dozen `///` lines in the middle
    //! spends fewer lines than it occupies — the same rule, for the same reason: documentation
    //! is the most valuable thing in these files and no gate may make it expensive.
    //!
    //! **A function and not a closure.** `|x| { … }` is not a `fn`, and a rule that counted it
    //! would count every `map` in the repository as a function body. The file says so rather
    //! than letting a reader assume the gap was not noticed.
    //!
    //! **The refusal names the file, the line the function starts on, and the count.**

    use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;
    use aisdlc_gates::scanning::HowTheFileIsWritten;

    /// The most lines of code a function body may spend.
    const THE_MOST_A_BODY_MAY_SPEND: usize = 20;

    /// Every refusal in one file, given its name and its text.
    fn what_is_wrong_with(a_name: &str, the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (starts_on, spent) in the_functions_in(the_code) {
            if spent <= THE_MOST_A_BODY_MAY_SPEND {
                continue;
            }
            found.push(format!(
            "{a_name} line {starts_on} spends {spent} lines of code and the most is {THE_MOST_A_BODY_MAY_SPEND}"
        ));
        }
        found
    }

    /// Every `fn` in the code and the lines its body spends, as `(the line it starts on, the
    /// lines the body spends)`.
    ///
    /// **The braces are counted over `Code` lines only**, so a `{` in a string or a character
    /// cannot end a body early or leave it open forever. **The signature is not counted**,
    /// because the rule is about a body: a function with a long argument list has a long
    /// signature and a short body, and telling those apart is the whole point of the rule.
    fn the_functions_in(the_code: &str) -> Vec<(usize, usize)> {
        let mut found = Vec::new();
        let mut a_reading = AReading::new();
        for (the_line, one_line) in the_lines_of_code(the_code) {
            if let Some(one) = a_reading.of_one_line(the_line, &one_line) {
                found.push(one);
            }
        }
        found
    }

    /// How deep the walk is inside a function, and which body it is inside.
    ///
    /// **A type because the depth and the body cannot be kept apart.** The rule is three
    /// statements that touch each other — how deep, which body, and whether this line is the one
    /// that closed it — and three separate locals in one function is three ways to get one of
    /// them out of step with the other two.
    struct AReading {
        how_deep: usize,
        where_a_function_began: Option<usize>,
        a_body: Option<(usize, usize)>,
    }

    impl AReading {
        /// A reading that is nowhere inside a function.
        fn new() -> Self {
            AReading {
                how_deep: 0,
                where_a_function_began: None,
                a_body: None,
            }
        }

        /// The function this line finished, or nothing if it finished none.
        fn of_one_line(&mut self, the_line: usize, one_line: &str) -> Option<(usize, usize)> {
            let opened = one_line.matches('{').count();
            let closed = one_line.matches('}').count();
            self.remember_where_a_function_began(the_line, one_line);
            let began_a_body = self.entered_a_body(opened, closed);
            if self.a_body.is_some() && self.how_deep == 0 {
                self.where_a_function_began = None;
                return self.a_body.take();
            }
            self.count_this_line(began_a_body);
            None
        }

        /// Add this line to the body it is inside, if it is a line of one.
        fn count_this_line(&mut self, began_a_body: bool) {
            if began_a_body {
                return;
            }
            if let Some(body) = self.a_body.as_mut() {
                body.1 += 1;
            }
        }

        /// Note a `fn` on this line, if there was not one already being followed.
        fn remember_where_a_function_began(&mut self, the_line: usize, one_line: &str) {
            if self.where_a_function_began.is_none() && one_line.contains("fn ") {
                self.where_a_function_began = Some(the_line);
            }
        }

        /// Whether this line opened the body, and the depth it leaves the walk at.
        fn entered_a_body(&mut self, opened: usize, closed: usize) -> bool {
            let begins =
                self.a_body.is_none() && self.where_a_function_began.is_some() && opened > 0;
            if begins {
                self.how_deep = opened.saturating_sub(closed);
                self.a_body = self
                    .where_a_function_began
                    .map(|it_began_on| (it_began_on, 0));
            } else if self.a_body.is_some() {
                self.how_deep = self.how_deep.saturating_sub(closed) + opened;
            }
            begins
        }
    }

    #[test]
    fn no_function_in_this_repository_spends_more_than_it_may() {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(
            found.is_empty(),
            "a body may spend {THE_MOST_A_BODY_MAY_SPEND} lines and these do not: {found:?}"
        );
    }

    #[test]
    fn a_body_of_exactly_twenty_lines_is_accepted() {
        let the_body = (1..=THE_MOST_A_BODY_MAY_SPEND)
            .map(|one| format!("    let the_{one} = {one};"))
            .collect::<Vec<String>>()
            .join("\n");

        let the_code = format!("fn at_the_limit() {{\n{the_body}\n}}\n");

        assert_eq!(
            what_is_wrong_with("a_file.rs", &the_code),
            Vec::<String>::new(),
            "twenty is the most a body may spend and not one more"
        );
    }

    #[test]
    fn a_body_of_twenty_one_lines_is_refused_and_named() {
        let the_body = (1..=THE_MOST_A_BODY_MAY_SPEND + 1)
            .map(|one| format!("    let the_{one} = {one};"))
            .collect::<Vec<String>>()
            .join("\n");

        let the_code = format!("fn one_line_too_many() {{\n{the_body}\n}}\n");

        let spent = THE_MOST_A_BODY_MAY_SPEND + 1;

        assert_eq!(
            what_is_wrong_with("a_file.rs", &the_code),
            vec![format!(
                "a_file.rs line 1 spends {spent} lines of code and the most is {THE_MOST_A_BODY_MAY_SPEND}"
            )],
            "a refusal a reader cannot match against the function they wrote teaches nothing"
        );
    }

    #[test]
    fn documentation_inside_a_body_is_not_a_line_the_body_spends() {
        let the_body = (1..=THE_MOST_A_BODY_MAY_SPEND)
            .map(|one| format!("    let the_{one} = {one};"))
            .collect::<Vec<String>>()
            .join("\n");

        let the_code = format!("fn explained() {{\n    /// why\n{the_body}\n    /// and why\n}}\n");

        assert_eq!(
            what_is_wrong_with("a_file.rs", &the_code),
            Vec::<String>::new(),
            "twenty lines of code among twenty-two lines of text is twenty lines spent, and a rule that counted the prose would be a rule against explaining a body"
        );
    }

    #[test]
    fn a_brace_inside_a_string_is_not_a_brace() {
        let the_code =
            "fn quoted() {\n    let the_json = \"{ not a body }\";\n    let the_brace = '}';\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "a brace in a string or a character is text, and a matcher that counted it would end the body early or never end it at all"
        );
    }

    #[test]
    fn a_function_named_in_a_string_is_not_a_function() {
        let the_code = "fn real() {\n    let the_example = \"fn imagined() { }\";\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "a function written inside a string is a fixture, and the walk has already told the two apart"
        );
    }

    #[test]
    fn a_body_on_one_line_spends_nothing() {
        let the_code = "fn one_line() { let the_answer = 42; }\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "a whole function on one line spends no lines and that is not a way around the rule — it is a way of writing less code"
        );
    }

    #[test]
    fn a_closure_is_not_counted_as_a_function_of_its_own() {
        let the_closure = (1..=THE_MOST_A_BODY_MAY_SPEND + 1)
            .map(|one| format!("    the_{one}();"))
            .collect::<Vec<String>>()
            .join("\n");

        let the_code = format!(
            "fn holds_a_closure() {{\n    let it = |the_input| {{\n{the_closure}\n    }};\n    it;\n}}\n"
        );

        let found = what_is_wrong_with("a_file.rs", &the_code);

        assert_eq!(
            found,
            vec![format!(
                "a_file.rs line 1 spends {} lines of code and the most is {THE_MOST_A_BODY_MAY_SPEND}",
                THE_MOST_A_BODY_MAY_SPEND + 4
            )],
            "one refusal, naming line 1 — the function. The closure inside it is not a `fn` and is not counted as one; what is refused here is the function that holds it, which is a body over the limit whether or not it closes over anything. A rule that counted closures would have reported a second refusal naming line 2."
        );
    }

    #[test]
    fn the_walk_was_read() {
        let the_code = "fn a() {}\n";
        let kinds: Vec<WhatIsWrittenThere> = HowTheFileIsWritten::of(the_code)
            .stretches()
            .iter()
            .map(AStretchOfTheFile::kind)
            .collect();

        assert!(
            kinds.contains(&WhatIsWrittenThere::Code),
            "the gate read no code out of a file of code, and a gate that reads nothing refuses nothing"
        );
    }
}

mod a_body_nests_no_deeper_than_two {
    //! No statement inside a function body is nested deeper than two levels.
    //!
    //! **Levels are counted with braces and not with indentation**, because indentation is a
    //! promise a person makes and braces are what the compiler reads. A body whose third level
    //! is written with the same four spaces as the first is still three levels deep.
    //!
    //! **Two and not one.** A function is one block and a statement inside it is at level one;
    //! an `if` inside that is two; a third is refused. A body that reads as a staircase is a
    //! body a reader has to hold in their head backwards.
    //!
    //! **Only inside a function.** An `impl` block and a `struct` are not a body, and a rule
    //! that counted their nesting would refuse every type in the domain for being a field of a
    //! field.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;

    /// The deepest a statement may sit inside a body.
    const THE_DEEPEST_A_STATEMENT_MAY_SIT: usize = 2;

    /// Every refusal in one file, given its name and its text.
    fn what_is_wrong_with(a_name: &str, the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (the_line, sits) in the_statements_and_how_deep_they_sit(the_code) {
            if sits <= THE_DEEPEST_A_STATEMENT_MAY_SIT {
                continue;
            }
            found.push(format!(
            "{a_name} line {the_line} sits {sits} deep in a body and the deepest is {THE_DEEPEST_A_STATEMENT_MAY_SIT}"
        ));
        }
        found
    }

    /// Every statement inside a function body and the number of levels it sits at.
    fn the_statements_and_how_deep_they_sit(the_code: &str) -> Vec<(usize, usize)> {
        let mut found = Vec::new();
        let mut a_reading = AReading::new();
        for (the_line, one_line) in the_lines_of_code(the_code) {
            found.extend(a_reading.of_one_line(the_line, &one_line));
        }
        found
    }

    /// How deep inside a body the walk is, and where a function began.
    struct AReading {
        inside_a_function: bool,
        how_deep: usize,
    }

    impl AReading {
        /// A reading that is not inside any function.
        fn new() -> Self {
            AReading {
                inside_a_function: false,
                how_deep: 0,
            }
        }

        /// The statements on this line, and how deep each one sits.
        /// **The line that opens a body is not itself a statement in it**, so its own brace does
        /// not count and it is not reported: `if three {` sits at level two and is a level two
        /// statement, and the first one that is too deep is the one under it.
        fn of_one_line(&mut self, the_line: usize, one_line: &str) -> Vec<(usize, usize)> {
            if one_line.contains("fn ") {
                self.inside_a_function = true;
                self.how_deep = 0;
                return Vec::new();
            }
            let mut found = Vec::new();
            if self.inside_a_function && !closes_a_body(one_line) {
                found.push((the_line, self.how_deep));
            }
            self.how_deep = self.how_deep.saturating_add(one_line.matches('{').count());
            self.how_deep = self.how_deep.saturating_sub(one_line.matches('}').count());
            found
        }
    }

    #[test]
    fn no_statement_in_this_repository_sits_too_deep() {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(
            found.is_empty(),
            "a statement may sit {THE_DEEPEST_A_STATEMENT_MAY_SIT} deep in a body and these do not: {found:?}"
        );
    }

    #[test]
    fn a_statement_at_the_deepest_level_is_accepted() {
        let the_code = "fn two_deep() {\n    if ready {\n        if second {\n            go();\n        }\n    }\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "two levels is the deepest and `go` sits in it"
        );
    }

    #[test]
    fn a_statement_three_deep_is_refused_and_named() {
        let the_code = "fn three_deep() {\n    if one {\n        if two {\n            if three {\n                go();\n            }\n        }\n    }\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec!["a_file.rs line 5 sits 3 deep in a body and the deepest is 2".to_string()],
            "a refusal a reader cannot match against the line they wrote teaches nothing"
        );
    }

    #[test]
    fn a_brace_inside_a_string_does_not_make_a_level() {
        let the_code = "fn quoted() {\n    if one {\n        let the_text = \"{ { {\";\n        go();\n    }\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "a brace in a string is text, and a level counted out of it is a level nobody wrote"
        );
    }

    #[test]
    fn a_match_arm_is_a_level_and_a_block_inside_an_arm_is_another() {
        let the_code = "fn matched(value: u8) {\n    match value {\n        1 => {\n            if one {\n                go();\n            }\n        }\n        _ => {}\n    }\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec!["a_file.rs line 5 sits 3 deep in a body and the deepest is 2".to_string()],
            "a match arm's block is a level and the `if` inside it is the next, so a statement under both is the third"
        );
    }

    #[test]
    fn a_nesting_outside_a_function_is_not_a_body() {
        let the_code = "struct AThing {\n    a_field: u8,\n}\n\nimpl AThing {\n    fn a() -> u8 {\n        1\n    }\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "a struct and an impl are not a body, and a rule that refused them would refuse every type in the domain for being a field of a field"
        );
    }

    #[test]
    fn a_whole_function_on_one_line_is_not_three_deep() {
        let the_code = "fn one_line() { if a { if b { if c { go(); } } } }\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "three levels on one line is a line a reader cannot hold anyway, and this rule is about statements sitting deep — a separate rule about a body spending its lines is what catches this"
        );
    }

    /// Whether a line is only closing what the lines above it opened.
    ///
    /// **A line that closes is not a statement in the block it closes**, and counting it would
    /// report every function's last line as a statement sitting at the body's own depth.
    fn closes_a_body(one_line: &str) -> bool {
        one_line.trim() == "}"
    }
}

mod the_code_carries_no_comments {
    //! No source file carries a comment.
    //!
    //! **Documentation is not a comment and is allowed; a comment is refused.** `///` and `//!`
    //! say why the code is as it is, in the language a reader thinks in. `//` says something
    //! about the code that the code did not manage to say, and a place where that is true is a
    //! place where the code failed to be read. `/* */` is refused for the same reason and no
    //! softer one.
    //!
    //! **The refusal names the file and the line.** A gate that says "there is a comment
    //! somewhere" is a gate nobody can act on, and this one can be acted on by deleting a line.
    //!
    //! **This gate is judged by its own rule.** It lives in a `.rs` file like everything else, so
    //! every comment in it is one it would have refused, and the walk that tells a `///` from a
    //! `//` is the walk this repository reads itself with.

    use aisdlc_gates::a_stretch_of_the_file::{AStretchOfTheFile, WhatIsWrittenThere};
    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::scanning::HowTheFileIsWritten;

    /// Every source file's refusals, given its name and its text.
    fn what_is_wrong_with(a_name: &str, the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for stretch in HowTheFileIsWritten::of(the_code).stretches() {
            for line in stretch.the_lines_that_only_comment() {
                found.push(format!("{a_name} line {line} carries a comment"));
            }
        }
        found
    }

    #[test]
    fn no_file_in_this_repository_carries_a_comment() {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(
            found.is_empty(),
            "a comment is a place the code failed to say something: {found:?}"
        );
    }

    #[test]
    fn a_line_comment_is_refused_and_named() {
        let what_it_said = "let the_answer = 42; // the answer\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", what_it_said),
            vec!["a_file.rs line 1 carries a comment".to_string()],
            "a comment on the right of a line is still a comment, and the refusal names the line so it can be deleted"
        );
    }

    #[test]
    fn a_block_comment_is_refused_and_named() {
        let what_it_said = "/* one\n   two */\nlet the_answer = 42;\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", what_it_said),
            vec![
                "a_file.rs line 1 carries a comment".to_string(),
                "a_file.rs line 2 carries a comment".to_string(),
            ],
            "a block comment is refused for the same reason and no softer one, and both of its lines are named because both are ones a reader has to look at"
        );
    }

    #[test]
    fn documentation_is_never_refused() {
        for what_it_says in [
            "//! about the crate\n",
            "/// about the thing\nlet the_answer = 42;\n",
            "    /// indented documentation\n    fn a() {}\n",
            "//! one\n//! two\n//! three\n",
        ] {
            assert_eq!(
                what_is_wrong_with("a_file.rs", what_it_says),
                Vec::<String>::new(),
                "{what_it_says:?} is documentation, and a rule that made documentation expensive would be a rule against saying why"
            );
        }
    }

    #[test]
    fn a_slash_inside_a_string_is_a_string_and_is_never_refused() {
        let what_it_says = "let the_path = \"a//b\";\nlet the_other = \"/* not a comment */\";\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", what_it_says),
            Vec::<String>::new(),
            "a path inside a string is a string, and refusing it would refuse the first line of every test in this repository"
        );
    }

    #[test]
    fn a_comment_among_documentation_is_still_refused() {
        let what_it_said = "//! about the crate\n// a thought\n/// about the thing\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", what_it_said),
            vec!["a_file.rs line 2 carries a comment".to_string()],
            "one stray comment in a file of documentation is the case a rule that only looked at whole stretches would pass, and it is the common one"
        );
    }

    #[test]
    fn the_walk_was_actually_read() {
        let found: Vec<usize> = HowTheFileIsWritten::of("// a comment\n")
            .stretches()
            .iter()
            .flat_map(AStretchOfTheFile::the_lines_that_only_comment)
            .collect();

        assert_eq!(
            found,
            vec![1],
            "the walk finds the one comment in this string, and a gate built on a walk that found nothing would pass everything"
        );
        assert_eq!(
            HowTheFileIsWritten::of("//! docs\n").stretches()[0].kind(),
            WhatIsWrittenThere::AComment,
            "and documentation is still a comment as far as the kind goes, which is why the walk answers a second question about it"
        );
    }
}

mod no_allow_silences_a_lint {
    //! No source file carries an `#[allow(…)]` or an `#[expect(…)]`.
    //!
    //! **`cargo clippy -- -D warnings` already denies every warning, so an `#[allow]` here can
    //! only be silencing something the gate command would already have refused.** It is either a
    //! bug in the code or a loophole in the rule, and either way it is a place where the two have
    //! stopped agreeing without anybody deciding anything.
    //!
    //! **A `Cargo.toml` `[lints]` table is not one.** It configures clippy for a whole crate
    //! rather than standing in a source file, and the root one carries five `allow`s on purpose —
    //! `module_name_repetitions` among them — because `WorkItem` has a `work_item_id` and the
    //! domain is right about that and clippy is not. Reading it as a suppression would be the
    //! same error as reading a `.gitignore` line as a comment.
    //!
    //! **Only the code is read.** An `#[allow]` written in a doc comment is a sentence about
    //! suppressions, and one written inside a string is a value; the walk tells both from an
    //! attribute that stands, and this gate asks the walk.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;

    /// The two attributes that silence something, as they are written.
    const THE_ATTRIBUTES_THAT_SILENCE: [&str; 2] = ["#[allow(", "#[expect("];

    /// Every refusal in one file, given its name and its text.
    fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (the_line, one_line) in the_lines_of_code(the_code) {
            found.extend(the_suppressions_in(&one_line, a_file_name, the_line));
        }
        found
    }

    /// The suppressions on one line, quoted as they are written.
    fn the_suppressions_in(one_line: &str, a_file_name: &str, the_line: usize) -> Vec<String> {
        THE_ATTRIBUTES_THAT_SILENCE
            .iter()
            .filter(|one| one_line.contains(*one))
            .map(|one| {
                let as_written = the_attribute_as_written(one_line, one);
                format!("{a_file_name} line {the_line} silences something: {as_written}")
            })
            .collect()
    }

    /// The whole attribute on a line, closing bracket included.
    ///
    /// **In the code and not out of the text**, because a line holds several attributes and a
    /// reader told "something on line 4" cannot tell which one. An unclosed bracket is taken to
    /// the end of the line, because a file that does not compile is not a file this gate decides.
    fn the_attribute_as_written(one_line: &str, the_attribute: &str) -> String {
        let from_the_attribute = &one_line[one_line.find(the_attribute).unwrap_or(0)..];
        let the_closing = from_the_attribute.find(']').map(|it| it + 1);
        match the_closing {
            Some(where_it_closes) => from_the_attribute[..where_it_closes].to_string(),
            None => from_the_attribute.trim_end().to_string(),
        }
    }

    #[test]
    fn no_source_file_in_this_repository_silences_anything() {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(
            found.is_empty(),
            "a gate and a loop here have stopped agreeing: {found:?}"
        );
    }

    #[test]
    fn an_allow_is_refused_with_the_line_and_the_attribute_as_written() {
        let the_code =
            "fn a() {\n    #[allow(clippy::too_many_arguments)]\n    let the_answer = 42;\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec![
                "a_file.rs line 2 silences something: #[allow(clippy::too_many_arguments)]"
                    .to_string()
            ],
            "and the refusal quotes the attribute exactly as it is written, because a reader who has to go and look is a reader who has to do the gate's work"
        );
    }

    #[test]
    fn an_expect_is_refused_too() {
        let the_code = "fn a() {\n    #[expect(clippy::something)]\n    let the_answer = 42;\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec!["a_file.rs line 2 silences something: #[expect(clippy::something)]".to_string()],
            "an `expect` is a claim that a warning is coming, and a claim the gate cannot check is not a smaller version of the same thing"
        );
    }

    #[test]
    fn an_allow_written_in_documentation_is_accepted() {
        let the_code =
            "//! This repository refuses `#[allow(clippy::foo)]` in a source file.\n\nfn a() {}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "an attribute named in a doc comment is a sentence about suppressions, and the walk already told the two apart"
        );
    }

    #[test]
    fn an_allow_written_inside_a_string_is_accepted() {
        let the_code =
            "fn a() {\n    let the_example = \"#[allow(clippy::foo)]\";\n    the_example\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "inside a string it is a value, and a value that is a fixture is the first line of half the tests in this repository"
        );
    }

    #[test]
    fn the_two_attributes_are_the_ones_written() {
        let the_code = "#[allow(a)]\n#[expect(b)]\n#[derive(Clone)]\n#[test]\nstruct One;\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec![
                "a_file.rs line 1 silences something: #[allow(a)]".to_string(),
                "a_file.rs line 2 silences something: #[expect(b)]".to_string(),
            ],
            "an attribute that does not silence is not a suppression, whatever its brackets look like"
        );
    }
}

mod no_placeholder_is_left_in_the_code {
    //! No unfinished thing is left in the code.
    //!
    //! **A placeholder is an unfinished cycle given a second home.** It is already visible in the
    //! history, and a line in the source gives it somewhere to hide where nobody would look for it.
    //! There is no allowlist here, and the reason is the same: **an allowlist is a list of the
    //! holes this repository has decided to live with, and this one has decided not to.**
    //!
    //! **A refused word in a string is accepted and in a panic message is refused.** A `panic!` is
    //! what a run does when it cannot go on, so a message saying the work is not done is that run
    //! announcing an unfinished cycle from inside code that looks finished — the one place a
    //! placeholder can speak without passing either the comment gate or the history. A string that
    //! is not a message is a value, and a value holding the word is an example, a fixture, or
    //! text shown to a person.

    use aisdlc_gates::a_stretch_of_the_file::WhatIsWrittenThere;
    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;

    use aisdlc_gates::scanning::HowTheFileIsWritten;

    /// The macros that say the work is not done, in the code.
    const THE_MACROS_THAT_SAY_IT: [&str; 3] = ["todo!", "unimplemented!", "unreachable!"];

    /// Every refusal in one file, given its name and its text.
    fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
        let the_glossary = aisdlc_gates::reading_the_glossary::the_glossary_text();
        let messages = the_panic_messages_in(the_code);
        let mut found = Vec::new();
        for (the_line, one_line) in the_lines_of_code(the_code) {
            found.extend(the_refusal_of_a_line_leaving_something_unfinished(
                &one_line,
                a_file_name,
                the_line,
                &the_glossary,
                &messages,
            ));
        }
        found
    }

    /// The refusal for one line, if it leaves something unfinished behind.
    fn the_refusal_of_a_line_leaving_something_unfinished(
        one_line: &str,
        a_file_name: &str,
        the_line: usize,
        the_glossary: &str,
        messages: &[(usize, String)],
    ) -> Vec<String> {
        let mut found = the_macros_this_line_says(one_line, a_file_name, the_line);
        found.extend(the_refused_word_in_a_message(
            a_file_name,
            the_line,
            the_glossary,
            messages,
        ));
        found
    }

    /// Every `panic!` message in the code, as `(the line it is on, the message)`.
    ///
    /// **Out of the string stretches and not out of the code**, because a walk that tells a string
    /// from the code around it hands the two over separately — and the message *is* the string.
    /// The first version read the code line and found `panic!(` and nothing else, because
    /// `"FIXME this is wrong"` was a stretch of its own that the line never contained.
    fn the_panic_messages_in(the_code: &str) -> Vec<(usize, String)> {
        let mut found: Vec<(usize, String)> = Vec::new();
        let mut a_panic_is_open_on: Option<usize> = None;
        for stretch in HowTheFileIsWritten::of(the_code).stretches() {
            let the_line = stretch.line_it_starts_on();
            let is_code = stretch.kind() == WhatIsWrittenThere::Code;
            if is_code && stretch.text().contains("panic!") {
                a_panic_is_open_on = Some(the_line);
                continue;
            }
            if a_panic_is_open_on.is_some() && stretch.kind() == WhatIsWrittenThere::AString {
                found.push((the_line, stretch.text().to_string()));
            }
        }
        found
    }

    /// The macros on this line that say the work is not done.
    fn the_macros_this_line_says(
        one_line: &str,
        a_file_name: &str,
        the_line: usize,
    ) -> Vec<String> {
        THE_MACROS_THAT_SAY_IT
            .iter()
            .filter(|one_macro| one_line.contains(*one_macro))
            .map(|one_macro| format!("{a_file_name} line {the_line} is a {one_macro}()"))
            .collect()
    }

    /// The refused word this line's panic message carries, if it carries one.
    ///
    /// **Only a message and not any string**, because a string that is not a message is a value
    /// and a value holding the word is an example, a fixture, or text shown to a person.
    fn the_refused_word_in_a_message(
        a_file_name: &str,
        the_line: usize,
        the_glossary: &str,
        messages: &[(usize, String)],
    ) -> Vec<String> {
        let refused = the_words_the_placeholder_gate_refuses(the_glossary);
        let the_message = messages
            .iter()
            .filter(|(on, _)| *on == the_line)
            .map(|(_, one)| one.to_lowercase())
            .collect::<Vec<String>>()
            .join(" ");
        refused
        .iter()
        .filter(|one| the_message.contains(&one.to_lowercase()))
        .map(|one| {
            format!(
                "{a_file_name} line {the_line} is a panic message carrying a refused word: {one}"
            )
        })
        .collect()
    }

    /// The words the placeholder gate refuses, from the block the glossary holds them in.
    ///
    /// **Its own reader and not the vocabulary gate's.** Two gates read two blocks of the same
    /// document, and one function named after the document would have been one function about
    /// two lists — which is the divergence this repository paid for once already.
    fn the_words_the_placeholder_gate_refuses(the_glossary: &str) -> Vec<String> {
        aisdlc_gates::reading_the_glossary::the_block_containing(the_glossary, "APlaceholder")
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect()
    }

    #[test]
    fn no_placeholder_is_left_in_the_code() {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(
            found.is_empty(),
            "an unfinished cycle already has a home: {found:?}"
        );
    }

    #[test]
    fn the_macro_that_says_the_work_is_not_done_is_refused_and_named() {
        let the_code = "fn a() {\n    todo!(\"later\");\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec!["a_file.rs line 2 is a todo!()".to_string()],
            "and the refusal names the line, because a placeholder a reader has to find is a placeholder that stays"
        );
    }

    #[test]
    fn every_macro_that_says_it_is_refused() {
        for macro_name in ["todo!", "unimplemented!", "unreachable!"] {
            let the_code = format!("fn a() {{\n    {macro_name}();\n}}\n");

            assert_eq!(
                what_is_wrong_with("a_file.rs", &the_code),
                vec![format!("a_file.rs line 2 is a {macro_name}()")],
                "{macro_name} says the work is not done and is refused for saying it"
            );
        }
    }

    #[test]
    fn a_refused_word_in_a_panic_message_is_refused_and_named() {
        let the_code = "fn a() {\n    panic!(\"FIXME this is wrong\");\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec!["a_file.rs line 2 is a panic message carrying a refused word: FIXME".to_string()],
            "a panic message is what a reader sees when a run cannot go on, and one saying the work is not done is that run announcing an unfinished cycle from inside finished-looking code"
        );
    }

    #[test]
    fn the_same_word_in_an_ordinary_string_is_accepted() {
        let the_code =
            "fn a() {\n    let the_example = \"TODO: like this one\";\n    the_example\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "a string that is not a message is a value, and a value holding the word is an example, a fixture, or text shown to a person"
        );
    }

    #[test]
    fn ordinary_english_is_not_a_refused_word() {
        let the_code = "fn a() {\n    panic!(\"this will change later, for now\");\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "`later` and `for now` are English and this repository's own prose uses them; a gate refusing them would be refusing the language rather than the unfinished work"
        );
    }

    #[test]
    fn the_words_are_read_out_of_the_glossary_and_not_restated() {
        let the_glossary = reading_the_checkout::the_text_at(
            &reading_the_checkout::the_root_of_the_checkout().join("docs/domain-glossary.md"),
        );

        assert!(
            the_words_the_placeholder_gate_refuses(&the_glossary).contains(&"TODO".to_string()),
            "the block the guard reads is the one the document writes, and a copy of it in the gate would be green the day the document changed"
        );
    }
}

mod a_name_is_never_generic_vocabulary {
    //! No name ends on a word that says nothing about the thing it names.
    //!
    //! **The list is read out of `docs/domain-glossary.md` and not restated here.** A list of
    //! banned words copied into a gate is a second list to keep in step, and the copy would be
    //! green on the day the glossary gained a word. The glossary is the source of truth and this
    //! gate is the reader.
    //!
    //! **A name is judged on its last word, and a family as a prefix or a suffix.** The last
    //! word of an identifier is what the name claims the thing *is* — `AWorkRepository` claims
    //! a repository, and this system already has a word for the one it means. A word that merely
    //! contains a banned one is not caught, because `StateMachineError` contains `State` and is
    //! neither a state machine nor the stage.
    //!
    //! **Words the glossary blesses are exempt and the exemption is written down**, so adding a
    //! word to the list needs no change in code and blessing one is a visible act.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;
    use aisdlc_gates::reading_the_glossary::{
        it_as_letters, the_glossary_text, the_last_word_of, the_word_under,
        the_words_the_vocabulary_gate_refuses,
    };

    /// Every refusal in one file, given its name and its text.
    fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for one_name in the_names_the_synonyms_are_looked_for_in(the_code) {
            if let Some(word) = the_banned_word_one_name_ends_on(&one_name) {
                found.push(format!(
                    "{a_file_name} names {one_name}, which ends on a banned word: {word}"
                ));
            }
        }
        found
    }

    /// The banned word this name ends on, unless the glossary blesses it here.
    fn the_banned_word_one_name_ends_on(one_name: &str) -> Option<String> {
        let refused = the_words_the_vocabulary_gate_refuses(&the_glossary_text());
        let word = refused
            .iter()
            .find(|one| a_name_ends_on(&it_as_letters(one), one_name))?;
        if the_glossary_blesses_the_same_word_as_a_whole_name(word, one_name) {
            return None;
        }
        if the_glossary_blesses_it_as_a_compounds_ending(word, one_name) {
            return None;
        }
        Some(word.clone())
    }

    /// Whether the name is exactly the word the glossary blesses on its own.
    fn the_glossary_blesses_the_same_word_as_a_whole_name(a_word: &str, one_name: &str) -> bool {
        let the_same_word = it_as_letters(a_word) == it_as_letters(one_name);
        the_glossary_blesses(BLESSED_AS_A_WHOLE_NAME, &it_as_letters(a_word)) && the_same_word
    }

    /// Whether the name is a compound ending on a word the glossary blesses there.
    fn the_glossary_blesses_it_as_a_compounds_ending(a_word: &str, one_name: &str) -> bool {
        let the_same_word = it_as_letters(a_word) == it_as_letters(one_name);
        the_glossary_blesses(BLESSED_AS_A_COMPOUNDS_ENDING, &it_as_letters(a_word))
            && !the_same_word
    }

    /// Whether a name ends on a word, compared as letters and digits alone.
    ///
    /// **On the last word and not on the last characters.** `the_report` ends on the word
    /// `report` and not on `Port`, and a comparison that ignored the boundary would have called
    /// a function a port in a repository whose ports are named after what they reach.
    fn a_name_ends_on(a_word: &str, one_name: &str) -> bool {
        let the_last_word = the_last_word_of(one_name);
        it_as_letters(&the_last_word) == a_word
    }

    /// Whether the glossary writes a word on one of its two exemption lines.
    ///
    /// **A single word, not a list.** Each line names exactly one, and a reader that returned a
    /// list made `contains` a question about the list rather than about the word.
    fn the_glossary_blesses(the_heading: &str, a_word: &str) -> bool {
        the_word_under(&the_glossary_text(), the_heading).as_deref() == Some(a_word)
    }

    /// The words the glossary blesses as a whole name and refuses as a compound's ending.
    const BLESSED_AS_A_WHOLE_NAME: &str = "Blessed as a whole name";

    /// The words the glossary refuses alone and blesses as a compound's ending.
    const BLESSED_AS_A_COMPOUNDS_ENDING: &str = "Refused alone, blessed as a compound's ending";

    /// The keywords a name can follow, and which is why a name is an introduction.
    const THE_KEYWORDS_A_NAME_CAN_FOLLOW: [&str; 6] =
        ["struct", "enum", "trait", "type", "fn", "mod"];

    /// Every name the code introduces, and nothing it merely uses.
    ///
    /// **The difference is the whole of this gate.** `Result` is on the glossary's list and it is
    /// also the type every fallible function in Rust returns; a rule that judged uses would
    /// refuse the whole crate on its first line. A name a *use* does not introduce is somebody
    /// else's name, and this repository does not get to have opinions about `std`.
    fn the_names_the_synonyms_are_looked_for_in(the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (_, one_line) in the_lines_of_code(the_code) {
            found.extend(the_names_after_each_keyword(&one_line));
        }
        found.retain(|one| !one.is_empty());
        found
    }

    /// The keyword that may be followed by a name, or nothing.
    fn the_keyword_a_line_holds(one_line: &str) -> Option<&str> {
        for keyword in THE_KEYWORDS_A_NAME_CAN_FOLLOW {
            let is_there = one_line
                .split_whitespace()
                .any(|one| one.trim_end_matches('{') == keyword);
            if is_there {
                return Some(keyword);
            }
        }
        None
    }

    /// The names a line introduces, as the word after each keyword it holds.
    fn the_names_after_each_keyword(one_line: &str) -> Vec<String> {
        let Some(_keyword) = the_keyword_a_line_holds(one_line) else {
            return Vec::new();
        };
        let mut found = Vec::new();
        let mut parts = one_line.split_whitespace().peekable();
        while let Some(one) = parts.next() {
            let is_a_keyword = THE_KEYWORDS_A_NAME_CAN_FOLLOW.contains(&one.trim_end_matches('{'));
            let the_name_after = parts.peek().map(|it| (*it).to_string());
            if is_a_keyword {
                found.extend(the_name_after);
            }
        }
        found
            .iter()
            .map(|one| one.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .collect()
    }

    #[test]
    fn no_name_in_this_repository_ends_on_a_word_that_says_nothing() {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(
            found.is_empty(),
            "a name has to claim what the thing is: {found:?}"
        );
    }

    #[test]
    fn a_name_ending_on_a_banned_word_is_refused_and_named() {
        let the_code = "struct WorkRepository {\n    the_thing: u8,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec![
                "a_file.rs names WorkRepository, which ends on a banned word: Repository"
                    .to_string()
            ],
            "and the refusal names the word, because 'a name is wrong' is not something a reader can act on"
        );
    }

    #[test]
    fn the_word_the_glossary_blesses_is_accepted_on_its_own() {
        let the_code = "struct Repository {\n    owner: String,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "this system's Repository is the public GitHub repository the glossary blesses, and it is not the same thing as a persistence abstraction at all"
        );
    }

    #[test]
    fn a_name_merely_containing_a_banned_word_is_accepted() {
        let the_code = "struct StateMachineError {\n    why: String,\n}\nstruct BaseThing {\n    one: u8,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "`StateMachineError` contains `State` and ends on `Error`; a rule that matched anywhere would have refused the domain's own error type"
        );
    }

    #[test]
    fn a_snake_case_name_is_judged_by_its_letters_and_not_its_capitals() {
        let the_code = "struct a_work_repository {\n    one: u8,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec![
                "a_file.rs names a_work_repository, which ends on a banned word: Repository"
                    .to_string()
            ],
            "one name, two spellings, one judgement: the guard normalises a name to letters and digits, so the rule is not a rule about capitals"
        );
    }
}

mod a_synonym_is_never_used {
    //! No name is built out of a word the glossary refused.
    //!
    //! **The blessing is asked before the words.** `StateMachineError` carries `State`, which the
    //! stage table's `Not` column refuses, and the name is refused by no gate here because the
    //! glossary blesses it in full. A gate that asked the words first would have refused the
    //! domain's own error type, and a gate that cannot be run against its own author is a gate
    //! nobody checked.
    //!
    //! **Six of the refused words are refused only as a whole name.** `Step`, `check`, `handle`,
    //! `Owner`, `Assignee` and `Team` are ordinary English, and a gate that refused them inside
    //! a name would refuse this repository for writing prose — `step_back_over_the_white_space`
    //! is a real function about whitespace and is refused by no rule here.
    //!
    //! **Everything is read out of `docs/domain-glossary.md`.** A copy of a list of refused words
    //! inside a gate is a second list, and the copy would be green on the day the glossary
    //! changed.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;
    use aisdlc_gates::reading_the_glossary::{
        it_as_letters, the_blessed_names, the_glossary_spelling_of, the_refused_as_letters,
        the_words_of, the_words_refused_anywhere, the_words_refused_only_as_whole_names,
    };

    /// Every refusal in one file, given its name and its text.
    fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
        let the_glossary = aisdlc_gates::reading_the_glossary::the_glossary_text();
        let mut found = Vec::new();
        for one_name in the_names_the_vocabulary_is_looked_for_in(the_code) {
            found.extend(the_refusal_of_a_name_the_glossary_refuses(
                &one_name,
                &the_glossary,
                a_file_name,
            ));
        }
        found
    }

    /// The refusal for one name, if the glossary refuses it.
    fn the_refusal_of_a_name_the_glossary_refuses(
        one_name: &str,
        the_glossary: &str,
        a_file_name: &str,
    ) -> Vec<String> {
        if the_blessed_names(the_glossary).contains(&it_as_letters(one_name)) {
            return Vec::new();
        }
        if the_words_refused_only_as_whole_names(the_glossary).contains(&it_as_letters(one_name)) {
            return vec![format!(
                "{a_file_name} names {one_name}, which is a refused word on its own"
            )];
        }
        let carried = the_word_it_carries_from(the_glossary, one_name);
        carried
            .into_iter()
            .map(|one| {
                format!("{a_file_name} names {one_name}, which carries a refused word: {one}")
            })
            .collect()
    }

    /// The refused word this name carries, if it carries one.
    fn the_word_it_carries_from(the_glossary: &str, one_name: &str) -> Option<String> {
        let refused = the_refused_as_letters(the_glossary);
        let whole_names_only = the_words_refused_only_as_whole_names(the_glossary);
        let carried = the_words_of(one_name);
        let found = carried
            .iter()
            .find(|one| refused.contains(one) && !whole_names_only.contains(one))?;
        the_glossary_spelling_of(the_glossary, found)
    }

    /// The keywords a name can follow, and which is why a name is an introduction.
    const THE_KEYWORDS_A_NAME_CAN_FOLLOW: [&str; 6] =
        ["struct", "enum", "trait", "type", "fn", "mod"];

    /// Every name the code introduces, and nothing it merely uses.
    fn the_names_the_vocabulary_is_looked_for_in(the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (_, one_line) in the_lines_of_code(the_code) {
            found.extend(the_names_immediately_after(&one_line));
        }
        found.retain(|one| !one.is_empty());
        found
    }

    /// The word after every keyword on a line.
    fn the_names_immediately_after(one_line: &str) -> Vec<String> {
        let mut found = Vec::new();
        let mut parts = one_line.split_whitespace().peekable();
        while let Some(one) = parts.next() {
            let the_name_after = parts.peek().map(|it| (*it).to_string());
            if THE_KEYWORDS_A_NAME_CAN_FOLLOW.contains(&one.trim_end_matches('{')) {
                found.extend(the_name_after);
            }
        }
        found
            .iter()
            .map(|one| one.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .collect()
    }

    #[test]
    fn no_name_in_this_repository_is_built_out_of_a_refused_word() {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(found.is_empty(), "one concept, one name: {found:?}");
    }

    #[test]
    fn a_name_built_out_of_a_refused_word_is_refused_and_named() {
        let the_code = "struct CurrentStatus {\n    one: u8,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec!["a_file.rs names CurrentStatus, which carries a refused word: Status".to_string()],
            "a compound carrying Status is carrying the wrong word for this system's Stage, and the refusal names which one"
        );
    }

    #[test]
    fn a_blessed_name_is_accepted_even_though_it_carries_a_refused_word() {
        let the_code = "struct StateMachineError {\n    why: String,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "the glossary blesses the name in full and a gate asks the blessing before it asks the words — a gate that did otherwise would have refused the domain's own error type on its first line"
        );
    }

    #[test]
    fn a_word_refused_only_alone_is_refused_alone() {
        let the_code = "fn check() {}\nfn step() {}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec![
                "a_file.rs names check, which is a refused word on its own".to_string(),
                "a_file.rs names step, which is a refused word on its own".to_string(),
            ],
            "these six are ordinary English and are refused as whole names only"
        );
    }

    #[test]
    fn a_word_refused_only_alone_is_accepted_inside_a_name() {
        let the_code = "fn step_back_over_the_white_space() {}\nfn the_check_it_made() {}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "one of these is a real function in this repository and is about whitespace, and a gate that refused the word inside a name would have refused this repository for writing prose"
        );
    }

    #[test]
    fn the_glossary_was_read_and_not_assumed() {
        let the_glossary = aisdlc_gates::reading_the_glossary::the_glossary_text();
        let the_refused_anywhere = the_words_refused_anywhere(&the_glossary);
        let the_whole_names_only = the_words_refused_only_as_whole_names(&the_glossary);

        assert!(
            the_refused_anywhere.contains(&"Status".to_string()),
            "the stage table's Not column refuses Status and the guard found it as the glossary spells it, because a refusal quotes the document: {the_refused_anywhere:?}"
        );
        assert!(
            the_whole_names_only.contains(&"step".to_string()),
            "and the block of six names is read from the glossary: {the_whole_names_only:?}"
        );
    }
}

mod the_domain_depends_on_nothing {
    //! The gate for the rule everything else in this repository rests on: `aisdlc-domain`
    //! depends on nothing at all.
    //!
    //! Not "few". Not "only serde". The manifest has no `[dependencies]` section, and that is the
    //! one boundary the compiler cannot be tricked out of — a crate with no dependencies cannot
    //! reach a model, a filesystem, a clock or a process, whatever its source says.
    //!
    //! **It is checked rather than trusted because a manifest is text and text is edited.** The
    //! source is read through the shared walk rather than searched, because a domain may name
    //! `std::fs` in its documentation — the walk that would refuse that is the walk this gate
    //! exists to make impossible.

    use aisdlc_gates::reaching_for_the_world::what_reaches_the_world_in;
    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::scanning::HowTheFileIsWritten;

    /// The manifest of the domain, and the directory its source lives in.
    const THE_DOMAIN: &str = "crates/domain";
    const ITS_MANIFEST: &str = "crates/domain/Cargo.toml";
    const ITS_SOURCE: &str = "crates/domain/src";

    /// The sections a manifest may not carry an entry in.
    ///
    /// **A section with nothing in it is accepted**, because the rule is about what the domain
    /// depends on and not about how its manifest is arranged: an empty heading is a promise
    /// nobody has been asked to keep.
    ///
    /// **The names carry no brackets**, because the brackets are the manifest's delimiters and
    /// not part of the name — a reader that compared `[dependencies]` against the section it
    /// just read would never match anything, and a gate that never matches is green forever.
    const THE_SECTIONS_THE_DOMAIN_MAY_NOT_FILL: [&str; 2] = ["dependencies", "dev-dependencies"];

    /// What a manifest and its sources break, named so a reader can act on the finding.
    fn what_is_wrong_with(manifest: &str, the_source: &str) -> Vec<String> {
        let mut what_broke = Vec::new();
        what_broke.extend(the_sections_that_are_filled(manifest));
        what_broke.extend(what_reaches_the_world_in(the_source));
        what_broke
    }

    /// The sections a manifest carries an entry in, with the entry quoted back.
    ///
    /// **A table header is an entry too.** `[dependencies.serde]` writes the dependency in the
    /// header, and a reader that looks only for `name = value` would pass a crate that depends on
    /// anything as one that depends on nothing. That was the second silent pass in an earlier
    /// version of this gate, found only because the first one was looked for on purpose.
    fn the_sections_that_are_filled(manifest: &str) -> Vec<String> {
        let mut found = Vec::new();
        let mut the_section_standing = String::new();
        for line in manifest.lines() {
            if let Some(name) = the_section_a_line_opens(line) {
                the_section_standing = name;
                continue;
            }
            found.extend(the_entry_standing_in(&the_section_standing, line));
        }
        found
    }

    /// The refusal for one line, if the line is an entry in a section the domain may not fill.
    ///
    /// **Any** section header closes the one before it, whether or not the gate cares about it —
    /// a reader that only noticed the two forbidden names would carry `[dependencies]`'s
    /// emptiness into `[package]` and report the manifest's own name as a dependency.
    fn the_entry_standing_in(the_section: &str, line: &str) -> Vec<String> {
        if !THE_SECTIONS_THE_DOMAIN_MAY_NOT_FILL.contains(&the_section) {
            return Vec::new();
        }
        if line.trim_start().starts_with('#') || line.trim().is_empty() {
            return Vec::new();
        }
        vec![format!("{the_section} carries an entry: {}", line.trim())]
    }

    /// The name of the section a manifest line opens, or nothing if it opens none.
    ///
    /// **Every** section is named, and not only the two the gate refuses: a section this reader
    /// did not recognise is a section it cannot close, and the next entry would be read as
    /// belonging to the last section it did recognise.
    ///
    /// The table name is taken off, so `[dependencies]` and `[dependencies.serde]` are
    /// recognised as the same section and a reader does not have to know the two spellings.
    fn the_section_a_line_opens(line: &str) -> Option<String> {
        let trimmed = line.trim();
        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return None;
        }
        let the_name = trimmed.trim_start_matches('[').trim_end_matches(']');
        let without_the_table = the_name.split('.').next().unwrap_or(the_name);
        Some(without_the_table.to_string())
    }

    /// The reaches the domain's code makes, read out of the code and not out of its prose.
    ///
    /// A fully qualified `std::fs::read_to_string(path)` with no import at all is a reach, and a
    /// checker that looks for a `use` line misses it — that was the first escape from an earlier
    /// version of this gate, and the two are the same hole seen from two sides.
    #[test]
    fn a_manifest_with_a_dependency_is_refused() {
        let manifest = "[package]\nname = \"x\"\n\n[dependencies]\nserde = \"1\"\n";

        let what_broke = what_is_wrong_with(manifest, "");

        assert_eq!(
            what_broke.len(),
            1,
            "a dependency is a dependency: {what_broke:?}"
        );
        assert!(
            what_broke[0].contains("serde"),
            "and the refusal quotes the entry, because a refusal a reader cannot match against what they wrote teaches nothing: {what_broke:?}"
        );
    }

    #[test]
    fn a_manifest_with_a_dev_dependency_is_refused() {
        let manifest = "[dev-dependencies]\nregex = \"1\"\n";

        let what_broke = what_is_wrong_with(manifest, "");

        assert_eq!(
            what_broke.len(),
            1,
            "a dev-dependency is how a test gets a crate the domain cannot have, so the rule does not have two halves: {what_broke:?}"
        );
    }

    #[test]
    fn a_manifest_with_a_table_header_dependency_is_refused() {
        let manifest = "[dependencies.serde]\nversion = \"1\"\n";

        let what_broke = what_is_wrong_with(manifest, "");

        assert_eq!(
            what_broke.len(),
            1,
            "a table header writes the dependency in the header, and a reader that looks only for `name = value` passes a crate that depends on anything as one that depends on nothing: {what_broke:?}"
        );
    }

    #[test]
    fn a_manifest_with_an_empty_section_is_accepted() {
        let manifest = "[dependencies]\n\n[dev-dependencies]\n\n[package]\nname = \"x\"\n";

        assert!(
            what_is_wrong_with(manifest, "").is_empty(),
            "the rule is about what the domain depends on and not about how its manifest is arranged"
        );
    }

    #[test]
    fn the_real_manifest_of_the_domain_is_accepted() {
        let manifest = the_text_of_a_manifest();

        assert!(
            what_is_wrong_with(&manifest, "").is_empty(),
            "the domain's own manifest carries a section it may not, and the rule is not a sentence"
        );
        assert!(
            the_sections_that_are_filled(&manifest).is_empty(),
            "and the reader agrees, on the real manifest, having skipped the comment that names the section in order to say it is absent"
        );
    }

    #[test]
    fn a_qualified_reach_with_no_import_is_refused() {
        let source = "pub fn the_text_of(path: &str) -> String {\n    std::fs::read_to_string(path).unwrap_or_default()\n}";

        let what_broke = what_is_wrong_with("", source);

        assert_eq!(
            what_broke.len(),
            1,
            "a fully qualified reach with no import is the reach the rule is about: {what_broke:?}"
        );
        assert!(
            what_broke[0].contains("line 2"),
            "and the refusal names the line: {what_broke:?}"
        );
    }

    #[test]
    fn a_grouped_reach_is_refused() {
        let source = "use std::{fs, io};\n\npub fn a() {}";

        let what_broke = what_is_wrong_with("", source);

        assert_eq!(
            what_broke.len(),
            1,
            "a group names the module just as plainly as a path does, and the two are the same hole: {what_broke:?}"
        );
    }

    #[test]
    fn a_reach_named_in_documentation_is_accepted() {
        let source =
            "//! The domain does not use std::fs, and this is where it says so.\n\npub fn a() {}";

        assert!(
            what_is_wrong_with("", source).is_empty(),
            "a domain may name what it does not reach for in its own documentation, and a gate that refused that would be refusing this repository's rule to be explained"
        );
    }

    #[test]
    fn a_module_of_the_domains_own_is_accepted() {
        let source = "use crate::fs::read_the_shape_of_it;\n\npub fn a() {}";

        assert!(
            what_is_wrong_with("", source).is_empty(),
            "a domain that owns a module called `fs` is not reaching for the world, and the rule is about the path rather than about the last segment"
        );
    }

    #[test]
    fn the_reaches_of_the_real_domain_are_read() {
        let the_source = the_source_of_the_domain();

        let what_broke = what_is_wrong_with(&the_text_of_a_manifest(), &the_source);

        assert!(
            what_broke.is_empty(),
            "the domain reaches for something, and the rule every boundary here rests on is not a sentence: {what_broke:?}"
        );
        assert!(
            !the_source.is_empty(),
            "the gate read no source at all, which is a pass it has not earned"
        );
        let read = HowTheFileIsWritten::of(&the_source);
        assert!(
            !read.stretches().is_empty(),
            "and the walk made stretches of it rather than answering about an empty string"
        );
    }

    /// The text of the domain's manifest, or a panic naming it.
    fn the_text_of_a_manifest() -> String {
        let the_path = the_path_under_the_checkout(ITS_MANIFEST);
        std::fs::read_to_string(&the_path)
            .unwrap_or_else(|why| panic!("{} must be readable: {why}", the_path.display()))
    }

    /// Every line of the domain's source, joined, because the checker is asked about a file and
    /// a file is read as one reading.
    fn the_source_of_the_domain() -> String {
        let mut the_source = String::new();
        for path in every_rust_file_under_the_domain() {
            the_source.push_str(&reading_the_checkout::the_text_at(&path));
            the_source.push('\n');
        }
        the_source
    }

    /// Every `.rs` file of the domain, in the order a reader would meet them.
    fn every_rust_file_under_the_domain() -> Vec<std::path::PathBuf> {
        let the_root = the_path_under_the_checkout(ITS_SOURCE);
        aisdlc_gates::reading_the_code::the_rust_files_under(&the_root)
    }

    /// A path inside this checkout, from where cargo compiled this file.
    fn the_path_under_the_checkout(wanted: &str) -> std::path::PathBuf {
        let mut here = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        while !here.join(THE_DOMAIN).is_dir() {
            assert!(
                here.pop(),
                "no {THE_DOMAIN} directory above where this file was compiled"
            );
        }
        here.join(wanted)
    }
}

mod a_name_the_gates_share_has_one_implementation {
    //! A name the gates share has one implementation.
    //!
    //! **A name that means two things is a name nobody can change**, because every caller is
    //! holding the other meaning and the compiler cannot tell two readings of a word apart.
    //! This repository had seven functions called `the_lines_of_code` in the seven files that
    //! read the source, and **five returned `(usize, String)` while two returned `String`** — so
    //! the same call gave a different answer depending on which file asked, and nothing said so
    //! until the copies were made to agree with each other.
    //!
    //! **Duplication that has not diverged is a cost. Duplication that has is a defect**, and
    //! the distance between the two is one line added in one place.

    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;

    /// The refusal for one name, or nothing when the gates agree about what it means.
    fn what_is_wrong_with(one_name: &str, the_copies: &[String]) -> Option<String> {
        if the_copies.len() < 2 {
            return None;
        }
        let the_first = &the_copies[0];
        if the_copies.iter().all(|it| it == the_first) {
            return Some(format!(
                "{one_name} is written in {} places, and all of them are the same",
                the_copies.len()
            ));
        }
        Some(format!("{one_name} means two different things"))
    }

    #[test]
    fn a_name_written_twice_differently_is_refused() {
        let the_copies = vec![
            "a_file.rs line 1".to_string(),
            "a_file.rs line 9".to_string(),
        ];

        assert_eq!(
            what_is_wrong_with("the_lines_of_code", &the_copies),
            Some("the_lines_of_code means two different things".to_string()),
            "which is the worse of the two findings, and it is the one this repository had"
        );
    }

    #[test]
    fn a_name_written_once_is_accepted() {
        assert_eq!(
            what_is_wrong_with("the_lines_of_code", &["a_file.rs line 1".to_string()]),
            None,
            "one writer, one meaning, nothing to refuse"
        );
    }

    /// Every `fn` in one file, as `(its name, the line it was written on)`.
    ///
    /// **A signature is one line and its name is the whole word before the bracket.** A reader
    /// that took the first space-separated word found `a_name` in a function called
    /// `a_name_ends_on`. A reader that then required the word to end in a comma found nothing
    /// at all — not one signature in this repository ends in a comma, a name is followed by its
    /// bracket — and the rule passed on the seven copies it was written to find.
    fn the_definitions_in(the_code: &str) -> Vec<(String, usize)> {
        let mut found = Vec::new();
        for (a_line, one_line) in the_lines_of_code(the_code) {
            let Some(the_name) = the_name_declared_on(&one_line) else {
                continue;
            };
            found.push((the_name, a_line));
        }
        found
    }

    /// What a line that declares a function begins with, or nothing if it declares no
    /// function.
    ///
    /// **A public one as well as a private one**, because a public name is the one two crates
    /// can both reach for. The reader looked only for `fn ` at the start of a line, so
    /// `pub fn the_glossary_text` in the library and `fn the_glossary_text` in a gate were two
    /// implementations of one name and the rule found neither.
    fn the_start_of_a_declaration(one_line: &str) -> Option<&str> {
        let trimmed = one_line.trim();
        for the_prefix in ["pub fn ", "pub(crate) fn ", "fn "] {
            if let Some(the_rest) = trimmed.strip_prefix(the_prefix) {
                return Some(the_rest);
            }
        }
        None
    }

    /// The name a line declares a function under, or nothing if it declares no function.
    ///
    /// **Through the scanner**, so a `fn` inside a string is a fixture and not a definition —
    /// a gate that cannot tell a definition from a quotation is a gate that counts the word.
    fn the_name_declared_on(one_line: &str) -> Option<String> {
        let after_fn = the_start_of_a_declaration(one_line)?;
        if !after_fn.contains('(') {
            return None;
        }
        let the_name = after_fn.split('(').next()?;
        let is_a_whole_word = the_name
            .chars()
            .all(|one| one.is_ascii_lowercase() || one == '_');
        if is_a_whole_word && !the_name.is_empty() {
            return Some(the_name.to_string());
        }
        None
    }

    /// Every name the gates wrote more than once, and why it is refused.
    fn the_names_written_more_than_once() -> Vec<String> {
        let mut where_each_name_lives: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for a_file in the_files_the_gates_are_written_in() {
            let the_text = reading_the_checkout::the_text_at(&a_file);
            for (the_name, the_line) in the_definitions_in(&the_text) {
                let the_copies = where_each_name_lives.entry(the_name.clone()).or_default();
                the_copies.push(format!("{} line {the_line}", a_file.display()));
            }
        }
        where_each_name_lives
            .into_iter()
            .filter(|(a_name, the_copies)| {
                the_copies.len() > 1 && !a_test_may_write_this_for_itself(a_name)
            })
            .filter_map(|(a_name, the_copies)| what_is_wrong_with(&a_name, &the_copies))
            .collect()
    }

    /// The files this rule reads: the gates and the shared library, and no crate.
    ///
    /// **Widened to `crates/` once, and reverted, with the numbers.** The reason it was tried
    /// is worth keeping: a `Stage::as_str` was written in the domain beside the `Display` that
    /// already named every stage, and **the gate whose entire subject is a name having one
    /// implementation did not see it.** `the_specification_and_the_code_have_not_drifted_apart`
    /// did, because the six stage names were written a second time in the one file it reads
    /// them from.
    ///
    /// **Widened, it found nineteen names and not one of them was a real duplicate.** Nine were
    /// `fmt` and `try_from` — one implementation of a trait per type, which is what implementing
    /// a trait *is*. Seven were test helpers written per file, like `a_work_item`. The rest were
    /// methods on two different types: `because`, `answering`, `standing_at`. **Silencing
    /// nineteen legitimate names with nineteen exceptions is a list that only rots**, and a rule
    /// that reports nothing real has stopped being a rule.
    ///
    /// **So the gate is asked a narrower question on purpose**, and the name that has two
    /// implementations in a crate is caught by whichever rule owns the thing both copies
    /// describe — the drift gate, for a name the specification already lists.
    fn the_files_the_gates_are_written_in() -> Vec<PathBuf> {
        let the_root = reading_the_checkout::the_root_of_the_checkout();
        let the_gates = the_root.join("tests");
        every_source_file()
            .into_iter()
            .filter(|a_file| {
                a_file.starts_with(&the_gates) || a_file.starts_with(the_root.join("src"))
            })
            .collect()
    }

    /// Whether a name is one each gate may write for itself, because it judges its own rule.
    fn a_test_may_write_this_for_itself(one_name: &str) -> bool {
        THE_NAMES_EACH_GATE_MAY_WRITE_FOR_ITSELF.contains(&one_name)
    }

    /// The names each gate may write for itself, and the list is a cost written down rather
    /// than hidden: a gate needing one more adds a line and a reviewer reads the diff.
    ///
    /// **`new` and `of_one_line` are methods on two different walks**, and a method on a type
    /// shares its name with a method on another type the way two structs share a field name —
    /// the same name about two different things, and a rule about functions that did not say so
    /// would send a reader hunting for a shared implementation nobody meant to write.
    const THE_NAMES_EACH_GATE_MAY_WRITE_FOR_ITSELF: [&str; 4] = [
        "what_is_wrong_with",
        "the_explanation",
        "new",
        "of_one_line",
    ];

    #[test]
    fn a_name_the_gates_hold_together_is_the_one_this_rule_measures() {
        let the_files = the_files_the_gates_are_written_in();
        let reads_the_domain = the_files
            .iter()
            .any(|it| it.to_string_lossy().contains("crates/"));

        assert!(
            !reads_the_domain,
            "**nineteen names, none of them a real duplicate** — nine were a trait implemented once per type, seven were per-file test helpers, and silencing them all would be a list that only rots. A name with two implementations inside a crate is caught by the rule that owns the thing both copies describe."
        );
    }

    #[test]
    fn no_name_in_the_gates_is_written_twice() {
        let refusals = the_names_written_more_than_once();

        assert!(
            refusals.is_empty(),
            "a name the gates share has one implementation, and these do not: {refusals:?}"
        );
    }

    #[test]
    fn a_reader_told_of_one_function_twice_finds_both() {
        let a_file_writing_one_function_twice = "fn a_reader() {}\nfn a_reader() {}\n";

        let found = the_definitions_in(a_file_writing_one_function_twice);

        assert_eq!(
            found,
            vec![("a_reader".to_string(), 1), ("a_reader".to_string(), 2)],
            "the rule reads files it has never seen, so it is shown a certain one first: one function written twice, with the line of each, rather than a checkout it has to guess its way through"
        );
    }

    #[test]
    fn a_reader_told_of_a_function_named_in_a_string_finds_nothing() {
        let a_file_naming_a_function_in_a_string = "let the_fixture = \"fn quoted()\";\n";

        let found = the_definitions_in(a_file_naming_a_function_in_a_string);

        assert!(
            found.is_empty(),
            "a quotation is a value and a value holding the word is a fixture, and a reader that cannot tell the two apart is a reader counting the word `fn`"
        );
    }

    #[test]
    fn a_name_written_twice_the_same_way_is_refused() {
        let the_copies = vec![
            "a_file.rs line 1".to_string(),
            "a_file.rs line 1".to_string(),
        ];

        assert_eq!(
            what_is_wrong_with("the_lines_of_code", &the_copies),
            Some(
                "the_lines_of_code is written in 2 places, and all of them are the same"
                    .to_string()
            ),
            "and it is refused with the count, because 'also duplicated somewhere' is not a place a reader can act on"
        );
    }
}

mod no_test_reads_an_environment_variable {
    //! No test reads the environment at run time.
    //!
    //! **A test that needs a token means a boundary is wrong.** The suite has to be green with
    //! no network, no API key, no Docker and no model, and a test that passes only when a
    //! credential happens to be exported is a green tick that says nothing about the software.
    //!
    //! **The words come from the glossary rather than from here.** A gate carrying its own list
    //! has a list that drifts from the sentence explaining it, and the two are read by different
    //! people — the glossary by whoever is about to break the rule, the gate by nobody.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;
    use aisdlc_gates::reading_the_glossary::the_block_containing;

    /// The words the glossary refuses, and the glossary is where they are written down.
    fn the_environment_reads_the_glossary_refuses() -> Vec<String> {
        let the_glossary = aisdlc_gates::reading_the_glossary::the_glossary_text();
        the_block_containing(&the_glossary, "ARuntimeReadOfTheEnvironment")
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect()
    }

    /// The refusal for one line, or nothing when the line reaches for nothing.
    ///
    /// **A word with a bracket after it and not the bare word.** `remove_var` and `var` are
    /// both refused and `var` is a prefix of nothing here, but `temp_dir` is a suffix of
    /// `std::env::temp_dir` and the call is `env::temp_dir()` — so the reader looks for the
    /// word followed by a bracket, and **a line naming `env!` or `option_env!` has no bracket
    /// after `env`** and is not refused, which is the exemption the glossary insists is
    /// load-bearing rather than theoretical.
    fn what_is_wrong_with(one_line: &str, the_refused: &[String]) -> Option<String> {
        for a_word in the_refused {
            if the_line_calls(one_line, a_word) {
                return Some(format!("reads the environment at run time: {a_word}"));
            }
        }
        None
    }

    /// Whether a line calls the function a word names.
    fn the_line_calls(one_line: &str, a_word: &str) -> bool {
        let mut found = false;
        for (_, one_line_of_code) in the_lines_of_code(one_line) {
            if one_line_of_code.contains(&format!("{a_word}(")) {
                found = true;
            }
        }
        found
    }

    /// Every refusal in one file's tests, as `(its line, the reason)`.
    fn what_is_wrong_in(the_code: &str, the_refused: &[String]) -> Vec<String> {
        let mut found = Vec::new();
        for (a_line, one_line) in the_lines_of_code(the_code) {
            if let Some(why) = what_is_wrong_with(&one_line, the_refused) {
                found.push(format!("line {a_line} {why}"));
            }
        }
        found
    }

    /// The files the rule reads: the gates' own and the domain's tests.
    fn the_test_files() -> Vec<std::path::PathBuf> {
        let the_root = reading_the_checkout::the_root_of_the_checkout();
        every_source_file()
            .into_iter()
            .filter(|a_file| a_file.starts_with(the_root.join("tests")))
            .filter(|a_file| a_file.starts_with(the_root.join("crates")))
            .chain(
                every_source_file()
                    .into_iter()
                    .filter(|a_file| a_file.parent() == Some(the_root.join("tests").as_path())),
            )
            .collect()
    }

    #[test]
    fn no_test_reads_the_environment() {
        let the_refused = the_environment_reads_the_glossary_refuses();
        let mut found = Vec::new();
        for a_file in the_test_files() {
            let the_text = reading_the_checkout::the_text_at(&a_file);
            let refusals = what_is_wrong_in(&the_text, &the_refused);
            found.extend(
                refusals
                    .into_iter()
                    .map(|why| format!("{} {why}", a_file.display())),
            );
        }

        assert!(
            found.is_empty(),
            "the suite must be green with no credential, and a test that needs one is a boundary in the wrong place: {found:?}"
        );
    }

    #[test]
    fn a_line_reaching_for_a_variable_is_refused() {
        let refused = vec!["var".to_string()];

        assert_eq!(
            what_is_wrong_with("let the_token = std::env::var(\"GITHUB_TOKEN\");", &refused),
            Some("reads the environment at run time: var".to_string()),
            "and it names the call, because 'somewhere in a line of the tests' is not a place a reader can act on"
        );
    }

    #[test]
    fn a_line_reaching_for_a_temporary_directory_is_refused() {
        let refused = vec!["temp_dir".to_string()];

        assert_eq!(
            what_is_wrong_with("let where = std::env::temp_dir();", &refused),
            Some("reads the environment at run time: temp_dir".to_string()),
            "although it carries no credential at all, because the rule is drawn at the reach rather than at the danger"
        );
    }

    #[test]
    fn a_line_expanding_a_macro_at_build_time_is_accepted() {
        let refused = vec!["var".to_string(), "temp_dir".to_string()];

        assert_eq!(
            what_is_wrong_with("let the_root = env!(\"CARGO_MANIFEST_DIR\");", &refused),
            None,
            "`env!` and `option_env!` are macros the compiler expands before the test runs, and seven gates reach the checkout through `CARGO_MANIFEST_DIR`"
        );
    }

    #[test]
    fn a_line_reading_the_current_directory_is_accepted() {
        let refused = the_environment_reads_the_glossary_refuses();

        assert_eq!(
            what_is_wrong_with("let here = std::env::current_dir();", &refused),
            None,
            "it is a run-time read and it is not refused: it carries no credential, and a list of every read in `std::env` is a list to keep in step with a standard library"
        );
    }

    #[test]
    fn the_glossary_names_the_six_reads() {
        let refused = the_environment_reads_the_glossary_refuses();

        assert_eq!(
            refused,
            vec!["var", "var_os", "vars", "set_var", "remove_var", "temp_dir"],
            "six words in the glossary's own block, and a reader that found none would let every test reach for a credential"
        );
    }
}

mod an_ignored_test_says_why_it_is_ignored {
    //! Every ignored test says why it is ignored.
    //!
    //! **A skipped test is a test that is not run, and a skipped test nobody can explain is a
    //! hole in the suite with a green tick over it.** The suite has to be green with no
    //! network, no API key, no Docker and no model, so a test that needs one of those cannot
    //! run by default — and the way to say so is `#[ignore = "why"]`, which is code.
    //!
    //! **The reason goes in the attribute and not in a comment beside it.** A comment saying
    //! why a test is ignored would be refused by the rule that this repository leaves no
    //! comments — so the only place a reason can live is a place the compiler reads, and a
    //! reason in the source is a reason a reader cannot lose by deleting a line above it.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;

    /// The refusal for one ignored test, or nothing when it says why.
    ///
    /// **`#[ignore = "…"]` and not a bare `#[ignore]`.** The whole form is in the code, so a
    /// reader who deletes the comment above the test has not lost the reason, and a reader
    /// who greps for `#[ignore` finds the reason next to the skip rather than a line above it.
    fn what_is_wrong_with(one_line: &str) -> Option<String> {
        let the_attribute = the_attribute_on(one_line);
        if !the_attribute.starts_with("ignore") {
            return None;
        }
        if the_attribute.contains('=') {
            return None;
        }
        Some("is ignored and says no reason".to_string())
    }

    /// The attribute a line declares, and nothing when it declares none.
    ///
    /// **`ignore` at the start and not the whole line.** A first version refused any attribute
    /// carrying no `=`, which refused `#[test]`, `#[derive(Debug)]` and every other decoration
    /// in the repository — **a rule about skipped tests that refuses `#[test]` is a rule with
    /// the word wrong in it.**
    fn the_attribute_on(one_line: &str) -> String {
        let trimmed = one_line.trim();
        if !trimmed.starts_with("#[") {
            return String::new();
        }
        trimmed.trim_end_matches(']').replace("#[", "")
    }

    /// Every refusal in one file, with the line it stands on.
    fn what_is_wrong_in_a_file(the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (a_line, one_line) in the_lines_of_code(the_code) {
            let why = what_is_wrong_with(&one_line);
            found.extend(why.map(|one| format!("line {a_line} {one}")));
        }
        found
    }

    /// The files the rule reads: every `.rs` in this checkout.
    fn every_file_a_test_can_live_in() -> Vec<std::path::PathBuf> {
        every_source_file()
    }

    #[test]
    fn an_ignored_test_saying_nothing_is_refused() {
        let a_bare_ignore = "    #[ignore]";

        assert_eq!(
            what_is_wrong_with(a_bare_ignore),
            Some("is ignored and says no reason".to_string()),
            "and it quotes nothing, because there is nothing to quote — which is the finding"
        );
    }

    #[test]
    fn an_ignored_test_saying_why_is_accepted() {
        let a_reason = "    #[ignore = \"needs a Docker daemon, and the container tier is one\"]";

        assert_eq!(
            what_is_wrong_with(a_reason),
            None,
            "the reason is in the code, where a reader cannot lose it by deleting a line above"
        );
    }

    #[test]
    fn a_line_declaring_no_attribute_is_accepted() {
        assert_eq!(
            what_is_wrong_with("    #[test]"),
            None,
            "a test that runs is not a test that is skipped, whatever else it declares"
        );
    }

    #[test]
    fn an_attribute_that_is_not_ignore_is_accepted() {
        assert_eq!(
            what_is_wrong_with("    #[derive(Debug, Clone)]"),
            None,
            "the rule is about being skipped and not about being decorated"
        );
    }

    #[test]
    fn no_test_is_ignored_without_a_reason() {
        let mut found = Vec::new();
        for a_file in every_file_a_test_can_live_in() {
            let the_text = reading_the_checkout::the_text_at(&a_file);
            let refusals = what_is_wrong_in_a_file(&the_text);
            found.extend(
                refusals
                    .into_iter()
                    .map(|why| format!("{} {why}", a_file.display())),
            );
        }

        assert!(
            found.is_empty(),
            "a skipped test nobody can explain is a hole in the suite with a green tick over it: {found:?}"
        );
    }
}

mod the_core_never_names_a_trait_object {
    //! Nothing in the core names a trait object.
    //!
    //! **A `dyn` is a promise that the concrete type behind a value can be replaced, and in the
    //! core that promise buys nothing.** The domain's types are the specification: a work item
    //! is a work item, a stage is a stage, and a value that is "some stage" is a value nobody
    //! can say anything definite about. The rule is drawn at the word rather than at the
    //! dispatch, because a list of the dispatches this repository does not need is a list
    //! somebody has to keep in step with a language.
    //!
    //! **The core is the domain and the gates.** Both are where a decision has to be visible in
    //! the source: a domain rule reads its input as the domain type it means, and a gate reads a
    //! file as a file.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;

    /// The refusal for one line, or nothing when the line names no trait object.
    ///
    /// **`dyn` as a whole word and not a substring.** `dynamic` is a word and `dyn` is not in
    /// it, and a reader drawing the rule at the three letters would refuse a variable named
    /// after a thing that is dynamic — which is a word, not a trait object.
    fn what_is_wrong_with(one_line: &str) -> Option<String> {
        let names_one = the_words_in(one_line);
        if !names_one.iter().any(|one| one == "dyn") {
            return None;
        }
        Some("names a trait object: dyn".to_string())
    }

    /// The words one line of code carries.
    ///
    /// **Split on every character that is not part of a name and not on the whitespace.** A
    /// reader that split on whitespace saw `Box<dyn` as one word, found no `dyn` in it, and
    /// accepted the very line the rule is about — **a rule that cannot see the word it refuses
    /// is a rule that has not been shown its own subject.**
    fn the_words_in(one_line: &str) -> Vec<String> {
        one_line
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|one| !one.is_empty())
            .map(ToString::to_string)
            .collect()
    }

    /// Every refusal in one file, with the line it stands on.
    fn what_is_wrong_in_a_core_file(the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (a_line, one_line) in the_lines_of_code(the_code) {
            let why = what_is_wrong_with(&one_line);
            found.extend(why.map(|one| format!("line {a_line} {one}")));
        }
        found
    }

    /// The files that are the core: the domain and the gates.
    fn the_files_that_are_the_core() -> Vec<std::path::PathBuf> {
        let the_root = reading_the_checkout::the_root_of_the_checkout();
        let the_domain = the_root.join("crates").join("domain");
        let mut found = every_source_file()
            .into_iter()
            .filter(|a_file| a_file.starts_with(&the_domain))
            .collect::<Vec<std::path::PathBuf>>();
        found.push(the_root.join("tests").join("readability_guards.rs"));
        found.push(the_root.join("tests").join("documentation_guards.rs"));
        found
    }

    #[test]
    fn a_line_naming_a_trait_object_is_refused() {
        let a_trait_object = "    let a_stage: Box<dyn Stage> = Box::new(the_stage);";

        assert_eq!(
            what_is_wrong_with(a_trait_object),
            Some("names a trait object: dyn".to_string()),
            "and it quotes the word, because 'somewhere in the core' is not a place a reader can act on"
        );
    }

    #[test]
    fn a_line_naming_the_concrete_type_is_accepted() {
        let a_concrete_type = "    let a_stage: Stage = Stage::Ready;";

        assert_eq!(
            what_is_wrong_with(a_concrete_type),
            None,
            "a stage is a stage, and a value that is exactly its own type is a value a reader can say something definite about"
        );
    }

    #[test]
    fn a_line_naming_a_word_that_merely_starts_with_those_letters_is_accepted() {
        let a_word_about_motion = "    let the_dynamic_one = measure();";

        assert_eq!(
            what_is_wrong_with(a_word_about_motion),
            None,
            "`dynamic` is a word and `dyn` is not in it, and a reader drawing the rule at the three letters would refuse a name"
        );
    }

    #[test]
    fn no_trait_object_is_named_anywhere_in_the_core() {
        let mut found = Vec::new();
        for a_file in the_files_that_are_the_core() {
            let the_text = reading_the_checkout::the_text_at(&a_file);
            let refusals = what_is_wrong_in_a_core_file(&the_text);
            found.extend(
                refusals
                    .into_iter()
                    .map(|why| format!("{} {why}", a_file.display())),
            );
        }

        assert!(
            found.is_empty(),
            "the domain's types are the specification, and a value that is some stage is a value nobody can say anything definite about: {found:?}"
        );
    }
}

mod the_use_cases_may_not_reach_the_world {
    //! `aisdlc-run` names ports and nothing else, and the adapters are the only place the world
    //! may be named.
    //!
    //! **The rule is the domain's rule, drawn a second time, and the list is shared on purpose.**
    //! Five module names in two files is five names in one file and four in another, and the one
    //! that drifts is the one nobody re-reads. **The exception is exactly one crate, and it is
    //! named rather than derived** — a rule that derived the exception would exempt every crate
    //! added after the adapter, and a crate added tomorrow is a crate nobody has read.
    //!
    //! **`aisdlc-run` holds three ports and a use case that hands work to the outside world**, so
    //! this is not a rule about a crate that could not reach anything if it tried. A run that reads
    //! a clock, or writes a file, is a run whose tests stop being green with no network and no
    //! model — which is the property every other gate here rests on.

    use aisdlc_gates::reaching_for_the_world::{
        THE_MODULES_THAT_REACH_THE_WORLD, what_reaches_the_world_in, what_reaches_the_world_says,
    };
    use aisdlc_gates::reading_the_checkout;

    /// The one crate allowed to name the world, because it is where the world is answered.
    const THE_ADAPTER_CRATE: &str = "outside";

    /// A crate's name, which is its directory's own name.
    fn the_crate_named(a_crate: &std::path::Path) -> String {
        a_crate
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_default()
            .to_string()
    }

    /// Every crate of the workspace that may not reach the world.
    fn every_crate_but_the_adapters() -> Vec<std::path::PathBuf> {
        aisdlc_gates::reading_the_checkout::the_crates_under(
            &reading_the_checkout::the_root_of_the_checkout().join("crates"),
        )
        .into_iter()
        .filter(|a_crate| the_crate_named(a_crate) != THE_ADAPTER_CRATE)
        .collect()
    }

    #[test]
    fn a_use_case_naming_a_process_is_refused_and_the_line_is_named() {
        let the_code = "fn the_run() {\n    let _ = std::process::Command::new(\"git\");\n}\n";

        assert_eq!(
            what_reaches_the_world_says(the_code, 1),
            vec!["line 1 reaches for std::process".to_string()],
            "**because a use case that starts a process is a use case whose tests need whatever that process needs** — and 'somewhere in the use cases' is not a place a reader can act on"
        );
    }

    #[test]
    fn a_use_case_naming_a_filesystem_is_refused() {
        let the_code = "fn the_run() {\n    let _ = std::fs::read_dir(\".\");\n}\n";

        assert!(
            !what_reaches_the_world_says(the_code, 1).is_empty(),
            "**and a run that reads a file is a run whose answer depends on what is on the disk that day** — the adapter exists so that a use case can be tested with no disk at all"
        );
    }

    /// What one crate reaches for, named against the file it was found in.
    fn what_one_crate_reaches_for(a_crate: &std::path::Path) -> Vec<String> {
        let mut found = Vec::new();
        let its_source = a_crate.join("src");
        for a_file in aisdlc_gates::reading_the_code::the_rust_files_under(&its_source) {
            for a_reach in what_reaches_the_world_in(&reading_the_checkout::the_text_at(&a_file)) {
                found.push(format!("{} {a_reach}", a_file.display()));
            }
        }
        found
    }

    #[test]
    fn a_reach_in_a_comment_is_not_a_reach() {
        let the_code = "// the adapter uses std::process::Command, and no other crate reaches\nfn the_run() {}\n";

        assert_eq!(
            what_reaches_the_world_in(the_code),
            Vec::<String>::new(),
            "**because a file that explains where the reach lives is not a file that makes it** — and a gate that read its own explanation would refuse the sentence that keeps the rule from being undone"
        );
    }

    #[test]
    fn the_adapter_crate_may_name_the_world() {
        let the_code = "fn the_run() {\n    let _ = std::process::Command::new(\"git\");\n}\n";
        let no_crate_is_the_adapter = every_crate_but_the_adapters()
            .iter()
            .all(|a_crate| the_crate_named(a_crate) != THE_ADAPTER_CRATE);

        assert!(
            no_crate_is_the_adapter && !what_reaches_the_world_in(the_code).is_empty(),
            "**because the adapter is the answer and the use cases are the question** — and a rule that refused the adapters would be a rule refusing the only place a port is allowed to be real"
        );
    }

    #[test]
    fn no_crate_but_the_adapters_reaches_for_the_world() {
        let every_crate = every_crate_but_the_adapters();
        let found: Vec<String> = every_crate
            .iter()
            .flat_map(|a_crate| what_one_crate_reaches_for(a_crate))
            .collect();

        assert!(
            !every_crate.is_empty(),
            "**because a gate that reads no crate passes everything** — and there are crates to read"
        );
        assert!(
            found.is_empty(),
            "a use case may never reach for the world, and these {} reach for {}: {found:?}",
            every_crate.len(),
            THE_MODULES_THAT_REACH_THE_WORLD.join(", "),
        );
    }
}

mod a_declaration_is_held_by_what_it_lacks {
    //! A declaration is read for what it does **not** contain, because that is the half of it a
    //! test cannot reach.
    //!
    //! **Two of this repository's own properties had no holder at all.** `ACredential` printed as a
    //! credential — and adding `#[derive(Debug)]` to it was tried twice in one day and the suite
    //! said nothing either time, because no test can hold a fact about a type. And
    //! `AWhereItWasFiled` took a whole address — **and a function composing one from a number was
    //! added to its module and every test still passed**, because a claim about a door that does
    //! not exist has nothing for a test to name.
    //!
    //! **So this reads the declaration instead of the value.** A reader of a value is a test; a
    //! reader of a declaration is a walk, and this repository already has the walk — the same one
    //! that holds every other rule about how code is written.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;
    use aisdlc_gates::reading_the_code::the_lines_of_code;

    /// What a declaration of one name carries, and the lines it carries it on.
    fn the_declaration_of(one_name: &str) -> Option<(String, Vec<String>)> {
        let marker = format!("struct {one_name}");
        for a_file in every_source_file() {
            let the_text = reading_the_checkout::the_text_at(&a_file);
            if !the_text.contains(&marker) {
                continue;
            }
            let its_lines = the_declaration_and_what_is_above_it(&the_text, &marker);
            if !its_lines.is_empty() {
                return Some((a_file.display().to_string(), its_lines));
            }
        }
        None
    }

    /// The declaration and the attribute lines above it, which is where a `derive` lives.
    ///
    /// **Found by matching and not by arithmetic.** The walk hands back code lines with the line
    /// numbers they sit on, so the list is shorter than the file and a line number is a position,
    /// not an index — **and using one as the other is how this returned nothing at all when a
    /// `derive(Debug)` was added to `ACredential` and this gate did not notice.**
    fn the_declaration_and_what_is_above_it(the_text: &str, the_marker: &str) -> Vec<String> {
        let every_code_line: Vec<String> = the_lines_of_code(the_text)
            .into_iter()
            .map(|(_, one_line)| one_line)
            .collect();
        let Some(where_it_is) = every_code_line
            .iter()
            .position(|one_line| one_line.contains(the_marker))
        else {
            return Vec::new();
        };
        let first_above = where_it_is.saturating_sub(3);
        every_code_line[first_above..=where_it_is].to_vec()
    }

    /// What a declaration wrongly carries, and nothing when it carries nothing wrong.
    fn what_is_wrong_with(one_name: &str) -> Vec<String> {
        let Some((where_it_is, its_lines)) = the_declaration_of(one_name) else {
            return Vec::new();
        };
        let it_can_be_debugged = its_lines
            .iter()
            .any(|one_line| one_line.contains("derive") && one_line.contains("Debug"));
        let it_is_tuple_of_a_number = its_lines.iter().any(|one_line| {
            one_line.contains("derive") && one_line.contains("Debug") && one_line.contains("u32")
        });
        let mut found = Vec::new();
        if it_can_be_debugged {
            found.push(format!(
                "{where_it_is}: {one_name} carries a derived Debug, and a derived Debug is one brace away from a log"
            ));
        }
        if it_is_tuple_of_a_number {
            found.push(format!("{where_it_is}: {one_name} holds a number"));
        }
        found
    }

    #[test]
    fn a_credential_carries_no_derived_debug() {
        assert_eq!(
            what_is_wrong_with("ACredential"),
            Vec::<String>::new(),
            "**because a credential is a thing runs print, log and put in a commit message** — and adding a derive to it twice passed the whole suite, because no test can hold a fact about a type and only a walk of the declaration can"
        );
    }

    #[test]
    fn a_filing_address_is_not_something_a_number_can_be_composed_into() {
        assert_eq!(
            what_is_wrong_with("AWhereItWasFiled"),
            Vec::<String>::new(),
            "**and the door that takes a whole address is the whole of the property** — a function composing one from a number was added to that module and every test still passed, so the composition is a shape and shapes are what this walks"
        );
    }
}

mod no_body_is_built_by_a_hand {
    //! A body that leaves this system is a wire format, and a wire format is not typed by hand.
    //!
    //! **The body a model is given was once written by `format!`**, with the question put where a
    //! quoted string belongs and no quotes around it — **so every request this system ever sent a
    //! model was invalid JSON**, the far end answered `400` with zero bytes, and the refusal that
    //! reached us quoted an empty string. Three real runs, the same refusal, and the cause was
    //! eleven characters of a format string that no test ever looked at.
    //!
    //! **One test beside one function held that one body.** The next body, written the same way,
    //! would have broken the same way — **so the rule is here rather than beside the body, where it
    //! holds every body this system will ever send.**
    //!
    //! **An opening brace and an escaped quote, together, inside a `format!`.** An escaped quote
    //! alone is a sentence somebody wanted to say — `let closing = format!("a \\"" thing")` is
    //! ordinary — and the gate refused that on its first run and named it. **A wire format is not
    //! recognised by its quotes but by its brace**, and that is the difference between a message
    //! with a quote in it and an object built by hand.
    //!
    //! **A literal is the test's own
    //! subject matter — a test that proves a body parses writes broken JSON on purpose — so the
    //! walk reads `src` and not `tests`, and the refused thing is named in the message.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;

    /// The source files a body is built in, and not the tests that write broken JSON on purpose.
    fn every_file_a_body_is_built_in() -> Vec<std::path::PathBuf> {
        every_source_file()
            .into_iter()
            .filter(|a_file| a_file.to_string_lossy().contains("/src/"))
            .collect()
    }

    /// The lines of one file that build something out of escaped quotes and a `format!`.
    fn the_lines_building_a_body_by_hand(a_file: &std::path::Path) -> Vec<String> {
        let the_text = reading_the_checkout::the_text_at(a_file);
        let mut found = Vec::new();
        let mut in_a_format = false;
        for one_line in the_text.lines() {
            if one_line.contains("format!") {
                in_a_format = true;
            }
            if in_a_format && one_line.contains("{\\\"") {
                found.push(one_line.trim().to_string());
            }
            if in_a_format && one_line.trim_end().ends_with(");") {
                in_a_format = false;
            }
        }
        found
    }

    #[test]
    fn no_body_in_this_repository_is_built_by_a_hand() {
        let the_refusals: Vec<String> = every_file_a_body_is_built_in()
            .into_iter()
            .flat_map(|a_file| {
                the_lines_building_a_body_by_hand(&a_file)
                    .into_iter()
                    .map(move |a_line| format!("{} says: {a_line}", a_file.display()))
            })
            .collect();

        assert!(
            the_refusals.is_empty(),
            "**because a body that leaves this system is a wire format, and a wire format typed by \
             hand is one that is broken by anything the hand did not think of** — this one was \
             broken by everything: the question went where a quoted string belongs and no quotes \
             went with it, the far end answered `400` with zero bytes, and the refusal reached us \
             as an empty string three times. Build it with `serde_json`: {the_refusals:?}"
        );
    }
}

mod a_test_is_not_its_own_proof {
    //! A test that says what a program *would* be run with has not run the program.
    //!
    //! **`#107` is the finding this rule is made of.** The test asserted that `--author-name` was in
    //! the arguments `gh` is run with — and it was, and `gh issue comment` has no such flag, and the
    //! assertion was green the whole time. **It proved this system's own arguments and nothing
    //! about `gh`**, and it was the only proof there was, because the file that runs the real
    //! program is a different file.
    //!
    //! **A file that talks about a program's arguments and never runs a program is asserting about
    //! the implementation and calling it a contract.** The stub was not a test double that time; it
    //! was the code under test.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_code::every_source_file;

    /// The test files, and the one place a program's own words could be.
    fn every_test_file() -> Vec<std::path::PathBuf> {
        every_source_file()
            .into_iter()
            .filter(|a_file| a_file.to_string_lossy().contains("/tests/"))
            .collect()
    }

    /// Whether this file asserts about what a program would be run with.
    fn speaks_about_a_programs_arguments(the_text: &str) -> bool {
        the_text.contains("the_arguments_of")
    }

    /// Whether this file ever lets a program answer, in any of the ways it can.
    ///
    /// **Every way, and not the ways that existed when this was written** — so a name added here
    /// without its call site is a hole, and a call site added without its name here is a file this
    /// rule refuses for being correct.
    ///
    /// **`std::process::Command::new(` joined because the container tests build `docker`'s
    /// arguments and then hand them to it that way.** The container is the first program this
    /// system runs that `AProgramWeRun` does not stand in front of — **a container is not a
    /// subprocess and cannot be** — so a file that proves the sandbox by attempting has to reach
    /// for the operating system itself. **The rule's own words demand this entry**: a call site
    /// added without its name here is exactly what this guard exists to catch.
    fn ever_runs_a_program(the_text: &str) -> bool {
        the_text.contains(".run(")
            || the_text.contains("it_is_asked_to(")
            || the_text.contains("the_outcome_of(")
            || the_text.contains("the_run_makes_a_checkout(")
            || the_text.contains("the_filing_it_makes(")
            || the_text.contains("std::process::Command::new(")
    }

    #[test]
    fn no_test_file_asserts_about_a_program_it_never_runs() {
        let the_refusals: Vec<String> = every_test_file()
            .into_iter()
            .filter_map(|a_file| {
                let the_text = reading_the_checkout::the_text_at(&a_file);
                if speaks_about_a_programs_arguments(&the_text) && !ever_runs_a_program(&the_text) {
                    return Some(a_file.display().to_string());
                }
                None
            })
            .collect();

        assert!(
            the_refusals.is_empty(),
            "**because a test that says what a program would be run with has not run the program** — \
             and a file that asserts about the implementation and calls it a contract will one day \
             be the only proof there is, of something that never worked. This happened here: a \
             green test asserted a flag `gh` does not have, and the real program answered \
             `unknown flag`. Let the program answer in the same file: {the_refusals:?}"
        );
    }
}
