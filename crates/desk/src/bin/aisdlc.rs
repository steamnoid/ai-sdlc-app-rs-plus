//! The program a person runs: `aisdlc` with what a run needs, the process told the rest.
//!
//! **Three arguments on the command line and four variables in the process.** The arguments say
//! what to work on and where; the process says where a model answers, which one, with what key,
//! and who answers at the gates. **A refusal names what is missing**, and nothing is copied
//! before everything is read.

fn main() -> Result<(), String> {
    let what_a_person_typed = std::env::args().skip(1).collect::<Vec<String>>();
    let the_environment =
        aisdlc_outside::the_environment::TheEnvironment::reading_what_the_process_was_told();
    let mut what_this_desk_was_given =
        aisdlc_desk::what_the_process_told_this_desk::what_the_process_told_this_desk(
            &what_a_person_typed,
            &the_environment,
        )
        .map_err(|why_not| format!("{why_not}"))?;
    aisdlc_desk::the_run_is_started::the_run_is_started_with(&mut what_this_desk_was_given)
        .map(|said| println!("{said}"))
        .map_err(|why_not| format!("{why_not}"))
}
