//! The rules, each one a test that refuses to let its rule be broken.
//!
//! This package has an empty library on purpose. It exists so that `tests/` at the
//! root of the workspace is a test target cargo discovers, and so that the gates are
//! the only code in this repository permitted to **read** the other crates rather than
//! call them.
//!
//! That distinction is the reason it is a package and not a directory inside a layer.
//! A gate that lived in `aisdlc-domain` would have to reach through the layers to see
//! whether they obeyed, and the thing being checked and the thing doing the checking
//! would share a dependency direction.
//!
//! It may carry dev-dependencies and no runtime dependency. Test code is allowed a crate
//! that production code is not, and a test helper that production could also reach is a
//! helper that has stopped being one.
