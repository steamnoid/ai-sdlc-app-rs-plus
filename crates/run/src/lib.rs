//! The use cases, and the ports they are written against.
//!
//! A port is a question this crate cannot answer by itself. Each one is a trait declared
//! here and implemented in `aisdlc-outside`, so the adapters know the ports and the ports
//! know nothing about the adapters.
//!
//! **There is no `dyn` here.** The ports are generic and the composition root — the one
//! place a concrete type is named — monomorphises them, so no trait has to be made
//! object-safe and no `async` macro has to rewrite a signature. The cost is a longer type at
//! each boundary and a louder compile error when a test double drifts from the real thing.
//!
//! Nothing here may name an organisation, a filesystem or a runtime. Those are ports, and a
//! use case that says `reqwest` instead of `AGitHubWeReadFrom` has moved a decision out of
//! the type and into a line of code.
