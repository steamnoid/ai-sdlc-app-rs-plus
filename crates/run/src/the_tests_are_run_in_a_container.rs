//! A repository's own tests, run in a container that cannot see the host.
//!
//! **Executing a repository is the dangerous operation in this system, not reading one.** An
//! implementation agent writes code and a verification agent then runs **that repository's own
//! test suite** — arbitrary code from a pull request, with the developer's credentials and file
//! system within reach.
//!
//! **The refusals come in a fixed order and the order is the rule**: a machine with no container is
//! refused whatever the checkout declares, **because a run that inspected the work before refusing
//! would already have done the work it was refusing to do safely.**

use std::path::Path;

use crate::a_program_we_run::AProgramWeRun;
use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// What running one suite answers, **and the name is short because a signature carrying the full
/// answer is a signature rustfmt splits in two — and a split signature makes this repository's
/// own nesting guard count a brace that has nothing to do with the body.**
type WhatRunningASuiteAnswers = Result<WhatTheSuiteSaid, TheTestsCouldNotBeRun>;

/// What a repository says about its own tests, **and never a command line.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSuiteARepositoryDeclares {
    /// The image the suite needs, **and the only thing of the host's toolchain that crosses over.**
    pub the_toolchain_image: String,
    /// How to install what the suite needs, **which is the one step allowed a network.**
    pub how_to_install: Vec<String>,
    /// How to run it.
    pub how_to_run: Vec<String>,
    /// Who runs it inside the image, **and never root — a container running as root with the clone
    /// mounted is a container that can write to the working tree it was only asked to read.**
    pub who_runs_it: String,
}

/// What came of running it, **and whether it ran at all is the first thing here.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheSuiteSaid {
    /// Whether the install worked, **and a suite whose install failed has not run.**
    pub did_the_dependencies_install: bool,
    /// What the suite wrote, **copied and never summarised.**
    pub what_it_wrote: String,
}

/// A suite that ran, **or the one thing that stopped it from having run.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASuiteThatRan(pub String);

/// A report claiming a suite failed when it never ran, **quoted whole because the words are the
/// only evidence.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AReportOfASuiteThatNeverRan(pub String);

impl WhatTheSuiteSaid {
    /// An install that failed, **and it is a value and not an early return because a struct
    /// literal inside an `if` is the third level of nesting this repository's guard refuses.**
    pub fn the_install_never_worked(
        the_install: &crate::a_program_we_run::WhatTheProgramSaid,
    ) -> Self {
        Self {
            did_the_dependencies_install: false,
            what_it_wrote: the_install.the_words().to_string(),
        }
    }

    /// How this run is reported, **and an install that failed is not a red suite.**
    ///
    /// **A suite whose dependencies did not install has not run**, so reporting it red **would be
    /// a claim about code nobody executed** — and the half of a finding the sister
    /// implementation's history records that was right.
    pub fn how_the_run_is_reported(&self) -> Result<ASuiteThatRan, AReportOfASuiteThatNeverRan> {
        if !self.did_the_dependencies_install {
            return Err(AReportOfASuiteThatNeverRan(self.what_it_wrote.clone()));
        }
        Ok(ASuiteThatRan(self.what_it_wrote.clone()))
    }
}

impl ASuiteThatRan {
    /// What the suite said, **in the suite's own words.**
    pub fn what_it_said(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AReportOfASuiteThatNeverRan {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "the suite never ran, and reporting it as a red suite would be a claim about code \
             nobody executed — the install said: {}",
            self.0
        )
    }
}

impl std::error::Error for AReportOfASuiteThatNeverRan {}

/// Why a repository's own tests could not be run, **and the order of the first two is a rule.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheTestsCouldNotBeRun {
    /// This machine has nothing that could run a container, **whatever the checkout declares.**
    NoContainerToRunThemIn,
    /// The checkout says nothing about its own tests.
    NoSuiteWasDeclared,
}

impl std::fmt::Display for TheTestsCouldNotBeRun {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let why_not = match self {
            TheTestsCouldNotBeRun::NoContainerToRunThemIn => String::from(
                "this machine has nothing that could run a container, and a run that inspected the \
                 work before refusing would already have done the work it was refusing to do safely",
            ),
            TheTestsCouldNotBeRun::NoSuiteWasDeclared => String::from(
                "the checkout says nothing about its own tests, and a repository this system cannot \
                 test is a repository it cannot verify a change against",
            ),
        };
        write!(formatter, "{why_not}")
    }
}

impl std::error::Error for TheTestsCouldNotBeRun {}

/// Whether this machine has anything that could run a container.
fn is_there_a_container(the_docker: &impl AProgramWeRun) -> bool {
    let asked = WhatWeTellAProgramToDo::the_toolchain_image_is_there();
    the_docker.run(&asked).did_it_work()
}

/// The repository's own tests are run, **or the one thing that stopped them.**
///
/// **The container is asked before the checkout is.** The other order would inspect the work of a
/// repository this machine cannot safely execute, **which is the work the refusal exists to avoid
/// having done.**
pub fn the_tests_in(
    the_docker: &impl AProgramWeRun,
    what_the_repository_declares: Option<TheSuiteARepositoryDeclares>,
    where_the_clone_lands: &Path,
) -> WhatRunningASuiteAnswers {
    if !is_there_a_container(the_docker) {
        return Err(TheTestsCouldNotBeRun::NoContainerToRunThemIn);
    }
    let the_suite =
        what_the_repository_declares.ok_or(TheTestsCouldNotBeRun::NoSuiteWasDeclared)?;
    Ok(the_suite.how_it_went(the_docker, where_the_clone_lands))
}

/// The install and then the run, **and the install's own answer is what decides
/// `did_the_dependencies_install`.
///
/// **It answers the suite and not a `Result`, and that is a deletion rather than an omission.**
/// The first version carried a third refusal, `TheContainerRefused`, for a `docker` that refused
/// to start — **and nothing in this system could raise it**, because an install that failed is
/// already the "the suite never ran" answer and a run that failed is a red suite. **A refusal
/// variant nothing can raise is an abstraction without a caller**, and telling a container-start
/// failure from a failing test by reading `stderr` would be exactly the guess this system's
/// ports exist to avoid.
impl TheSuiteARepositoryDeclares {
    /// How the whole of it went: the install, then the run, **and whether the install worked
    /// first.**
    ///
    /// **A method on the suite and not a free function taking it**, because the suite knows which
    /// image it needs and which commands it declared — **and the immediate reason is the
    /// signature: a free function carrying the suite's type and a path is 132 characters, which
    /// rustfmt splits, and a split signature makes this repository's own nesting guard count a
    /// brace that has nothing to do with the body.** The third refusal was deleted for the same
    /// family of reason: nothing here could raise it.
    pub fn how_it_went(
        &self,
        the_docker: &impl AProgramWeRun,
        the_clone: &Path,
    ) -> WhatTheSuiteSaid {
        let the_install = the_docker.run(&self.how_it_is_installed_in(the_clone));
        if the_install.did_it_work() {
            return what_the_suite_answered(the_docker, self, the_clone);
        }
        WhatTheSuiteSaid::the_install_never_worked(&the_install)
    }

    /// The request that installs what this suite needs, **in a checkout at this place.**
    pub fn how_it_is_installed_in(&self, the_clone: &Path) -> WhatWeTellAProgramToDo {
        WhatWeTellAProgramToDo::the_suite_is_installed(self, the_clone)
    }

    /// The request that runs this suite, **in a checkout at this place.**
    pub fn how_it_is_run_in(&self, the_clone: &Path) -> WhatWeTellAProgramToDo {
        WhatWeTellAProgramToDo::the_suite_is_run(self, the_clone)
    }
}

/// What the suite itself said, **which is only ever reached when its dependencies installed.**
fn what_the_suite_answered(
    the_docker: &impl AProgramWeRun,
    the_suite: &TheSuiteARepositoryDeclares,
    the_clone: &Path,
) -> WhatTheSuiteSaid {
    let the_run = the_docker.run(&the_suite.how_it_is_run_in(the_clone));
    WhatTheSuiteSaid {
        did_the_dependencies_install: true,
        what_it_wrote: the_run.the_words().to_string(),
    }
}
