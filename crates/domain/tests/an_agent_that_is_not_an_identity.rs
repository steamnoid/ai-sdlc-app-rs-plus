//! The gate for RULE 2.5: an agent is an `AgentIdentifier`, and a string that only means
//! absent never becomes one.
//!
//! **This is the strict one of the two names, and the reason is the state machine.** It
//! decides whether an agent is present by asking whether there is one, so the only thing
//! this type has to guarantee is that a name it did hold really is an agent. That is why
//! `"none"` is refused here and is not refused on a work item, and why the constructor
//! below carries that sentence in full: a reader who does not have it will read the check
//! as pedantry, and it is not pedantry, it is the biconditional.
//!
//! Every case is written out here rather than read out of the code it checks, for the
//! reason `the_stage_is_a_closed_set_of_six.rs` writes down at length: a test that reads
//! its own expectation from the code it checks proves nothing.

use aisdlc_domain::{AgentIdentifier, WorkItemId};

/// The three names that mean absent rather than name anybody, and the whole of what this
/// type refuses.
///
/// **The two blanks and the one word, and the word is a whole word.** `"None"`, `"NONE"`
/// and `" none "` are on the list below instead, because the thing being refused is the
/// value a machine writes to say "no agent" — not a word that happens to look like one.
const THE_NAMES_THAT_MEAN_ABSENT: [&str; 3] = ["", "   ", "none"];

/// The names that only look absent, and are agents all the same.
///
/// **Case is not tidied and a name is not trimmed before it is compared.** An agent is
/// identified by the exact string it is registered under, so `"None"` is an agent called
/// `None`, `"NONE"` is an agent called `NONE`, and `" none "` is an agent whose name has
/// spaces in it. Lowercasing any of them would answer about an agent nobody registered,
/// and trimming one would answer about a different agent than the one that was asked for.
const THE_NAMES_THAT_MERELY_LOOK_ABSENT: [&str; 3] = ["None", "NONE", " none "];

#[test]
fn a_name_that_only_means_absent_is_refused() {
    for name in THE_NAMES_THAT_MEAN_ABSENT {
        let refused = AgentIdentifier::named(name);
        assert!(
            refused.is_err(),
            "{name:?} only means that no agent is here, and the state machine decides whether an agent is present by asking whether there is one"
        );
    }
}

#[test]
fn the_refusal_is_an_agent_that_is_not_an_identity() {
    let Err(refusal) = AgentIdentifier::named("none") else {
        panic!("the word none is not an identity, so the refusal is the only answer");
    };
    let said = std::any::type_name_of_val(&refusal);
    assert!(
        said.ends_with("::AnAgentThatIsNotAnIdentity"),
        "a string that only means absent is refused as an agent that is not an identity, and this crate carries two refusals that have to be told apart: {said}"
    );
}

#[test]
fn a_refused_name_comes_back_as_it_was_written() {
    for name in THE_NAMES_THAT_MEAN_ABSENT {
        let Err(refusal) = AgentIdentifier::named(name) else {
            panic!("{name:?} only means absent, so there is a refusal to read back");
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
    let Err(refusal) = AgentIdentifier::named("none") else {
        panic!("the word none is not an identity, so the refusal is the only answer");
    };
    let said = refusal.to_string();
    assert!(
        said.contains("none"),
        "a reader debugging a run reads the written refusal rather than the accessor, so it carries the name too: {said}"
    );
}

#[test]
fn a_name_that_merely_looks_absent_is_an_agent() {
    for name in THE_NAMES_THAT_MERELY_LOOK_ABSENT {
        let Ok(named) = AgentIdentifier::named(name) else {
            panic!("{name:?} is not the value a machine writes to mean no agent, so it names one");
        };
        assert_eq!(
            named.to_string(),
            name,
            "Display writes the name exactly as written, so a name a person typed and a name a run reads back are the same name"
        );
    }
}

#[test]
fn the_constructor_and_the_trait_agree_about_every_name() {
    for name in THE_NAMES_THAT_MEAN_ABSENT
        .into_iter()
        .chain(THE_NAMES_THAT_MERELY_LOOK_ABSENT)
    {
        let from_the_constructor = AgentIdentifier::named(name);
        let from_the_trait = AgentIdentifier::try_from(name);
        assert_eq!(
            from_the_constructor.is_ok(),
            from_the_trait.is_ok(),
            "there is one answer to whether {name:?} is an identity, and the constructor and the trait are two ways of asking the same question"
        );
    }
}

#[test]
fn the_two_types_disagree_about_the_word_none_and_both_are_right() {
    assert!(
        WorkItemId::named("none").is_ok(),
        "the input is the word none, and a work item may be tracked by it: a work item nobody has named yet and an agent that does not exist are different sentences, so refusing it here would refuse a delivery for the name its own tracker gave it"
    );
    assert!(
        AgentIdentifier::named("none").is_err(),
        "the same input is refused here, because the state machine decides whether an agent is present by asking whether there is one, so a work item in IN_PROGRESS_BY_AGENT holding a blank agent would pass the biconditional while nobody was working on it"
    );
}
