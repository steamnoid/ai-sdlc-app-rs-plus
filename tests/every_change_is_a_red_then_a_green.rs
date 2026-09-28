//! The gate for RULE 0.2: every change is a RED and then a GREEN, and the pair is the
//! evidence.
//!
//! ```text
//! test(<scope>): RED — <the specific expectation that fails>
//! feat(<scope>): <the behaviour that now holds>
//! ```
//!
//! **A GREEN with no RED before it is a violation, and it is refused by name.** Reasoning
//! cannot be put under a failing test before it exists, so what is put under one is the
//! contract and the plumbing, and the commit pair is the record of that. A pair is either
//! there or it is not, and the refusal says which expectation was never demanded: "you did
//! not write a test" is a complaint, and "no failing expectation was ever demanded for this
//! behaviour" is a diagnosis.
//!
//! ## The first accommodation: a repair is one commit, and its own body is the evidence
//!
//! **A repair of a pre-existing breakage, where a test already demanded the change, is one
//! `fix()` commit with the RED described in its body.** The rule does not pretend the RED is
//! in the history: it is not, and a gate that manufactured a fake one would be breaking
//! RULE 0.2 to enforce RULE 0.2. So the repair carries the name of the RED in the place
//! where the RED would have been, and a `fix` whose body names none is refused.
//!
//! A fixture on each side, and both are in this file:
//! `a_repair_whose_body_names_a_red_is_accepted` and
//! `a_repair_whose_body_names_no_red_is_refused`.
//!
//! **The cost of this accommodation, stated rather than discovered: this gate is stricter
//! than the repairs in this repository's own history.** Sixteen of the seventeen `fix`
//! commits before this gate's own first commit describe the defect they repaired and none
//! of them except one uses the word `RED`; `98ac80e` is the only one that does. A gate
//! reading them would refuse sixteen of seventeen, every one of them legitimately, which
//! is what "a gate that refused the repository's own history would be a gate that had to be
//! disabled on day one" describes. They are all before the boundary below, so the verdict
//! is unaffected, and what the gate creates is a convention rather than one it reads: from
//! the next repair, the body says which RED it answers, and the refusal says so in the same
//! sentence.
//!
//! ## The second accommodation: a squashed commit carries the pair in its message
//!
//! **`main` is squashed, so a pair lands as one commit whose message records both halves.**
//! The gate therefore accepts a commit whose body names a RED and a GREEN, one line per
//! half, which is the shape `git` writes when a pull request is squashed and which `#3` on
//! this repository carries today. **Either order is accepted and that is deliberate**: a
//! message naming both halves is a record of the pair, and refusing it over the order two
//! lines were pasted in would be a gate refusing a complete record for a formatting detail.
//!
//! A fixture on each side: `a_squashed_commit_recording_both_halves_is_accepted` and
//! `a_squashed_commit_recording_only_one_half_is_refused`.
//!
//! **Neither accommodation is a weakening, and the argument is one sentence each.** A rule
//! that refused both would be refusing the rule as it actually runs in this repository, and
//! a gate nobody can switch on is a rule that exists only in a document. What the
//! accommodations cost is written in the two sections above and nowhere is it hidden: a
//! commit can now buy its own exemption by writing words in its own message, and a
//! `fix` body can be a sentence about a defect rather than the name of a RED. **A person
//! who lies in a commit message can defeat this gate, and that is the whole of the
//! exposure.**
//!
//! ## The boundary: this gate's own first commit
//!
//! **The walk stops at the commit that added this file and judges nothing older.** A gate
//! cannot judge the history that predates it, and it must not try: those commits were
//! written under no such rule, they are not a rubric, and a gate that went through them
//! would report the whole of this repository's past as violations and be turned off. The
//! commit is found from git rather than assumed, and
//! `a_change_written_before_this_gate_began_is_accepted` proves the walk stops.
//!
//! ## Which history was read, and that it is said
//!
//! **`git log` in a worker's worktree and `git log` in a pull request checkout are
//! different histories**, and a gate that does not say which one it read is a gate whose
//! verdict cannot be reproduced. So every refusal carries the history it was read from, in
//! `how_this_history_was_read`.
//!
//! ## A branch arriving is not a change
//!
//! **A commit with more than one parent is a branch arriving rather than a change**, and
//! it is accepted for a reason rather than by an exception: it brought in commits, every one
//! of which is in this same log and is judged on its own, and the commits it brought in
//! are exactly the commits that carry the pairs.
//!
//! `required_linear_history` is on for `main` — `gh api` says so, and it was turned on by
//! `#1` — **and `main` still holds twenty merge commits**, which the setting does not reach
//! backwards. So the reason a commit has more than one parent is asked for rather than
//! assumed, and a squashed commit, a fast-forward and a merge commit are three answers to
//! three different questions.
//!
//! ## `docs` and `chore` need no RED, and here is what that costs
//!
//! **A `docs` commit that changes `AGENTS.md` needs no test, and this gate says so rather
//! than inventing a check it cannot build.** A `chore` is in the same list for the same
//! reason.
//!
//! **The exemption is about the label, and a label is a claim, so the gate takes the claim
//! at face value.** It reads messages and not diffs, which means it cannot tell a `docs`
//! commit that touched a document from one that changed a behaviour, and pretending
//! otherwise would be a gate pretending. **So: nothing in this gate stops a behaviour
//! change arriving in a commit labelled `docs`, and that is a reviewer's job.** The
//! repository has two real instances of the size of the hole, and both are in the history
//! before this gate's boundary: `e0ecff0` is a `docs` commit that added `pub mod
//! scanning;` to `src/lib.rs` and sixteen lines of documentation, and `4fe1ec6` is a
//! `chore` that wrote a thousand lines including three crates of Rust.
//!
//! What stops it, in descending order of what it is worth, and honestly labelled:
//!
//! - **The suite, not this rule.** A change that alters behaviour has to leave the gates
//!   green, and the gates judge the tree rather than the commit. That is the real answer
//!   for behaviour, and it is weaker today than it will be: the domain has two closed-set
//!   tests and no invariant gate yet, so a mislabelled change to a transition would be
//!   caught by nothing until Phase 2 lands.
//! - **The pull request.** A squashed commit on `main` must carry both halves whatever the
//!   branch commits were called, so the exposure is one commit per pull request, and the
//!   pull request is where a diff is read.
//! - **A person.** Plainly: the human gate. This gate cannot see a diff, so it cannot be
//!   the answer.
//!
//! **The narrower check is buildable and is deliberately not built here**: ask
//! `git show --name-only` whether a `docs` or `chore` commit names a `.rs` file, and refuse
//! it if it does. It is one more call and about ten lines. It is left out because it is a
//! second capability — a gate on whether a label is true, rather than on whether a change
//! was demanded — and RULE 0.2 is one capability per cycle. **It is the first thing to add
//! to this file when somebody wants the exemption to be more than a claim.**
//!
//! ## Why this is not a search for the letters in a subject
//!
//! **The kind of a commit is the word it is written under, and that is everything before
//! the first bracket or colon.** `feat(stage)`, `test(stage)`, `chore` and `docs` are four
//! answers; a search for `feat` would also answer for `featish`, and one for `test` would
//! answer for `testing`. The kinds are a table, which is what a rule in this repository
//! looks like, and the table is closed: **a commit whose subject names no kind this gate
//! knows is refused**, because the accepted list being closed is the whole of what makes
//! the rule a rule rather than a mood.
//!
//! ## Why this test runs `git` and is not `shell.run_tests` in a container
//!
//! **The history is trusted input; a repository's own test suite is not, and those are not
//! the same thing.** This is a test reading the repository it lives in, from the git binary
//! that made it, with no credential and no network. `shell.run_tests` is the boundary that
//! exists because an implementation agent writes code and a verification agent then runs
//! that code with a developer's credentials in reach; a container for `git log` would be a
//! container around the one thing in this system that is not arbitrary code from a pull
//! request.
//!
//! ## The holes, named rather than closed
//!
//! - **A commit with a subject this gate does not name is refused, and a person can work
//!   around that by calling it a `chore`.** That is the same hole the `docs` exemption has,
//!   and it is the reason the two have to be argued about together.
//! - **`fix` requires the word `RED`, and this repository's own repairs do not use it.**
//!   Argued above, with the count.
//! - **The two real-history assertions below are assertions about ancestors.** A `main`
//!   rewritten by a squash would remove the branch pair and the squashed pair they name,
//!   and the gate would then fail. **That failure is a fact and not a defect**, and it is
//!   cheaper than a gate that agrees with a history it cannot see.
//! - **A shallow checkout judges nothing.** `git log` in a checkout of `fetch-depth: 1`
//!   returns one commit, this gate's own first commit is in it, and the walk stops at it
//!   having judged nothing at all. `the_reading_reached_a_commit_that_was_built_on_another`
//!   is the assertion that refuses that state rather than agreeing with it, and the fix is
//!   `with: fetch-depth: 0` on the checkout step of `.github/workflows/gate.yml`, which is
//!   a file this change does not own. **Until that is done the gate will fail in CI, and it
//!   should: a gate that is green while judging nothing is the failure this repository
//!   exists to make impossible.**

use std::process::Command;

use aisdlc_gates::scanning::the_root_of_the_checkout;

/// The file whose first commit is where this gate begins judging, named rather than
/// written from `file!`, because what git has to be asked is a path and `file!` gives a
/// path the answer to that question is not written in.
const THIS_GATES_OWN_FILE: &str = "tests/every_change_is_a_red_then_a_green.rs";

/// What `git log` writes for one commit: the hash, the parents, the subject and the body.
///
/// The two separators are bytes a subject cannot hold and a parsed line cannot be mistaken
/// for, which is the whole reason a message is read with git's own separators rather than
/// split on a newline: a body carries newlines and a subject may carry either of these.
const THE_FORMAT_OF_A_COMMIT: &str = "--format=%H%x00%P%x00%s%x00%b%x1e";

/// The byte that divides the four fields of one commit, which no subject in this
/// repository holds and which a line ending can therefore not be confused with.
const WHAT_DIVIDES_THE_FIELDS_OF_A_COMMIT: &str = "\u{0}";

/// The byte that ends one commit, which is a control character a body does not carry.
const WHAT_ENDS_A_COMMIT: &str = "\u{1e}";

/// How many commits the reading must have found before its agreement with them means
/// anything, because a walk that read nothing would refuse nothing and be green.
const THE_FEWEST_COMMITS_A_HISTORY_MUST_HOLD: usize = 3;

/// The word that names the kind no subject in this repository has ever named.
///
/// Empty rather than absent, so that it can be compared with a line like any other word
/// and match none of them, which is what a `None` would have to be turned into by hand at
/// every place it is asked about.
const THE_WORD_NAMING_NO_KIND: &str = "";

/// The kinds of commit this gate has a rule about, and the word each is written under.
///
/// **The list is closed and there is no list of exceptions**, because an exception list is
/// a hole the next person finds a use for. `chore` and `docs` are two entries for one kind
/// rather than one kind with two names, so that the kind is the thing the rule asks about
/// and the words are only how a subject happens to spell it.
const THE_KINDS_AND_THE_WORDS_THAT_NAME_THEM: [(WhatACommitIs, &str); 6] = [
    (WhatACommitIs::ARedThatFails, "test"),
    (WhatACommitIs::AGreenThatHolds, "feat"),
    (WhatACommitIs::AChangeThatKeepsTheBehaviour, "refactor"),
    (WhatACommitIs::ARepairOfSomethingAlreadyBroken, "fix"),
    (WhatACommitIs::ChoresAndWords, "chore"),
    (WhatACommitIs::ChoresAndWords, "docs"),
];

/// What a commit's subject says it is, which is the first question the rule asks and the
/// only one it asks about the words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhatACommitIs {
    /// `test(<scope>)` — the half of a cycle that fails before anything is built.
    ARedThatFails,
    /// `feat(<scope>)` — the half of a cycle that answers the failing expectation.
    AGreenThatHolds,
    /// `refactor(<scope>)` — a change that keeps the behaviour and moves it.
    AChangeThatKeepsTheBehaviour,
    /// `fix(<scope>)` — a repair of a breakage a test had already found.
    ARepairOfSomethingAlreadyBroken,
    /// `chore(<scope>)` and `docs(<scope>)` — neither claims to change behaviour.
    ChoresAndWords,
    /// A subject naming no word in the table, which the rule has no reading for.
    SomethingThisGateHasNoRuleAbout,
}

impl WhatACommitIs {
    /// The word a commit is written under when it is this kind, and the empty word for the
    /// kind no subject has ever named.
    ///
    /// The table is the whole of where a word comes from, so a seventh kind is a line in
    /// the table and nowhere else.
    fn the_word_naming_it(self) -> &'static str {
        THE_KINDS_AND_THE_WORDS_THAT_NAME_THEM
            .iter()
            .find(|(the_kind, _)| *the_kind == self)
            .map_or(THE_WORD_NAMING_NO_KIND, |(_, the_word)| *the_word)
    }
}

/// One commit as `git log` wrote it: its hash, the commits it was built on, its subject and
/// its body.
///
/// **All four are carried because all four are read.** The hash is what a refusal names and
/// what a pair is matched by, the parents answer both whether this commit is a change and
/// what it was built on, the subject names the kind, and the body is the only place a
/// repair or a squash can carry its evidence. The parents are kept as git wrote them
/// rather than reduced to a first one, because how many there are is a question this gate
/// asks and reducing them first would have thrown the answer away.
#[derive(Debug)]
struct ACommitInTheHistory {
    its_hash: String,
    the_commits_it_was_built_on: String,
    its_subject: String,
    its_body: String,
}

impl ACommitInTheHistory {
    /// Whether the message of this commit records both halves of a cycle.
    ///
    /// **The body and not the subject.** The subject of a squashed commit is written by
    /// whoever squashed it and says `test/feat(<scope>)`, which is a label; the body is
    /// the pair as git recorded it, one line per commit that went in, and a label is not
    /// evidence of anything. **Either order is accepted**, because a message naming both
    /// halves is a record of the pair and refusing it over the order two lines were pasted
    /// in would be refusing a complete record.
    fn the_message_records_both_halves(&self) -> bool {
        let the_red = WhatACommitIs::ARedThatFails.the_word_naming_it();
        let the_green = WhatACommitIs::AGreenThatHolds.the_word_naming_it();

        the_message_names_a_half(&self.its_body, the_red)
            && the_message_names_a_half(&self.its_body, the_green)
    }

    /// Whether this commit brought in a branch rather than making a change.
    ///
    /// More than one parent is the whole of the test, and a merge commit is the ordinary
    /// case: it is a branch arriving, and the commits it brought in are in this same log
    /// and are judged on their own. A squashed commit and a fast-forward are the other two
    /// answers and both have one parent, which is why the count is asked rather than the
    /// word `merge` being looked for.
    fn brings_in_a_branch(&self) -> bool {
        self.the_commits_it_was_built_on.split_whitespace().count() > 1
    }

    /// The commit this one was built on, and nothing at all for the first commit of a
    /// repository, which was built on nothing.
    fn the_commit_it_was_built_on(&self) -> Option<&str> {
        self.the_commits_it_was_built_on.split_whitespace().next()
    }
}

/// The name a line of a commit message is written under, which is every word before the
/// first bracket or colon and with a leading bullet stepped over.
///
/// **One reading for a subject and for a line of a body, and that is deliberate**: a
/// squashed body names its halves in exactly the spelling a subject uses, and two readings
/// of the same shape is one of them going to disagree with the other. A line is only a
/// name when everything before its bracket or colon is one word, so a sentence that
/// mentions a half in the middle is a sentence.
fn the_name_it_is_written_under(line: &str) -> &str {
    let under_the_bullet = line.trim_start().trim_start_matches('*');
    let before_the_scope = under_the_bullet.split(['(', ':']).next();
    before_the_scope.unwrap_or(under_the_bullet).trim()
}

/// What a commit's subject says it is, and the kind no word in the table names when the
/// subject names none.
fn what_a_commit_is(subject: &str) -> WhatACommitIs {
    let the_name = the_name_it_is_written_under(subject);

    THE_KINDS_AND_THE_WORDS_THAT_NAME_THEM
        .iter()
        .find(|(_, the_word)| *the_word == the_name)
        .map_or(
            WhatACommitIs::SomethingThisGateHasNoRuleAbout,
            |(the_kind, _)| *the_kind,
        )
}

/// Whether a commit's message has a line written under `the_word`.
///
/// **An empty word matches nothing**, and that is what makes the empty word usable as the
/// answer for a kind no subject names: it is compared with every line and refuses all of
/// them, rather than matching every line because every line begins with nothing.
fn the_message_names_a_half(message: &str, the_word: &str) -> bool {
    if the_word.is_empty() {
        return false;
    }
    message
        .lines()
        .any(|line| the_name_it_is_written_under(line) == the_word)
}

/// Whether a commit's body names a RED, which is the marker this repository writes in the
/// subject of a RED and therefore the only evidence a repair has to carry.
///
/// **Read as a whole word and not as letters**, so a body that says `brought` does not
/// name one. The letters are taken off both ends and the middle compared, which is what
/// lets a message write the word in backticks or with a colon after it and still be read.
fn the_body_names_a_red(body: &str) -> bool {
    body.split_whitespace()
        .any(|word| word.trim_matches(|one: char| !(one.is_alphanumeric() || one == '_')) == "RED")
}

/// Whether the commit this one was built on is a RED, which is the only place a change that
/// was demanded can be recorded on a branch.
///
/// **The first parent and not the commit before it in the log, and the difference is
/// whether a branch has merges.** A log of a history with merges is not a line, so the
/// commit that happens to be printed above this one is whichever of several branches git
/// reached first; the parent is the branch tip this change was actually made on top of,
/// and it is the same fact whether the branch has one parent or twenty. A parent this
/// reading never saw is not a RED, because a rule that accepts a hash it could not read is
/// a rule that accepts a hash.
fn the_commit_it_was_built_on_was_a_red(
    commit: &ACommitInTheHistory,
    the_whole_history: &[ACommitInTheHistory],
) -> bool {
    let Some(the_one_it_was_built_on) = commit.the_commit_it_was_built_on() else {
        return false;
    };
    let found = the_whole_history
        .iter()
        .find(|one| one.its_hash == the_one_it_was_built_on);
    let Some(the_one_it_was_built_on) = found else {
        return false;
    };

    what_a_commit_is(&the_one_it_was_built_on.its_subject) == WhatACommitIs::ARedThatFails
}

/// The expectation a change of this kind had to be demanded by before it was made, and
/// nothing at all for a kind that owed none.
///
/// **The order of the questions is the rule and it is three questions long.** A branch
/// arriving is not a change at all. A message recording both halves is a cycle however its
/// subject spells it. The rest are asked by what the subject names, and a `test` is
/// answered with nothing because a RED owes nothing: it is the demand itself.
fn the_expectation_that_was_never_demanded(
    commit: &ACommitInTheHistory,
    the_whole_history: &[ACommitInTheHistory],
) -> Option<&'static str> {
    if commit.brings_in_a_branch() || commit.the_message_records_both_halves() {
        return None;
    }
    match what_a_commit_is(&commit.its_subject) {
        WhatACommitIs::ChoresAndWords | WhatACommitIs::ARedThatFails => None,
        WhatACommitIs::AGreenThatHolds => {
            the_green_that_was_never_asked_for(commit, the_whole_history)
        }
        WhatACommitIs::AChangeThatKeepsTheBehaviour => {
            the_change_that_was_never_asked_for(commit, the_whole_history)
        }
        WhatACommitIs::ARepairOfSomethingAlreadyBroken => the_repair_that_names_no_red(commit),
        WhatACommitIs::SomethingThisGateHasNoRuleAbout => {
            Some("no cycle, no repair and no chore, and nothing that failed before this")
        }
    }
}

/// What a GREEN owes, which is a failing expectation written down before it was made.
fn the_green_that_was_never_asked_for(
    commit: &ACommitInTheHistory,
    the_whole_history: &[ACommitInTheHistory],
) -> Option<&'static str> {
    if the_commit_it_was_built_on_was_a_red(commit, the_whole_history) {
        return None;
    }
    Some("a failing expectation, in a test(…) commit it was built on")
}

/// What a rename owes, which is the behaviour it was to keep, written down before it.
fn the_change_that_was_never_asked_for(
    commit: &ACommitInTheHistory,
    the_whole_history: &[ACommitInTheHistory],
) -> Option<&'static str> {
    if the_commit_it_was_built_on_was_a_red(commit, the_whole_history) {
        return None;
    }
    Some("the behaviour it was to keep, named by a test(…) commit it was built on")
}

/// What a repair owes, which is the RED its own body says it is answering.
///
/// **The body and not the commit it was built on, and the reason is that a repair has no
/// RED beside it**: it is a repair of a breakage that was already there, so the commit
/// before it is whatever the breakage was repaired on top of and says nothing about what
/// was failing. The body is the only place the evidence can be, which is why a body that
/// describes a defect without naming the expectation is refused — the description is a
/// claim about the past and the name of the expectation is the claim that is checkable.
fn the_repair_that_names_no_red(commit: &ACommitInTheHistory) -> Option<&'static str> {
    if the_body_names_a_red(&commit.its_body) {
        return None;
    }
    Some("the RED this repair answers, named in its own body")
}

/// What one refusal says: the commit, its subject, and the expectation never demanded.
fn what_this_refusal_says(commit: &ACommitInTheHistory, the_expectation: &str) -> String {
    format!(
        "{} {}\n    which expectation was never demanded: {the_expectation}",
        commit.its_hash, commit.its_subject
    )
}

/// Every change in the history that was written after this gate began and that no failing
/// expectation was ever demanded for, one refusal per change.
///
/// **The walk stops at `the_gate_began_at` and judges nothing older**, and it returns
/// rather than falls off the end, because a history in which that commit is absent is a
/// history this gate cannot place itself in — and `this_gate_is_in_the_history_it_read` is
/// the assertion that says so out loud, rather than a green run agreeing with everything.
fn what_the_history_would_have_to_answer_for(
    the_whole_history: &[ACommitInTheHistory],
    the_gate_began_at: &str,
) -> Vec<String> {
    let mut refused = vec![];
    for commit in the_whole_history {
        if commit.its_hash == the_gate_began_at {
            return refused;
        }
        let Some(the_expectation) =
            the_expectation_that_was_never_demanded(commit, the_whole_history)
        else {
            continue;
        };
        refused.push(what_this_refusal_says(commit, the_expectation));
    }
    refused
}

/// What `git` wrote, asked in the root of this checkout, with a panic naming the question
/// when it has nothing to say.
///
/// **A `git` that failed and an empty answer are the same thing to a caller that only
/// looks at the text, and an empty answer is a gate that judges nothing and is green.**
/// That is why the exit is asked about rather than the bytes being taken whatever they
/// are: a reading that stopped early has to say it stopped.
fn git_wrote(asked: &[&str]) -> String {
    let said = Command::new("git")
        .args(asked)
        .current_dir(the_root_of_the_checkout())
        .output()
        .unwrap_or_else(|why| {
            panic!("git {asked:?} has to be runnable to read this history: {why}")
        });

    assert!(
        said.status.success(),
        "git {asked:?} refused, and a reading that stopped there judges nothing: {}",
        String::from_utf8_lossy(&said.stderr)
    );
    String::from_utf8_lossy(&said.stdout).into_owned()
}

/// One commit out of the four fields git wrote for it, over the two separators.
fn the_commit_written_in(written: &str) -> ACommitInTheHistory {
    let mut fields = written.split(WHAT_DIVIDES_THE_FIELDS_OF_A_COMMIT);
    let its_hash = fields.next().unwrap_or_default().to_string();
    let the_commits_it_was_built_on = fields.next().unwrap_or_default().to_string();
    let its_subject = fields.next().unwrap_or_default().trim().to_string();
    let its_body = fields.next().unwrap_or_default().to_string();

    ACommitInTheHistory {
        its_hash,
        the_commits_it_was_built_on,
        its_subject,
        its_body,
    }
}

/// Every commit git wrote for the question it was asked, newest first.
fn the_commits_git_wrote_for(asked: &[&str]) -> Vec<ACommitInTheHistory> {
    let written = git_wrote(asked);
    let mut read = vec![];
    for one_commit in written.split(WHAT_ENDS_A_COMMIT) {
        if !one_commit.trim().is_empty() {
            read.push(the_commit_written_in(one_commit));
        }
    }
    read
}

/// The whole history of this checkout, newest commit first, which is the history this run
/// was handed and the only one it may judge.
fn the_commits_this_checkout_holds() -> Vec<ACommitInTheHistory> {
    the_commits_git_wrote_for(&["log", THE_FORMAT_OF_A_COMMIT])
}

/// The commit that added this gate's own file, which is where the gate begins judging, and
/// nothing at all while that file has never been committed.
///
/// **The oldest of the commits that added it, and the oldest is the last line** because
/// `git log` prints the newest first. A file that was added, deleted and added again was
/// still in the history while it was gone, and a gate that moved its own boundary
/// forwards with its own file would stop judging the commits in between.
fn the_commit_that_added_this_gate() -> Option<String> {
    let added = git_wrote(&["log", "--diff-filter=A", "--format=%H", THIS_GATES_OWN_FILE]);

    added.lines().last().map(str::to_string)
}

/// The name of the history this run was handed, so that a verdict can be reproduced.
///
/// **Asked of git and not of the environment**, because a repository's remote and a
/// checkout of a pull request are the two cases the question has and a variable is the
/// other one. A detached head says `HEAD`, which is the honest answer for a checkout with
/// no branch on it.
fn which_history_was_given() -> String {
    git_wrote(&["rev-parse", "--abbrev-ref", "HEAD"])
        .trim()
        .to_string()
}

/// What the reading found and where it read it, which is in every refusal because a verdict
/// without it cannot be reproduced on another machine.
fn how_this_history_was_read() -> String {
    let the_history = the_commits_this_checkout_holds();
    let where_it_was_read = which_history_was_given();
    let the_gate_began_at = the_commit_that_added_this_gate();

    format!(
        "{the_gate_began_at:?} is the commit this gate began at, read out of {where_it_was_read}, which holds {} commits",
        the_history.len()
    )
}

/// A cycle the real branch history holds, as the subject of the GREEN, and nothing at all
/// when it holds none.
///
/// **Read out of the commits on the disk and not out of a fixture, because a fixture made
/// up by hand is not evidence that a pair was ever written here.** A RED and a GREEN are a
/// pair when the GREEN was built on the RED, which is the same fact the rule asks about and
/// so cannot be evidence for itself; it is evidence that the shape exists in this history
/// at all, and the judgement is proved on data instead.
fn a_cycle_the_real_branch_history_holds(
    the_whole_history: &[ACommitInTheHistory],
) -> Option<String> {
    the_whole_history
        .iter()
        .filter(|one| what_a_commit_is(&one.its_subject) == WhatACommitIs::ARedThatFails)
        .find_map(|a_red| {
            let the_green_built_on_it = the_whole_history
                .iter()
                .find(|one| one.the_commit_it_was_built_on() == Some(a_red.its_hash.as_str()));
            the_green_built_on_it.map(|the_green| the_green.its_subject.clone())
        })
}

/// A squashed cycle the real history holds, as the subject of the commit that carries it,
/// and nothing at all when it holds none.
fn a_squashed_cycle_the_real_history_holds(
    the_whole_history: &[ACommitInTheHistory],
) -> Option<String> {
    the_whole_history
        .iter()
        .find(|one| one.the_message_records_both_halves())
        .map(|one| one.its_subject.clone())
}

/// A commit in a fixture history, written the way `git log` would have written it.
///
/// The names are words rather than hashes because a fixture has no repository to be real
/// to, and the commit a fixture was built on is named rather than computed because a pair
/// is a fact about two commits standing on one another and not about a hash. An empty
/// parent is a commit that was built on nothing, which is what the first commit of a
/// repository is.
fn a_commit_named(
    its_name: &str,
    its_subject: &str,
    its_body: &str,
    the_one_it_was_built_on: &str,
) -> ACommitInTheHistory {
    ACommitInTheHistory {
        its_hash: its_name.to_string(),
        the_commits_it_was_built_on: the_one_it_was_built_on.to_string(),
        its_subject: its_subject.to_string(),
        its_body: its_body.to_string(),
    }
}

/// A history written as one line per commit — its name, its subject, its body, and the
/// commit it was built on — newest first, which is the order `git log` hands them over in
/// and the order the walk reads them in.
fn a_history_of(every_commit: &[(&str, &str, &str, &str)]) -> Vec<ACommitInTheHistory> {
    let mut written = vec![];
    for (its_name, its_subject, its_body, the_one_it_was_built_on) in every_commit {
        written.push(a_commit_named(
            its_name,
            its_subject,
            its_body,
            the_one_it_was_built_on,
        ));
    }
    written
}

/// One line of a fixture history: the commit's name, its subject, its body, and the commit
/// it was built on.
///
/// A tuple and not four arguments threaded through every test, because a fixture that says
/// what it is in one line is read in one line and a fixture spread over nine is a
/// paragraph of punctuation.
type OneLineOfAFixtureHistory = (&'static str, &'static str, &'static str, &'static str);

/// The commit the walk stops at in every fixture, and which no fixture history holds, so
/// that a fixture is judged in full rather than from a boundary that is not there.
const THE_GATE_BEGAN: &str = "the_gate_began";

/// The subject every RED in this file is written under, in the spelling this repository
/// writes: the kind, a scope in brackets, and then the marker.
const THE_SUBJECT_OF_A_RED: &str =
    "test(history): RED — a change that was never demanded is refused";

/// The first commit of a fixture history, and the RED every pair in this file is built on.
const A_RED_BUILT_ON_NOTHING: OneLineOfAFixtureHistory =
    ("the_first", THE_SUBJECT_OF_A_RED, "", "");

/// The second commit of a fixture history, and the GREEN the whole rule is about.
const THE_GREEN_BUILT_ON_THE_RED: OneLineOfAFixtureHistory = (
    "the_second",
    "feat(history): the history is read and judged",
    "",
    "the_first",
);

/// A history holding one RED and nothing else, which is the whole of what a RED owes.
const A_HISTORY_HOLDING_ONE_RED: [OneLineOfAFixtureHistory; 1] = [A_RED_BUILT_ON_NOTHING];

/// A history holding a cycle, newest first: the GREEN and then the RED it was built on.
const A_HISTORY_HOLDING_A_CYCLE: [OneLineOfAFixtureHistory; 2] =
    [THE_GREEN_BUILT_ON_THE_RED, A_RED_BUILT_ON_NOTHING];

/// A history holding a GREEN built on a document, which is a change with no RED under it
/// anywhere and is refused for that rather than for the kind of the commit beneath.
const A_HISTORY_HOLDING_A_GREEN_BUILT_ON_A_DOCUMENT: [OneLineOfAFixtureHistory; 2] = [
    THE_GREEN_BUILT_ON_THE_RED,
    (
        "the_first",
        "docs: the gate table says which gates exist",
        "",
        "",
    ),
];

/// A history holding a rename built on a chore, which is the other of the two changes the
/// rule refuses and is refused for the behaviour it was to keep.
const A_HISTORY_HOLDING_A_RENAME_BUILT_ON_A_CHORE: [OneLineOfAFixtureHistory; 2] = [
    (
        "the_second",
        "refactor(history): the helpers move into the library",
        "",
        "the_first",
    ),
    (
        "the_first",
        "chore: the workspace, the three crates, and the glossary",
        "",
        "",
    ),
];

/// A history holding a cycle with one commit between the two halves, which is the fixture
/// for "no intervening commit" and the one a reader is most likely to believe is a pair.
const A_HISTORY_HOLDING_A_COMMIT_BETWEEN_THE_HALVES: [OneLineOfAFixtureHistory; 3] = [
    (
        "the_third",
        "feat(history): the history is read and judged",
        "",
        "the_second",
    ),
    ("the_second", "chore: the licence", "", "the_first"),
    A_RED_BUILT_ON_NOTHING,
];

/// A history holding a change written before this gate began, and the commit that added
/// this gate, which is where the walk stops and judges nothing older.
const A_HISTORY_HOLDING_A_CHANGE_BEFORE_THE_GATE: [OneLineOfAFixtureHistory; 2] = [
    (
        "ancient",
        "feat(domain): a stage is a type, and the set is closed",
        "",
        "",
    ),
    (THE_GATE_BEGAN, THE_SUBJECT_OF_A_RED, "", ""),
];

/// The body a squash carries when a cycle went in as two commits: one line per commit, in
/// the order they were written, which is the shape `git` produces for a squashed pull
/// request and the shape `#3` on this repository carries today.
const THE_BODY_OF_A_SQUASHED_CYCLE: &str = "\
* test(twenty-lines): RED — a body of twenty-one lines is refused
  the fixture counted twenty-one and the walk refused it

* feat(twenty-lines): every body is walked, counted and named
  the refusal names the file and the count";

/// A body naming the RED a repair answers, which is the whole of what a repair owes.
const THE_BODY_OF_A_REPAIR_THAT_NAMES_ITS_RED: &str = "\
The walk counted the fixture at twenty-two lines and the refusal named the file.

RED: a_body_of_twenty_two_lines_is_refused";

/// A body describing a defect without naming the RED it answers, which is what a repair
/// whose author wrote about the past rather than about the failing expectation looks like.
const THE_BODY_OF_A_REPAIR_THAT_NAMES_NO_RED: &str = "\
The gate refused the fixture at twenty-two counted lines and the refusal named the file and
the count, so the walk was counting a body that holds nothing but documentation.";

#[test]
fn a_test_commit_on_its_own_is_accepted() {
    let the_history = a_history_of(&A_HISTORY_HOLDING_ONE_RED);

    let refused = what_the_history_would_have_to_answer_for(&the_history, THE_GATE_BEGAN);

    assert!(
        refused.is_empty(),
        "a RED is the demand and owes nothing, and refusing it would be refusing the rule: {refused:?}"
    );
}

#[test]
fn a_feat_commit_built_on_a_red_is_accepted() {
    let the_history = a_history_of(&A_HISTORY_HOLDING_A_CYCLE);

    let refused = what_the_history_would_have_to_answer_for(&the_history, THE_GATE_BEGAN);

    assert!(
        refused.is_empty(),
        "the two commits of a cycle, and the pair is the whole of what the rule asks for: {refused:?}"
    );
}

#[test]
fn a_repair_whose_body_names_a_red_is_accepted() {
    let named_plainly = a_commit_named(
        "the_first",
        "fix(twenty-lines): the gate refused the gate's own first draft",
        THE_BODY_OF_A_REPAIR_THAT_NAMES_ITS_RED,
        "",
    );
    let written_in_backticks = a_commit_named(
        "the_first",
        "fix(twenty-lines): the gate refused the gate's own first draft",
        "RED: `a_body_of_twenty_two_lines_is_refused`",
        "",
    );

    for a_repair in [named_plainly, written_in_backticks] {
        let refused = what_the_history_would_have_to_answer_for(&[a_repair], THE_GATE_BEGAN);
        assert!(
            refused.is_empty(),
            "the word is the marker and a message may write it in backticks and still name it: {refused:?}"
        );
    }
}

#[test]
fn a_chore_is_accepted() {
    let the_history = a_history_of(&[(
        "the_first",
        "chore: the workspace, the three crates, and the glossary",
        "",
        "",
    )]);

    let refused = what_the_history_would_have_to_answer_for(&the_history, THE_GATE_BEGAN);

    assert!(
        refused.is_empty(),
        "a chore claims no behaviour, and this gate takes the claim at face value: {refused:?}"
    );
}

#[test]
fn a_docs_is_accepted() {
    let about_the_rules = a_commit_named(
        "the_first",
        "docs: the role table and the code now say the same word",
        "",
        "",
    );
    let with_a_scope_in_it = a_commit_named(
        "the_first",
        "docs(gates): the gate table says which gates exist",
        "",
        "",
    );

    for a_document in [about_the_rules, with_a_scope_in_it] {
        let refused = what_the_history_would_have_to_answer_for(&[a_document], THE_GATE_BEGAN);
        assert!(
            refused.is_empty(),
            "a document changes no behaviour, and the two spellings of the kind are one kind: {refused:?}"
        );
    }
}

#[test]
fn a_squashed_commit_recording_both_halves_is_accepted() {
    let a_squash = a_commit_named(
        "the_first",
        "test/feat(twenty-lines): every body is walked, counted and named",
        THE_BODY_OF_A_SQUASHED_CYCLE,
        "",
    );

    let refused = what_the_history_would_have_to_answer_for(&[a_squash], THE_GATE_BEGAN);

    assert!(
        refused.is_empty(),
        "main is squashed, so this is the shape a cycle lands in, and a gate that refused it would be refusing main: {refused:?}"
    );
}

#[test]
fn a_squashed_commit_recording_the_halves_the_other_way_round_is_accepted() {
    let written_green_first = format!(
        "* feat(history): the history is read and judged\n{}\n* test(history): RED — a change that was never demanded is refused",
        "  the two refusal cases the rule has"
    );
    let a_squash = a_commit_named(
        "the_first",
        "test/feat(history): a cycle is a RED and a GREEN",
        &written_green_first,
        "",
    );

    let refused = what_the_history_would_have_to_answer_for(&[a_squash], THE_GATE_BEGAN);

    assert!(
        refused.is_empty(),
        "a message naming both halves is a record of the pair, and the order two lines were pasted in is not the record: {refused:?}"
    );
}

#[test]
fn a_branch_arriving_is_accepted() {
    let a_landing = ACommitInTheHistory {
        its_hash: "the_first".to_string(),
        the_commits_it_was_built_on: "the_second  the_third".to_string(),
        its_subject: "merge(history): a cycle is a RED and a GREEN".to_string(),
        its_body: String::new(),
    };

    let refused = what_the_history_would_have_to_answer_for(&[a_landing], THE_GATE_BEGAN);

    assert!(
        refused.is_empty(),
        "a commit with more than one parent brought in commits that are judged on their own: {refused:?}"
    );
}

#[test]
fn a_change_written_before_this_gate_began_is_accepted() {
    let the_history = a_history_of(&A_HISTORY_HOLDING_A_CHANGE_BEFORE_THE_GATE);

    let refused = what_the_history_would_have_to_answer_for(&the_history, THE_GATE_BEGAN);

    assert!(
        refused.is_empty(),
        "those commits were written under no such rule, and judging them would report this repository's past as violations: {refused:?}"
    );
}

#[test]
fn a_feat_commit_with_no_red_before_it_is_refused() {
    let the_history = a_history_of(&A_HISTORY_HOLDING_A_GREEN_BUILT_ON_A_DOCUMENT);

    let refused = what_the_history_would_have_to_answer_for(&the_history, THE_GATE_BEGAN);

    assert_eq!(refused.len(), 1, "and one refusal, not two: {refused:?}");
    assert!(
        refused[0].contains("feat(history): the history is read and judged"),
        "the refusal names the commit by its subject, which is what a reader searches for: {refused:?}"
    );
    assert!(
        refused[0].contains("a failing expectation, in a test(…) commit it was built on"),
        "and it says which expectation was never demanded rather than that no test was written: {refused:?}"
    );
}

#[test]
fn a_refactor_commit_with_no_red_before_it_is_refused() {
    let the_history = a_history_of(&A_HISTORY_HOLDING_A_RENAME_BUILT_ON_A_CHORE);

    let refused = what_the_history_would_have_to_answer_for(&the_history, THE_GATE_BEGAN);

    assert_eq!(refused.len(), 1, "and one refusal, not two: {refused:?}");
    assert!(
        refused[0]
            .contains("the behaviour it was to keep, named by a test(…) commit it was built on"),
        "a rename is refused for what it owed rather than for what a GREEN owes: {refused:?}"
    );
}

#[test]
fn a_change_with_a_commit_between_it_and_its_red_is_refused() {
    let the_history = a_history_of(&A_HISTORY_HOLDING_A_COMMIT_BETWEEN_THE_HALVES);

    let refused = what_the_history_would_have_to_answer_for(&the_history, THE_GATE_BEGAN);

    assert_eq!(
        refused.len(),
        1,
        "and the RED alone does not reach past a commit: {refused:?}"
    );
}

#[test]
fn a_repair_whose_body_names_no_red_is_refused() {
    let a_repair = a_commit_named(
        "the_first",
        "fix(twenty-lines): the gate refused the gate's own first draft",
        THE_BODY_OF_A_REPAIR_THAT_NAMES_NO_RED,
        "",
    );

    let refused = what_the_history_would_have_to_answer_for(&[a_repair], THE_GATE_BEGAN);

    assert_eq!(
        refused.len(),
        1,
        "a repair with no RED in its body is one: {refused:?}"
    );
    assert!(
        refused[0].contains("the RED this repair answers, named in its own body"),
        "and the refusal says where the evidence was supposed to be: {refused:?}"
    );
}

#[test]
fn a_squashed_commit_recording_only_one_half_is_refused() {
    let a_squash = a_commit_named(
        "the_first",
        "test/feat(placeholder): no placeholder is left in the code",
        "A todo! is an unfinished cycle given a home in a line of code, and an unfinished \
cycle is visible in the history.",
        "",
    );

    let refused = what_the_history_would_have_to_answer_for(&[a_squash], THE_GATE_BEGAN);

    assert_eq!(
        refused.len(),
        1,
        "a label carrying one half is not a record of a pair: {refused:?}"
    );
    assert!(
        refused[0].contains("no cycle, no repair and no chore"),
        "and the refusal says the subject named no kind the rule has a reading for: {refused:?}"
    );
}

#[test]
fn a_refusal_names_the_commit_the_subject_and_the_expectation() {
    let a_green = a_commit_named(
        "0123456789abcdef0123456789abcdef01234567",
        "feat(history): the history is read and judged",
        "",
        "",
    );

    let refused = what_the_history_would_have_to_answer_for(&[a_green], THE_GATE_BEGAN);

    assert_eq!(refused.len(), 1, "the fixture is one refusal: {refused:?}");
    assert!(
        refused[0].starts_with("0123456789abcdef0123456789abcdef01234567 feat(history):"),
        "a refusal a reader cannot find the commit in is a refusal nobody can act on: {refused:?}"
    );
    assert!(
        refused[0].contains("\n    which expectation was never demanded: "),
        "and the three things are on separate lines so that each is read on its own: {refused:?}"
    );
}

/// **A gate that read no history and found no violation would be green and would be a
/// lie**, so the walk is asked what it found before anything is judged. Three commits is
/// the floor, and a reading that returned one — which is what a shallow checkout holds — is
/// a reading that has agreed with everything it never saw.
#[test]
fn the_walk_read_the_history_before_it_judged_any_of_it() {
    let the_history = the_commits_this_checkout_holds();

    assert!(
        the_history.len() >= THE_FEWEST_COMMITS_A_HISTORY_MUST_HOLD,
        "the walk read {} commits, which is too few for this repository's history",
        the_history.len()
    );
}

/// **The one assertion that says the walk saw a chain at all.** A `git log` in a checkout
/// of `fetch-depth: 1` returns the tip and nothing else, this gate's own first commit is
/// in it, and the walk then stops at it having judged nothing. The fix is
/// `with: fetch-depth: 0` on the checkout step of the gate workflow, and until that file
/// carries it this gate is right to be red in CI rather than green.
#[test]
fn the_reading_reached_a_commit_that_was_built_on_another() {
    let the_history = the_commits_this_checkout_holds();
    let how_many_were_built_on_another = the_history
        .iter()
        .filter(|one| one.the_commit_it_was_built_on().is_some())
        .count();

    assert!(
        how_many_were_built_on_another > 0,
        "no commit in the reading was built on another, so the history was one commit deep and this gate judged nothing. {}",
        how_this_history_was_read()
    );
}

/// A gate that cannot find its own boundary is a gate that cannot say where it begins, and
/// the state where it cannot is the state where the file has never been committed — which
/// is the state this file is in during the cycle that writes it.
#[test]
fn this_gate_is_in_the_history_it_read() {
    let the_history = the_commits_this_checkout_holds();
    let the_gate_began_at = the_commit_that_added_this_gate();

    let Some(the_gate_began_at) = the_gate_began_at else {
        panic!(
            "no commit in this history added {THIS_GATES_OWN_FILE}, so there is no boundary and nothing is judged"
        );
    };
    assert!(
        the_history
            .iter()
            .any(|one| one.its_hash == the_gate_began_at),
        "git says {the_gate_began_at} added this file and the reading did not find it"
    );
}

/// A pair in the real branch history, which is the shape the rule exists to recognise and
/// which no fixture can prove was ever written here.
#[test]
fn the_real_branch_history_holds_a_red_and_a_green() {
    let the_history = the_commits_this_checkout_holds();

    let held = a_cycle_the_real_branch_history_holds(&the_history);

    assert!(
        held.is_some(),
        "no commit in this history is a GREEN built on a RED. {}",
        how_this_history_was_read()
    );
}

/// A squashed pair in the real history, which is the shape `main` lands a cycle in and
/// which the gate has to accept or the repository cannot be built out of.
#[test]
fn the_real_history_holds_a_squashed_cycle() {
    let the_history = the_commits_this_checkout_holds();

    let held = a_squashed_cycle_the_real_history_holds(&the_history);

    assert!(
        held.is_some(),
        "no commit in this history has a message naming both halves, so the second accommodation has never been exercised here. {}",
        how_this_history_was_read()
    );
}

/// A verdict cannot be reproduced unless the refusal says which history it was read from,
/// and this is the assertion that the sentence is not empty and carries the count.
#[test]
fn the_report_says_which_history_was_read_and_how_much_of_it() {
    let how_many = the_commits_this_checkout_holds().len();
    let said = how_this_history_was_read();
    let where_it_was_read = which_history_was_given();

    assert!(
        !where_it_was_read.is_empty(),
        "git named no history, and a refusal that cannot say where it was read from cannot be reproduced"
    );
    assert!(
        said.contains(&how_many.to_string()),
        "the sentence carries no count, so a reader cannot tell a whole history from a tip: {said}"
    );
}

/// The real gate, and the only assertion here that reads the repository rather than a
/// fixture. Every one above is a demonstration; this one is the rule, and it is the one
/// worth keeping if the rest were deleted.
#[test]
fn every_change_is_a_red_then_a_green() {
    let the_history = the_commits_this_checkout_holds();
    let the_gate_began_at = the_commit_that_added_this_gate();
    let Some(the_gate_began_at) = the_gate_began_at else {
        return;
    };

    let refused = what_the_history_would_have_to_answer_for(&the_history, &the_gate_began_at);

    assert!(
        refused.is_empty(),
        "a change nothing had failed for is a change nobody watched being wrong first, and \
         the pair is the whole of the record. Judged out of {}:\n{refused:?}",
        how_this_history_was_read()
    );
}
