//! Which agent picks up a piece of work, and that the work itself says.
//!
//! **The agent was a name in a function and not a fact about the work**, so every work item in this
//! system was picked up by the repository discovery agent — **including a work about a proposal,
//! which is the product analyst's question and not discovery's.** That is the same defect as a work
//! item that named `"a person"` whoever asked: **the code filling in a fact it should have been
//! given.**
//!
//! **Discovery is the exception and it is stated, because no role answers for it.** `Role` is
//! `PO | DEV | QA | SEC | ARCH`, **and the repository discovery agent answers for none of them**, so
//! naming one would be inventing a discipline the specification does not have.

use aisdlc_run::a_run_picks_up_the_work::{
    a_run_picks_up_the_work_it_was_given, the_agent_takes_the_work_into_its_own_hands,
    the_agent_that_answers_for,
};

/// The role a work item is answered by.
fn a_role(named: &str) -> aisdlc_domain::role::Role {
    aisdlc_domain::role::Role::try_from(named).expect("a discipline")
}

/// A work item standing where work begins, **with one discipline accountable for it.**
fn a_work_the_role_answers(named_role: &str) -> aisdlc_domain::work_item::WorkItem {
    aisdlc_domain::work_item::WorkItem::standing_at(
        aisdlc_domain::work_item_id::WorkItemId::try_from("steamnoid/one#10").expect("a name"),
        aisdlc_domain::who_commissioned::WhoCommissioned::try_from("llm agent").expect("a name"),
        aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
        a_role(named_role),
        aisdlc_domain::stage::Stage::Idle,
    )
    .expect("a work item at a stage")
}

/// The agent each discipline names, **for a claim about the table rather than about one work.**
fn the_agents_the_disciplines_name() -> Vec<String> {
    ["PO", "DEV", "QA", "SEC", "ARCH"]
        .iter()
        .map(|named_role| {
            the_agent_that_answers_for(&a_role(named_role))
                .expect("every discipline has an agent")
                .to_string()
        })
        .collect()
}

#[test]
fn every_role_has_an_agent_and_not_one_of_them_is_discovery() {
    let the_agents = the_agents_the_disciplines_name();

    assert_eq!(
        the_agents.len(),
        5,
        "**because `Role` is five disciplines and each is answered by somebody** — a refusal here \
         would mean a discipline nothing in this system can answer: {the_agents:?}"
    );
    assert!(
        the_agents.iter().all(|one| one != "aisdlc-discover"),
        "**because the repository discovery agent answers for none of the five** — and naming one of \
         them would be inventing a discipline the specification does not have: {the_agents:?}"
    );
}

#[test]
fn the_product_analysts_agent_is_the_name_the_specification_gives_it() {
    assert_eq!(
        the_agent_that_answers_for(&a_role("PO"))
            .expect("the product analyst has an agent")
            .to_string(),
        "aisdlc-po-analyze",
        "**because §10 lists it, and transcribing a name is not inventing one** — a name a system \
         chose for itself would be a name nothing could be held to"
    );
}

#[test]
fn a_run_picks_up_the_agent_the_work_itself_names() {
    let a_proposal_about_a_repository = a_work_the_role_answers("PO");

    let picked_up = a_run_picks_up_the_work_it_was_given(&a_proposal_about_a_repository)
        .expect("the table allows this edge");

    assert_eq!(
        picked_up.the_agent_holding_it().map(|who| who.to_string()),
        Some(String::from("aisdlc-po-analyze")),
        "**because the work already carries the discipline accountable for it** — and a work about a \
         proposal is the product analyst's question, **so the repository discovery agent picking it \
         up is this code filling in a fact the work was holding**: {picked_up:?}"
    );
}

#[test]
fn five_disciplines_are_answered_by_five_agents_and_not_fewer() {
    let the_agents = the_agents_the_disciplines_name();

    let mut the_distinct_ones = the_agents.clone();
    the_distinct_ones.sort();
    the_distinct_ones.dedup();
    assert_eq!(
        the_distinct_ones.len(),
        the_agents.len(),
        "**because five disciplines answered by fewer than five agents is one discipline wearing \
         several names** — and the specification's table is the source: {the_agents:?}"
    );
}

#[test]
fn the_agent_taking_the_work_on_is_the_one_the_work_names_and_not_one_this_file_carries() {
    let a_proposal_about_a_repository = a_work_the_role_answers("PO");

    let picked_up = a_run_picks_up_the_work_it_was_given(&a_proposal_about_a_repository)
        .expect("the table allows this edge");
    let in_progress = the_agent_takes_the_work_into_its_own_hands(&picked_up)
        .expect("the table allows pickup to become work in progress");

    assert_eq!(
        in_progress
            .the_agent_holding_it()
            .map(|who| who.to_string()),
        Some(String::from("aisdlc-po-analyze")),
        "**because four test files used to compose this transition by hand with a name written next \
         to them** — and that is how one wrong agent survived in four places: {in_progress:?}"
    );
}
