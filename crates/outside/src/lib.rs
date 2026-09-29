//! The implementations of the ports, and nothing else.
//!
//! Every type that crosses a boundary is written by hand here rather than derived, because
//! `aisdlc-domain` knows nothing about serialization. A wire type builds its domain value by
//! calling the same validating doors a fresh one goes through, so a checkpoint cannot say
//! something the domain refuses.
//!
//! **This is the only crate that names a concrete organisation, a filesystem or a process**,
//! and the reason it may is that every one of those is behind a port the rest of the system
//! is written against. A change here changes what the world does; a change anywhere else
//! changes what the system asks for.

pub mod the_program_as_it_runs;
pub mod the_project_on_disk;
