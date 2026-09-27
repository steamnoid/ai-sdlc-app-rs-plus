//! The rules, each one a test that refuses to let its rule be broken.
//!
//! This package exists so that `tests/` at the root of the workspace is a test target
//! cargo discovers, and so that the gates are the only code in this repository permitted
//! to **read** the other crates rather than call them.
//!
//! That distinction is the reason it is a package and not a directory inside a layer.
//! A gate that lived in `aisdlc-domain` would have to reach through the layers to see
//! whether they obeyed, and the thing being checked and the thing doing the checking
//! would share a dependency direction.
//!
//! It may carry dev-dependencies and no runtime dependency. Test code is allowed a crate
//! that production code is not, and a test helper that production could also reach is a
//! helper that has stopped being one.
//!
//! ## Why the library is not empty
//!
//! It holds the code the gates share — the scanner that tells a comment from a `//` inside
//! a string, from a lifetime, from a raw string. **That code is production code**, by the
//! same argument that a test double is production code: five gates call it, so it has
//! five callers and one name, and a second copy inside one of them would be two names
//! waiting to disagree.
//!
//! It is public, which is not a style choice. A private helper in a test binary is
//! reported as dead the moment the test that does not use it is compiled, and silencing
//! that is `#[allow(dead_code)]` — which this repository forbids. A public item in a
//! library is reachable, so nothing has to be silenced and the ban can stay absolute.

pub mod scanning;
