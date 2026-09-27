//! The implementations of the ports, and nothing else.
//!
//! Every type that crosses a boundary is written by hand here rather than derived, because
//! `aisdlc-domain` knows nothing about serialization. A wire type builds its domain value
//! by calling the same validating doors a fresh one goes through, so a checkpoint cannot
//! say something the domain refuses.
