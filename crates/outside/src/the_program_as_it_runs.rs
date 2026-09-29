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
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// One of our own programs, run on this host.
pub struct TheProgramAsItRuns {
    which_program: WhichProgramWeRun,
}

impl TheProgramAsItRuns {
    /// Our programs, as they really are on this host.
    pub fn as_it_is_on_this_host() -> Self {
        Self {
            which_program: WhichProgramWeRun::Git,
        }
    }

    /// Which program this will run, and it is never chosen by a caller.
    pub fn which_program_it_runs(&self) -> WhichProgramWeRun {
        self.which_program
    }
}

/// The arguments of one request, one thing each and never a line.
///
/// **A list and not a `String`,** because a caller that cannot count its arguments cannot tell
/// one from three, and a formatted line is a shell's opinion about what it means.
pub fn the_arguments_of(what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> Vec<String> {
    match what_it_is_asked_to_do {
        WhatWeTellAProgramToDo::CloneTheRepository { the_clone_of } => vec![
            "clone".to_string(),
            the_clone_of.fetch_from(),
            the_clone_of.where_it_lands().display().to_string(),
        ],
        WhatWeTellAProgramToDo::TheCommitItStandsOn { the_clone_of } => vec![
            "-C".to_string(),
            the_clone_of.where_it_lands().display().to_string(),
            "rev-parse".to_string(),
            "HEAD".to_string(),
        ],
        WhatWeTellAProgramToDo::TheRefItStandsOn { the_clone_of } => vec![
            "-C".to_string(),
            the_clone_of.where_it_lands().display().to_string(),
            "rev-parse".to_string(),
            "--abbrev-ref".to_string(),
            "HEAD".to_string(),
        ],
        WhatWeTellAProgramToDo::NameWhereTheCodeWasWrittenIn { the_clone_of } => {
            vec!["nothing".to_string()]
        }
    }
}

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
        let what_it_ran = std::process::Command::new(self.which_program.name_on_the_host())
            .args(the_arguments_of(what_it_is_asked_to_do))
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
