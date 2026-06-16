//! User-facing command implementations grouped by workflow.
//!
//! `main.rs` owns CLI routing. Each command module owns the details a developer
//! should only need when changing that workflow.

pub(crate) mod artifact;
pub(crate) mod compose;
pub(crate) mod coverage;
pub(crate) mod diff;
pub(crate) mod generate;
pub(crate) mod golden;
pub(crate) mod outputs;
pub(crate) mod quality;
pub(crate) mod query;
pub(crate) mod schema;
pub(crate) mod trace;
pub(crate) mod validate;
pub(crate) mod workspace;
