//! The rules: what a work item is, and which moves are legal.
//!
//! **This crate has no dependencies at all — not "few", not "only serde".** The manifest
//! has no `[dependencies]` section, and that is the one rule in this repository the compiler
//! enforces without being asked. Every other boundary rests on it: an organisation, a
//! filesystem, a clock and a process are all outside, and a domain that could reach one
//! would be a domain whose rules a model could argue with.
//!
//! Nothing here may reach for a model, a filesystem, a clock or a process. Reading the
//! repository, writing files, running tests and keeping state are all deterministic and all
//! live in `aisdlc-outside`, behind a port declared in `aisdlc-run`.
//!
//! **It also has no `async` and no `unsafe`.** The first because a domain that can suspend
//! is a domain whose invariants are checked by something other than a function returning a
//! value; the second because `unsafe_code = "forbid"` is a refusal rather than a warning,
//! and a warning can be argued with in a merge.
//!
//! The manifest's lack of a `[dependencies]` section is checked by a gate rather than trusted,
//! because a manifest is text and text is edited.

pub mod a_clone;
pub mod agent_identifier;
pub mod approval;
pub mod commissioned_request;
pub mod repository;
pub mod role;
pub mod stage;
pub mod state_machine;
pub mod the_addresses;
pub mod where_an_artifact_lands;
pub mod where_the_clone_of_a_work_item_lands;
pub mod which_organisation_reasons;
pub mod who_commissioned;
pub mod why_cloning_the_repository_failed;
pub mod work_item;
pub mod work_item_id;
