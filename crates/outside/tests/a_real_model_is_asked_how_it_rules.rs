//! A real model, asked over the network, ruling on a real question.
//!
//! **Everything in this repository so far has been a double.** The contract was proved against a
//! stub that answers with a shape no endpoint returns, and the one time that mattered the model
//! could not rule at all and the suite was green. **This is the tier that watched a real model fail
//!**, and it is the only thing in this system that can say the plumbing works against something
//! that is not this repository.
//!
//! **It is ignored, and it says so, and it prints why it skipped.** A run that skipped silently is a
//! green run that called nothing, and a green run that called nothing is a lie told with a tick.

use aisdlc_domain::approval::{AnArtifactVersion, WhichArtifact, WhoDecided};
use aisdlc_domain::stage::Stage;
use aisdlc_domain::work_item::WorkItem;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_outside::how_this_deployment_is_set_up::how_this_deployment_is_set_up;
use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_run::a_model_is_asked_how_it_rules::{
    WhereAModelIs, the_model_is_asked_how_it_rules_on,
};

/// This deployment, or the reason there is nothing to ask.
fn a_deployment_that_may_ask_a_model() -> Option<(TheProgramAsItRuns, WhereAModelIs)> {
    let what_the_process_was_told = TheEnvironment::reading_what_the_process_was_told();
    let how_it_is_set_up = how_this_deployment_is_set_up(&what_the_process_was_told);
    let the_credential = how_it_is_set_up.the_credential().cloned();
    if !how_it_is_set_up.may_a_model_rule().may_be_asked() {
        eprintln!(
            "SKIPPED: AISDLC_A_MODEL_RULES_HERE is not 'yes' on this deployment, and a model that \
             may not rule is a model this system will not ask"
        );
        return None;
    }
    let Some(the_credential) = the_credential else {
        eprintln!("SKIPPED: the flag is on and OPENAI_API_KEY is not set, so nothing was asked");
        return None;
    };
    let where_the_model_is = WhereAModelIs::where_a_deployment_put_its_model(
        what_the_process_was_told.the_variable_a_deployment_set("LLM_ENDPOINT"),
        what_the_process_was_told.the_variable_a_deployment_set("LLM_MODEL"),
    );
    let where_the_model_is = match where_the_model_is {
        Ok(where_it_is) => where_it_is,
        Err(why_not) => {
            eprintln!("SKIPPED: {why_not}");
            return None;
        }
    };
    Some((
        TheProgramAsItRuns::as_it_is_on_this_host(Some(the_credential)),
        where_the_model_is,
    ))
}

/// The name a real model's ruling is recorded under, and never a person's.
fn the_model_that_ruled() -> WhoDecided {
    WhoDecided::try_from("a model: the configured one").expect("a name")
}

fn a_question_about_one_requirement() -> aisdlc_run::ApprovalQuestion {
    aisdlc_run::the_question_put_to_somebody(
        &WorkItem::standing_at(
            WorkItemId::try_from("A-REAL-MODEL-1").expect("a name"),
            aisdlc_domain::who_commissioned::WhoCommissioned::try_from("a person").expect("a name"),
            aisdlc_domain::repository::Repository::try_from("steamnoid/ai-sdlc-app-rs-plus")
                .expect("two halves"),
            aisdlc_domain::role::Role::try_from("PO").expect("a discipline"),
            Stage::AwaitingHumanApproval,
        )
        .expect("a work item answering a request can stand at the gate"),
        &WhichArtifact::try_from("ARequirements").expect("a name"),
        AnArtifactVersion::try_from(1).expect("one is a version"),
    )
    .expect("a work item at the gate can be asked about")
}

#[test]
#[ignore = "asks a real model over the network, and needs a credential and the flag"]
fn a_real_model_rulings_is_read_out_of_its_own_answer() {
    let Some((the_program, where_the_model_is)) = a_deployment_that_may_ask_a_model() else {
        return;
    };

    let how_it_went = the_model_is_asked_how_it_rules_on(
        &the_program,
        &a_question_about_one_requirement(),
        &the_model_that_ruled(),
        &where_the_model_is,
    );

    let the_ruling = how_it_went.expect(
        "a real model was asked and this system could not read its answer — which is the defect this \
         tier exists to find, and the answer is quoted in the error",
    );
    eprintln!(
        "the real model at {} said {}",
        where_the_model_is.the_endpoint,
        if the_ruling.approved {
            "APPROVED"
        } else {
            "REFUSED"
        }
    );
    assert!(
        the_ruling.decided_by == the_model_that_ruled(),
        "**because the record names the model and not a person** — a real model that answered here \
         has been recorded, and if the name on the record is anybody's, the audit trail is forged"
    );
}
