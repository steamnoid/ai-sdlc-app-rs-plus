//! The use cases, and the ports they are written against.
//!
//! A port is a question this crate cannot answer by itself. Each one is a trait declared
//! here and implemented in `aisdlc-outside`, so the adapters know the ports and the ports
//! know nothing about the adapters.
//!
//! There is no `dyn` in this crate. The ports are generic and `aisdlc-desk` — the
//! composition root, the one place a concrete type is named — monomorphises them.
