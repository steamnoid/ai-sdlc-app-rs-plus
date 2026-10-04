//! The desk: the front door a person walks in through.
//!
//! **This crate starts runs and nothing else.** The rules are in `aisdlc-domain`, the use cases in
//! `aisdlc-run`, and the implementations of the ports in `aisdlc-outside`; **a rule that arrives here
//! is a rule with two copies**, and the copy with tests on it is not this one.

pub mod what_a_run_was_asked_for;
