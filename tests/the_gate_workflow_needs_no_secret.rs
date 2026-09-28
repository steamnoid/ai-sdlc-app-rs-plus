//! The gate for the other end of RULE 0.4: the gate workflow declares no secret.
//!
//! `no_test_reads_an_environment_variable` proves the suite needs no credential at run
//! time. This proves the other half: that the job which runs the suite asks for none
//! either. **They are one rule seen from two ends, and either alone is half a claim.** A
//! suite that reads no credential while the workflow hands it one has a gate that is
//! satisfied by a suite nobody bothers to satisfy, and a workflow that declares no secret
//! while a test reads one has proved only that the secret was never used — which is the
//! same as proving nothing at all.
//!
//! ## The workflow is read as text, and that is a decision with a price
//!
//! **The scanner in `src/scanning.rs` does not read this file and does not try.** It reads
//! Rust: it tells a `//` from two slashes inside a string, and it tells a lifetime from a
//! character. A workflow is YAML, and none of those five kinds means anything in it. So
//! the file is read as text and as text it is judged, and **a text reading is not a YAML
//! parser.** Everything below is the whole of what is understood: a line, a colon, a key
//! and how far in it is written.
//!
//! Four things that costs, named rather than discovered:
//!
//! - **A `secrets` written in a form this search misses.** The rule is the text
//!   `secrets.` and nothing else, and GitHub also accepts a bracketed form — `secrets
//!   ['A_NAME']` — which is not caught. A line whose `secrets` carries no dot at all, and
//!   a `#` comment mentioning the word at all, are also not caught. **That last one is
//!   deliberate rather than missed, and this repository's own workflow holds one.** The
//!   existing comment says the job has no `secrets` of any kind, and a rule drawn at the
//!   word would refuse the sentence that records the rule. A gate that cannot be written
//!   about is a gate its next finding is not read from.
//! - **An `env:` written on one line.** `env: { A_NAME: a_value }` is a block of
//!   environment variables and is not one to this reading, which follows an `env:` on a
//!   line of its own and the lines written further in than it.
//! - **A name this gate does not expect.** `A_PORT` with the password written into it is
//!   a credential and is not a name any of the eight words below is in, because there is
//!   no way to know from the name that it is one. The words are blunt and the scope is
//!   narrow, which is the only reason being blunt is safe here.
//! - **A tab, an anchor and an alias.** The block of a key is found by comparing how far
//!   each line is written in, and a file indented with tabs answers that question
//!   differently. `env: *the_credentials` is a block of environment variables this reading
//!   does not open, and a workflow can name its secrets in a way a line-searching gate has
//!   no way to follow.
//!
//! ## What a refusal says, and what is asked first
//!
//! The line it is written on and the line as the file wrote it, for the same reason every
//! other gate in this repository names both: a gate that says "something is wrong
//! somewhere" is a gate nobody can act on.
//!
//! **The file is asked what it is before it is asked what it holds.** A workflow that
//! failed to parse and a workflow with no secret in it look the same to a reading that
//! read nothing, and a guard that agreed with a file it never read is the failure this
//! repository exists to prevent.

use std::fmt::Write as _;
use std::path::PathBuf;

use aisdlc_gates::scanning::{self};

/// The text a GitHub workflow names a stored credential with, and the only shape of it
/// this gate refuses.
///
/// The dot is in the rule rather than the word, and the reason is above: a `#` comment
/// naming the word is this repository recording the rule, and a rule that reads its own
/// documentation as a breach is a rule whose next finding nobody believes. The cost is
/// named in the same place — a bracketed `secrets['A_NAME']` carries no dot.
const THE_PATH_INTO_A_STORED_CREDENTIAL: &str = "secrets.";

/// The key that opens a block of environment variables, written on a line of its own.
const THE_KEY_THAT_OPENS_A_BLOCK_OF_ENVIRONMENT_VARIABLES: &str = "env:";

/// The words a credential is called by, in whatever the deployment calls it.
///
/// **`A_NAME` is a name nobody invented to be here.** `GITHUB_TOKEN`, `OPENAI_API_KEY`,
/// `AWS_SECRET_ACCESS_KEY` and `CARGO_REGISTRIES_CRATES_IO_TOKEN` all hold one of the
/// eight, and those are the four a run of this system would reach for. `KEY` is the
/// bluntest of them and the reason the scope is a block of environment variables and not
/// a file: the cache step of this repository's own workflow writes `key:`, and a rule
/// drawn at the file would refuse it.
const THE_WORDS_A_CREDENTIAL_IS_NAMED_WITH: [&str; 8] = [
    "TOKEN",
    "SECRET",
    "PASSWORD",
    "PASSWD",
    "CREDENTIAL",
    "API_KEY",
    "PRIVATE_KEY",
    "KEY",
];

/// One secret the workflow declares, with the line it stands on and the line as the file
/// wrote it.
#[derive(Debug)]
struct ADeclaredSecret {
    /// The line the declaration is written on, counted from 1.
    the_line_it_is_written_on: usize,
    /// The whole line, exactly as the file has it.
    the_line_as_it_is_written: String,
}

/// The workflow this gate reads, which is the one that runs the suite.
///
/// The path is built from the root of the checkout rather than written out, because the
/// tests do not run from the root of the checkout and a workflow named as a relative path
/// is a path that is right on one machine.
fn the_path_to_the_gate_workflow() -> PathBuf {
    scanning::the_root_of_the_checkout()
        .join(".github")
        .join("workflows")
        .join("gate.yml")
}

/// The workflow this gate reads, as text.
///
/// The shared reader is used rather than a copy of it, because a second read that panics
/// differently from the first is two readers already disagreeing, and
/// `a_name_the_gates_share_has_one_implementation` is right about that.
fn the_text_of_the_gate_workflow() -> String {
    scanning::the_text_at(&the_path_to_the_gate_workflow())
}

/// Every secret `workflow` declares, in the order the file writes them.
///
/// **One pass over the lines, and both questions asked of each of them.** Two separate
/// readings would report their findings in the order they happened to find them, and a
/// refusal listing a workflow's lines out of their own order is a list a reader has to
/// put back together before it can be acted on.
fn the_declared_secrets_in(workflow: &str) -> Vec<ADeclaredSecret> {
    let credential_lines = the_lines_declaring_a_credential(workflow);
    let mut declared = Vec::new();
    for (at, line) in workflow.lines().enumerate() {
        if this_line_declares_a_secret(line, at + 1, &credential_lines) {
            declared.push(ADeclaredSecret {
                the_line_it_is_written_on: at + 1,
                the_line_as_it_is_written: line.to_string(),
            });
        }
    }
    declared
}

/// Whether this line declares a secret: either it names the stored credentials, or it sets
/// a variable inside a block of environment variables to a name a credential is called by.
///
/// The two are asked together so that the refusal reports each line once, in the order the
/// file wrote it, rather than in the order two separate readings happened to find them.
fn this_line_declares_a_secret(line: &str, on_line: usize, credential_lines: &[usize]) -> bool {
    line.contains(THE_PATH_INTO_A_STORED_CREDENTIAL) || credential_lines.contains(&on_line)
}

/// The line numbers of every line setting a variable inside a block of environment
/// variables to a name a credential is called by.
fn the_lines_declaring_a_credential(workflow: &str) -> Vec<usize> {
    every_line_inside_a_block_of_environment_variables(workflow)
        .iter()
        .filter(|(_, written)| this_line_names_a_credential(written))
        .map(|(on_line, _)| *on_line)
        .collect()
}

/// Every line standing inside a block of environment variables, each with the line it is
/// written on.
///
/// **The block of a key is the lines written further in than the key, and that is the
/// whole of the structure this reading has.** The one place the workflow's shape matters
/// is a place it is not understood: a line at the same distance is the next key and not
/// the last line of this one, and a line written further out is a child whatever it
/// names.
fn every_line_inside_a_block_of_environment_variables(workflow: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = workflow.lines().collect();
    let mut inside = Vec::new();
    for (at, line) in lines.iter().enumerate() {
        if let Some(how_far) = where_a_block_of_environment_variables_opens(line) {
            inside.extend(the_lines_written_further_in(&lines, at + 1, how_far));
        }
    }
    inside
}

/// How far in `line` the first thing written on it stands, and nothing at all for a line
/// with nothing written on it.
///
/// A line holding nothing is not a child and is not a closing of a block either: a blank
/// line and a comment standing in the margin both leave the block they are written beside
/// open, and a block that ended on a blank line would end on the first of the two.
fn how_far_a_line_is_indented(line: &str) -> Option<usize> {
    let written = line.trim_start();
    if written.is_empty() {
        return None;
    }
    Some(line.len() - written.len())
}

/// How far in this line an `env:` that opens a block of environment variables stands, and
/// nothing at all when this line opens no such block.
///
/// **The two questions are asked by one function because the second answers the first.**
/// A key that opens nothing has no indentation to compare a child against, so asking for
/// the indentation of a line that opens no block would return a number this reading has no
/// standing to use.
fn where_a_block_of_environment_variables_opens(line: &str) -> Option<usize> {
    if line.trim() != THE_KEY_THAT_OPENS_A_BLOCK_OF_ENVIRONMENT_VARIABLES {
        return None;
    }
    how_far_a_line_is_indented(line)
}

/// The lines from `the_line_after_the_key` onwards that are written further in than
/// `how_far`, each with the line it is written on, and nothing at all once a line is
/// written no further in than that.
///
/// **The key's own line is not among the lines looked for, and that is why the walk
/// starts one line on.** A key is written exactly as far in as its own threshold, so a
/// walk that started on it would stop on it and find every block of environment variables
/// in the repository to be empty.
fn the_lines_written_further_in(
    lines: &[&str],
    the_line_after_the_key: usize,
    how_far: usize,
) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for (at, line) in lines.iter().enumerate().skip(the_line_after_the_key) {
        if this_line_closes_the_block(line, how_far) {
            break;
        }
        if how_far_a_line_is_indented(line).is_some() {
            found.push((at + 1, line.to_string()));
        }
    }
    found
}

/// Whether this line is written no further in than the key, which is what ends a block.
///
/// **A line with nothing written on it does not end one, and that is the whole of what
/// this asks.** A blank line and a comment standing in the margin both leave the block
/// they are written beside open, and a block that ended on either of them would find every
/// block in a workflow to be one line long the moment the file had a blank line in it.
fn this_line_closes_the_block(line: &str, how_far: usize) -> bool {
    how_far_a_line_is_indented(line).is_some_and(|written| written <= how_far)
}

/// Whether this line writes a name a credential is called by, followed by a colon.
///
/// **The name is everything before the first colon, because a value carries colons of its
/// own** and a rule that read the whole line would refuse a line whose value merely looked
/// like a credential — which is the value of `${{ secrets.A_NAME }}`, and refusing that for
/// the wrong reason would teach a reader to ignore the refusal.
fn this_line_names_a_credential(line: &str) -> bool {
    let name = line.split_once(':').map_or("", |(name, _)| name);
    let shouted = name.to_uppercase();
    THE_WORDS_A_CREDENTIAL_IS_NAMED_WITH
        .iter()
        .any(|word| shouted.contains(word))
}

/// What this gate has to answer for: one line per refusal, with the line it is written on
/// and the line as the file wrote it.
fn what_the_workflow_would_have_to_answer_for() -> String {
    let mut said = String::new();
    for one_refused in the_declared_secrets_in(&the_text_of_the_gate_workflow()) {
        writeln!(
            said,
            "line {}: {}",
            one_refused.the_line_it_is_written_on, one_refused.the_line_as_it_is_written
        )
        .expect("a `String` is written to rather than a file, so this cannot fail");
    }
    said
}

/// The one secret this gate refuses in `workflow`, and a sentence naming the rule when
/// there is not exactly one.
fn the_one_secret_refused_in(workflow: &str, because: &str) -> ADeclaredSecret {
    let refused = the_declared_secrets_in(workflow);

    assert_eq!(
        refused.len(),
        1,
        "{because}, and this is what it refused: {refused:?}"
    );
    refused.into_iter().next().expect("the count above is one")
}

/// Assert that `workflow` declares nothing, and say what it refused when it did.
fn the_workflow_declares_nothing(workflow: &str, because: &str) {
    let refused = the_declared_secrets_in(workflow);

    assert!(refused.is_empty(), "{because}, and it refused: {refused:?}");
}

#[test]
fn a_stored_credential_handed_to_a_step_is_refused() {
    let workflow = "\
name: gate

jobs:
  gate:
    steps:
      - name: Tests
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: cargo test --workspace --no-fail-fast
";
    let refused = the_one_secret_refused_in(
        workflow,
        "this is the shape the rule exists for: a job that cannot run for anybody who has not handed it a credential",
    );

    assert_eq!(
        refused.the_line_it_is_written_on, 8,
        "the declaration is the eighth line of this workflow"
    );
    assert!(
        refused
            .the_line_as_it_is_written
            .contains("secrets.GITHUB_TOKEN"),
        "and the line is quoted whole rather than described: {}",
        refused.the_line_as_it_is_written
    );
}

#[test]
fn a_credential_pasted_into_a_job_as_a_literal_is_refused() {
    let workflow = "\
jobs:
  gate:
    env:
      OPENAI_API_KEY: sk-a-literal-nobody-can-rotate
    steps:
      - run: cargo test --workspace --no-fail-fast
";

    the_one_secret_refused_in(
        workflow,
        "the worst of the two shapes, and the one the search for the stored credentials cannot see: a key typed into a file that is in the repository",
    );
}

#[test]
fn a_credential_set_on_the_whole_workflow_is_refused() {
    let workflow = "\
name: gate
env:
  CARGO_REGISTRIES_CRATES_IO_TOKEN: a-literal
jobs:
  gate:
    runs-on: ubuntu-latest
";

    the_one_secret_refused_in(
        workflow,
        "a job-level block and a step-level block are both blocks of environment variables and this reading does not tell the three levels apart, which is deliberate: a credential at the top of a workflow is the same breach four lines lower",
    );
}

#[test]
fn a_cache_key_is_accepted() {
    let workflow = "\
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
          key: ${{ runner.os }}-cargo-${{ hashFiles('Cargo.lock') }}
";

    the_workflow_declares_nothing(
        workflow,
        "this is the line that decides the width of the words above: the cache key of this repository's own workflow is called `key`, and a rule drawn at the file rather than at a block of environment variables would refuse the workflow it exists to protect",
    );
}

#[test]
fn an_environment_variable_named_for_nothing_secret_is_accepted() {
    let workflow = "\
    env:
      CARGO_TERM_COLOR: always
      RUST_BACKTRACE: '1'
";

    the_workflow_declares_nothing(
        workflow,
        "a block of environment variables that sets two settings rather than a credential, and the block is read rather than skipped, which is what the two tests above are read against",
    );
}

#[test]
fn a_comment_naming_the_stored_credentials_is_accepted() {
    let workflow = "\
# this job has no `secrets` of any kind, and a test that needed one would not run
jobs:
  gate:
    runs-on: ubuntu-latest
";

    the_workflow_declares_nothing(
        workflow,
        "this is the sentence the existing workflow writes about itself, and a rule drawn at the word rather than at the two words and the dot would refuse the file that records the rule",
    );
}

/// The real gate, and the only assertion here that reads the repository rather than a
/// fixture.
#[test]
fn the_gate_workflow_needs_no_secret() {
    let said = what_the_workflow_would_have_to_answer_for();

    assert!(
        said.is_empty(),
        "a workflow that asks for a credential is a suite that is not run for everybody, and the two halves of this rule are one rule: a secret the suite never reads is still a secret nobody else can run with. Declared here, by line and by line:\n{said}"
    );
}

/// **A file that could not be read and a file holding nothing look the same to a reading
/// that read nothing**, and a guard that agreed with the second is a guard that agreed with
/// the first. So the file is asked what it is before it is asked what it holds: it is
/// there, it is not empty, and it is a workflow rather than some other file with the
/// right name.
#[test]
fn the_file_this_gate_read_is_the_workflow_it_names() {
    let workflow = the_text_of_the_gate_workflow();

    assert!(
        !workflow.trim().is_empty(),
        "the workflow is empty, so a reading that found no secret in it agreed with a file nobody wrote"
    );
    for what_a_workflow_has_to_have in ["jobs:", "steps:"] {
        assert!(
            workflow.contains(what_a_workflow_has_to_have),
            "the file this gate read holds no {what_a_workflow_has_to_have}, so it is not the workflow this gate names and every assertion above was made about a fixture"
        );
    }
}
