//! The gate for RULE 2.5: a work item is tracked by a `WorkItemId`, and a name that is
//! blank never becomes one.
//!
//! **The property that matters is the compiler's, and it is why most of this file is
//! documentation.** A work item cannot be approved, persisted or pointed at by an
//! approval without a name, and `WorkItemId` has no door that takes a name without
//! asking, so a caller holding one cannot have built it out of nothing at all. What the
//! compiler cannot hold is everything around that property: which names are refused, what
//! a refusal says, and whether the name comes back the way it was sent. Those are the
//! assertions below.
//!
//! Every case is written out here rather than read out of the code it checks, for the
//! reason `the_stage_is_a_closed_set_of_six.rs` writes down at length: a test that reads
//! its own expectation from the code it checks proves nothing.

use aisdlc_domain::WorkItemId;

/// The names that name nothing, and the whole of what this type refuses.
///
/// Two and not a rule, on purpose. A table written as a repetition of a space would prove
/// the rule and say nothing about which names are in it, and the one thing a reader wants
/// to know about a name this type accepts is the answer, spelled out where it can be read.
const THE_NAMES_THAT_NAME_NOTHING: [&str; 2] = ["", "   "];

/// The names a work item may be tracked by, even though each of them looks like nothing.
///
/// **A blank is the only thing refused, and `"none"` is on this list.** A work item
/// nobody has named yet and an agent that does not exist are different sentences, and
/// this type is the first of them: a tracker of this deployment hands out identifiers,
/// and one of the identifiers it hands out is the string `none`. Refusing it here would
/// refuse a delivery for the name its own tracker gave it. The agent is the second
/// sentence, and it is the one the state machine asks about, so it is refused there and
/// not here.
const THE_NAMES_THAT_ONLY_LOOK_LIKE_NOTHING: [&str; 4] =
    ["0", "none", "  x  ", "aisdlc-po-analyze"];

#[test]
fn a_blank_name_is_refused() {
    for name in THE_NAMES_THAT_NAME_NOTHING {
        let refused = WorkItemId::named(name);
        assert!(
            refused.is_err(),
            "{name:?} names nothing, and a work item cannot be approved, persisted or pointed at by an approval without a name"
        );
    }
}

#[test]
fn the_refusal_is_a_work_item_that_names_nothing() {
    let Err(refusal) = WorkItemId::named("   ") else {
        panic!("three spaces name nothing, so the refusal is the only answer");
    };
    let said = std::any::type_name_of_val(&refusal);
    assert!(
        said.ends_with("::AWorkItemThatNamesNothing"),
        "a blank name is refused as a work item that names nothing, and this crate carries two refusals that have to be told apart: {said}"
    );
}

#[test]
fn a_refused_name_comes_back_as_it_was_written() {
    for name in THE_NAMES_THAT_NAME_NOTHING {
        let Err(refusal) = WorkItemId::named(name) else {
            panic!("{name:?} names nothing, so there is a refusal to read back");
        };
        assert_eq!(
            refusal.what_was_looked_for(),
            name,
            "the name is refused and not the trimmed name, because a name this system silently tidies is a name a lookup will miss"
        );
    }
}

#[test]
fn a_refusal_writes_the_name_it_refused() {
    let Err(refusal) = WorkItemId::named("   ") else {
        panic!("three spaces name nothing, so the refusal is the only answer");
    };
    let said = refusal.to_string();
    assert!(
        said.contains("   "),
        "a reader debugging a run reads the written refusal rather than the accessor, so it carries the name too: {said}"
    );
}

#[test]
fn a_name_that_only_looks_like_nothing_is_a_work_item_id() {
    for name in THE_NAMES_THAT_ONLY_LOOK_LIKE_NOTHING {
        let Ok(tracked_by) = WorkItemId::named(name) else {
            panic!("{name:?} is a name, and a work item is tracked by whatever it is tracked by");
        };
        assert_eq!(
            tracked_by.to_string(),
            name,
            "Display writes the name exactly as written, so a name a person typed and a name a run reads back are the same name"
        );
    }
}

#[test]
fn the_constructor_and_the_trait_agree_about_every_name() {
    for name in THE_NAMES_THAT_NAME_NOTHING
        .into_iter()
        .chain(THE_NAMES_THAT_ONLY_LOOK_LIKE_NOTHING)
    {
        let from_the_constructor = WorkItemId::named(name);
        let from_the_trait = WorkItemId::try_from(name);
        assert_eq!(
            from_the_constructor.is_ok(),
            from_the_trait.is_ok(),
            "there is one answer to whether {name:?} is a name, and the constructor and the trait are two ways of asking the same question"
        );
    }
}
