//! What the quality analyst would check, and the link between a scenario and a requirement.
//!
//! **A scenario is not a wish and a requirement is not a test.** A test somebody writes and then
//! watches pass proves nothing about the work; **a test written from a requirement is the only
//! thing that turns a requirement into evidence**, so every scenario here names the requirement it
//! proves and a gate may see a plan only when every name in it resolves.
//!
//! **The requirements are the product analyst's and not a second copy.** The requirements the
//! quality analyst is handed *are* the ones a person approved at the gate before it, **and two
//! types with the same shape are two names for one thing.**

use std::fmt;

use crate::what_the_product_analyst_concluded::ARequirement;

/// One check, and the requirement it is a check of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AScenario {
    /// The requirement this is a check of, **and a blank resolves to nothing.**
    pub which_requirement_it_proves: String,
    /// What is looked at, **and a blank proves nothing.**
    pub what_is_checked: String,
    /// Where the check runs, **which may be a suite or a person and never is left unsaid.**
    pub where_it_runs: String,
}

/// What the quality analyst concluded, **and nothing it was not asked about.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatWouldProveTheWork {
    /// The requirements a person approved, **and every one of them needs a scenario.**
    pub requirements: Vec<ARequirement>,
    /// The scenarios, **and every one of them has to name a requirement above.**
    pub scenarios: Vec<AScenario>,
}

/// Why these scenarios cannot be put in front of a person, **and each is a different fact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatWouldProveNothing {
    /// No scenario at all, **so nothing would be checked.**
    NothingWouldBeChecked,
    /// A requirement with no way of being known to hold, **quoted whole.**
    ARequirementNothingChecks {
        /// What was asked, **because the fault is the requirement's and not the scenario's.**
        what_was_asked: String,
    },
    /// A requirement nothing points at, **quoted whole.**
    ARequirementNoScenarioProves {
        /// What was asked, **because a person has to find which of their own words was skipped.**
        what_was_asked: String,
    },
    /// A scenario naming a requirement nobody asked for, **quoted whole.**
    AScenarioAboutSomethingElse {
        /// What the scenario claims to prove, **and it names a thing rather than an index.**
        what_it_claims_to_prove: String,
    },
    /// A scenario that says what it looks at as a blank, **quoted whole.**
    AScenarioThatWouldProveNothing {
        /// The requirement it claimed, **so the person reading it knows which check went missing.**
        about_which_requirement: String,
    },
}

impl WhatWouldProveNothing {
    /// The requirement has no observable, **and it is the requirement's fault and not the
    /// scenario's** — so it is named here and nowhere else.
    pub fn a_requirement_nothing_checks(what_was_asked: &str) -> Self {
        WhatWouldProveNothing::ARequirementNothingChecks {
            what_was_asked: what_was_asked.to_string(),
        }
    }

    /// The requirement has nothing pointing at it, **which is the half of the work nobody will
    /// notice.**
    pub fn a_requirement_no_scenario_proves(what_was_asked: &str) -> Self {
        WhatWouldProveNothing::ARequirementNoScenarioProves {
            what_was_asked: what_was_asked.to_string(),
        }
    }

    /// The scenario points at a requirement nobody asked for, **and it names the thing rather
    /// than an index into a list the person reading it cannot see.**
    pub fn a_scenario_about_something_else(what_it_claims_to_prove: &str) -> Self {
        WhatWouldProveNothing::AScenarioAboutSomethingElse {
            what_it_claims_to_prove: what_it_claims_to_prove.to_string(),
        }
    }

    /// The scenario names no check, **and it carries the requirement so a reader knows which check
    /// went missing.**
    pub fn a_scenario_that_would_prove_nothing(about_which_requirement: &str) -> Self {
        WhatWouldProveNothing::AScenarioThatWouldProveNothing {
            about_which_requirement: about_which_requirement.to_string(),
        }
    }
}

impl fmt::Display for WhatWouldProveNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            WhatWouldProveNothing::NothingWouldBeChecked => String::from(
                "no scenario at all, and a plan of requirements with no scenario on any of them \
                 approves evidence for work that was never proved",
            ),
            WhatWouldProveNothing::ARequirementNothingChecks { what_was_asked } => format!(
                "{what_was_asked} names no way of being known to hold, and a scenario pointed at \
                 one checks a thing nobody could have checked either — so the fault is the \
                 requirement and not the scenario"
            ),
            WhatWouldProveNothing::ARequirementNoScenarioProves { what_was_asked } => format!(
                "{what_was_asked} has no scenario, and one requirement proved out of two is a \
                 delivery whose unproved half is the half nobody will notice"
            ),
            WhatWouldProveNothing::AScenarioAboutSomethingElse {
                what_it_claims_to_prove,
            } => format!(
                "a scenario claims to prove {what_it_claims_to_prove}, which nobody asked for, and \
                 it is a test that will keep passing after the work is done wrong"
            ),
            WhatWouldProveNothing::AScenarioThatWouldProveNothing {
                about_which_requirement,
            } => format!(
                "a scenario about {about_which_requirement} names no check, and it is the most \
                 convincing line in a plan and the least useful"
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for WhatWouldProveNothing {}

/// Whether a scenario names something to look at, **and a blank is not a thing to look at.**
fn names_something_to_check(scenario: &AScenario) -> bool {
    !scenario.what_is_checked.trim().is_empty()
}

/// Whether a scenario points at a requirement, **and it has to be one of the ones above.**
fn points_at_a_requirement(scenario: &AScenario, what_was_asked: &[String]) -> bool {
    what_was_asked.contains(&scenario.which_requirement_it_proves)
}

impl WhatWouldProveTheWork {
    /// Whether a gate may be shown this at all, **and every check is a link that has to close.**
    ///
    /// **The requirements are checked before the scenarios on purpose.** A requirement with no
    /// observable makes every scenario pointed at it worthless, **and refusing it as the
    /// requirement it is sends the reader to the right line** rather than to a check that was
    /// never going to prove anything.
    pub fn what_the_gate_may_be_shown(&self) -> Result<(), WhatWouldProveNothing> {
        self.each_requirement_is_watchable()?;
        self.each_scenario_points_at_one()?;
        self.each_requirement_has_a_scenario()
    }

    /// What the requirements say they are, **and the list every scenario has to resolve into.**
    ///
    /// **Built here and not passed around**, because a list a caller could have got wrong is a
    /// list the refusals below would be checking against **rather than the artifact's own words.**
    fn what_was_asked(&self) -> Vec<String> {
        self.requirements
            .iter()
            .map(|one| one.what_is_asked.clone())
            .collect()
    }

    /// Whether every requirement says how anybody would know it holds.
    fn each_requirement_is_watchable(&self) -> Result<(), WhatWouldProveNothing> {
        for one_requirement in &self.requirements {
            let it_says_how = one_requirement
                .how_it_is_known_to_hold
                .iter()
                .any(|one_way| !one_way.trim().is_empty());
            if !it_says_how {
                return Err(WhatWouldProveNothing::a_requirement_nothing_checks(
                    &one_requirement.what_is_asked,
                ));
            }
        }
        Ok(())
    }

    /// Whether every scenario names a requirement above **and something to look at.**
    fn each_scenario_points_at_one(&self) -> Result<(), WhatWouldProveNothing> {
        for one_scenario in &self.scenarios {
            let what_was_asked = self.what_was_asked();
            if !points_at_a_requirement(one_scenario, &what_was_asked) {
                return Err(WhatWouldProveNothing::a_scenario_about_something_else(
                    &one_scenario.which_requirement_it_proves,
                ));
            }
            if !names_something_to_check(one_scenario) {
                return Err(WhatWouldProveNothing::a_scenario_that_would_prove_nothing(
                    &one_scenario.which_requirement_it_proves,
                ));
            }
        }
        Ok(())
    }

    /// Whether every requirement has something pointing at it.
    fn each_requirement_has_a_scenario(&self) -> Result<(), WhatWouldProveNothing> {
        if self.scenarios.is_empty() {
            return Err(WhatWouldProveNothing::NothingWouldBeChecked);
        }
        for one_requirement in self.what_was_asked() {
            let points_at_this_one = self
                .scenarios
                .iter()
                .any(|one_scenario| one_scenario.which_requirement_it_proves == one_requirement);
            if !points_at_this_one {
                return Err(WhatWouldProveNothing::a_requirement_no_scenario_proves(
                    &one_requirement,
                ));
            }
        }
        Ok(())
    }
}
