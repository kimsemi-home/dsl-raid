mod blocked;
mod change;
mod evidence;
mod fixture;
mod hash;
mod receipt;

use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

fn manifest() -> Value {
    base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high())
}

fn issues(value: &Value) -> Vec<String> {
    super::super::agent_run::semantic_issues(value)
}
