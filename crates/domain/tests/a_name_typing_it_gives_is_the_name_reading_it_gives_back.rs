//! Two names, and the fact that they refuse different things.
//!
//! **`WorkItemId` and `AgentIdentifier` are both newtypes over `String`, and pinning them
//! together in one file is the point of this one.** They look alike, they are written the
//! same way, and they refuse different things: a work item's name is refused when it is
//! blank, and an agent's is refused when it is blank *or* when it is the word that means
//! absent. A reader who had met only one of them would guess the other. A test that held
//! each separately would leave the difference to be inferred from the source.
//!
//! **`Display` writes the name as written and nothing else** — no trimming, no lowercasing.
//! A name a person typed and a name a run reads back are the same name, and a system that
//! tidies one of them is a system whose lookups miss.
//!
//! No file is read here, because this is a test of the domain and the domain may not reach
//! for a filesystem. The specification's account of these two is checked by a gate.

use aisdlc_domain::agent_identifier::{AgentIdentifier, AnAgentThatIsNotAnIdentity};
use aisdlc_domain::work_item_id::{AWorkItemThatNamesNothing, WorkItemId};

#[test]
fn a_name_typing_it_gives_is_the_name_reading_it_gives_back() {
    let name = WorkItemId::try_from("aisdlc-po-analyze").expect("a name is not blank");

    assert_eq!(
        name.to_string(),
        "aisdlc-po-analyze",
        "a name a person typed and a name a run reads back are the same name"
    );
    assert_eq!(
        name.to_string(),
        "aisdlc-po-analyze",
        "and there is no second way to read a name: the accessor and the Display are one thing, and two would be two spellings"
    );
}

#[test]
fn a_work_items_name_is_written_exactly_as_it_was_typed() {
    for typed in ["  x  ", "AISDLC-PO", "with a space", "tab\there"] {
        let name = WorkItemId::try_from(typed).expect("these are all names");

        assert_eq!(
            name.to_string(),
            typed,
            "the name is written as written: no trimming, no recasing, and a caller that sent {typed:?} gets {typed:?} back"
        );
    }
}

#[test]
fn a_work_items_name_that_says_nothing_is_refused() {
    for blank in ["", "   ", "\t", "\n  \n"] {
        let result = WorkItemId::try_from(blank);

        assert_eq!(
            result,
            Err(AWorkItemThatNamesNothing(blank.to_string())),
            "a work item cannot be approved, persisted or pointed at without a name, and {blank:?} is not one"
        );
    }
}

#[test]
fn the_refused_name_is_refused_and_not_the_trimmed_one() {
    let result = WorkItemId::try_from("   ");

    let refused = result.expect_err("three spaces is not a name");

    assert_eq!(
        refused.0, "   ",
        "a name this system silently tidies is a name a lookup will miss, so what was sent is what is reported back"
    );
}

#[test]
fn the_refusal_says_what_was_refused() {
    let said = AWorkItemThatNamesNothing("  ".to_string()).to_string();

    assert!(
        said.contains("  "),
        "a refusal a reader cannot match against what they wrote teaches nothing: {said}"
    );
}

#[test]
fn an_agents_name_is_written_exactly_as_it_was_typed() {
    let name = AgentIdentifier::try_from("aisdlc-dev-impl").expect("a name is not absent");

    assert_eq!(
        name.to_string(),
        "aisdlc-dev-impl",
        "the name is written as written"
    );
}

#[test]
fn an_agents_name_that_says_nothing_is_refused() {
    for absent in ["", "   ", "\t"] {
        let result = AgentIdentifier::try_from(absent);

        assert_eq!(
            result,
            Err(AnAgentThatIsNotAnIdentity(absent.to_string())),
            "the state machine decides whether an agent is present by asking whether there is one, and {absent:?} is not one"
        );
    }
}

#[test]
fn the_word_that_means_absent_is_refused_as_an_agents_name() {
    let result = AgentIdentifier::try_from("none");

    assert_eq!(
        result,
        Err(AnAgentThatIsNotAnIdentity("none".to_string())),
        "absence is Option::None and never a string that says absent, or a work item in IN_PROGRESS_BY_AGENT would pass the biconditional while nobody worked on it"
    );
}

#[test]
fn a_name_that_merely_contains_the_reserved_word_is_still_a_name() {
    for typed in ["none-of-the-above", "someone", "nonesuch", "a-none"] {
        let result = AgentIdentifier::try_from(typed);

        assert_eq!(
            result.ok().map(|name| name.to_string()),
            Some(typed.to_string()),
            "the reserved word is a word and not a substring: {typed:?} identifies somebody"
        );
    }
}

#[test]
fn the_reserved_word_is_refused_as_written_and_not_by_its_letters() {
    let shouted = AgentIdentifier::try_from("NONE");

    assert_eq!(
        shouted.ok().map(|name| name.to_string()),
        Some("NONE".to_string()),
        "the specification reserves one word as written; a case-insensitive rule would refuse a name a person may legitimately choose, and that is a rule the glossary does not carry"
    );
}

#[test]
fn the_two_names_refuse_different_things() {
    let a_work_item_tracked_by_none = WorkItemId::try_from("none");
    let an_agent_called_none = AgentIdentifier::try_from("none");

    assert_eq!(
        a_work_item_tracked_by_none
            .ok()
            .map(|name| name.to_string()),
        Some("none".to_string()),
        "a work item may be tracked by the name none, and the glossary refuses only a blank there"
    );
    assert_eq!(
        an_agent_called_none.err(),
        Some(AnAgentThatIsNotAnIdentity("none".to_string())),
        "an agent may not, because the state machine asks whether there is one"
    );
}
