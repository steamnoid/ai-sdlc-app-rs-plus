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
        let _which_program = WhichProgramWeRun::Git;
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
    let _what_it_is_asked_to_do = what_it_is_asked_to_do;
    vec!["nothing".to_string()]
}

impl AProgramWeRun for TheProgramAsItRuns {
    /// Run it, with no shell, and report whether it worked and what it said.
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        let _what_it_is_asked_to_do = what_it_is_asked_to_do;
        WhatTheProgramSaid::of(true, "")
    }
}
