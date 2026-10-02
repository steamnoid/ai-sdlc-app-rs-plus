//! A person answers at a terminal, and what they were shown while they answered.
//!
//! **This is the ordinary case and the one every gate was built for.** The policy that arrived last
//! cycle is the second way a gate is answered, **and a system with only that way is a system where
//! nobody ever reads anything.**
//!
//! **A blank line is a refusal and not an approval.** A terminal that treats Enter as yes would
//! approve every artifact in a run nobody watched, **and an unattended run would then deliver work
//! to a remote for which no person ever read a word.**

use aisdlc_domain::approval::{AnArtifactVersion, Approval, WhichArtifact, WhoDecided};
use aisdlc_run::a_person_answers_at_a_terminal::{APersonAtTheTerminal, WhatWasShownToThem};
use aisdlc_run::{APersonToDecide, ApprovalQuestion};

/// What somebody typed, **in three lines: a word, and then their reason.**
fn they_typed(their_word: &str, their_reason: &str) -> String {
    format!("{their_word}\n{their_reason}")
}

/// A person who has already given their name.
fn a_person_who_is() -> WhoDecided {
    WhoDecided::try_from("paligakrzychu").expect("a name is not blank")
}

fn a_question() -> ApprovalQuestion {
    let the_work_item = aisdlc_domain::work_item::WorkItem::standing_at(
        aisdlc_domain::work_item_id::WorkItemId::try_from("WELLBEINGT-5").expect("a name"),
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name"),
        aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
            .expect("two halves"),
        aisdlc_domain::role::Role::try_from("PO").expect("a discipline"),
        aisdlc_domain::stage::Stage::AwaitingHumanApproval,
    )
    .expect("a work item answering a request can stand at the gate");

    aisdlc_run::the_question_put_to_somebody(
        &the_work_item,
        &WhichArtifact::try_from("DiscoveryReport").expect("a name"),
        AnArtifactVersion::try_from(1).expect("a version"),
    )
    .expect("a work item at the gate can be asked about")
}

/// The artifact as it sits on disk, **which is what the person is shown rather than a summary.**
fn the_artifact_on_disk() -> String {
    "A discovery report: two languages, a build system, a runner, and evidence files.".to_string()
}

/// A terminal somebody is answering at.
fn a_terminal(what_they_typed: &str) -> APersonAtTheTerminal {
    let the_words_they_wrote = String::from(what_they_typed);

    APersonAtTheTerminal::the_person_at(
        a_person_who_is(),
        move || the_words_they_wrote.clone(),
        the_artifact_on_disk,
    )
    .expect("a person is not a machine")
}

#[test]
fn a_person_is_shown_the_artifact_itself_and_names_what_it_is() {
    let mut the_terminal = a_terminal(&they_typed("yes", "the report names a build system"));

    let _ = the_terminal.how_they_rule_on(&a_question());

    let what_was_shown = the_terminal.what_they_were_shown();
    assert!(
        matches!(what_was_shown, Some(WhatWasShownToThem::TheArtifact { .. })),
        "**because a person shown a summary is not looking at the thing they are approving** — and \
         the summary is a lossy thing produced by the run being judged. What they were shown: \
         {what_was_shown:?}"
    );
}

#[test]
fn the_artifact_shown_names_its_version_and_the_artifact() {
    let mut the_terminal = a_terminal(&they_typed("yes", "the report names a build system"));

    let _ = the_terminal.how_they_rule_on(&a_question());

    let what_they_read = the_terminal.what_they_read();
    assert!(
        what_they_read.contains("DiscoveryReport")
            && what_they_read.contains("version 1")
            && what_they_read.contains("two languages"),
        "**because an approval of an exact artifact at an exact version is the whole of what a gate \
         records** — and a person shown the artifact without its name cannot tell one artifact from \
         the next. What they read: {what_they_read}"
    );
}

#[test]
fn a_person_is_recorded_in_their_own_name_and_never_in_a_machines_namespace() {
    let mut the_terminal = a_terminal(&they_typed("yes", "the report names a build system"));

    let the_ruling = the_terminal.how_they_rule_on(&a_question());

    assert_eq!(
        the_ruling.decided_by.to_string(),
        "paligakrzychu",
        "**because a person's name in a record is the one thing that cannot be edited** — and a \
         person who could be recorded as `autopilot:` would be a person whose decision could not be \
         told apart from a machine's. It recorded: {:?}",
        the_ruling.decided_by
    );
}

#[test]
fn a_person_may_not_sign_as_an_agent_and_is_refused_before_anything_is_shown() {
    let somebody_pretending = APersonAtTheTerminal::the_person_at(
        WhoDecided::try_from("aisdlc-dev-impl").expect("a name is not blank"),
        || they_typed("yes", "the report names a build system"),
        the_artifact_on_disk,
    );

    let why_it_was_refused = somebody_pretending.err().map(|why| why.0);

    assert_eq!(
        why_it_was_refused,
        Some(String::from("aisdlc-dev-impl")),
        "**because a name in the agent namespace at this terminal is somebody trying to make a \
         machine's decision** — and refusing at construction means it happens **before the artifact \
         is read**, so a run that cannot produce a human ruling stops **without having spent the \
         person's attention on it.**"
    );
}

#[test]
fn a_blank_line_is_a_refusal_and_not_an_approval() {
    let mut an_unwatched_terminal = a_terminal("");

    let the_ruling = an_unwatched_terminal.how_they_rule_on(&a_question());
    let what_it_left_behind = Approval::what_the_ruling_left_behind(&the_ruling);

    assert_eq!(
        what_it_left_behind, None,
        "**because a terminal that treats Enter as yes would approve every artifact in a run nobody \
         watched** — and an unattended run would then deliver work to a remote for which no person \
         ever read a word. It read: {:?}",
        the_ruling.reason
    );
}
