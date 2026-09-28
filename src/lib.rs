//! The rules, each one a test that refuses to let its rule be broken.
//!
//! This package exists for two reasons, and the second is the one that is easy to forget.
//!
//! **The gates are the only code in this repository permitted to read the other crates
//! rather than call them**, and code that reads its neighbours cannot sit inside a layer — it
//! would share a dependency direction with the thing it is checking. So the gates are a
//! package of the workspace and not a part of the system.
//!
//! **A virtual workspace discovers no integration tests**, so without this package `tests/`
//! at the root is a directory cargo never looks at and `cargo test` is green while running
//! nothing. That is a green tick from a test that proved nothing, and it is the failure this
//! whole repository exists to prevent.
//!
//! It may carry dev-dependencies and no runtime dependency. Test code is allowed a crate
//! production code is not, and a test helper that production could also reach has stopped
//! being one.
//!
//! Its library will hold the code the gates share, because a test double is production code:
//! sixteen gates will call the scanner, and sixteen gates each carrying a copy is sixteen
//! names waiting to disagree. It is public so nothing has to be silenced — a private helper
//! in a test binary is reported dead the moment the test that does not use it is compiled,
//! and the only ways to stop that are `#[allow(dead_code)]`, which a gate in this repository
//! refuses, or a copy.

pub mod a_stretch_of_the_file;
pub mod reading_the_checkout;
pub mod reading_the_code;
pub mod reading_the_glossary;
pub mod scanning;
