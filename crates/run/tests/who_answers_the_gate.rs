//! Who answers the gate, and the name a decision is recorded under.
//!
//! **Two ways a gate is answered and they must not be able to look the same.** A person rules in
//! their own name; **a policy answers in a namespace of its own, and the namespace is applied by
//! the thing that writes the record and never by the thing that reads the setting** — because that
//! is the only place it cannot be bypassed, **and a policy that could be recorded as a person would
//! forge the one thing this record exists to be.**

use aisdlc_domain::approval::WhoDecided;
use aisdlc_run::who_answers_the_gate::{
    APolicyNobodyCanQuote, WhoAnswersTheGate, the_policy_a_deployment_named,
    the_name_the_decision_is_recorded_under,
};

#[test]
fn a_person_is_recorded_verbatim_and_no_namespace_is_added_to_them() {
    let whoever_answers = WhoAnswersTheGate::APersonNamed(WhoDecided::try_from("paligakrzychu").expect("a name"));

    let the_name = the_name_the_decision_is_recorded_under(&whoever_answers);

    assert_eq!(
        the_name.to_string(), "paligakrzychu",
        "**because a person's name in a record is the one thing that cannot be edited** — adding a \
         namespace to somebody would make them harder to find in the very table meant to name them, \
         **and it would be a lie about what they are.** It recorded: {the_name}"
    );
}

#[test]
fn a_policy_is_recorded_under_the_namespace_and_the_name_is_ours_to_quote() {
    let the_policy = the_policy_a_deployment_named(Some("review-what-the-run-found"))
        .expect("a bare policy name is one a person can quote");

    let the_name = the_name_the_decision_is_recorded_under(&the_policy);

    assert_eq!(
        the_name.to_string(), "autopilot:review-what-the-run-found",
        "**because a decision recorded as nobody is not a record, and the namespace is what makes \
         the policy nameable in an argument later** — a decision somebody cannot quote is a \
         decision they cannot question. It recorded: {the_name}"
    );
}

#[test]
fn a_deployment_that_named_no_policy_gets_a_person_and_not_a_policy() {
    let whoever_answers = the_policy_a_deployment_named(None).expect("naming nothing is not a failure");

    assert!(
        matches!(whoever_answers, WhoAnswersTheGate::APersonNamed(_)),
        "**because the ordinary case is a person at a terminal, and a deployment that named nothing \
         must not get an autopilot** — otherwise the setting nobody wrote decides the gate. It gave: \
         {whoever_answers:?}"
    );
}

#[test]
fn a_policy_name_saying_nothing_is_refused_and_the_truncation_is_named() {
    let the_refusal = the_policy_a_deployment_named(Some("   "));

    assert!(
        matches!(&the_refusal, Err(APolicyNobodyCanQuote::ThePolicySaysNothing)),
        "**because a truncated value is the shape this takes** — `autopilot:` with nothing behind \
         it is a claim that carries no claim, **and the fallback would be a run answering its own \
         gate with a policy nobody can quote in an argument.** It refused: {the_refusal:?}"
    );
}

#[test]
fn the_namespace_is_applied_here_and_nowhere_else() {
    let a_policy_naming_itself = WhoAnswersTheGate::ThisPolicyAnswering {
        which_policy: String::from("autopilot:review-what-the-run-found"),
    };

    let the_name = the_name_the_decision_is_recorded_under(&a_policy_naming_itself);

    assert_eq!(
        the_name.to_string(), "autopilot:autopilot:review-what-the-run-found",
        "**because this is the whole of the design and it is deliberately ugly.** The namespace is \
         applied by the thing that writes the record **and not by the reader of the setting**, so \
         there is one place it happens and a caller cannot skip it — **and a deployment that \
         prefixed its own policy name gets a doubled prefix, which is loud and not subtle.** A \
         policy that could be recorded as a person would forge the one thing this record is."
    );
}
