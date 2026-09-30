//! The program, run on the host, about a request the run made in a named way.
//!
//! **This is the only crate that names a process**, and a change here changes what the world
//! does; everything else names `AProgramWeRun`.
//!
//! **No shell, and the arguments are one thing each.** A formatted command line is where a
//! value read out of a model becomes part of a command, so there is nothing to format and no
//! shell to hand a string to.
//!
//! **The program is named by the port's closed set and never by the arguments**, so a name can
//! never travel as data, and the first argument is what to do rather than which program.

use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid, WhichProgramWeRun};
use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

use crate::the_environment::TheEnvironment;

/// One of our own programs, run on this host.
pub struct TheProgramAsItRuns {
    which_program: WhichProgramWeRun,
    the_credential: Option<aisdlc_run::a_credential::ACredential>,
    what_a_deployment_named_besides: Vec<String>,
}

impl TheProgramAsItRuns {
    /// Our programs, as they really are on this host, with the credential a deployment supplied.
    ///
    /// **The credential is an argument and not a thing this struct finds for itself**, because a
    /// program that reads its own configuration has moved a decision out of a type and into a
    /// line of code — and because `None` has to be written down at every call site, where a
    /// program that quietly went looking would leave no trace of having found nothing.
    pub fn as_it_is_on_this_host(
        the_credential: Option<aisdlc_run::a_credential::ACredential>,
    ) -> Self {
        Self {
            which_program: WhichProgramWeRun::Git,
            the_credential,
            what_a_deployment_named_besides: Vec::new(),
        }
    }

    /// What a deployment said a model must be asked besides the body, one header each.
    ///
    /// **A deployment fact and not a gateway's name written into this system.** A proxy in front of
    /// a model can require anything, so the deployment says what it requires and this system sends
    /// it — **and a line that is not a header is not sent, because a request that guesses what a
    /// host needs is the same invention as a base endpoint.**
    #[must_use]
    pub fn with_what_a_deployment_named(
        mut self,
        what_the_process_was_told: &TheEnvironment,
    ) -> Self {
        let what_it_named = what_the_process_was_told
            .the_variable_a_deployment_set("LLM_HEADERS")
            .unwrap_or_default();
        self.what_a_deployment_named_besides = what_it_named
            .lines()
            .map(str::trim)
            .filter(|one| !one.is_empty())
            .map(str::to_string)
            .collect();
        self
    }

    /// Which program this will run, and it is never chosen by a caller.
    pub fn which_program_it_runs(&self) -> WhichProgramWeRun {
        self.which_program
    }

    /// Which program carries a request, chosen here and not by the caller.
    ///
    /// **A question about a model is `curl` and nothing else**, and a caller that could name the
    /// program would be a caller that could send a credential somewhere that is not a model —
    /// which is the one thing a credential exists to prevent.
    fn which_program_for(
        &self,
        what_it_is_asked_to_do: &WhatWeTellAProgramToDo,
    ) -> WhichProgramWeRun {
        match what_it_is_asked_to_do {
            WhatWeTellAProgramToDo::AskAModelHowItRules { .. } => WhichProgramWeRun::Curl,
            _ => self.which_program,
        }
    }

    /// The arguments of one request, one thing each and never a line.
    ///
    /// **A list and not a `String`,** because a caller that cannot count its arguments cannot
    /// tell one from three, and a formatted line is a shell's opinion about what it means.
    ///
    /// **A method and not a free function, because the arguments depend on which program is
    /// carrying them** — a credential is one of them, and a free function that knew nothing
    /// about the program is what produced a translation that was wrong by the time it was
    /// merged.
    pub fn the_arguments_of(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> Vec<String> {
        match what_it_is_asked_to_do {
            WhatWeTellAProgramToDo::AskAModelHowItRules {
                the_endpoint,
                the_body,
                ..
            } => self.asked_a_model_of(the_endpoint, the_body),
            _ => what_git_is_asked_for(what_it_is_asked_to_do),
        }
    }
}

/// The arguments of everything `git` is asked for, one thing each and never a line.
fn what_git_is_asked_for(what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> Vec<String> {
    match what_it_is_asked_to_do {
        WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of } => vec![
            "clone".to_string(),
            the_clone_of.fetch_from(),
            the_clone_of.where_it_lands().display().to_string(),
        ],
        WhatWeTellAProgramToDo::TheCommitItStandsOn { the_clone_of } => {
            inside_the_clone(the_clone_of, &["rev-parse", "HEAD"])
        }
        WhatWeTellAProgramToDo::TheRefItStandsOn { the_clone_of } => {
            inside_the_clone(the_clone_of, &["rev-parse", "--abbrev-ref", "HEAD"])
        }
        WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { the_clone_of } => inside_the_clone(
            the_clone_of,
            &["remote", "add", "upstream", &the_clone_of.fetch_from()],
        ),
        WhatWeTellAProgramToDo::AskAModelHowItRules { .. } => Vec::new(),
    }
}

/// The arguments of a question about a clone that already exists, run inside it.
///
/// **`-C` and the directory are one thing each, like everything else.** Six repetitions of the
/// same two arguments is a list of six places to change, and a list like that is a list to keep
/// in step with git.
fn inside_the_clone(the_clone_of: &ARepositoryToClone, the_question: &[&str]) -> Vec<String> {
    let mut the_arguments = vec![
        "-C".to_string(),
        the_clone_of.where_it_lands().display().to_string(),
    ];
    the_arguments.extend(the_question.iter().map(|one| (*one).to_string()));
    the_arguments
}

impl TheProgramAsItRuns {
    /// The arguments asking a model, and nothing at all when there is no credential behind them.
    ///
    /// **`-s` first, because a program that narrates is a program whose narration can be mistaken
    /// for what it found** — the first real run came back with curl's transfer table where the
    /// model's words should have been.
    ///
    /// **Nothing rather than a request without a credential.** A request that goes out
    /// unauthenticated comes back in words this system did not write, and **the one fact a caller
    /// can usefully be given is that nothing left.**
    fn asked_a_model_of(&self, the_endpoint: &str, the_body: &str) -> Vec<String> {
        let Some(the_credential) = self.the_credential.as_ref() else {
            return Vec::new();
        };
        let the_authorization = format!(
            "Authorization: Bearer {}",
            the_credential.the_key_to_authenticate_with()
        );
        let mut the_arguments = vec!["-s", "-X", "POST", the_endpoint, "-H"];
        the_arguments.push("Content-Type: application/json");
        the_arguments.push("-H");
        the_arguments.push(&the_authorization);
        for one_header in &self.what_a_deployment_named_besides {
            the_arguments.push("-H");
            the_arguments.push(one_header);
        }
        the_arguments.push("-d");
        the_arguments.push(the_body);
        the_arguments.into_iter().map(str::to_string).collect()
    }
}

/// What the program run says when a request has no program to run, and says so rather than
/// running one with no arguments.
///
/// **Said in words and not left to a panic.** A caller reading a refusal can act on it, and a
/// request with no program is a fact about this system's own incompleteness — **it is the one
/// refusal in this file that is ours and not the program's.**
const NO_CREDENTIAL_IS_CARRIED_FOR_THIS_REQUEST: &str = "no credential is carried for this request, and a request that goes out without one comes back in words this system did not write";

/// What the program wrote, and both streams, because git writes progress to one and trouble to
/// the other — **a caller told only about the stream it did not expect is told nothing.**
fn the_words_of(what_it_ran: &std::process::Output) -> String {
    let out = String::from_utf8_lossy(&what_it_ran.stdout).to_string();
    let trouble = String::from_utf8_lossy(&what_it_ran.stderr).to_string();
    format!("{out}\n{trouble}").trim().to_string()
}

impl AProgramWeRun for TheProgramAsItRuns {
    /// Run it, with no shell, and report whether it worked and what it said.
    ///
    /// **A program that could not even be started is a refusal and not a crash**, and it says
    /// so in the same words a failure would: the caller has to tell *git is not installed* from
    /// *there is no such repository*, and this is the half of that telling that is not git's.
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        let the_arguments = self.the_arguments_of(what_it_is_asked_to_do);
        if the_arguments.is_empty() {
            return WhatTheProgramSaid::of(false, NO_CREDENTIAL_IS_CARRIED_FOR_THIS_REQUEST);
        }
        let what_it_ran = std::process::Command::new(
            self.which_program_for(what_it_is_asked_to_do)
                .name_on_the_host(),
        )
        .args(&the_arguments)
        .output();
        match what_it_ran {
            Ok(what_it_ran) => {
                WhatTheProgramSaid::of(what_it_ran.status.success(), &the_words_of(&what_it_ran))
            }
            Err(why_it_did_not_start) => {
                WhatTheProgramSaid::of(false, &why_it_did_not_start.to_string())
            }
        }
    }
}
