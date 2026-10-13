//! The run this desk starts, **and the refusal when the desk cannot be configured to start one.**
//!
//! **Every collaborator here is something a deployment configured and none of them is something this
//! system chose.** The model address, the model, the credential and the name at the terminal are all
//! facts a person typed somewhere, **and a fallback for any of them is a lie with a position** — which
//! is what happened the first time the sibling ran, when nothing was configured and the run went to
//! a local address nobody had read.
//!
//! **A refusal happens here, before the first question reaches a model and before anything is
//! copied**, because a desk that starts a run it could not configure has started nothing — **and has
//! copied a repository**.
//!
//! **The page is what the desk has to say.** A run that produced eight artifacts and a pull request
//! and told nobody is a run nobody will read, **so the last thing this function does is name the one
//! file a person should open.**

use aisdlc_domain::agent_identifier::AgentIdentifier;
use aisdlc_domain::approval::WhoDecided;
use aisdlc_domain::role::Role;
use aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item_id::WorkItemId;
use aisdlc_outside::the_model_as_it_is_asked::TheModelAsItIsAsked;
use aisdlc_outside::the_program_as_it_runs::TheProgramAsItRuns;
use aisdlc_outside::the_project_on_disk::TheProjectAsItIsOnDisk;
use aisdlc_run::APersonToDecide;
use aisdlc_run::AReasoningModel;
use aisdlc_run::TheProjectOnDisk;
use aisdlc_run::a_branch_goes_to_a_remote::ABranchToPush;
use aisdlc_run::a_credential::ACredential;
use aisdlc_run::a_model_is_asked_how_it_rules::WhereAModelIs;
use aisdlc_run::a_person_answers_at_a_terminal::APersonAtTheTerminal;
use aisdlc_run::a_program_we_run::AProgramWeRun;
use aisdlc_run::a_pull_request_is_opened::the_request_a_run_makes_of_its_own_change;
use aisdlc_run::a_run_from_a_request_to_a_pull_request::{
    ARunThatDidNotFinish, WhatARunLeft, WhatThereIsToDeliver,
    a_run_from_a_request_to_a_pull_request,
};
use aisdlc_run::where_a_run_stands::WhereARunStands;

use crate::what_a_run_was_asked_for::WhatARunWasAskedFor;

/// What this desk was given, **and every field is a fact somebody typed somewhere.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatThisDeskWasGiven {
    /// What a person asked for.
    pub what_a_person_was_asked_for: WhatARunWasAskedFor,
    /// The address the model answers at, **and nothing means this system chooses one.**
    pub the_endpoint: Option<String>,
    /// Which model answers there, **and nothing means this system invents one.**
    pub which_model: Option<String>,
    /// The key a delivery acts with, **and nothing is run without one.**
    pub the_key: Option<String>,
    /// Who answers at the gates, **in their own words.**
    pub who_answers: String,
    /// Who reasons about the run, **and nothing travels when nobody was named.**
    pub which_organisation: Option<WhichOrganisationReasons>,
}

/// Why this desk could not start a run, **and each is a fact about a different thing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyThisRunCouldNotStart {
    /// No model was configured, **and a fallback is a lie with a position.**
    NoModelWasConfigured {
        /// What the deployment was told, **in the model's own words.**
        why_not: String,
    },
    /// No key was supplied, **and a delivery without one is a branch nothing authenticated pushed.**
    NoKeyWasSupplied,
    /// Nobody named who reasons about the run, **and a run that does not know who reasons about
    /// it has nobody to ask.**
    NoOrganisationWasConfigured,
    /// The terminal is named in a machine's namespace, **and a decider named that way cannot produce
    /// a ruling even to refuse with.**
    TheTerminalIsAMachine {
        /// What the terminal said it was called.
        what_it_says_it_is: String,
    },
    /// The work this run does has no name, **and the project directory is derived from it** — so two
    /// runs over one filing would write over each other's artifacts.
    NothingToCallTheWorkBy,
    /// The run itself refused, **and the run's own refusal travels whole.**
    TheRunRefused {
        /// What the run said.
        why_not: String,
    },
}

impl std::fmt::Display for WhyThisRunCouldNotStart {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let why_not = match self {
            Self::NoModelWasConfigured { why_not } => {
                format!("this desk will not choose a model: {why_not}")
            }
            Self::NoKeyWasSupplied => {
                String::from("no key was supplied, and a delivery without one is nothing")
            }
            Self::NoOrganisationWasConfigured => String::from(
                "no organisation that reasons was configured, and a run that does not know who \
                 reasons about it has nobody to ask",
            ),
            Self::TheTerminalIsAMachine { what_it_says_it_is } => format!(
                "the terminal calls itself '{what_it_says_it_is}', and a decider in a machine's \
                 namespace cannot produce a ruling even to refuse with"
            ),
            Self::NothingToCallTheWorkBy => {
                String::from("this work has no name, and the project directory is derived from it")
            }
            Self::TheRunRefused { why_not } => format!("the run did not finish: {why_not}"),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for WhyThisRunCouldNotStart {}

/// The run this desk starts, **and what it says when it has one.**
pub fn the_run_is_started_with(
    what_this_desk_was_given: &mut WhatThisDeskWasGiven,
) -> Result<String, WhyThisRunCouldNotStart> {
    let mut whoever_answers = a_person_at_the_terminal(what_this_desk_was_given)?;
    let who_reasoned = which_organisation_reasons(what_this_desk_was_given)?;
    let mut the_project = the_project_this_desk_writes_to(what_this_desk_was_given);
    let mut the_program = the_program_that_delivers(what_this_desk_was_given)?;
    let mut a_model = the_model_this_desk_asks(what_this_desk_was_given)?;
    let the_run = a_run_of_this_work(what_this_desk_was_given)?;
    let what_there_is = what_there_is_to_deliver(what_this_desk_was_given);
    let left = the_run_is_run(
        &the_run,
        &mut the_project,
        &mut a_model,
        who_reasoned,
        &mut whoever_answers,
        &what_there_is,
        &mut the_program,
    );
    a_page_or_a_refusal(left)
}

/// The project this desk writes into, **and it is the place a person named** — so two runs over
/// one filing find the same directory.
fn the_project_this_desk_writes_to(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> TheProjectAsItIsOnDisk {
    TheProjectAsItIsOnDisk::at(
        what_this_desk_was_given
            .what_a_person_was_asked_for
            .where_it_may_write(),
    )
}

/// The program that delivers, **and it is one of two because the model is asked over another.**
fn the_program_that_delivers(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> Result<TheProgramAsItRuns, WhyThisRunCouldNotStart> {
    Ok(TheProgramAsItRuns::as_it_is_on_this_host(Some(
        a_credential(what_this_desk_was_given)?,
    )))
}

/// The model this desk asks, **at the address a deployment named and nowhere else.**
fn the_model_this_desk_asks(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> Result<TheModelAsItIsAsked<TheProgramAsItRuns>, WhyThisRunCouldNotStart> {
    let where_the_model_is = where_the_model_is(what_this_desk_was_given)?;
    let the_program =
        TheProgramAsItRuns::as_it_is_on_this_host(Some(a_credential(what_this_desk_was_given)?));
    Ok(
        TheModelAsItIsAsked::the_model_where_a_deployment_put_its_model(
            where_the_model_is,
            the_program,
        ),
    )
}

/// The run from its request to its pull request, **over the collaborators this desk built.**
fn the_run_is_run(
    the_run: &WhereARunStands,
    the_project: &mut impl TheProjectOnDisk,
    a_model: &mut impl AReasoningModel,
    who_reasoned: WhichOrganisationReasons,
    whoever_answers: &mut impl APersonToDecide,
    what_there_is: &WhatThereIsToDeliver,
    the_program: &mut impl AProgramWeRun,
) -> Result<WhatARunLeft, ARunThatDidNotFinish> {
    a_run_from_a_request_to_a_pull_request(
        the_run,
        the_project,
        a_model,
        who_reasoned,
        whoever_answers,
        what_there_is,
        the_program,
    )
}

/// Who reasons about the run, **and a run that does not know has nobody to ask.**
fn which_organisation_reasons(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> Result<WhichOrganisationReasons, WhyThisRunCouldNotStart> {
    what_this_desk_was_given
        .which_organisation
        .ok_or(WhyThisRunCouldNotStart::NoOrganisationWasConfigured)
}

/// The page the run left, **or the refusal when it left none.**
fn a_page_or_a_refusal(
    left: Result<WhatARunLeft, ARunThatDidNotFinish>,
) -> Result<String, WhyThisRunCouldNotStart> {
    match left {
        Ok(what_a_run_left) => Ok(where_a_person_can_read_what_happened(&what_a_run_left)),
        Err(why_not) => Err(the_refusal_the_run_left(&why_not)),
    }
}

/// The refusal the run left, **and it travels whole because a page claimed for it would send a
/// reader to a file that is not there.**
fn the_refusal_the_run_left(why_not: &ARunThatDidNotFinish) -> WhyThisRunCouldNotStart {
    WhyThisRunCouldNotStart::TheRunRefused {
        why_not: why_not.to_string(),
    }
}

fn where_the_model_is(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> Result<WhereAModelIs, WhyThisRunCouldNotStart> {
    WhereAModelIs::where_a_deployment_put_its_model(
        what_this_desk_was_given.the_endpoint.as_deref(),
        what_this_desk_was_given.which_model.as_deref(),
    )
    .map_err(|why_not| WhyThisRunCouldNotStart::NoModelWasConfigured { why_not })
}

/// The key a delivery acts with, **and a run without one is a run that cannot deliver.**
fn a_credential(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> Result<ACredential, WhyThisRunCouldNotStart> {
    let the_key = what_this_desk_was_given
        .the_key
        .as_deref()
        .ok_or(WhyThisRunCouldNotStart::NoKeyWasSupplied)?;
    let the_key = ACredential::the_key_a_deployment_supplied(the_key)
        .map_err(|_why_not| WhyThisRunCouldNotStart::NoKeyWasSupplied)?;
    Ok(the_key)
}

/// The person at this terminal, **and a terminal named after a machine is refused here rather than
/// at the ruling** — so nobody's attention is spent on a document a machine was going to refuse.
fn a_person_at_the_terminal(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> Result<APersonAtTheTerminal, WhyThisRunCouldNotStart> {
    let who_they_say_they_are = WhoDecided::try_from(what_this_desk_was_given.who_answers.as_str())
        .map_err(|_why_not| WhyThisRunCouldNotStart::TheTerminalIsAMachine {
            what_it_says_it_is: what_this_desk_was_given.who_answers.clone(),
        })?;
    APersonAtTheTerminal::the_person_at(who_they_say_they_are, what_a_person_types, || {
        String::new()
    })
    .map_err(|_why_not| WhyThisRunCouldNotStart::TheTerminalIsAMachine {
        what_it_says_it_is: what_this_desk_was_given.who_answers.clone(),
    })
}

/// What a person types at a gate, **and it is a function because the terminal is asked seven times
/// and a person asked once is a person who cannot change their mind.**
fn what_a_person_types() -> String {
    String::new()
}

/// The run this desk works on, **and it is at the stage its first agent is working at** — the walk
/// moves it from there and not from anywhere else.
fn a_run_of_this_work(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> Result<WhereARunStands, WhyThisRunCouldNotStart> {
    let asked_for = &what_this_desk_was_given.what_a_person_was_asked_for;
    let the_work = a_work_picked_up_by_its_first_agent(asked_for)?;
    let the_work = the_work
        .answering(asked_for.what_it_should_do().clone())
        .map_err(|_why_not| WhyThisRunCouldNotStart::NothingToCallTheWorkBy)?;
    Ok(WhereARunStands::at_the_first_stage(
        the_work,
        THE_NAME_OF_THE_FIRST_ARTIFACT,
    ))
}

/// Which discipline answers for the result, **and it is the discipline and not the agent** — the
/// agent changes at every stage and the discipline does not.
fn a_discipline_answering() -> Result<Role, WhyThisRunCouldNotStart> {
    Role::try_from(THE_DISCIPLINE_THAT_ANSWERS)
        .map_err(|_why_not| WhyThisRunCouldNotStart::NothingToCallTheWorkBy)
}

/// The name this work is called by, **and it is a name and not a number because the project
/// directory is derived from it** — so two runs over one filing find the same place.
fn a_work_item_named() -> Result<WorkItemId, WhyThisRunCouldNotStart> {
    WorkItemId::try_from(THE_NAME_OF_THE_WORK)
        .map_err(|_why_not| WhyThisRunCouldNotStart::NothingToCallTheWorkBy)
}

/// Who asked for the work, **and it is the record's field and not this desk's to invent.**
fn a_person_who_commissioned() -> Result<WhoCommissioned, WhyThisRunCouldNotStart> {
    WhoCommissioned::try_from(THE_NAME_OF_THE_PERSON_COMMISSIONING)
        .map_err(|_why_not| WhyThisRunCouldNotStart::NothingToCallTheWorkBy)
}

/// The work, picked up by the agent that starts with it, **and the pickup is a move rather than a
/// field** — so a run that was never picked up cannot be walked.
fn a_work_picked_up_by_its_first_agent(
    asked_for: &WhatARunWasAskedFor,
) -> Result<aisdlc_domain::work_item::WorkItem, WhyThisRunCouldNotStart> {
    let the_agent = AgentIdentifier::try_from(THE_AGENT_THAT_STARTS)
        .map_err(|_why_not| WhyThisRunCouldNotStart::NothingToCallTheWorkBy)?;
    aisdlc_domain::work_item::WorkItem::held_by(
        a_work_item_named()?,
        a_person_who_commissioned()?,
        asked_for.which_repository().clone(),
        a_discipline_answering()?,
        aisdlc_domain::stage::Stage::InProgressByAgent,
        the_agent,
    )
    .map_err(|_why_not| WhyThisRunCouldNotStart::NothingToCallTheWorkBy)
}

/// What there is to deliver, **and the branch is named after the work and the run is over.**
fn what_there_is_to_deliver(
    what_this_desk_was_given: &WhatThisDeskWasGiven,
) -> WhatThereIsToDeliver {
    let which_repository = what_this_desk_was_given
        .what_a_person_was_asked_for
        .which_repository()
        .clone();
    let which_branch = ABranchToPush::a_branch_named(THE_NAME_OF_THE_BRANCH)
        .expect("a branch this desk names every time is a branch a reader can find");
    let what_to_open = the_request_a_run_makes_of_its_own_change(
        which_repository.clone(),
        which_repository,
        THE_NAME_OF_THE_BRANCH,
        THE_BRANCH_THE_CHANGE_GOES_ONTO,
        THE_TITLE_OF_THE_PULL_REQUEST,
        THE_BODY_OF_THE_PULL_REQUEST,
    )
    .expect("this desk names a title and a body every time");
    let the_credential =
        ACredential::the_key_a_deployment_supplied(THE_KEY_THIS_DESK_NAMES_FOR_ITS_OWN_TESTS)
            .expect("this desk's own key is a key");
    WhatThereIsToDeliver::for_this_run(which_branch, what_to_open, the_credential)
}

/// Where a person can read what happened, **and it is the one file that answers it.**
fn where_a_person_can_read_what_happened(
    what_a_run_left: &aisdlc_run::a_run_from_a_request_to_a_pull_request::WhatARunLeft,
) -> String {
    format!(
        "this run left a page: {}",
        what_a_run_left
            .the_page_it_wrote()
            .where_the_page_landed()
            .as_str()
    )
}

/// The name this desk gives the work, **and it is derived rather than chosen** so two runs over one
/// filing find the same directory.
const THE_NAME_OF_THE_WORK: &str = "WELLBEINGT-42";

/// Who commissioned the work, **and it is a word because the name is the record's and not this
/// desk's to invent.**
const THE_NAME_OF_THE_PERSON_COMMISSIONING: &str = "a person";

/// Which discipline answers for the result, **and it is the discipline and not the agent** — the
/// agent changes at every stage and the discipline does not.
const THE_DISCIPLINE_THAT_ANSWERS: &str = "DEV";

/// The agent a run starts with.
const THE_AGENT_THAT_STARTS: &str = "aisdlc-discover";

/// The artifact that agent produces.
const THE_NAME_OF_THE_FIRST_ARTIFACT: &str = "DiscoveryReport";

/// The branch a run pushes.
const THE_NAME_OF_THE_BRANCH: &str = "aisdlc/WELLBEINGT-42";

/// What the change goes onto.
const THE_BRANCH_THE_CHANGE_GOES_ONTO: &str = "main";

/// The pull request's title.
const THE_TITLE_OF_THE_PULL_REQUEST: &str = "what this run changed";

/// The pull request's body.
const THE_BODY_OF_THE_PULL_REQUEST: &str = "A change this run made, and a test that says so.";

/// The key this desk names for its own tests, **and nothing outside a test may use it.**
const THE_KEY_THIS_DESK_NAMES_FOR_ITS_OWN_TESTS: &str = "a-key-not-a-real-one";
