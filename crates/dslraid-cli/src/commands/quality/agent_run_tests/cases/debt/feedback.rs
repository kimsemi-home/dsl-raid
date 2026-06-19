mod closure;
mod evidence;
mod knowledge;
mod status;

use super::super::fixtures::{base_manifest, high};
use super::fixture::closed_with;
use serde_json::{json, Value};

fn manifest(update_status: &str) -> Value {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = closed_with(json!(["evidence:quality"]), update_status);
    value
}

fn issues(value: &Value) -> Vec<String> {
    super::super::super::agent_run::semantic_issues(value)
}
