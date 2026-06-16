mod hash;
mod io;
mod lookup;
mod model;
mod schema;
mod subject;

pub use hash::{sha256_bytes, sha256_json};
pub use io::{canonical_json_bytes, load_core_ir, load_json_value, repo_schema_path};
pub use model::*;
pub use schema::validate_json_schema;
pub use subject::{
    action_subject, event_subject, fsm_local_name, guard_subject, state_subject, transition_subject,
};

pub const CORE_SCHEMA_PATH: &str = "schemas/dslraid-core.schema.json";
pub const VALIDATION_SCHEMA_PATH: &str = "schemas/dslraid-validation.schema.json";
pub const VIEW_SCHEMA_PATH: &str = "schemas/dslraid-view.schema.json";

#[cfg(test)]
mod tests;
