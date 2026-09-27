//! The gate for RULE 1.7: a name is never generic infrastructure vocabulary.
//!
//! It reads the list out of `docs/domain-glossary.md` rather than restating it, so the
//! guard follows the document instead of drifting from it. A name the glossary blesses
//! is allowed whatever its last word is, which is what lets `WorkItem` exist while
//! `item` sits on the list.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// The kinds of declaration a name may be read off. `pub use` is not one of them,
/// because a name brought in from elsewhere was declared somewhere this gate does not
/// read, and a gate that judges another repository's names is judging the wrong thing.
const THE_DECLARATION_KEYWORDS: [&str; 7] =
    ["struct", "enum", "trait", "type", "fn", "mod", "const"];

/// Letters and digits, lowercased, and nothing else. The comparison a name is judged by
/// is a comparison of shapes rather than of spellings, so `work_item`, `workItem` and
/// `Work Item` are one name to this guard and the guard says so in its failures.
fn normalised(text: &str) -> String {
    text.chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// The words an identifier is spoken in: split on an underscore, and on a letter that
/// follows a lowercase one, which is the two breaks `work_item_id` and `SessionManager`
/// between them both need. `ADataThing` is one spoken word here and not three, so the
/// noun inside it is out of this guard's reach; that is a property of this rule and it
/// is written down rather than pretended away. `UseCase` is on the list and is two
/// spoken words, so it is never the last one of anything and is caught as a family or
/// not at all, which is the same fact seen from the other side.
fn words_of(identifier: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut previous: Option<char> = None;
    for character in identifier.chars() {
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

/// Whether this character opens a word rather than continuing the one before it. It
/// exists so `words_of` reads as the two rules it enforces rather than as a loop.
fn begins_a_word(character: char, previous: Option<char>) -> bool {
    character == '_' || (character.is_uppercase() && previous.is_some_and(char::is_lowercase))
}

/// The last word of the name, which is what the name claims the thing **is**. It is the
/// last word rather than any word because `SessionManager` is about a manager whatever
/// else it is, and `work_item_id` is about an id.
fn head_noun(identifier: &str) -> String {
    match words_of(identifier).last() {
        Some(word) => word.clone(),
        None => String::new(),
    }
}

/// Whether the glossary blesses this whole name, which is the exemption that lets
/// `WorkItem` exist while `item` sits on the list. It is asked before either of the other
/// two questions, because a name the document blesses is a name this rule cannot refuse.
fn the_glossary_blesses(identifier: &str) -> bool {
    the_blessed_names().contains(&normalised(identifier))
}

/// Whether the name ends on a noun the list refuses, which is the claim the last word of
/// an identifier makes about what the thing is.
fn the_name_ends_on_a_banned_noun(identifier: &str) -> bool {
    banned_nouns().contains(&head_noun(identifier))
}

/// Whether the name is built on a family the list refuses. The family is looked for in the
/// first spoken word and not in any word, so `BaseRepository` is refused and `database`,
/// which merely holds the letters, is not.
fn the_name_is_built_on_a_banned_family(identifier: &str) -> bool {
    words_of(identifier)
        .first()
        .is_some_and(|first| banned_families().contains(first))
}

/// The judgement, in the order the questions are asked: the document's blessing, then the
/// last word, then the family the name is built on. A name that answers no to all three
/// is one the glossary allows, and the three questions are the whole rule rather than
/// three samples of it.
fn is_this_name_allowed(identifier: &str) -> bool {
    if the_glossary_blesses(identifier) {
        return true;
    }
    if the_name_ends_on_a_banned_noun(identifier) {
        return false;
    }
    !the_name_is_built_on_a_banned_family(identifier)
}

/// The glossary as it stands on disk, read on every call so an edit to the list is an
/// edit to this gate, with nothing re-declared here to fall behind.
fn the_glossary_text() -> String {
    let path = the_path_to("docs/domain-glossary.md");
    match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(problem) => panic!(
            "the glossary at {} is unreadable: {problem}",
            path.display()
        ),
    }
}

/// Where these tests run from: the manifest of the crate that holds them when cargo
/// built this file, and the working directory when it did not. Both are read rather
/// than assumed, so the gate finds the document from wherever it was compiled.
fn the_directory_the_tests_run_from() -> PathBuf {
    if let Some(manifest_directory) = option_env!("CARGO_MANIFEST_DIR") {
        return PathBuf::from(manifest_directory);
    }
    match std::env::current_dir() {
        Ok(here) => here,
        Err(problem) => panic!("there is no directory to look for the glossary in: {problem}"),
    }
}

/// The path of the first entry named `wanted` at or above this directory, so the gate
/// finds the document and the crates whichever crate the test target itself lives in.
/// A gate that silently read nothing would be green, so this refuses rather than
/// carrying on.
fn the_path_to(wanted: &str) -> PathBuf {
    let mut directory = the_directory_the_tests_run_from();
    loop {
        if directory.join(wanted).exists() {
            return directory.join(wanted);
        }
        match directory.parent() {
            Some(above) => directory = above.to_path_buf(),
            None => panic!(
                "no {wanted} above {}",
                the_directory_the_tests_run_from().display()
            ),
        }
    }
}

/// The `## Banned vocabulary` section on its own, which is where the list is written.
fn the_banned_vocabulary_section(glossary: &str) -> String {
    let mut inside = false;
    let mut section = String::new();
    for line in glossary.lines() {
        if line.starts_with("## Banned vocabulary") {
            inside = true;
            continue;
        }
        if !inside {
            continue;
        }
        if line.starts_with("## ") {
            break;
        }
        section.push_str(line);
        section.push('\n');
    }
    section
}

/// The prose under `## Banned vocabulary` is about the list rather than being the list,
/// so only the fenced block counts. Reading the prose as entries would make `Status` a
/// banned noun because the document explains that `Status` used to be called `Stage`.
fn the_fenced_text_block(section: &str) -> String {
    let mut inside = false;
    let mut block = String::new();
    for line in section.lines() {
        if line.trim() == "```text" {
            inside = true;
            continue;
        }
        if inside && line.trim().starts_with("```") {
            break;
        }
        if inside {
            block.push_str(line);
            block.push(' ');
        }
    }
    block
}

/// The entries of the list as they are written, one token each. A token carrying a `<`
/// is a family and every other token is a noun, which is how `Base<Domain>` and `Temp0`
/// are both read without either being mistaken for a word of its own kind.
fn the_banned_entries() -> Vec<String> {
    the_fenced_text_block(&the_banned_vocabulary_section(&the_glossary_text()))
        .split_whitespace()
        .map(String::from)
        .collect()
}

/// The generic nouns, lowercased. `item` is on this list, and `WorkItem` is on the
/// blessed list, and that collision is the reason a blessing is checked first.
fn banned_nouns() -> Vec<String> {
    the_banned_entries()
        .into_iter()
        .filter(|entry| !entry.contains('<'))
        .map(|entry| normalised(&entry))
        .collect()
}

/// The families, lowercased. A family is matched as the first spoken word, so
/// `BaseRepository` is refused while `database`, which merely holds the letters, is
/// not: a word that contains a banned one is not a name that ends on it.
fn banned_families() -> Vec<String> {
    the_banned_entries()
        .into_iter()
        .filter(|entry| entry.contains('<'))
        .map(|entry| normalised(entry.split('<').next().unwrap_or_default()))
        .collect()
}

/// The document with the `## Banned vocabulary` section taken out, which is the only
/// form the blessed names are read from. That removal is not optional: a prohibition
/// read as a term is exactly backwards, and an earlier version of this very guard proved
/// it by passing the name it existed to catch.
fn the_document_with_the_banned_section_removed(glossary: &str) -> String {
    let mut keeping = true;
    let mut remaining = String::new();
    for line in glossary.lines() {
        if line.starts_with("## Banned vocabulary") {
            keeping = false;
            continue;
        }
        if keeping && line.starts_with("## ") {
            keeping = true;
        }
        if keeping {
            remaining.push_str(line);
            remaining.push('\n');
        }
    }
    remaining
}

/// The document with its fenced blocks taken out, so the backticks that open a block are
/// not read as a code span and the words inside a block are not read as blessed names.
fn the_document_with_its_fenced_blocks_removed(text: &str) -> String {
    let mut inside = false;
    let mut remaining = String::new();
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            inside = !inside;
            continue;
        }
        if !inside {
            remaining.push_str(line);
            remaining.push('\n');
        }
    }
    remaining
}

/// Every bolded entry, read between the first and second `**` and no further, because
/// a bolded phrase is a thing the document chose to say rather than a name it declares.
fn bolded_entries(text: &str) -> Vec<String> {
    text.replace("**", "\u{1}")
        .split('\u{1}')
        .skip(1)
        .step_by(2)
        .map(String::from)
        .collect()
}

/// Every code span, which is where the glossary writes a name it blesses.
fn code_spans(text: &str) -> Vec<String> {
    the_document_with_its_fenced_blocks_removed(text)
        .split('`')
        .skip(1)
        .step_by(2)
        .map(String::from)
        .collect()
}

/// The names the glossary blesses, lowercased and normalised. A name is allowed when it
/// is one of these whatever its last word is, which is what lets `WorkItem` and `Stage`
/// exist in a repository that bans `item` and once banned `state`.
fn the_blessed_names() -> Vec<String> {
    let glossary = the_glossary_text();
    let without_the_list = the_document_with_the_banned_section_removed(&glossary);
    let mut names = bolded_entries(&without_the_list);
    names.extend(code_spans(&without_the_list));
    names.iter().map(|name| normalised(name)).collect()
}

/// Every Rust file this workspace is made of, found by walking down from the root: the
/// crates, and the gates' own library. The library is in the walk because a gate is code
/// in this repository too, and a rule applied only to other people's code is a rule about
/// other people.
fn every_rust_file_in_the_workspace() -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    for directory in ["crates", "src"] {
        collect_rust_files(&the_path_to(directory), &mut found);
    }
    found
}

/// One directory's worth of the walk. A directory this gate cannot open is skipped rather
/// than fatal, because a gate that stops on an unreadable directory judges nothing at
/// all and says so by passing.
fn collect_rust_files(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, found);
        } else if path.extension().is_some_and(|ending| ending == "rs") {
            found.push(path);
        }
    }
}

/// The names every `pub` declaration in one file claims to be. Only `pub` counts, and
/// only the seven kinds a type, a module or a constant is declared with.
fn public_names_in(text: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for line in text.lines() {
        collect_a_public_name(line, &mut names);
    }
    names
}

/// One line's worth: every `pub` on it, and for each of them the name the keyword after
/// it introduces. `pub(crate) async fn` and `pub struct WorkItem<T>` are both one name.
fn collect_a_public_name(line: &str, names: &mut Vec<String>) {
    for at in the_offsets_of_pub(line) {
        let after = &line[at + 3..];
        if let Some(name) = the_name_after_a_declaration_keyword(after) {
            names.push(name);
        }
    }
}

/// Where each `pub` on this line begins. A `pub` inside a longer word is not one, which
/// is why `publisher` is not read as a declaration.
fn the_offsets_of_pub(line: &str) -> Vec<usize> {
    let mut offsets: Vec<usize> = Vec::new();
    let mut from = 0;
    while let Some(at) = line[from..].find("pub") {
        let offset = from + at;
        if opens_a_declaration(&line[offset + 3..]) {
            offsets.push(offset);
        }
        from = offset + 3;
    }
    offsets
}

/// Whether the text after a `pub` begins a declaration, which it does with a space for
/// `pub fn` and with a bracket for `pub(crate) fn`, and otherwise with a letter.
fn opens_a_declaration(after: &str) -> bool {
    match after.chars().next() {
        Some('(') => true,
        Some(character) => character.is_whitespace(),
        None => false,
    }
}

/// The name the first declaration keyword after this `pub` introduces. The keyword is
/// looked for anywhere ahead rather than in the next place, so a modifier between `pub`
/// and the name costs nothing.
fn the_name_after_a_declaration_keyword(after: &str) -> Option<String> {
    let past_the_keyword = the_first_declaration_keyword(after)?;
    let named = after[past_the_keyword..].trim_start();
    let name: String = named
        .chars()
        .take_while(|character| character.is_alphanumeric() || *character == '_')
        .collect();
    if name.is_empty() {
        return None;
    }
    Some(name)
}

/// Where the earliest of the seven keywords comes to an end, if one of them is here at
/// all. The end rather than the beginning, because the name begins where the keyword
/// stops.
fn the_first_declaration_keyword(text: &str) -> Option<usize> {
    THE_DECLARATION_KEYWORDS
        .iter()
        .filter_map(|keyword| text.find(keyword).map(|at| at + keyword.len()))
        .min()
}

/// A name the glossary does not allow, and the file it was read out of. One refusal
/// rather than a bare string, because a failure that cannot name the file to open is a
/// failure nobody can act on.
struct ANameRefused {
    /// The file the name was declared in.
    file: PathBuf,
    /// The name as it was written there.
    name: String,
}

/// The names in the workspace this gate refuses, each with the file it came from.
fn names_refused_in_the_workspace() -> Vec<ANameRefused> {
    let mut refused: Vec<ANameRefused> = Vec::new();
    for path in every_rust_file_in_the_workspace() {
        refused.extend(the_names_refused_in(&path));
    }
    refused
}

/// One file's worth, judged name by name.
fn the_names_refused_in(path: &Path) -> Vec<ANameRefused> {
    let text = fs::read_to_string(path).unwrap_or_default();
    public_names_in(&text)
        .into_iter()
        .filter(|name| !is_this_name_allowed(name))
        .map(|name| ANameRefused {
            file: path.to_path_buf(),
            name,
        })
        .collect()
}

/// What a refusal says when there is one, and what it says when there is none, because
/// a guard that reported nothing on a breach and nothing on a pass is indistinguishable
/// from a guard that read nothing at all.
fn what_a_refused_name_says(refused: &[ANameRefused]) -> String {
    if refused.is_empty() {
        return String::from("every public name in this workspace is one the glossary allows");
    }
    let mut said =
        String::from("a name is a claim about what the thing is, and this is not one:\n");
    for one in refused {
        writeln!(said, "{}: {}", one.file.display(), one.name)
            .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

#[test]
fn the_glossary_offers_more_than_twenty_five_banned_entries() {
    let entries = the_banned_entries();
    assert!(
        entries.len() > 25,
        "the guard read {} entries, and a list that short is a list it did not read",
        entries.len()
    );
}

#[test]
fn the_glossary_blesses_work_item() {
    let blessed = the_blessed_names();
    assert!(
        blessed.contains(&normalised("WorkItem")),
        "WorkItem is bolded in the glossary, and a gate that cannot see it is judging the wrong document"
    );
}

#[test]
fn session_manager_is_refused() {
    assert!(
        !is_this_name_allowed("SessionManager"),
        "SessionManager ends on `manager`, and a name ending on a banned noun is refused"
    );
}

#[test]
fn work_item_manager_is_refused() {
    assert!(
        !is_this_name_allowed("work_item_manager"),
        "an underscored name is spoken in the same words, and ends on `manager`"
    );
}

#[test]
fn a_helper_is_refused() {
    assert!(
        !is_this_name_allowed("a_helper"),
        "a helper says what was written rather than what the thing is"
    );
}

#[test]
fn some_data_is_refused() {
    assert!(
        !is_this_name_allowed("SomeData"),
        "a word may be shouted and it is still the same word"
    );
}

#[test]
fn load_payload_is_refused() {
    assert!(
        !is_this_name_allowed("load_payload"),
        "the last word is the one that says what the thing is, and it is `payload`"
    );
}

#[test]
fn whatever_is_refused() {
    assert!(
        !is_this_name_allowed("Whatever"),
        "a name that admits it means nothing cannot claim to be a name"
    );
}

#[test]
fn base_repository_is_refused() {
    assert!(
        !is_this_name_allowed("BaseRepository"),
        "`Base<Domain>` is a family, and a family is matched as the first spoken word"
    );
}

#[test]
fn database_is_allowed_because_a_family_is_a_prefix() {
    assert!(
        is_this_name_allowed("database"),
        "`base` is a prefix of a name and not a word inside one, so `database` is not a `Base`"
    );
}

#[test]
fn model_port_is_refused() {
    assert!(
        !is_this_name_allowed("ModelPort"),
        "a port is `AReasoningModel` or `TheProjectOnDisk`, and this one names the pattern"
    );
}

#[test]
fn use_case_service_is_refused() {
    assert!(
        !is_this_name_allowed("UseCaseService"),
        "every one of its three words is generic, and the last one is the claim"
    );
}

#[test]
fn the_clock_factory_is_refused() {
    assert!(
        !is_this_name_allowed("the_clock_factory"),
        "a factory is `AClonedRepository` or `standing_at`, and this one names the pattern"
    );
}

#[test]
fn temp0_is_refused() {
    assert!(
        !is_this_name_allowed("temp0"),
        "a banned entry is a banned entry whether or not it holds a digit"
    );
}

#[test]
fn work_item_is_allowed() {
    assert!(
        is_this_name_allowed("WorkItem"),
        "WorkItem ends on `item`, which is on the list, and the glossary blesses the whole name"
    );
}

#[test]
fn stage_is_allowed() {
    assert!(
        is_this_name_allowed("Stage"),
        "`Stage` is the one word for the stage of a work item"
    );
}

#[test]
fn role_is_allowed() {
    assert!(
        is_this_name_allowed("Role"),
        "`Role` is the discipline accountable for the work"
    );
}

#[test]
fn repository_is_allowed() {
    assert!(
        is_this_name_allowed("Repository"),
        "the glossary blesses the word, and the fenced list does not name it at all"
    );
}

#[test]
fn a_reasoning_model_is_allowed() {
    assert!(
        is_this_name_allowed("AReasoningModel"),
        "a port is named after what it reaches, and this one is what reaches a model"
    );
}

#[test]
fn the_project_on_disk_is_allowed() {
    assert!(
        is_this_name_allowed("TheProjectOnDisk"),
        "a port named after where it reads is a domain name"
    );
}

#[test]
fn what_a_model_said_is_allowed() {
    assert!(
        is_this_name_allowed("WhatAModelSaid"),
        "the glossary says what it is, and `WhatAModelSaid` says what happened"
    );
}

#[test]
fn legal_transitions_from_is_allowed() {
    assert!(
        is_this_name_allowed("legal_transitions_from"),
        "the verb is `transition`, and this name ends on the table's own edge"
    );
}

#[test]
fn an_inspection_of_is_allowed() {
    assert!(
        is_this_name_allowed("an_inspection_of"),
        "a name may end on a preposition when that is where the sentence stops"
    );
}

#[test]
fn a_run_of_this_system_is_allowed() {
    assert!(
        is_this_name_allowed("a_run_of_this_system"),
        "naming the system a run belongs to is not generic vocabulary"
    );
}

#[test]
fn a_declaration_a_later_change_would_add_is_found() {
    let text = "pub struct SessionManager {\n    sessions: u8,\n}\n";
    assert_eq!(
        public_names_in(text),
        vec![String::from("SessionManager")],
        "the walk must see a declaration, or a scan of crates/ finds nothing and says nothing"
    );
}

#[test]
fn a_camel_case_declaration_is_found() {
    let text = "pub struct WhatAModelSaid(pub u8);\n";
    assert_eq!(
        public_names_in(text),
        vec![String::from("WhatAModelSaid")],
        "a type the domain will actually write must be found by the walk"
    );
}

#[test]
fn a_declaration_without_pub_is_left_alone() {
    let text = "struct SessionManager {\n    sessions: u8,\n}\n";
    assert!(
        public_names_in(text).is_empty(),
        "a private declaration is the implementation's own business"
    );
}

#[test]
fn a_use_declaration_is_left_alone() {
    let text = "pub use crate::domain::WorkItem;\n";
    assert!(
        public_names_in(text).is_empty(),
        "a name brought in from elsewhere was declared where this gate does not read"
    );
}

#[test]
fn no_public_name_in_the_workspace_uses_banned_vocabulary() {
    let files = every_rust_file_in_the_workspace();
    assert!(
        !files.is_empty(),
        "the walk read no file at all, which is a pass this gate has not earned"
    );
    let refused = names_refused_in_the_workspace();
    assert!(refused.is_empty(), "{}", what_a_refused_name_says(&refused));
}
