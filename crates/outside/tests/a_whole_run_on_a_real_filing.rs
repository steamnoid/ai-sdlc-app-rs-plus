//! A whole run, on a real filing, with a real model ruling on it.
//!
//! **Every part of this existed and none had ever been in one run.** A read and a write that did
//! not meet, a work item that stood at `IDLE`, a gate that took only a person's decision, and a
//! model that ruled with nowhere for the ruling to go — **four half-systems, each proved against the
//! real thing, and no proof that they compose.**
//!
//! **And that is the whole point of this tier.** Each half was true on its own; the question this
//! asks is whether the run that walks all of them is true at all, and **a run that dies at the
//! eleventh step has still told the truth about where the eleventh step is.**

use aisdlc_domain::stage::Stage;
use aisdlc_outside::a_filing_is_read::AFilingIsRead;
use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_run::a_model_is_asked_how_it_rules::the_model_is_asked_how_it_rules_on;
use aisdlc_run::a_ruling_is_recorded::the_gate_records_the_ruling_it_was_given;
use aisdlc_run::a_ruling_moves_the_work::the_ruling_moves_the_work;
use aisdlc_run::an_issue_to_read::AnIssueToRead;
use aisdlc_run::what_an_agent_has_produced::WhatAnAgentHasProduced;
use aisdlc_run::what_the_agent_presents::an_agent_presents_what_it_produced_and_the_question_is_put;

/// This deployment, or the reason there is nothing to ask.
fn this_deployment_that_may_ask_a_model() -> Option<(
    TheProgramAsItRuns,
    aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs,
)> {
    let what_the_process_was_told = TheEnvironment::reading_what_the_process_was_told();
    let how_it_is_set_up =
        aisdlc_outside::how_this_deployment_is_set_up::how_this_deployment_is_set_up(
            &what_the_process_was_told,
        );
    if !how_it_is_set_up.may_a_model_rule().may_be_asked() {
        eprintln!("SKIPPED: AISDLC_A_MODEL_RULES_HERE is not 'yes' here");
        return None;
    }
    let the_credential = how_it_is_set_up.the_credential().cloned();
    if the_credential.is_none() {
        eprintln!("SKIPPED: the flag is on and OPENAI_API_KEY is not set");
        return None;
    }
    let the_credential = the_credential.expect("just checked");
    let the_program = TheProgramAsItRuns::as_it_is_on_this_host(Some(the_credential))
        .with_what_a_deployment_named(&what_the_process_was_told);
    let where_the_model_is =
        aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs::where_a_deployment_put_its_model(
            what_the_process_was_told.the_variable_a_deployment_set("LLM_ENDPOINT"),
            what_the_process_was_told.the_variable_a_deployment_set("LLM_MODEL"),
        )
        .expect("a deployment names where its model is");
    Some((the_program, where_the_model_is))
}

/// The filing this run is about, as a person named it.
fn a_filing_the_run_was_given() -> AnIssueToRead {
    AnIssueToRead::the_issue_to_read(
        aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
        8,
    )
    .expect("eight is an issue number")
}

#[test]
#[ignore = "reads and writes GitHub and asks a real model, and needs gh authenticated and .env loaded"]
fn a_filing_is_read_ruled_on_recorded_and_moved_all_in_one_run() {
    let Some((the_program, where_the_model_is)) = this_deployment_that_may_ask_a_model() else {
        return;
    };

    let mut the_reader = AFilingIsRead;
    let what_was_read = the_reader
        .the_filing_is_read(&a_filing_the_run_was_given())
        .expect("issue 8 exists and was filed by a person");
    let what_it_produced =
        WhatAnAgentHasProduced::new("DiscoveryReport", 1).expect("declared here");
    let the_work = aisdlc_run::a_filing_is_answered::TheWork::a_filing_becomes(
        &what_was_read,
        "WELLBEINGT-42",
        "PO",
    );
    let work_an_agent_picks_up = the_work
        .the_run_picks_it_up()
        .expect("the table allows this edge");
    let work_in_progress = aisdlc_domain::state_machine::transition(
        work_an_agent_picks_up,
        Stage::InProgressByAgent,
        Some(aisdlc_run::a_run_picks_up_the_work::the_agent_that_picks_up_work()),
    )
    .expect("the table allows pickup to become work in progress");
    let a_question = an_agent_presents_what_it_produced_and_the_question_is_put(
        &work_in_progress,
        &what_it_produced,
    )
    .expect("an agent working on a work may present it");
    let who_ruled = aisdlc_domain::approval::WhoDecided::try_from("a model: the configured one")
        .expect("a name");
    eprintln!("ruling on {:?}", a_question.about_which_artifact());
    let the_ruling = the_model_is_asked_how_it_rules_on(
        &the_program,
        &a_question,
        &who_ruled,
        &where_the_model_is,
    )
    .expect("a real model was asked and its answer was read");
    let what_it_left = the_gate_records_the_ruling_it_was_given(&the_ruling);
    let the_work_after = the_ruling_moves_the_work(&work_in_progress, the_ruling.approved);
    let where_it_stands = the_work_after
        .as_ref()
        .ok()
        .map(aisdlc_domain::work_item::WorkItem::the_stage_it_stands_at);

    eprintln!(
        "ruled {:?}; recorded {:?}; the work now stands at {where_it_stands:?}",
        the_ruling.approved, what_it_left
    );

    assert!(
        where_it_stands == Some(Stage::Done) || where_it_stands == Some(Stage::AwaitingAgentPickup),
        "**because a ruling moves the work, and only to where the table says a ruling goes** — \\
         anything else means this run invented a move: {where_it_stands:?}"
    );
    assert_eq!(
        what_it_left.as_ref().map(|what| what.decided_by.0.as_str()),
        if the_ruling.approved {
            Some("a model: the configured one")
        } else {
            None
        },
        "**because a record names whoever answered and a refusal leaves nothing** — and a record of \\
         a refusal would be a record of something that was approved: {what_it_left:?}"
    );
    assert!(
        a_question
            .about_which_artifact()
            .0
            .as_str()
            .eq_ignore_ascii_case("discoveryreport"),
        "**and the gate was asked about the artifact that was presented** — because a gate asked \\
         about something else is a gate whose approval is about an artifact nobody saw: \\
         {a_question:?}"
    );
}
