//! What the architecture analyst concluded, and the one claim a manifest has to back.
//!
//! **Both a list and a sentence, because a paragraph is what a model answers with when nothing
//! holds it to a list** — and a paragraph about how the whole is put together is not something a
//! planner can plan against.
//!
//! **A claimed dependency the manifests do not declare is the worst thing here.** It is a run
//! planning a change against an architecture that does not exist, **and it is the failure the
//! architecture tier already measured against a real model on a real repository and measured
//! _passing_.** So the check is a comparison against the words the manifests say, **because a
//! model asked whether a dependency is real will answer yes.**

use std::fmt;

/// One part of the system, **what it is for, and what it says it needs.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AModule {
    /// What it is called, **in the spelling the repository uses.**
    pub what_it_is_called: String,
    /// What it is for, **and never a blank — that is a filename and not a module.**
    pub what_it_is_for: String,
    /// What it says it depends on, **each of which a manifest has to name.**
    pub what_it_depends_on: Vec<String>,
}

/// What the architecture analyst concluded, **and nothing it was not shown.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheArchitectureIs {
    /// The parts, **and a gate may see the artifact only when there is at least one.**
    pub modules: Vec<AModule>,
    /// How the whole is put together, in words.
    pub how_the_whole_is_put_together: String,
    /// What the project's manifests really name, **read out of the checkout and not asked for.**
    ///
    /// **The evidence travels with the report, and that is why it is a field.** A check whose
    /// other half is a parameter is a check whose other half a caller could have got wrong, **and
    /// a report that is only checkable beside something the run happened to still be holding is a
    /// report nobody can look at again a week later.**
    pub what_a_manifest_declares: Vec<String>,
}

/// Why this architecture cannot be put in front of a person, **and each is a different fact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoArchitectureWasDescribed {
    /// No module at all, **and a paragraph is not a list.**
    NoModuleWasNamed,
    /// A module with no purpose, **quoted whole because that is what a filename is.**
    AModuleNamedOnly {
        /// What it was called, **so a person can go and look at the thing itself.**
        what_it_is_called: String,
    },
    /// A dependency the project's manifests never named, **quoted whole and with the module.**
    ADependencyNoManifestDeclares {
        /// Which module claimed it, **because a dependency without its module is a mystery.**
        which_module: String,
        /// What it claimed, **spelled as the model spelled it.**
        what_it_claimed: String,
    },
}

impl NoArchitectureWasDescribed {
    /// The module is named and nothing else, **and a filename is what that is.**
    pub fn a_module_named_only(what_it_is_called: &str) -> Self {
        NoArchitectureWasDescribed::AModuleNamedOnly {
            what_it_is_called: what_it_is_called.to_string(),
        }
    }

    /// A module claimed something the manifests do not name, **and it carries both halves**
    /// **because a dependency without its module is a mystery.**
    pub fn a_dependency_no_manifest_declares(which_module: &str, what_it_claimed: &str) -> Self {
        NoArchitectureWasDescribed::ADependencyNoManifestDeclares {
            which_module: which_module.to_string(),
            what_it_claimed: what_it_claimed.to_string(),
        }
    }
}

impl fmt::Display for NoArchitectureWasDescribed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why_not = match self {
            NoArchitectureWasDescribed::NoModuleWasNamed => String::from(
                "no module was named, and a paragraph about how the whole is put together is not a \
                 list anybody can plan against",
            ),
            NoArchitectureWasDescribed::AModuleNamedOnly { what_it_is_called } => format!(
                "{what_it_is_called} is named without saying what it is for, which is a filename, \
                 and a name that does not say what the thing is is a name nobody can hold to"
            ),
            NoArchitectureWasDescribed::ADependencyNoManifestDeclares {
                which_module,
                what_it_claimed,
            } => format!(
                "{which_module} depends on {what_it_claimed}, which no manifest declares, and a \
                 module depending on something the project never asked for is a run planning a \
                 change against an architecture that does not exist"
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for NoArchitectureWasDescribed {}

/// What this artifact's check answers, **named because the full type is 43 characters long and a
/// signature carrying it is a signature this system's own readability guard splits in two.**
type TheRefusal = Result<(), NoArchitectureWasDescribed>;

/// Whether a module says what it is for, **and a blank says nothing.**
fn says_what_it_is_for(one_module: &AModule) -> bool {
    !one_module.what_it_is_for.trim().is_empty()
}

impl WhatTheArchitectureIs {
    /// Whether every dependency a module claims is one a manifest names.
    ///
    /// **The modules are checked before the dependencies, and the other way round sends the
    /// reader to the wrong line**: a module with no purpose is not a module, **so a dependency it
    /// claims is not worth checking before somebody has said what the thing is.**
    ///
    /// **`what_a_manifest_declares` is a field and not a parameter**, so the evidence travels with
    /// the report — **a check whose other half is an argument is a check whose other half a caller
    /// could have got wrong**, and a report only checkable beside something the run happened to
    /// still be holding is a report nobody can look at again a week later.
    pub fn the_dependencies_the_manifests_declare(&self) -> TheRefusal {
        if self.modules.is_empty() {
            return Err(NoArchitectureWasDescribed::NoModuleWasNamed);
        }
        self.each_module_says_what_it_is_for()?;
        self.each_dependency_was_declared()
    }

    /// Whether every module says what it is for, **because a name without one is a filename.**
    fn each_module_says_what_it_is_for(&self) -> TheRefusal {
        for one_module in &self.modules {
            if !says_what_it_is_for(one_module) {
                return Err(NoArchitectureWasDescribed::a_module_named_only(
                    &one_module.what_it_is_called,
                ));
            }
        }
        Ok(())
    }

    /// Whether every claimed dependency is one the manifests name.
    ///
    /// **One module at a time and not one dependency at a time**, because a loop inside a loop
    /// inside a condition is three levels deep and this system's own readability guard refuses it
    /// — **and the refusal is right: a reader has to hold three collections at once to follow it.**
    fn each_dependency_was_declared(&self) -> TheRefusal {
        for one_module in &self.modules {
            self.no_dependency_was_invented(one_module)?;
        }
        Ok(())
    }

    /// Whether one module invented a dependency, **and the refusal carries both halves because a
    /// dependency without its module is a mystery.**
    fn no_dependency_was_invented(&self, one_module: &AModule) -> TheRefusal {
        for one_dependency in &one_module.what_it_depends_on {
            let was_it_declared = self.what_a_manifest_declares.contains(one_dependency);
            if !was_it_declared {
                return Err(
                    NoArchitectureWasDescribed::a_dependency_no_manifest_declares(
                        &one_module.what_it_is_called,
                        one_dependency,
                    ),
                );
            }
        }
        Ok(())
    }
}
