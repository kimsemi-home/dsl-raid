use super::super::fixtures::adversarial;
use super::super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

pub(super) fn high_risk_manifest() -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["authority_gate"]["scope"] = json!("ontology");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = capacity("available", 1, 5, json!(["evidence:quality"]));
    value
}

pub(super) fn capacity(status: &str, depth: u64, max: u64, evidence: Value) -> Value {
    json!({
        "status": status,
        "queue_depth": depth,
        "max_queue_depth": max,
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "evidence": evidence
    })
}
