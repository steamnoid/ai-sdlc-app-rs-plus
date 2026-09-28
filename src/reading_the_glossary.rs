//! Every reading of `docs/domain-glossary.md` that a gate needs, in one place.
//!
//! **Two gates read the glossary and two copies would be two lists waiting to disagree.**
//! The sibling repository had four copies of a walk and the copies had already begun to
//! differ. Every function here takes the glossary's text, so a caller reads it once.
//!
//! **Each reader below was wrong at least once, and the reason is on the function.**

use std::path::Path;

use crate::reading_the_checkout;

/// The glossary's own text, read from the checkout.
pub fn the_glossary_text() -> String {
    let the_path = reading_the_checkout::the_root_of_the_checkout().join("docs/domain-glossary.md");
    reading_the_checkout::the_text_at(Path::new(&the_path))
}

/// A name or a word as letters and digits alone, so two spellings are one name.
pub fn it_as_letters(one: &str) -> String {
    one.chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_lowercase()
}

/// The last word of a name, whatever spelling the name is written in.
pub fn the_last_word_of(one_name: &str) -> String {
    let words = the_words_of(one_name);
    words.last().cloned().unwrap_or_default()
}

/// Every word of a name, in either spelling.
pub fn the_words_of(one_name: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let mut one_word = String::new();
    for one in one_name.chars() {
        if one == '_' {
            found.push(one_word.to_lowercase());
            one_word.clear();
            continue;
        }
        if one.is_uppercase() && !one_word.is_empty() {
            found.push(one_word.to_lowercase());
            one_word.clear();
        }
        one_word.push(one);
    }
    found.push(one_word.to_lowercase());
    found.retain(|one| !one.is_empty());
    found
}

/// The fenced block that holds a list, found by a word every such list contains.
///
/// **The word and not a line prefix.** A reader that took the lines starting with `Manager`
/// missed a word appended on a line of its own — and a word appended on its own line is
/// exactly how this glossary's list grew.
pub fn the_block_containing(the_glossary: &str, the_word: &str) -> String {
    let mut collecting = false;
    let mut the_block = Vec::new();
    for one_line in the_glossary.lines() {
        if !collecting {
            collecting = one_line.contains(the_word);
            continue;
        }
        if one_line.trim_start().starts_with("```") {
            break;
        }
        the_block.push(one_line);
    }
    the_block.join(" ")
}

/// The glossary's own line carrying a heading, or nothing.
pub fn the_line_under(the_glossary: &str, the_heading: &str) -> String {
    the_glossary
        .lines()
        .find(|one| one.contains(the_heading))
        .unwrap_or_default()
        .to_string()
}

/// The first word the heading's own line writes in backticks, or nothing.
///
/// **The heading's line and not the one after it.** A reader that skipped the line it had
/// found read the paragraph beneath and found no backticks at all, which it reported as an
/// empty list rather than as a mistake.
pub fn the_word_under(the_glossary: &str, the_heading: &str) -> Option<String> {
    let the_line = the_line_under(the_glossary, the_heading);
    the_line
        .split('`')
        .nth(1)
        .map(it_as_letters)
        .filter(|it| !it.is_empty())
}

/// Every table in the glossary whose header names a `Not` column.
pub fn the_tables_naming_what_they_refuse(the_glossary: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut one_table: Vec<String> = Vec::new();
    for one_line in the_glossary.lines() {
        if one_line.trim_start().starts_with('|') {
            one_table.push(one_line.to_string());
            continue;
        }
        found.extend(the_table_if_it_refuses(&one_table));
        one_table.clear();
    }
    found.extend(the_table_if_it_refuses(&one_table));
    found
}

/// The table, if its header names a `Not` column.
fn the_table_if_it_refuses(one_table: &[String]) -> Option<String> {
    let has_a_not_column = one_table.first()?.contains("Not");
    if !has_a_not_column {
        return None;
    }
    Some(one_table.join("\n"))
}

/// The words in a table's `Not` column, as the glossary spells them.
///
/// **The index comes from the header and is used for every row.** The first version asked
/// each row where its `Not` cell was, and a data row has no cell containing the word `Not` at
/// all — so the reader found three tables and read nothing out of any of them.
pub fn the_words_in_the_not_column_of(a_table: &str) -> Vec<String> {
    let header = match a_table.lines().next() {
        Some(one) => one.to_string(),
        None => return Vec::new(),
    };
    let Some(where_not) = header.split('|').position(|one| one.contains("Not")) else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for (index, one_line) in a_table.lines().enumerate() {
        if index == 0 {
            continue;
        }
        found.extend(the_backticked_words_in_cell(one_line, where_not));
    }
    found
}

/// The backticked words in one cell of one row.
fn the_backticked_words_in_cell(one_line: &str, where_the_cell_is: usize) -> Vec<String> {
    let cells: Vec<&str> = one_line.split('|').collect();
    let Some(the_cell) = cells.get(where_the_cell_is) else {
        return Vec::new();
    };
    the_cell
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// Every word a `Not` column refuses, wherever a name carries it.
///
/// **As the glossary spells them**, because a refusal quotes the document and a reader
/// looking for `Status` in a message that says `status` has to go and find the rule.
pub fn the_words_refused_anywhere(the_glossary: &str) -> Vec<String> {
    let mut found = Vec::new();
    for the_table in the_tables_naming_what_they_refuse(the_glossary) {
        found.extend(the_words_in_the_not_column_of(&the_table));
    }
    found.sort();
    found.dedup();
    found
}

/// The same words as letters and digits alone, for comparing.
pub fn the_refused_as_letters(the_glossary: &str) -> Vec<String> {
    let mut found: Vec<String> = the_words_refused_anywhere(the_glossary)
        .iter()
        .map(|one| it_as_letters(one))
        .collect();
    found.sort();
    found.dedup();
    found
}

/// How the glossary itself writes a word, so a refusal quotes the document.
pub fn the_glossary_spelling_of(the_glossary: &str, a_word: &str) -> Option<String> {
    the_words_refused_anywhere(the_glossary)
        .into_iter()
        .find(|one| it_as_letters(one) == a_word)
}

/// The words the glossary refuses only when a name **is** that word.
///
/// **Read between the fences, not up to the first one.** The first version stopped at the
/// opening fence and returned nothing, having found the block it was looking for and never
/// looked inside it.
pub fn the_words_refused_only_as_whole_names(the_glossary: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut looking = false;
    let mut inside = false;
    for one_line in the_glossary.lines() {
        if !looking {
            looking = one_line.contains("ordinary English");
            continue;
        }
        if !inside && one_line.trim_start().starts_with("```") {
            inside = true;
            continue;
        }
        if inside && one_line.trim_start().starts_with("```") {
            break;
        }
        if inside {
            found.extend(one_line.split_whitespace().map(str::to_lowercase));
        }
    }
    found
}

/// The words the generic-vocabulary gate refuses, from the block that holds them.
pub fn the_words_the_glossary_refuses(the_glossary: &str) -> Vec<String> {
    the_block_containing(the_glossary, "Manager")
        .split_whitespace()
        .filter(|word| !word.starts_with('`') && *word != "<Domain>")
        .map(str::to_string)
        .collect()
}

/// Every word the glossary blesses by naming it in backticks.
///
/// **A backtick in a `Not` column is not a blessing.** `Status` is written in backticks in
/// the stage table and is the word that table refuses, so a reader that took every
/// backticked word as blessed held the whole refused set on the blessed side and the gate
/// refused nothing at all. The glossary says the same thing about itself: a backtick is a
/// blessing whatever the sentence around it intends, which is why the `Not` words are taken
/// out by hand.
pub fn the_blessed_names(the_glossary: &str) -> Vec<String> {
    let not_a_blessing = the_refused_as_letters(the_glossary);
    let mut found = Vec::new();
    for word in the_glossary.split('`').skip(1).step_by(2) {
        let one = it_as_letters(word);
        if not_a_blessing.contains(&one) {
            continue;
        }
        found.push(one);
    }
    found.sort();
    found.dedup();
    found
}
