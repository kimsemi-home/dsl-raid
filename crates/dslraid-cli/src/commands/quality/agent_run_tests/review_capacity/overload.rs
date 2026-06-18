use super::super::fixtures::{base_manifest, high};
use super::super::fixtures_reviewer::adversarial;
use crate::commands::quality::agent_run;
use serde_json::{json, Value};

#[test]
fn queue_overflow_freezes_high_risk_sidecar_authority() {
    let mut value = high_risk_manifest();
    value["review_capacity"] = capacity("available", 6, 5);

    assert_eq!(
        agent_run::semantic_issues(&value),
        vec![
            "review capacity queue depth exceeds max",
            "review capacity queue overflow freezes high-risk sidecar authority"
        ]
    );
}

fn high_risk_manifest() -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["authority_gate"]["scope"] = json!("ontology");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = capacity("available", 1, 5);
    value
}

fn capacity(status: &str, depth: u64, max: u64) -> Value {
    json!({
        "status": status,
        "queue_depth": depth,
        "max_queue_depth": max,
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "evidence": ["evidence:quality"]
    })
}
