//! User-facing command implementations grouped by workflow.
//!
//! `main.rs` owns CLI routing. Each command module owns the details a developer
//! should only need when changing that workflow.

pub(crate) mod artifact;
pub(crate) mod coverage;
pub(crate) mod diff;
pub(crate) mod query;
pub(crate) mod trace;
