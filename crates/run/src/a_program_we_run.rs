//! A program we run ourselves, on the host, with our own arguments and our own inputs.
//!
//! **This is the first thing in this repository that executes anything**, so the shape of the
//! question is the whole of its safety: the port is asked what to do by a named value and never
//! handed a command line.
//!
//! **A closed set and not a name a caller supplies.** A port that runs whatever string it is
//! given is a port that runs whatever reaches it.
//!
//! **The working directory is a convenience and not a promise that anything may be run there.**
//! Being allowed to name a directory is not being allowed to run that directory.

use crate::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;

/// Which of our own programs to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichProgramWeRun {
    /// `git`.
    Git,
}

impl WhichProgramWeRun {
    /// Every program this system runs, and the only one it runs so far.
    pub fn every_program() -> Vec<WhichProgramWeRun> {
        vec![WhichProgramWeRun::Git]
    }

    /// The name the host knows this program by.
    pub fn name_on_the_host(&self) -> &'static str {
        match self {
            WhichProgramWeRun::Git => "git",
        }
    }
}

/// What a program said, and whether it said it successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheProgramSaid {
    did_it_work: bool,
    the_words: String,
}

impl WhatTheProgramSaid {
    /// What the program wrote, whether or not it worked.
    pub fn the_words(&self) -> &str {
        &self.the_words
    }

    /// Whether it worked, and not whether it looked happy.
    pub fn did_it_work(&self) -> bool {
        self.did_it_work
    }
}

/// Running one of our own programs on the host.
pub trait AProgramWeRun {
    /// Run one of our programs, asked to do one thing by a value and never by a command line.
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid;
}
