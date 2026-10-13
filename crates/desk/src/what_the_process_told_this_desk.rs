//! What the process told this desk, read together before anything is copied.
//!
//! **What a person typed with what the process was told, into everything a run starts with.**
//! The arguments say what to work on and where; the process says where a model answers, which
//! one, with what key, and who answers at the gates. **A model nobody configured travels as
//! nothing**, and the desk still refuses before it copies anything.

use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_outside::the_environment::TheEnvironment;

use crate::the_run_is_started::WhatThisDeskWasGiven;
use crate::what_a_run_was_asked_for::{
    WhatWasAskedForThatCannotMeanAnything, what_a_run_was_asked_for,
};

/// Where a model answers, as a deployment named it.
const WHERE_A_MODEL_ANSWERS: &str = "LLM_ENDPOINT";

/// Which model answers there, as a deployment named it.
const WHICH_MODEL_ANSWERS_THERE: &str = "LLM_MODEL";

/// The key a delivery acts with, as a deployment supplied it.
const THE_KEY_A_DEPLOYMENT_SUPPLIED: &str = "OPENAI_API_KEY";

/// Who answers at the gates, as a deployment named them.
const WHO_ANSWERS_AT_THE_GATES: &str = "AISDLC_WHO_DECIDES";

/// Who does the reasoning here, as a deployment named them.
const WHO_DOES_THE_REASONING_HERE: &str = "LLM_ORGANISATION";

/// Who answers when nobody was named, **because the person who ran the binary is the operator.**
const THE_ONE_RUNNING_THE_BINARY: &str = "the operator";

/// What the process told this desk, **typed and set together or the refusal naming the argument.**
pub fn what_the_process_told_this_desk(
    what_a_person_typed: &[String],
    the_environment: &TheEnvironment,
) -> Result<WhatThisDeskWasGiven, WhatWasAskedForThatCannotMeanAnything> {
    let what_a_person_was_asked_for = what_a_run_was_asked_for(what_a_person_typed)?;
    Ok(WhatThisDeskWasGiven {
        what_a_person_was_asked_for,
        the_endpoint: the_variable(WHERE_A_MODEL_ANSWERS, the_environment),
        which_model: the_variable(WHICH_MODEL_ANSWERS_THERE, the_environment),
        the_key: the_variable(THE_KEY_A_DEPLOYMENT_SUPPLIED, the_environment),
        who_answers: who_answers_at_the_gates(the_environment),
        which_organisation: which_organisation_reasons_here(the_environment)?,
    })
}

/// What one variable said, **and nothing when it said nothing at all.**
fn the_variable(its_name: &str, the_environment: &TheEnvironment) -> Option<String> {
    the_environment
        .the_variable_a_deployment_set(its_name)
        .map(String::from)
}

/// Who answers at the gates, **or the one running the binary when nobody was named.**
fn who_answers_at_the_gates(the_environment: &TheEnvironment) -> String {
    the_variable(WHO_ANSWERS_AT_THE_GATES, the_environment)
        .unwrap_or_else(|| String::from(THE_ONE_RUNNING_THE_BINARY))
}

/// Who reasons about the run, **and nothing when nobody was named.**
fn which_organisation_reasons_here(
    the_environment: &TheEnvironment,
) -> Result<Option<WhichOrganisationReasons>, WhatWasAskedForThatCannotMeanAnything> {
    let Some(what_was_said) = the_variable(WHO_DOES_THE_REASONING_HERE, the_environment) else {
        return Ok(None);
    };
    WhichOrganisationReasons::try_from(what_was_said.as_str())
        .map(Some)
        .map_err(
            |_why_not| WhatWasAskedForThatCannotMeanAnything::NoOrganisationThatReasons {
                what_was_said,
            },
        )
}
