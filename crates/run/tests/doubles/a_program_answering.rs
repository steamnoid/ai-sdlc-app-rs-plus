//! A program that answers `git` and `gh`.
//!
//! **One double per file, and not one file of doubles**, because `mod` compiles a copy into every
//! test binary that names it — **so a file holding four doubles is compiled four times and the three
//! a given binary does not use are reported dead**, and the only ways to stop that are an
//! `#[allow(dead_code)]`, which a gate in this repository refuses, or a copy.

use aisdlc_run::a_program_we_run::{AProgramWeRun, WhatTheProgramSaid};
use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo;
/// A program that answers `git` and `gh`, **and refuses the pull request when told to.**
pub struct AProgramAnswering {
    refuses_the_pull_request: bool,
}

impl AProgramWeRun for AProgramAnswering {
    fn run(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> WhatTheProgramSaid {
        let (did_it_work, what_it_says) = self.what_it_says_for(what_it_is_asked_to_do);
        WhatTheProgramSaid::of(did_it_work, &what_it_says)
    }
}

impl AProgramAnswering {
    /// A program that answers everything, **and one that can be told to refuse the pull request** —
    /// because a delivery that cannot fail is a delivery no test can prove anything about.
    pub fn refusing_the_pull_request(refuses_the_pull_request: bool) -> Self {
        Self {
            refuses_the_pull_request,
        }
    }

    fn what_it_says_for(&self, what_it_is_asked_to_do: &WhatWeTellAProgramToDo) -> (bool, String) {
        match what_it_is_asked_to_do {
            WhatWeTellAProgramToDo::ReadTheRemote { .. } => (true, String::new()),
            WhatWeTellAProgramToDo::PushTheBranch { .. } => {
                (true, String::from("to ../remote.git"))
            }
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } if self.refuses_the_pull_request => {
                (false, String::from("gh: no"))
            }
            WhatWeTellAProgramToDo::OpenAPullRequest { .. } => (
                true,
                String::from(
                    r#"{"number":17,"html_url":"https://github.com/steamnoid/wellbeing-tracker-public/pull/17"}"#,
                ),
            ),
            every_other_request => (true, format!("{every_other_request:?}")),
        }
    }
}
