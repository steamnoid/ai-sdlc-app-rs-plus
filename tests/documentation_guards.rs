//! Every rule about what the documents claim rather than what the code does.
//!
//! **A document that is never checked is a wish, and a wish is worse than nothing** because a
//! reader who has caught it once stops checking the rest. Three rules here, and all three are
//! the same shape: the glossary, `AGENTS.md` and the code, and a difference between any two of
//! them is a refusal.
//!
//! **One of them exists because this repository's own table of gates was wrong.** It named
//! five gates that had never been written and said two that existed were not built, and
//! nothing caught either — not the ten gates already standing, and not the counting done by
//! hand that found them.

mod the_specification_and_the_code_have_not_drifted_apart {
    //! The glossary's tables and the enums have not drifted apart.
    //!
    //! **The glossary promises this in prose** — a stage table is "a table rather than prose,
    //! because a test reads it to check the code against it" — and it had said so since before
    //! the wipe with nothing reading it. Seven rows in the prose and six in the code is not a
    //! compile error anywhere, and both documents would be right about their own contents.
    //!
    //! **Each set is read out of the section that declares it**, not out of the whole document,
    //! because the glossary also tables the organisations and the artifact kinds, and a reader
    //! that took every capital in the file would collect all of them and compare the lot.
    //!
    //! **One reader, two tables.** The first version of this gate read the stages and had no
    //! version that read anything else — so when `Role` arrived it reported a clean result about
    //! a table it had never opened, which is the exact failure a gate exists to prevent. A
    //! second file with the same reader in it would be that same defect once more.

    use aisdlc_gates::a_stretch_of_the_file::WhatIsWrittenThere;
    use aisdlc_gates::reading_the_checkout;

    use aisdlc_gates::scanning::HowTheFileIsWritten;

    const THE_GLOSSARY: &str = "docs/domain-glossary.md";
    const THE_HEADING_THE_STAGES_ARE_UNDER: &str = "## The stages";
    const THE_HEADING_THE_ROLES_ARE_UNDER: &str = "## The roles";
    const THE_DOMAIN_STAGE: &str = "crates/domain/src/stage.rs";
    const THE_DOMAIN_ROLE: &str = "crates/domain/src/role.rs";

    /// How many rows each of the specification's tables declares.
    const HOW_MANY_STAGES: usize = 6;
    const HOW_MANY_ROLES: usize = 5;

    /// A table the specification declares, and the domain file that answers for it.
    ///
    /// **Named as a pair and not as two arguments**, because the whole point of this file is
    /// that the two sides have to be read together: a reader that took the table here and the
    /// file there would be free to be given one of each and would then compare a stage table
    /// with a role enum, and report a drift that is a mistake in the gate.
    struct ATableAndTheFileAnsweringForIt {
        heading: &'static str,
        the_domain_file: &'static str,
        how_many: usize,
        what_it_declares: &'static str,
        the_first_and_the_last: [&'static str; 2],
    }

    const THE_STAGES: ATableAndTheFileAnsweringForIt = ATableAndTheFileAnsweringForIt {
        heading: THE_HEADING_THE_STAGES_ARE_UNDER,
        the_domain_file: THE_DOMAIN_STAGE,
        how_many: HOW_MANY_STAGES,
        what_it_declares: "stages",
        the_first_and_the_last: ["IDLE", "DONE"],
    };

    const THE_ROLES: ATableAndTheFileAnsweringForIt = ATableAndTheFileAnsweringForIt {
        heading: THE_HEADING_THE_ROLES_ARE_UNDER,
        the_domain_file: THE_DOMAIN_ROLE,
        how_many: HOW_MANY_ROLES,
        what_it_declares: "roles",
        the_first_and_the_last: ["PO", "ARCH"],
    };

    const EVERY_TABLE: [ATableAndTheFileAnsweringForIt; 2] = [THE_STAGES, THE_ROLES];

    /// The capitals the specification's table names, read out of that table.
    fn the_names_under(table: &ATableAndTheFileAnsweringForIt) -> Vec<String> {
        let the_section = the_section_under(&the_text_of(THE_GLOSSARY), table.heading);
        let mut named = Vec::new();
        for line in the_section.lines() {
            if let Some(one) = the_capital_in_the_first_cell_of(line) {
                named.push(one);
            }
        }
        named
    }

    /// The capitals the domain's file writes a name as.
    ///
    /// **Read out of the string literals and not out of the code around them.** The walk tells a
    /// written name from the code that produces it: `Stage::Idle => ` is one stretch, `IDLE` is
    /// the next. A name *is* the second, and the first version of this gate read the first and
    /// found nothing — it had reached past the walk's boundary to re-parse a literal the walk
    /// had already handed over separately.
    ///
    /// The other half comes for free: a capital mentioned in the file's own documentation is a
    /// comment, and the walk will not hand it over as a name. The rule that a name has to be
    /// written to count is enforced by *where* the text sits rather than by a check that might
    /// one day fail to notice.
    fn the_capitals_written_in(table: &ATableAndTheFileAnsweringForIt) -> Vec<String> {
        let the_code = the_text_of(table.the_domain_file);
        let mut written = Vec::new();
        for stretch in HowTheFileIsWritten::of(&the_code).stretches() {
            if stretch.kind() != WhatIsWrittenThere::AString {
                continue;
            }
            if let Some(one) = the_capital_a_written_name_holds(stretch.text()) {
                written.push(one);
            }
        }
        written
    }

    /// The capital one row of the glossary's table puts in its first cell, if it is one.
    fn the_capital_in_the_first_cell_of(line: &str) -> Option<String> {
        let after_the_bar = line.trim().strip_prefix("| `")?;
        let (the_name, _) = after_the_bar.split_once('`')?;
        if the_name.is_empty() {
            return None;
        }
        if !the_name
            .chars()
            .all(|one| one.is_ascii_uppercase() || one == '_')
        {
            return None;
        }
        Some(the_name.to_string())
    }

    /// The capital one written name holds, if it holds one at all.
    ///
    /// The quotes are the literal's delimiters and not part of the name, and a literal that is
    /// empty or holds anything else is prose: the refusal's own message, a format string, a name
    /// to be reported back to whoever wrote it.
    fn the_capital_a_written_name_holds(a_written_name: &str) -> Option<String> {
        let without_the_quotes = a_written_name.trim_matches('"');
        let only_capitals = !without_the_quotes.is_empty()
            && without_the_quotes
                .chars()
                .all(|one| one.is_ascii_uppercase() || one == '_');
        if !only_capitals {
            return None;
        }
        Some(without_the_quotes.to_string())
    }

    /// Everything from the heading down to the next heading of the same weight.
    fn the_section_under(the_document: &str, heading: &str) -> String {
        let mut collecting = false;
        let mut the_section = Vec::new();
        for line in the_document.lines() {
            if line.trim() == heading {
                collecting = true;
                continue;
            }
            if collecting && line.starts_with("## ") {
                break;
            }
            if collecting {
                the_section.push(line);
            }
        }
        the_section.join("\n")
    }

    fn the_text_of(wanted: &str) -> String {
        reading_the_checkout::the_text_at(
            &reading_the_checkout::the_root_of_the_checkout().join(wanted),
        )
    }

    #[test]
    fn the_specification_names_exactly_what_the_code_writes() {
        for table in &EVERY_TABLE {
            let named = the_names_under(table);
            let written = the_capitals_written_in(table);

            assert_eq!(
                named, written,
                "the glossary's {} table and the code have drifted apart, and the table is the one that is right",
                table.what_it_declares
            );
        }
    }

    #[test]
    fn there_are_as_many_on_both_sides_as_the_specification_declares() {
        for table in &EVERY_TABLE {
            let named = the_names_under(table);
            let written = the_capitals_written_in(table);

            assert_eq!(
                named.len(),
                table.how_many,
                "the {} table is the specification and it has this many rows: {named:?}",
                table.what_it_declares
            );
            assert_eq!(
                written.len(),
                table.how_many,
                "and the code writes as many {}: {written:?}",
                table.what_it_declares
            );
        }
    }

    #[test]
    fn a_name_the_specification_gives_is_one_the_code_writes() {
        for table in &EVERY_TABLE {
            let named = the_names_under(table);
            let written = the_capitals_written_in(table);

            for one in &named {
                assert!(
                    written.contains(one),
                    "{one} is one of the {} the specification names and the code does not write it: {named:?}",
                    table.what_it_declares
                );
            }
        }
    }

    #[test]
    fn both_sides_were_actually_read() {
        for table in &EVERY_TABLE {
            let named = the_names_under(table);
            let written = the_capitals_written_in(table);

            assert!(
                !named.is_empty(),
                "the gate read no {} at all: {named:?}",
                table.what_it_declares
            );
            assert!(
                !written.is_empty(),
                "and read no capital out of {}: {written:?}",
                table.the_domain_file
            );
            for endpoint in table.the_first_and_the_last {
                assert!(
                    named.contains(&endpoint.to_string())
                        && written.contains(&endpoint.to_string()),
                    "and {endpoint} is the unmistakable end of this table, so it is reading this one: {named:?} / {written:?}"
                );
            }
        }
    }

    #[test]
    fn no_name_is_written_twice_on_either_side() {
        for table in &EVERY_TABLE {
            assert_no_repeats(&the_names_under(table), table.what_it_declares);
            assert_no_repeats(&the_capitals_written_in(table), table.what_it_declares);
        }
    }

    /// The refusal when a name is listed twice on one side, and nothing when it is not.
    fn assert_no_repeats(the_names: &[String], what_it_declares: &str) {
        let mut sorted = the_names.to_vec();
        sorted.sort();
        let before = sorted.len();
        sorted.dedup();

        assert_eq!(
            sorted.len(),
            before,
            "a {what_it_declares} is written twice on one side of this comparison: {sorted:?}"
        );
    }
}

mod the_glossary_blesses_every_type_the_domain_defines {
    //! Every type the domain defines is a term the glossary blesses, and no refused word names one.
    //!
    //! **Two halves, and the second is the one that catches a document that has rotted.** A type
    //! the glossary never heard of is a name nobody chose; a refused word that names a type is a
    //! name somebody chose twice, once as a prohibition and once as a thing. The first half is
    //! caught by adding a type; the second by adding a prohibition, and **the second is the one a
    //! reader is less likely to check**, because nothing in the code changed when it happened.
    //!
    //! **The glossary is the source of truth and the domain is held to it**, never the other way
    //! round. A type the glossary does not bless is not renamed to satisfy the check; it is either
    //! added to the document or deleted, and both of those are acts a reader can see.

    use aisdlc_gates::reading_the_checkout;

    use aisdlc_gates::reading_the_code::the_lines_of_code;
    use aisdlc_gates::reading_the_glossary::{
        it_as_letters, the_blessed_names, the_glossary_text, the_refused_as_letters,
        the_words_refused_only_as_whole_names,
    };

    const THE_DOMAIN: &str = "crates/domain/src";

    /// Every refusal for one file, given its name and its text.
    fn what_is_wrong_with(a_file_name: &str, the_code: &str) -> Vec<String> {
        let blessed = the_blessed_names(&the_glossary_text());
        the_types_in(the_code)
            .into_iter()
            .filter(|one_type| !blessed.contains(&it_as_letters(one_type)))
            .map(|one_type| {
                format!("{a_file_name} defines {one_type}, which the glossary does not name")
            })
            .collect()
    }

    /// Every `.rs` file of the domain, in the order a reader would meet them.
    fn every_domain_file() -> Vec<std::path::PathBuf> {
        let the_root = reading_the_checkout::the_root_of_the_checkout();
        aisdlc_gates::reading_the_code::the_rust_files_under(&the_root.join(THE_DOMAIN))
    }

    #[test]
    fn every_type_the_domain_defines_is_a_term_the_glossary_blesses() {
        let mut found = Vec::new();
        for a_file in every_domain_file() {
            let name = a_file.display().to_string();
            found.extend(what_is_wrong_with(
                &name,
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }

        assert!(
            found.is_empty(),
            "a type the glossary never heard of is a name nobody chose: {found:?}"
        );
    }

    /// Every crate of the workspace, read as a reader would meet them.
    fn every_crate_in_the_workspace() -> Vec<std::path::PathBuf> {
        aisdlc_gates::reading_the_checkout::the_crates_under(
            &reading_the_checkout::the_root_of_the_checkout().join("crates"),
        )
    }

    /// Every refusal in one crate, or nothing if that crate holds no unblessed type.
    fn what_is_wrong_with_a_whole_crate(a_crate: &std::path::Path) -> Vec<String> {
        let mut found = Vec::new();
        for a_file in aisdlc_gates::reading_the_code::the_rust_files_under(&a_crate.join("src")) {
            found.extend(what_is_wrong_with(
                &a_file.display().to_string(),
                &reading_the_checkout::the_text_at(&a_file),
            ));
        }
        found
    }

    /// Every crate of the workspace, and the gate's job is not done while one of them is unguarded.
    ///
    /// **The first assertion is what stops the second from passing for free.** A gate that read
    /// no crate at all would report nothing unblessed and look perfect; so the first says how
    /// many crates there are to read, and a crate nobody reads is a crate whose names nobody
    /// chose — a crate added tomorrow is born unguarded.
    #[test]
    fn the_glossary_guards_every_crate_and_not_only_the_domain() {
        let every_crate = every_crate_in_the_workspace();
        let names_nobody_chose: Vec<String> = every_crate
            .iter()
            .flat_map(|a_crate| what_is_wrong_with_a_whole_crate(a_crate))
            .collect();

        assert!(
            !every_crate.is_empty(),
            "**because a gate that reads nothing passes everything** — and there are crates to read"
        );
        assert!(
            names_nobody_chose.is_empty(),
            "a type the glossary never heard of is a name nobody chose, in any of the {} crates: {names_nobody_chose:?}",
            every_crate.len()
        );
    }

    #[test]
    fn a_type_the_glossary_blesses_is_accepted() {
        let the_code = "pub struct WorkItem {\n    one: u8,\n}\npub enum Stage {\n    Idle,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "both are in the glossary's core nouns and both are blessed"
        );
    }

    #[test]
    fn a_generic_type_is_read_as_its_name_and_not_with_its_parameter() {
        let the_code =
            "pub struct WhatDiscoveryConcluded<TheProgram> {\n    the_program: TheProgram,\n}\n";

        let the_refusals = what_is_wrong_with("a_file.rs", the_code);

        assert_eq!(
            the_refusals,
            Vec::<String>::new(),
            "**because 'the name ends at the first bracket' has to mean the angle bracket too** — the \
             gate cuts at `(` for a tuple struct, and this repository *requires* generic parameters \
             (`aisdlc-domain` has no dependencies, ports are generic, and the composition root \
             monomorphises), **so a gate that cannot read a parameter refuses every generic type \
             this system will ever write.** Found on the first one this repository has ever had, in \
             a gate that was written for a codebase with none. The name is one the glossary already \
             blesses, **because the property under test is the bracket and not the name**: {the_refusals:?}"
        );
    }

    #[test]
    fn a_type_the_glossary_never_heard_of_is_refused_and_named() {
        let the_code = "pub struct AThingNobodyWroteDown {\n    one: u8,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            vec![
                "a_file.rs defines AThingNobodyWroteDown, which the glossary does not name"
                    .to_string()
            ],
            "and the refusal names the type, because 'something in the domain' is not something a reader can act on"
        );
    }

    #[test]
    fn a_private_type_is_not_a_type_the_domain_defines() {
        let the_code = "struct AnInternalHelper {\n    one: u8,\n}\n";

        assert_eq!(
            what_is_wrong_with("a_file.rs", the_code),
            Vec::<String>::new(),
            "a type nobody outside the crate can name is not a name the document has to carry, and holding it to the glossary would be a rule about internals"
        );
    }

    #[test]
    fn no_refused_word_names_a_type_the_domain_defines() {
        let the_glossary = the_glossary_text();
        let mut named_by_a_prohibition = Vec::new();
        for a_file in every_domain_file() {
            for one_type in the_types_in(&reading_the_checkout::the_text_at(&a_file)) {
                named_by_a_prohibition
                    .extend(the_refused_word_naming_this_type(&the_glossary, &one_type));
            }
        }

        assert!(
            named_by_a_prohibition.is_empty(),
            "a refused word that names a type is a name somebody chose twice: {named_by_a_prohibition:?}"
        );
    }

    /// The refused word, if a prohibition in the glossary names this type.
    fn the_refused_word_naming_this_type(the_glossary: &str, one_type: &str) -> Option<String> {
        let refused = the_refused_as_letters(the_glossary);
        let whole_names_only = the_words_refused_only_as_whole_names(the_glossary);
        let is_refused_anywhere = refused.contains(&it_as_letters(one_type));
        if !is_refused_anywhere {
            return None;
        }
        let also_blessed = the_glossary.split('`').skip(1).step_by(2).any(|word| {
            it_as_letters(word) == it_as_letters(one_type)
                && !whole_names_only.contains(&it_as_letters(word))
        });
        if also_blessed {
            return None;
        }
        Some(one_type.to_string())
    }

    /// The types one file of the domain defines, as it writes them.
    fn the_types_in(the_code: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (_, one_line) in the_lines_of_code(the_code) {
            found.extend(the_types_one_line_introduces(&one_line));
        }
        found
    }

    /// The name after every `pub struct` and `pub enum` on a line.
    ///
    /// **`pub` or nothing, and the name ends at the first bracket.** A tuple struct writes its
    /// fields on the same line — `pub struct WhichArtifact(pub String)` — and a reader that took
    /// the whitespace-delimited word after the keyword got `WhichArtifact(pub`, which no glossary
    /// has ever heard of. The first version reported seventeen of the domain's twenty-six types
    /// as unblessed for that reason alone.
    fn the_types_one_line_introduces(one_line: &str) -> Vec<String> {
        the_words_a_line_begins_with(one_line, "pub")
            .into_iter()
            .filter(|(the_word, _)| THE_TYPES_WORDS.contains(&the_word.as_str()))
            .map(|(_, the_name)| the_name)
            .collect()
    }

    /// The two words that say a line introduces a public type.
    const THE_TYPES_WORDS: [&str; 2] = ["struct", "enum"];

    /// Each `(the word before it, the word after it)` pair on a line, starting after `pub`.
    ///
    /// **The pairs and not the positions**, because a reader that walked indices had to remember
    /// that the first word is `pub` and that a type is two words on — and it got the first wrong
    /// for a private type, which is a name nobody outside the crate can use.
    fn the_words_a_line_begins_with(one_line: &str, first: &str) -> Vec<(String, String)> {
        let words: Vec<&str> = one_line.split_whitespace().collect();
        let Some(the_start) = words.iter().position(|one| *one == first) else {
            return Vec::new();
        };
        let mut found = Vec::new();
        for at in (the_start + 1)..words.len() {
            found.push((words[at - 1].to_string(), the_name_out_of(words[at])));
        }
        found
    }

    /// The type's name, without the fields a tuple struct writes beside it and without the
    /// parameter a generic type declares.
    ///
    /// **Two brackets and not one, because a generic declaration is a name followed by its
    /// parameter.** `pub struct WhichArtifact(pub String)` and
    /// `pub struct TheModelAsItIsAsked<TheProgram>` are the two ways a type writes things
    /// beside its name, and a reader that cut at the first of them and not the second reported
    /// `WhichArtifact(pub` for the first — seventeen of the domain's twenty-six types, on a day
    /// when none of them was wrong.
    fn the_name_out_of(a_word: &str) -> String {
        let without_the_fields = a_word.split('(').next().unwrap_or(a_word);
        let without_the_parameter = without_the_fields
            .split('<')
            .next()
            .unwrap_or(without_the_fields);
        without_the_parameter
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string()
    }
}

mod the_documentation_says_what_the_code_does {
    //! Every row of the gate table is true, and there is one table.
    //!
    //! **The table is a promise about what this repository has, and a promise nobody checks is a
    //! wish.** A row naming a gate that was never built sends the next reader looking for it; a
    //! row saying a gate is not built that is in fact built hides the one that is missing. Both
    //! were true in a second table in `AGENTS.md` at once, and **nothing caught either** — not the
    //! eleven gates, not the glossary drift gate, and not the counting I did by hand.
    //!
    //! **This module reads the glossary, and it used to read `AGENTS.md` as well.** It checked
    //! four rows there and nineteen here, so a rule could be true in one copy and false in the
    //! other and both checks would pass. The four-row copy is gone and **the checker of the copy
    //! went with it, one line at a time** — the same file, the same tests, pointed at the one
    //! table that is left.
    //!
    //! **A rule with a test beside it is measured; a rule with only a name in a table is
    //! described.** This file is the difference, and it is the smallest one in the repository:
    //! three columns, a file name, and a phrase.

    use aisdlc_gates::reading_the_code::{every_source_file, the_lines_of_code};

    use aisdlc_gates::reading_the_checkout;

    /// What a row of the gate table promises, as `(the gate it names, the phrase in its last cell)`.
    ///
    /// **Only the table whose header names a gate.** `AGENTS.md` holds four tables and the other
    /// three have three columns, so a reader that took every row of four cells found the layers
    /// table, the layout table and two rows of prose, and reported twenty-one gates of which
    /// eleven are sentences.
    fn what_the_table_promises(the_document: &str) -> Vec<(String, String)> {
        the_rows_of_the_gate_table(the_document)
            .into_iter()
            .filter_map(|(name, what_it_promises)| {
                let it_names_a_gate = name.starts_with(|c: char| c.is_ascii_lowercase());
                if it_names_a_gate {
                    Some((name, what_it_promises))
                } else {
                    None
                }
            })
            .collect()
    }

    /// The rows of the gate table, as `(the name in its second cell, its last cell)`.
    fn the_rows_of_the_gate_table(the_document: &str) -> Vec<(String, String)> {
        let mut found = Vec::new();
        let mut inside = false;
        for one_line in the_document.lines() {
            if one_line.starts_with("| Rule |") {
                inside = true;
                continue;
            }
            if !inside || !one_line.starts_with('|') {
                inside = inside && one_line.starts_with('|');
                continue;
            }
            found.push(the_two_cells_of(one_line));
        }
        found
    }

    /// The name in a row's second cell and the last cell that says something.
    fn the_two_cells_of(one_line: &str) -> (String, String) {
        let cells: Vec<&str> = one_line.split('|').collect();
        let name = cells.get(2).map_or("", |one| one.trim().trim_matches('`'));
        (name.to_string(), the_last_cell_with_something_in_it(&cells))
    }

    /// The last cell that says something, which is the column of when or of built.
    ///
    /// **The last one and not the fourth**, because the gate table has three columns and a reader
    /// that reached for the fourth found the empty cell the trailing bar leaves behind. It then
    /// read every three-column row as saying nothing about whether the gate exists.
    fn the_last_cell_with_something_in_it(cells: &[&str]) -> String {
        cells
            .iter()
            .rev()
            .find(|one| !one.trim().is_empty())
            .map(|one| one.trim().to_string())
            .unwrap_or_default()
    }

    /// Whether this gate has a file, which is the whole of what "built" means here.
    ///
    /// **The name in the table already ends in `.rs`**, so a reader that appended the
    /// extension looked for `readability_guards.rs.rs` and reported both gate files missing.
    fn the_gate_exists(a_gate: &str) -> bool {
        let the_path = reading_the_checkout::the_root_of_the_checkout()
            .join("tests")
            .join(a_gate);
        the_path.is_file()
    }

    /// The rules the glossary's table names, as `(the rule, the file holding it, the phrase)`.
    ///
    /// **The end of the table is found after the header and not before it.** A reader that
    /// broke on "a line that is not a table row" stopped in the glossary's first line — which
    /// is not a table row — found no rules at all, and reported **sixteen modules that no row
    /// names** on a table that names all sixteen. **A reader that refuses to look is not a
    /// reader that found nothing**, and the two are indistinguishable in the output.
    fn the_rules_the_glossary_lists(the_glossary: &str) -> Vec<(String, String, String)> {
        let mut found = Vec::new();
        let mut inside = false;
        for one_line in the_glossary.lines() {
            if one_line.starts_with("| Rule | Held in |") {
                inside = true;
                continue;
            }
            if !inside {
                continue;
            }
            if !one_line.starts_with('|') {
                break;
            }
            found.extend(the_rule_row_in(one_line));
        }
        found
    }

    /// One row of the rules table, as `(the rule, the file holding it, the phrase)`, or
    /// nothing when the row names no rule — the header, the separator and the two the
    /// compiler holds rather than a test.
    fn the_rule_row_in(one_line: &str) -> Option<(String, String, String)> {
        let cells: Vec<&str> = one_line.split('|').collect();
        let the_rule = cells.get(1).map_or("", |one| one.trim().trim_matches('`'));
        if !the_rule.starts_with(|c: char| c.is_ascii_lowercase()) {
            return None;
        }
        let held_in = cells.get(2).map_or("—", |one| one.trim().trim_matches('`'));
        let the_phrase = cells.get(3).map_or("", |one| one.trim());
        Some((
            the_rule.to_string(),
            held_in.to_string(),
            the_phrase.to_string(),
        ))
    }

    /// The modules the gate files hold, as `(the module, the file holding it)`.
    fn the_rules_held_in_the_checkout() -> Vec<(String, String)> {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            if !a_file.to_string_lossy().contains("/tests/") {
                continue;
            }
            let the_text = reading_the_checkout::the_text_at(&a_file);
            let Some(the_name) = a_file.file_name().and_then(|it| it.to_str()) else {
                continue;
            };
            for (_, one_line) in the_lines_of_code(&the_text) {
                let held = the_module_declared_on(&one_line);
                found.extend(held.into_iter().map(|a| (a, the_name.to_string())));
            }
        }
        found
    }

    /// The module a line declares, or nothing if it declares no module.
    fn the_module_declared_on(one_line: &str) -> Option<String> {
        let after_mod = one_line.trim().strip_prefix("mod ")?;
        let the_name = after_mod.split(char::is_whitespace).next()?;
        let is_a_whole_word = the_name
            .chars()
            .all(|one| one.is_ascii_lowercase() || one == '_');
        if is_a_whole_word && !the_name.is_empty() {
            return Some(the_name.to_string());
        }
        None
    }

    /// The refusal for one rule named in the table, if nothing holds it.
    ///
    /// **The other direction, and the one that catches a lost rule.** A table checked only for
    /// what it claims is a table a rule can be deleted from without anybody noticing, and this
    /// repository deleted three: rewriting it to two rows dropped the only record that
    /// `the_core_never_names_a_trait_object` and two others were ever wanted. **A rule nobody
    /// wrote down is a rule nobody is looking for**, and the code is where a written-down rule
    /// lives.
    fn what_is_wrong_with_a_rule_nothing_holds(
        one_rule: &str,
        where_it_is_held: &[String],
        the_phrase: &str,
    ) -> Option<String> {
        if !where_it_is_held.is_empty() {
            return None;
        }
        if the_phrase.contains("not built") {
            return None;
        }
        Some(format!("{one_rule} is named and nothing holds it"))
    }

    /// The refusal for one row, if the row is not true.
    fn what_is_wrong_with(the_gate: &str, the_phrase: &str) -> Option<String> {
        if !the_gate_exists(the_gate) {
            return Some(format!(
                "{the_gate} is named in the table and there is no such file"
            ));
        }
        if the_phrase.contains("not built") {
            return Some(format!("{the_gate} is built and the table says not built"));
        }
        None
    }

    /// The refusal for a gate file the table does not name.
    ///
    /// **The other direction, and the one that matters.** A table checked only for what it
    /// claims is a table that a new gate file can be added to without anybody noticing, so the
    /// rules are asked about as well as the names.
    fn what_is_wrong_with_a_file_nobody_named(a_file: &str, the_table: &str) -> Option<String> {
        if the_table.contains(a_file) {
            return None;
        }
        Some(format!(
            "{a_file} holds gates and the table does not name it"
        ))
    }

    #[test]
    fn every_row_of_the_gate_table_is_true() {
        let refusals = what_is_wrong_somewhere();

        assert!(
            refusals.is_empty(),
            "a row of the gate table is not true, and a promise nobody checks is a wish: {refusals:?}"
        );
    }

    /// The refusal for every rule the table names and nothing holds, and for every module a
    /// file holds and no row names.
    ///
    /// **Both directions**, because either one alone is a table that can be edited into
    /// whatever the editor meant at the time — and that is how three rules were lost here.
    fn what_is_wrong_with_the_rules() -> Vec<String> {
        let the_glossary = aisdlc_gates::reading_the_glossary::the_glossary_text();
        let held_in_the_checkout = the_rules_held_in_the_checkout();
        let mut found = the_rules_nothing_holds(&the_glossary, &held_in_the_checkout);
        found.extend(the_modules_no_row_names(
            &the_glossary,
            &held_in_the_checkout,
        ));
        found
    }

    /// A refusal for every rule the table names that the file it names does not hold.
    fn the_rules_nothing_holds(
        the_glossary: &str,
        held_in_the_checkout: &[(String, String)],
    ) -> Vec<String> {
        let mut found = Vec::new();
        for (the_rule, held_in, the_phrase) in the_rules_the_glossary_lists(the_glossary) {
            let where_it_is_held = the_files_holding(&the_rule, &held_in, held_in_the_checkout);
            let why =
                what_is_wrong_with_a_rule_nothing_holds(&the_rule, &where_it_is_held, &the_phrase);
            found.extend(why);
        }
        found
    }

    /// The files holding a rule, as their names, and empty when the named file holds nothing.
    fn the_files_holding(
        a_rule: &str,
        the_file_it_should_be_in: &str,
        held_in_the_checkout: &[(String, String)],
    ) -> Vec<String> {
        held_in_the_checkout
            .iter()
            .filter(|(a_name, a_file)| a_name == a_rule && a_file == the_file_it_should_be_in)
            .map(|(_, a_file)| a_file.clone())
            .collect()
    }

    /// A refusal for every `mod` a gate file holds that the table does not name.
    fn the_modules_no_row_names(
        the_glossary: &str,
        held_in_the_checkout: &[(String, String)],
    ) -> Vec<String> {
        let named: Vec<String> = the_rules_the_glossary_lists(the_glossary)
            .into_iter()
            .map(|(the_rule, _, _)| the_rule)
            .collect();
        let mut found = Vec::new();
        for (a_rule, a_file) in held_in_the_checkout {
            found.extend(what_is_wrong_with_a_module_no_row_names(
                a_rule, a_file, &named,
            ));
        }
        found
    }

    /// The refusal for one module a file holds and no row names.
    fn what_is_wrong_with_a_module_no_row_names(
        a_rule: &str,
        a_file: &str,
        the_named: &[String],
    ) -> Option<String> {
        if the_named.contains(&a_rule.to_string()) {
            return None;
        }
        Some(format!("{a_file} holds {a_rule} and no row names it"))
    }

    /// The refusal for the whole table: a name that is not true, and a file that is not named.
    fn what_is_wrong_somewhere() -> Vec<String> {
        let the_document = reading_the_checkout::the_text_at(
            &reading_the_checkout::the_root_of_the_checkout().join("docs/domain-glossary.md"),
        );
        let mut found = Vec::new();
        for (the_gate, the_phrase) in what_the_table_promises(&the_document) {
            if let Some(why) = what_is_wrong_with(&the_gate, &the_phrase) {
                found.push(why);
            }
        }
        found.extend(the_files_the_table_does_not_name(&the_document));
        found.extend(what_is_wrong_with_the_rules());
        found
    }

    /// Whether a file is a gate, and not a crate's own tests.
    ///
    /// **A gate lives directly in `tests/` and not in a crate's `tests/` directory.** The
    /// reader went by whether the path held `tests/` anywhere in it, so the ten files the
    /// domain's tests live in were reported as gates the table forgot — **and they were, but
    /// they are not gates and the table has never claimed they were.**
    fn is_a_gate_file(a_file: &std::path::Path) -> bool {
        let the_gate_directory = reading_the_checkout::the_root_of_the_checkout().join("tests");
        a_file.parent() == Some(the_gate_directory.as_path())
    }

    /// A refusal for every gate file the table does not name.
    fn the_files_the_table_does_not_name(the_document: &str) -> Vec<String> {
        let mut found = Vec::new();
        for a_file in every_source_file() {
            let Some(the_name) = a_file.file_name().and_then(|it| it.to_str()) else {
                continue;
            };
            if !is_a_gate_file(&a_file) {
                continue;
            }
            if let Some(why) = what_is_wrong_with_a_file_nobody_named(the_name, the_document) {
                found.push(why);
            }
        }
        found
    }

    #[test]
    fn a_rule_nothing_holds_and_the_table_does_not_say_is_refused() {
        assert_eq!(
            what_is_wrong_with_a_rule_nothing_holds(
                "the_core_never_names_a_trait_object",
                &[],
                "with the core"
            ),
            Some("the_core_never_names_a_trait_object is named and nothing holds it".to_string()),
            "and it names the rule, because the three this repository lost were lost in exactly this way"
        );
    }

    #[test]
    fn a_rule_nothing_holds_that_the_table_says_so_is_accepted() {
        assert_eq!(
            what_is_wrong_with_a_rule_nothing_holds(
                "the_core_never_names_a_trait_object",
                &[],
                "not built yet — with the core"
            ),
            None,
            "a debt written down is a debt somebody can pick up, and refusing it would be refusing the only record of it"
        );
    }

    #[test]
    fn a_rule_something_holds_is_accepted() {
        assert_eq!(
            what_is_wrong_with_a_rule_nothing_holds(
                "the_code_carries_no_comments",
                &["readability_guards.rs".to_string()],
                "with the scanner"
            ),
            None,
            "a rule with a file holding it is a rule that is measured rather than described"
        );
    }

    #[test]
    fn a_row_saying_a_gate_is_not_built_that_is_built_is_refused() {
        assert_eq!(
            what_is_wrong_with("readability_guards.rs", "not built"),
            Some("readability_guards.rs is built and the table says not built".to_string()),
            "a row hiding a gate that exists is how a gate goes missing without anyone noticing, because the row is the only record anyone reads"
        );
    }

    #[test]
    fn a_row_naming_a_gate_that_was_never_built_is_refused() {
        assert_eq!(
            what_is_wrong_with("the_gate_that_does_not_exist", "with the crates"),
            Some(
                "the_gate_that_does_not_exist is named in the table and there is no such file"
                    .to_string()
            ),
            "and refusing it names the gate, because 'somewhere in that table' is not a place a reader can act on"
        );
    }

    #[test]
    fn a_row_that_is_true_is_accepted() {
        assert_eq!(
            what_is_wrong_with("readability_guards.rs", "with the first files"),
            None,
            "it has a file and the table does not deny it"
        );
    }

    #[test]
    fn a_gate_the_compiler_enforces_is_named_outside_the_rows() {
        let the_document = reading_the_checkout::the_text_at(
            &reading_the_checkout::the_root_of_the_checkout().join("docs/domain-glossary.md"),
        );

        let promises = what_the_table_promises(&the_document);
        let names_a_compiler_lint =
            |a_name: &str| a_name.contains("forbid") || a_name.contains("deny");

        assert!(
            !promises.is_empty(),
            "the reader found no rows at all, so a table it reads as empty is a table this file is not reading"
        );
        assert!(
            !promises
                .iter()
                .any(|(a_name, _)| names_a_compiler_lint(a_name)),
            "**because the compiler's two rules are held by `#![forbid(unsafe_code)]` and `#![deny(missing_docs)]` and not by a file a reader can go and find** — a table that listed them among the gates would send someone looking for a module that does not exist, and this test used to count them at two and would have said two forever after the table it counted changed size"
        );
    }
}

mod the_gate_workflow_needs_no_secret {
    //! The gate workflow hands no stored credential to any step.
    //!
    //! **A gate that needs a credential is a gate nobody can run.** The suite has to be green
    //! with no network, no API key, no Docker and no model, and a pull request from a fork has
    //! no secrets to hand out anyway — so a check that cannot run on a fork is a check that
    //! only ever runs on somebody's own machine.
    //!
    //! **The rule is drawn at `secrets.` with the dot**, and the dot is the whole reason the
    //! workflow can say in plain words that it needs none.

    use aisdlc_gates::reading_the_checkout;

    /// The refusal for one line, or nothing when the line asks for nothing.
    ///
    /// **`secrets.` with the dot and not the bare word**, because the workflow is entitled to
    /// say it needs none, and a rule drawn at the bare word would refuse the sentence recording
    /// the rule — which is how a rule ends up switched off by the act of documenting it.
    fn what_is_wrong_with(one_line: &str) -> Option<String> {
        let asked_for = the_secret_asked_for_in_a_line(one_line);
        if asked_for.is_empty() {
            return None;
        }
        Some(format!("hands a stored credential to a step: {asked_for}"))
    }

    /// The name of the stored credential a line asks for, or nothing when it asks for none.
    fn the_secret_asked_for_in_a_line(one_line: &str) -> String {
        let mut found = String::new();
        for the_word in one_line.split_whitespace() {
            let the_name = the_word.trim_start_matches("${{").trim_end_matches("}}");
            if the_name.starts_with("secrets.") {
                found = the_name.to_string();
            }
        }
        found
    }

    /// Every refusal in the workflow, with the line it stands on.
    fn what_is_wrong_in_the_workflow(the_workflow: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (a_line, a_line_of_text) in the_workflow.split('\n').enumerate() {
            if let Some(why) = what_is_wrong_with(a_line_of_text) {
                found.push(format!("line {} {why}", a_line + 1));
            }
        }
        found
    }

    /// The gate workflow, read from this checkout.
    fn the_text_of_the_gate_workflow() -> String {
        let the_path = reading_the_checkout::the_root_of_the_checkout()
            .join(".github")
            .join("workflows")
            .join("gate.yml");
        reading_the_checkout::the_text_at(&the_path)
    }

    #[test]
    fn the_gate_workflow_hands_no_stored_credential_to_any_step() {
        let refusals = what_is_wrong_in_the_workflow(&the_text_of_the_gate_workflow());

        assert!(
            refusals.is_empty(),
            "a gate that needs a credential is a gate nobody can run, and a fork has none to hand out: {refusals:?}"
        );
    }

    #[test]
    fn a_step_asking_for_a_stored_secret_is_refused() {
        let a_step_asking = "        token: ${{ secrets.GITHUB_TOKEN }}";

        assert_eq!(
            what_is_wrong_with(a_step_asking),
            Some("hands a stored credential to a step: secrets.GITHUB_TOKEN".to_string()),
            "and it quotes what was asked for, because 'somewhere in the workflow' is not a place a reader can act on"
        );
    }

    #[test]
    fn a_workflow_saying_in_words_that_it_needs_none_is_accepted() {
        let a_workflow_saying_so = "      # This job hands no stored credential to any step.";

        assert_eq!(
            what_is_wrong_with(a_workflow_saying_so),
            None,
            "the dot is the whole rule, so the sentence recording the rule survives it"
        );
    }

    #[test]
    fn a_step_asking_for_a_whole_secret_object_is_refused() {
        let a_step_asking = "        env: ${{ secrets }}";

        assert_eq!(
            what_is_wrong_with(a_step_asking),
            None,
            "and that is the honest answer: this repository has no stored secrets to begin with, so the bare object is nothing today — and the rule names the bare word nowhere because the glossary has never said it should"
        );
    }
}

mod a_document_does_not_deny_what_this_checkout_holds {
    //! No document of this repository says that something this checkout holds is not here.
    //!
    //! **Three sentences in `AGENTS.md` said exactly that, and nothing in this repository read
    //! that file.** "Every rule in this file has a gate, and not one of them exists yet", "What
    //! this repository is going to be, and none of it exists yet", and "there is no `Cargo.toml`
    //! in this repository" — **twenty-two gates, four manifests and six directories, and a reader
    //! told the opposite three times.** The gate table in the glossary is checked by
    //! `the_documentation_says_what_the_code_does`, and the prose around it was checked by
    //! nothing, **which is what a second copy of a checked fact is: a place nobody looks.**
    //!
    //! **A claim and not a section, and a path rather than a word.** See the glossary's
    //! `ADenialOfWhatThisRepositoryHolds` for why the rule is drawn at the phrase and at the
    //! path, and what each of those two decisions costs.

    use aisdlc_gates::reading_the_checkout;
    use aisdlc_gates::reading_the_glossary::{
        the_glossary_text, the_paths_the_checkout_holds, the_words_a_denial_is_made_of,
    };

    /// The documents a reader is sent to before the code, and the one the glossary reads.
    const THE_DOCUMENTS_A_READER_IS_SENT_TO: [&str; 3] =
        ["AGENTS.md", "README.md", "docs/domain-glossary.md"];

    /// Every path a claim names, given its text — backticks and a fenced block both.
    fn the_paths_a_claim_names(a_claim: &str) -> Vec<String> {
        let mut found: Vec<String> = a_claim
            .split('`')
            .skip(1)
            .step_by(2)
            .map(str::to_string)
            .collect();
        found.extend(the_paths_in_the_fenced_blocks_of(a_claim));
        found.retain(|one| !one.contains(' ') && !one.is_empty());
        found
    }

    /// The path-shaped words inside every fenced block of a claim.
    fn the_paths_in_the_fenced_blocks_of(a_claim: &str) -> Vec<String> {
        let mut found = Vec::new();
        let mut inside = false;
        for one_line in a_claim.lines() {
            if one_line.trim_start().starts_with("```") {
                inside = !inside;
                continue;
            }
            if inside {
                found.extend(one_line.split_whitespace().map(str::to_string));
            }
        }
        found
    }

    /// Every claim of a document, and a fenced block belongs to the claim above it.
    ///
    /// **A block first and a claim second, because a layout is written as a fenced block under
    /// the sentence that denies it.** A reader that ended a claim at every blank line asked the
    /// layout on its own, found no denial in it, and reported nothing — **which is a reader that
    /// looks and finds nothing, and the two are indistinguishable in the output.**
    fn the_claims_of(a_document: &str) -> Vec<String> {
        let mut found: Vec<String> = Vec::new();
        let mut one_claim = String::new();
        for one_block in the_blocks_of(a_document) {
            if !one_claim.is_empty() && one_block_is_prose(&one_block) {
                found.push(std::mem::take(&mut one_claim));
            }
            one_claim.push_str(&one_block);
            one_claim.push('\n');
        }
        found.push(one_claim);
        found.retain(|one| !one.trim().is_empty());
        found
    }

    /// Whether a block is a paragraph, **and a paragraph is what starts a claim.**
    fn one_block_is_prose(one_block: &str) -> bool {
        !one_block.trim_start().starts_with("```")
    }

    /// Every block of a document, a fenced one whole and a paragraph of its own.
    ///
    /// **A blank line ends a paragraph and does not end a fence**, because a layout block has
    /// blank lines in it and a reader that treated every blank line as the end of a block cut
    /// each layout into its lines and asked each line on its own.
    fn the_blocks_of(a_document: &str) -> Vec<String> {
        let mut found: Vec<String> = Vec::new();
        let mut one_block: Vec<&str> = Vec::new();
        let mut inside_a_fence = false;
        for one_line in a_document.lines() {
            if one_line.trim_start().starts_with("```") {
                inside_a_fence = !inside_a_fence;
            }
            if one_line_ends_a_block(one_line, inside_a_fence) {
                found.push(one_block.join("\n"));
                one_block.clear();
                continue;
            }
            one_block.push(one_line);
        }
        found.push(one_block.join("\n"));
        found.retain(|one| !one.trim().is_empty());
        found
    }

    /// Whether a line ends the block being read, which a blank line outside a fence does.
    fn one_line_ends_a_block(one_line: &str, inside_a_fence: bool) -> bool {
        !inside_a_fence && one_line.trim().is_empty()
    }

    /// The refusal for one claim, if it denies something the checkout holds.
    fn what_is_wrong_with(a_claim: &str, the_paths_the_checkout_has: &[String]) -> Option<String> {
        let denial = the_denial_in(a_claim)?;
        let every_path_it_names = the_paths_a_claim_names(a_claim);
        let named_here: Vec<&String> = every_path_it_names
            .iter()
            .filter(|one| the_paths_the_checkout_has.contains(one))
            .collect();
        if named_here.is_empty() {
            return None;
        }
        Some(format!(
            "says \"{denial}\" and names {named_here:?}, which this checkout holds"
        ))
    }

    /// The first phrase of the list this claim is written with, or nothing when it is written
    /// with none of them.
    fn the_denial_in(a_claim: &str) -> Option<String> {
        the_words_a_denial_is_made_of(&the_glossary_text())
            .into_iter()
            .find(|one| a_claim.contains(one.as_str()))
    }

    #[test]
    fn no_document_denies_what_this_checkout_holds() {
        let the_paths = the_paths_the_checkout_holds();
        let mut found: Vec<String> = Vec::new();
        for a_document in &THE_DOCUMENTS_A_READER_IS_SENT_TO {
            found.extend(the_refusals_in(a_document, &the_paths));
        }

        assert!(
            found.is_empty(),
            "a document that denies what this checkout holds is a reader sent the wrong way first, \
             and two such sentences stood in AGENTS.md while twenty-two gates were green: {found:?}"
        );
    }

    /// Every refusal in one document, with the claim it stands on.
    fn the_refusals_in(a_document: &str, the_paths: &[String]) -> Vec<String> {
        let the_text = reading_the_checkout::the_text_at(
            &reading_the_checkout::the_root_of_the_checkout().join(a_document),
        );
        the_claims_of(&the_text)
            .iter()
            .enumerate()
            .filter_map(|(at, a_claim)| {
                let why = what_is_wrong_with(a_claim, the_paths)?;
                Some(format!("{a_document} claim {at} {why}"))
            })
            .collect()
    }

    #[test]
    fn a_claim_naming_a_path_this_checkout_holds_is_refused() {
        let the_paths = the_paths_the_checkout_holds();
        let a_claim = "**None of these run yet** — there is no `Cargo.toml` in this repository.";

        assert_eq!(
            what_is_wrong_with(a_claim, &the_paths),
            Some(
                "says \"there is no\" and names [\"Cargo.toml\"], which this checkout holds"
                    .to_string()
            ),
            "and the refusal quotes the phrase and the path, because \"somewhere in that document\" \
             is not a place a reader can act on"
        );
    }

    #[test]
    fn a_claim_naming_a_path_nothing_holds_is_accepted() {
        let a_claim = "**`ARepositoryToWriteTo`** — there is no `crates/the_desk/` and this file names it as the place one would be.";

        assert_eq!(
            what_is_wrong_with(a_claim, &the_paths_the_checkout_holds()),
            None,
            "**because a document is allowed to say what is not here** — that is the one claim \
             that makes a target legible, and a rule that refused it would forbid this \
             repository from saying what it is going to be"
        );
    }

    #[test]
    fn a_denial_that_names_nothing_is_accepted_and_that_is_what_this_rule_gives_up() {
        let a_claim = "**Every rule in this file has a gate, and not one of them exists yet.**";

        assert_eq!(
            what_is_wrong_with(a_claim, &the_paths_the_checkout_holds()),
            None,
            "**because `them` is a pronoun and resolving it is reading the sentence** — and this \
             repository refuses to judge prose. **This sentence stood in AGENTS.md while \
             twenty-two gates were green, and this rule does not catch it**; it was caught by a \
             person reading it, and the glossary says so. A rule that pretended to catch it would \
             be judging meaning, and the first sentence it read wrongly would be a refusal nobody \
             could act on"
        );
    }

    #[test]
    fn a_claim_making_no_denial_is_accepted() {
        let a_claim = "**Every rule in this file has a gate, and every one of them exists.** `crates/domain/` holds the rules.";

        assert_eq!(
            what_is_wrong_with(a_claim, &the_paths_the_checkout_holds()),
            None,
            "and the same sentence with the denial taken out is accepted, which is what makes this \
             a rule about the denial and not a rule about the words"
        );
    }

    #[test]
    fn a_layout_written_as_a_fenced_block_is_one_claim_with_the_denial_above_it() {
        let a_claim = "**none of it exists yet.**\n\n```text\ncrates/\n  domain/\n  real/\n```";

        let the_refusal = what_is_wrong_with(a_claim, &the_paths_the_checkout_holds());

        assert_eq!(
            the_refusal,
            Some(
                "says \"none of it exists\" and names [\"crates/\"], which this checkout holds"
                    .to_string()
            ),
            "**because a layout is written as a block under the sentence that denies it** — a \
             reader that split on blank lines asked the block on its own and found no denial in \
             it, which is a reader that looks and reports nothing. **And `real/` is absent from \
             the refusal while `crates/` is in it**, because that directory has not been made and \
             a target this repository has written down is not a denial: the two are the same list \
             and only one of them is a claim"
        );
    }

    #[test]
    fn the_denials_were_actually_read() {
        let the_denials = the_words_a_denial_is_made_of(&the_glossary_text());

        assert_eq!(
            the_denials,
            vec![
                "there is no",
                "not one of them exists",
                "none of it exists",
                "none of these run",
                "do not exist",
                "does not exist"
            ],
            "**because a gate that reads an empty list passes everything**, and six is the number \
             the three sentences in AGENTS.md were written with — so a phrase dropped from the \
             glossary is a sentence that rots with nothing watching it"
        );
    }
}
