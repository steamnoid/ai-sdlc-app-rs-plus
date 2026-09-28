//! An agent may ask for work and may not rule on the result.
//!
//! **That prohibition is an assertion here and not a convention**, which is why the refusal
//! is a type that exists to be raised rather than a paragraph a reader is asked to respect.
//! An earlier implementation carried the check in the gate; putting it on the decision
//! itself means a ruling read from a checkpoint is refused on the way in, and not only when
//! it happens to reach a gate.
//!
//! **The namespace is what makes the rule decidable.** `aisdlc-` is an agent and
//! `autopilot:` is a policy, and until this repository's glossary was corrected the two
//! documents disagreed about which was which — a policy sharing the agents' namespace
//! would have been refused along with them.
//!
//! No file is read here, because a test of the domain may not reach for a filesystem.

use aisdlc_domain::approval::{
    AgentDecidedError, AnArtifactVersion, Approval, ApprovalDecision, WhichArtifact, WhoDecided,
};
use aisdlc_domain::work_item_id::WorkItemId;

const THE_PREFIX_AN_AGENT_IS_NAMED_WITH: &str = "aisdlc-";

fn a_work_item() -> WorkItemId {
    WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank")
}

fn the_first_version() -> AnArtifactVersion {
    AnArtifactVersion::try_from(1).expect("one is a version")
}

fn the_report() -> WhichArtifact {
    WhichArtifact::try_from("QaPlan").expect("a name is not blank")
}

fn a_person() -> WhoDecided {
    WhoDecided::try_from("paligakrzychu").expect("a name is not blank")
}

fn an_agent() -> WhoDecided {
    WhoDecided::try_from("aisdlc-dev-impl").expect("a name is not blank")
}

fn a_policy() -> WhoDecided {
    WhoDecided::try_from("autopilot:approve-low-risk").expect("a name is not blank")
}

fn a_ruling_by(decided_by: WhoDecided, approved: bool) -> ApprovalDecision {
    ApprovalDecision::the_ruling_about(
        a_work_item(),
        the_report(),
        the_first_version(),
        approved,
        decided_by,
    )
    .expect("this is not an agent")
}

#[test]
fn an_agent_may_not_rule_on_its_own_result() {
    for agent in ["aisdlc-dev-impl", "aisdlc-po-analyze", "aisdlc-approval"] {
        let who = WhoDecided::try_from(agent).expect("a name is not blank");

        let result = ApprovalDecision::the_ruling_about(
            a_work_item(),
            the_report(),
            the_first_version(),
            true,
            who,
        );

        assert_eq!(
            result,
            Err(AgentDecidedError(agent.to_string())),
            "{agent} is an agent, and an agent approving its own output is forbidden rather than discouraged"
        );
    }
}

#[test]
fn an_agent_is_refused_even_when_it_refuses() {
    let result = ApprovalDecision::the_ruling_about(
        a_work_item(),
        the_report(),
        the_first_version(),
        false,
        an_agent(),
    );

    assert!(
        result.is_err(),
        "an agent may not rule either way: a refusal from an agent is a model output, and a gate that accepted it would be reading itself"
    );
}

#[test]
fn the_refusal_says_who_tried_to_rule_and_why() {
    let said = AgentDecidedError("aisdlc-dev-impl".to_string()).to_string();

    assert!(
        said.contains("aisdlc-dev-impl"),
        "a refusal naming nobody teaches nothing: {said}"
    );
    assert!(
        said.contains("result"),
        "and it says the rule rather than only the verdict, so the reader learns it: {said}"
    );
}

#[test]
fn a_person_and_a_policy_may_both_rule() {
    for who in [a_person(), a_policy()] {
        let ruling = a_ruling_by(who.clone(), true);

        assert_eq!(
            ruling.decided_by.to_string(),
            who.to_string(),
            "{who} ruled and the record says so verbatim: a person and a policy are both real deciders"
        );
    }
}

#[test]
fn a_policy_is_not_read_as_an_agent_because_the_namespaces_differ() {
    assert!(
        !a_policy()
            .to_string()
            .starts_with(THE_PREFIX_AN_AGENT_IS_NAMED_WITH),
        "autopilot: is not aisdlc-, and the whole rule depends on the two namespaces being apart"
    );
    assert!(
        an_agent()
            .to_string()
            .starts_with(THE_PREFIX_AN_AGENT_IS_NAMED_WITH)
    );
}

#[test]
fn an_approval_leaves_a_record_behind() {
    let approved = a_ruling_by(a_person(), true);

    let record = Approval::what_the_ruling_left_behind(&approved);

    assert_eq!(
        record.map(|it| (it.decided_by.to_string(), it.the_version.to_string())),
        Some(("paligakrzychu".to_string(), "1".to_string())),
        "an affirmative ruling leaves a record naming who ruled and which version they ruled about"
    );
}

#[test]
fn a_refusal_leaves_no_record_and_that_is_not_a_fault() {
    let refused = a_ruling_by(a_person(), false);

    assert_eq!(
        Approval::what_the_ruling_left_behind(&refused),
        None,
        "a refusal is a decision too and it produces no record, so silence is the outcome and not a failure"
    );
}

#[test]
fn a_record_names_the_exact_artifact_at_the_exact_version() {
    let ruling = a_ruling_by(a_person(), true);

    let record = Approval::what_the_ruling_left_behind(&ruling).expect("approved");

    assert_eq!(
        record.which_artifact,
        the_report(),
        "the record names the artifact"
    );
    assert_eq!(
        record.the_version,
        the_first_version(),
        "and the version of it"
    );
    assert_eq!(
        record.work_item_id,
        a_work_item(),
        "and the work item it is about"
    );
}

#[test]
fn a_ruling_carries_the_who_verbatim_and_never_a_replacement() {
    let ruling = a_ruling_by(a_policy(), true);

    assert_eq!(
        ruling.decided_by.to_string(),
        "autopilot:approve-low-risk",
        "the ruling keeps the decider exactly as it was given, and a record naming a person for a policy's decision would be a forged audit trail"
    );
}
