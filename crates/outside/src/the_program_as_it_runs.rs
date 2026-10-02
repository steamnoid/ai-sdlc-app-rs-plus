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
use std::path::Path;

use aisdlc_run::the_repository_to_clone_into::ARepositoryToClone;
use aisdlc_run::the_tests_are_run_in_a_container::TheSuiteARepositoryDeclares;
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
            WhatWeTellAProgramToDo::AskAModel { .. }
            | WhatWeTellAProgramToDo::OpenAPullRequest { .. } => WhichProgramWeRun::Curl,
            WhatWeTellAProgramToDo::TheToolchainImageIsThere
            | WhatWeTellAProgramToDo::TheSuiteIsInstalled { .. }
            | WhatWeTellAProgramToDo::TheSuiteIsRun { .. } => WhichProgramWeRun::Docker,
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
            WhatWeTellAProgramToDo::AskAModel {
                the_endpoint,
                the_body,
                ..
            } => self.asked_a_model_of(the_endpoint, the_body),
            WhatWeTellAProgramToDo::OpenAPullRequest {
                where_it_goes,
                the_body,
            } => self.asked_of_the_api(where_it_goes, the_body),
            WhatWeTellAProgramToDo::TheToolchainImageIsThere
            | WhatWeTellAProgramToDo::TheSuiteIsInstalled { .. }
            | WhatWeTellAProgramToDo::TheSuiteIsRun { .. } => {
                what_a_container_is_asked_for(what_it_is_asked_to_do)
            }
            _ => what_git_is_asked_for(what_it_is_asked_to_do),
        }
    }
}

/// The arguments of everything `git` is asked for, one thing each and never a line.
///
/// **The clone is one arm and the rest is one question**, because there are now six requests
/// about a clone and a match arm each is a list of six places to change.
fn what_git_is_asked_for(what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> Vec<String> {
    if what_it_is_asked_to_do.is_answered_by_gh() {
        return Vec::new();
    }
    match what_it_is_asked_to_do {
        WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of } => vec![
            "clone".to_string(),
            the_clone_of.fetch_from(),
            the_clone_of.where_it_lands().display().to_string(),
        ],
        _ => what_git_is_asked_about(what_it_is_asked_to_do),
    }
}

/// What `git` is asked inside a clone that already exists, **and nothing at all for a request
/// that is not about one.
///
/// **One parameter and not two, and that is the whole of how this function is written.** The
/// gate counting how deep a statement sits reads a line naming a function as the start of a
/// body, so the brace that opens the body is never counted — **and a second parameter would
/// have put the signature on two lines, which counts it, which puts every arm of the match one
/// level deeper than it looks.** Every other function in this file has been one line for the
/// same reason, without the gate ever having said so out loud.
fn what_git_is_asked_about(request: &WhatWeTellAProgramToDo) -> Vec<String> {
    let Some(the_clone_of) = request.the_clone_it_is_about() else {
        return Vec::new();
    };
    match request {
        WhatWeTellAProgramToDo::TheCommitItStandsOn { .. } => to_read_the_commit(the_clone_of),
        WhatWeTellAProgramToDo::RecordTheChange { every_path } => to_record_the_change(every_path),
        WhatWeTellAProgramToDo::TheRefItStandsOn { .. } => to_read_the_ref(the_clone_of),
        WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { .. } => {
            to_name_where_the_code_was_written_in(the_clone_of)
        }
        WhatWeTellAProgramToDo::ReadTheRemote { which_branch, .. } => {
            to_read_the_remote(the_clone_of, which_branch)
        }
        WhatWeTellAProgramToDo::PushTheBranch { which_branch, .. } => {
            to_push_the_branch(the_clone_of, which_branch)
        }
        _ => Vec::new(),
    }
}

/// What `docker` is asked for anything to do with a repository's own tests, **and one function
/// rather than three arms in the match above.
///
/// **The match grew by six lines when the container arrived and the twenty-line guard refused it**,
/// so the arms moved here whole. **A translation with three containers in it was two translations
/// wearing one name**, and this is the same reason `what_git_is_asked_about` exists for the six
/// requests about a clone.
fn what_a_container_is_asked_for(request: &WhatWeTellAProgramToDo) -> Vec<String> {
    match request {
        WhatWeTellAProgramToDo::TheToolchainImageIsThere => to_ask_whether_a_container_is_there(),
        WhatWeTellAProgramToDo::TheSuiteIsInstalled {
            the_suite,
            where_the_clone_lands,
        } => to_install_what_the_suite_needs(the_suite, where_the_clone_lands),
        WhatWeTellAProgramToDo::TheSuiteIsRun {
            the_suite,
            where_the_clone_lands,
        } => to_run_the_suite(the_suite, where_the_clone_lands),
        _ => Vec::new(),
    }
}

/// Where the clone is inside the container, **and it is `/the-clone` because that is the only
/// path the container is ever told about.**
const WHERE_THE_CLONE_IS_INSIDE_THE_CONTAINER: &str = "/the-clone";

/// The one word that turns a mount into a thing the container may only read.
///
/// **`:ro` and not a separate flag**, because it is a property of the mount and not a property of
/// the container: **one `-v` can be read-only or it can be written, and a second flag beside it
/// would be a second thing whose absence silently means "writable".** The word is on the end of the
/// mount itself, where a reader looking for what the container may touch finds it.
const AND_MAY_ONLY_BE_READ: &str = ":ro";

/// A process may not gain a privilege it does not already hold.
///
/// **`--cap-drop=ALL` removes the capabilities a process holds and says nothing about the ones it
/// can gain**, so a setuid binary inside the image is the way back in. This is the one flag that
/// closes it, **and a sibling implementation in this workspace already carries it** — the constant
/// is written as a list here for the same reason it is there.
const NO_NEW_PRIVILEGES: [&str; 2] = ["--security-opt", "no-new-privileges"];

/// An editable install writes `.egg-info` into the checkout it installs from, **so this is the one
/// step that may write and the reason is a fact about Python and not about this system.**
const AN_INSTALL_MAY_WRITE: bool = true;

/// A suite may not write the code it is judging, **and the constant is here so a reader of the
/// call site sees the choice rather than a `true` or a `false`.**
const A_SUITE_MAY_NOT_WRITE: bool = false;

/// What `docker` is asked when this system needs to know whether it can run anything at all.
fn to_ask_whether_a_container_is_there() -> Vec<String> {
    vec![String::from("info")]
}

/// The arguments every container of a repository's own code is given, **in this order.
///
/// **`--cap-drop=ALL`, because a container that may do anything may do anything to the mount.** A
/// half-configured container is worse than no container, **because it reads as isolation while
/// still handing over the host's keys.**
///
/// **`--user` and not root, because the clone is mounted read-write and a container running as
/// root can write to the working tree it was only asked to read.**
///
/// **One `-v` and never two, and it is the clone. The developer's home directory, the SSH keys,
/// the `~/.config/gh` a delivery needs and the docker socket are all on this machine, and a
/// second mount is a second thing to audit and a second thing to forget.**
fn the_arguments_every_container_is_given(
    the_suite: &TheSuiteARepositoryDeclares,
    where_it_lands: &Path,
    may_the_suite_write: bool,
) -> Vec<String> {
    let mut the_arguments = vec![
        String::from("run"),
        String::from("--rm"),
        String::from("--cap-drop=ALL"),
    ];
    the_arguments.extend(NO_NEW_PRIVILEGES.iter().map(|one| (*one).to_string()));
    the_arguments.push(String::from("--user"));
    the_arguments.push(the_suite.who_runs_it.clone());
    the_arguments.push(String::from("-v"));
    the_arguments.push(the_one_mount_of(where_it_lands, may_the_suite_write));
    the_arguments.push(String::from("-w"));
    the_arguments.push(String::from(WHERE_THE_CLONE_IS_INSIDE_THE_CONTAINER));
    the_arguments
}

/// The one mount, **and the write is a word on the mount and not a flag beside it.**
fn the_one_mount_of(where_it_lands: &Path, may_the_suite_write: bool) -> String {
    let the_clone = format!(
        "{}:{WHERE_THE_CLONE_IS_INSIDE_THE_CONTAINER}",
        where_it_lands.display()
    );
    if may_the_suite_write {
        return the_clone;
    }
    format!("{the_clone}{AND_MAY_ONLY_BE_READ}")
}

/// Install what the suite needs, **and this is the one step given a network.**
///
/// **A network and not none, and only here.** A suite that has never installed its dependencies
/// has never run, **so the install is the one moment the container is allowed to reach out** — and
/// the run itself, which is the part that executes the repository's own code, gets `none`.
fn to_install_what_the_suite_needs(
    the_suite: &TheSuiteARepositoryDeclares,
    where_it_lands: &Path,
) -> Vec<String> {
    let mut the_arguments =
        the_arguments_every_container_is_given(the_suite, where_it_lands, AN_INSTALL_MAY_WRITE);
    the_arguments.push(String::from("-e"));
    the_arguments.push(String::from("HOME=/tmp"));
    the_arguments.push(the_suite.the_toolchain_image.clone());
    the_arguments.push(String::from("sh"));
    the_arguments.push(String::from("-c"));
    the_arguments.push(the_suite.how_to_install.join(" "));
    the_arguments
}

/// Run a repository's own suite, **and this is the request the whole sandbox exists for.**
///
/// **`--network none` here and nowhere else.** A test suite that can open a socket can send this
/// machine's credential to a host of its choosing, **and the credential is the reason the container
/// gets no mount of the home directory in the first place** — so closing the network is what makes
/// the missing mount mean something.
fn to_run_the_suite(the_suite: &TheSuiteARepositoryDeclares, where_it_lands: &Path) -> Vec<String> {
    let mut the_arguments =
        the_arguments_every_container_is_given(the_suite, where_it_lands, A_SUITE_MAY_NOT_WRITE);
    the_arguments.push(String::from("--network"));
    the_arguments.push(String::from("none"));
    the_arguments.push(String::from("-e"));
    the_arguments.push(String::from("HOME=/tmp"));
    the_arguments.push(the_suite.the_toolchain_image.clone());
    the_arguments.push(String::from("sh"));
    the_arguments.push(String::from("-c"));
    the_arguments.push(the_suite.how_to_run.join(" "));
    the_arguments
}

/// What a commit would be run with, **staged and committed in one program and then read back.**
///
/// **`--` before the paths, because a path beginning with a dash is an option.** A plan names
/// files, **and a file named `-something` would be a flag this system passed to `git` on its own
/// behalf** — so the paths are quoted after `--` and nothing a plan supplies reaches `git` as an
/// option.
fn to_record_the_change(every_path: &[String]) -> Vec<String> {
    let mut the_arguments = vec![String::from("add"), String::from("--")];
    the_arguments.extend(every_path.iter().cloned());
    the_arguments
}

/// The commit the clone stands on, **read by name and never composed into the address.**
fn to_read_the_commit(the_clone_of: &ARepositoryToClone) -> Vec<String> {
    inside_the_clone(the_clone_of, &["rev-parse", "HEAD"])
}

/// The ref the clone stands on, **which may be a branch and may be `HEAD`.**
fn to_read_the_ref(the_clone_of: &ARepositoryToClone) -> Vec<String> {
    inside_the_clone(the_clone_of, &["rev-parse", "--abbrev-ref", "HEAD"])
}

/// What `ls-remote` would be run with, **and the branch is one whole argument.**
fn to_read_the_remote(the_clone_of: &ARepositoryToClone, which_branch: &str) -> Vec<String> {
    inside_the_clone(
        the_clone_of,
        &["ls-remote", "--heads", "origin", which_branch],
    )
}

/// What a push would be run with, **and `HEAD` rather than the commit.
///
/// **`HEAD:refs/heads/<name>` and not a local branch name**, because pushing the branch the
/// checkout is standing on would move *that* branch on the remote.
fn to_push_the_branch(the_clone_of: &ARepositoryToClone, which_branch: &str) -> Vec<String> {
    inside_the_clone(
        the_clone_of,
        &["push", "origin", &format!("HEAD:refs/heads/{which_branch}")],
    )
}

/// The arguments that name the origin of a clone, **run inside the clone that was already made.**
///
/// **And not in the match that builds every other git argument**, because adding one request to
/// `gh` had already made one function longer than a reader may hold **and the reason was three lines
/// away from where anyone would look.**
fn to_name_where_the_code_was_written_in(the_clone_of: &ARepositoryToClone) -> Vec<String> {
    inside_the_clone(
        the_clone_of,
        &["remote", "add", "upstream", &the_clone_of.fetch_from()],
    )
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
        self.asked_over_https_of(the_endpoint, the_body, &["-s"])
    }

    /// The arguments a request to the API is run with, **and they are not the model's.**
    ///
    /// **`-f` and not the model's arguments, because only a `2xx` is a pull request** and
    /// `curl` without it writes a `422` body and returns zero. **A run that read a number out of
    /// a refusal page would be recording a delivery at a link to nothing** — and the rule this
    /// repository already follows for a file is the same one, read the other way round.
    fn asked_of_the_api(&self, where_it_goes: &str, the_body: &str) -> Vec<String> {
        self.asked_over_https_of(where_it_goes, the_body, &["-s", "-f"])
    }

    /// The arguments any request over HTTPS is run with, **and the credential is one of them.**
    ///
    /// **A list and not a `String`,** because a formatted command line is a shell's opinion
    /// about what it means and there is no shell here to have that opinion.
    fn asked_over_https_of(
        &self,
        where_it_goes: &str,
        the_body: &str,
        the_flags: &[&str],
    ) -> Vec<String> {
        let Some(the_authorization) = self.the_authorization_for_a_request() else {
            return Vec::new();
        };
        let mut the_arguments: Vec<String> =
            the_flags.iter().map(|one| (*one).to_string()).collect();
        the_arguments.extend(["-X", "POST", where_it_goes].map(str::to_string));
        let the_headers = ["Content-Type: application/json", "-H", &the_authorization];
        the_arguments.extend(the_headers.map(str::to_string));
        self.each_header_a_deployment_named_into(&mut the_arguments);
        the_arguments.push(String::from("-d"));
        the_arguments.push(the_body.to_string());
        the_arguments
    }

    /// The header carrying the credential, **or nothing at all when there is no credential.**
    ///
    /// **`None` and not an empty `Authorization: Bearer `, because that is a claim about the
    /// caller that is false** and a host may answer it differently from a request that makes no
    /// claim at all.
    fn the_authorization_for_a_request(&self) -> Option<String> {
        let the_credential = self.the_credential.as_ref()?;
        let the_key = the_credential.the_key_to_authenticate_with();
        Some(format!("Authorization: Bearer {the_key}"))
    }

    /// Every header a deployment named, **each one whole and beside its own `-H`.**
    ///
    /// **A header is a name and a value and not two arguments** — and a name split from its
    /// value is a header `curl` will send to a host it was not addressed to.
    fn each_header_a_deployment_named_into(&self, the_arguments: &mut Vec<String>) {
        for one_header in &self.what_a_deployment_named_besides {
            the_arguments.push(String::from("-H"));
            the_arguments.push(one_header.clone());
        }
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
